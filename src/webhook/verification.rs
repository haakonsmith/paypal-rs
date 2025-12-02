//! This module provides the ability to cryptographically verify the webhook messages
//!
//! The main entry point is this: [verify_paypal_webhook_signature]
//!
//! This will download and cache certificates using a lru cache.

use std::num::NonZeroUsize;

use base64::{DecodeError, Engine};
use lru::LruCache;
use rsa::signature::Verifier;
use rsa::{
    pkcs1::DecodeRsaPublicKey,
    pkcs1v15::{Signature, VerifyingKey},
    RsaPublicKey,
};
use sha2::Sha256;
use x509_parser::prelude::FromDer;

/// Errors that can occur during webhook signature validation.
///
/// These errors indicate problems with the cryptographic verification process itself,
/// not with fetching or parsing the certificate.
#[derive(Debug, thiserror::Error)]
pub enum PayPalWebhookValidationError {
    /// PayPal currently only supports `SHA256withRSA` for webhook signatures.
    #[error("Unsupported authentication algorithm {0}")]
    UnsupportedAuthAlgo(String),
    /// The `paypal-transmission-sig` header value is not valid base64.
    #[error("Signature could not be b64 decoded {0}")]
    InvalidSignatureB64(#[from] DecodeError),
    /// The decoded signature bytes could not be parsed as an RSA signature.
    #[error("Signature could not be decoded {0}")]
    InvalidSignature(#[from] signature::Error),
}

/// Errors that can occur when loading or parsing PayPal's signing certificate.
///
/// The certificate URL is provided in the `paypal-cert-url` header and must point
/// to a valid PayPal domain (`api.paypal.com` or `api.sandbox.paypal.com`).
#[derive(Debug, thiserror::Error)]
pub enum PayPalWebhookCertificateError {
    /// Failed to fetch the certificate from PayPal's servers.
    #[error("Failed to fetch {0}")]
    Reqwest(#[from] reqwest::Error),
    /// The certificate URL does not point to a valid PayPal domain.
    /// This is a security check to prevent certificate spoofing attacks.
    #[error("Invalid Certificate URL {0}")]
    InvalidCertificateUrl(String),
    /// The certificate data is not valid PEM format.
    #[error("Failed to Deserialise Certificate {0}")]
    PEMLoading(#[from] pem::PemError),
    /// The certificate does not contain a valid RSA public key.
    #[error("Invalid Certificate {0}")]
    RSA(#[from] rsa::pkcs1::Error),
    /// The certificate is not a valid X.509 certificate.
    #[error("Invalid Certificate {0}")]
    X509Nom(#[from] x509_parser::nom::Err<x509_parser::error::X509Error>),
}

/// Combined error type for the full verification flow (certificate loading + signature validation).
///
/// Returned by [`verify_paypal_webhook_signature`] which handles both certificate
/// fetching and signature verification.
#[derive(Debug, thiserror::Error)]
pub enum PayPalWebhookValidationCertError {
    /// An error occurred while fetching or parsing the certificate.
    #[error(transparent)]
    Certificate(#[from] PayPalWebhookCertificateError),
    /// An error occurred during signature validation.
    #[error(transparent)]
    Validation(#[from] PayPalWebhookValidationError),
}

/// Parameters extracted from PayPal webhook HTTP headers required for signature verification.
///
/// PayPal sends these values as HTTP headers with each webhook delivery. The signature
/// is computed over: `{transmission_id}|{transmission_time}|{webhook_id}|{crc32(body)}`.
pub struct WebhookParams {
    /// Unique ID of the transmission. Header: `paypal-transmission-id`
    pub transmission_id: String,
    /// ISO 8601 timestamp of when the message was sent. Header: `paypal-transmission-time`
    pub transmission_time: String,
    /// Base64-encoded RSA signature. Header: `paypal-transmission-sig`
    pub transmission_sig: String,
    /// Signing algorithm (must be `SHA256withRSA`). Header: `paypal-auth-algo`
    pub auth_algo: String,
}

/// Verifies a PayPal webhook signature using a pre-loaded verification key.
///
/// Use this function when you want to manage certificate caching yourself.
/// For automatic certificate fetching and caching, use [`verify_paypal_webhook_signature`].
///
/// Returns `Ok(true)` if the signature is valid, `Ok(false)` if verification failed
/// (signature mismatch), or `Err` if the inputs are malformed.
///
/// # Arguments
/// * `params` - Header values from the webhook request
/// * `body` - The raw request body (must be the exact bytes received, not re-serialized)
/// * `webhook_id` - Your webhook's ID from PayPal's Application Management dashboard
/// * `verifying_key` - RSA public key extracted from PayPal's certificate
#[tracing::instrument(skip_all)]
pub fn verify_paypal_webhook_signature_with_key(
    WebhookParams {
        transmission_id,
        transmission_time,
        transmission_sig,
        auth_algo,
    }: WebhookParams,
    body: &str,
    webhook_id: &str,
    verifying_key: &VerifyingKey<Sha256>,
) -> Result<bool, PayPalWebhookValidationError> {
    // Verify algorithm
    if auth_algo != "SHA256withRSA" {
        return Err(PayPalWebhookValidationError::UnsupportedAuthAlgo(auth_algo));
    }

    // Create expected signature string with CRC32 of body
    let crc = crc32fast::hash(body.as_bytes());
    let expected_sig = format!("{}|{}|{}|{}", transmission_id, transmission_time, webhook_id, crc);

    // Decode signature from base64
    let signature_bytes = base64::engine::general_purpose::STANDARD
        .decode(&transmission_sig)
        .inspect_err(|e| tracing::error!(?e, transmission_sig, "Failed to decode signature"))?;

    let signature = Signature::try_from(signature_bytes.as_slice())
        .inspect_err(|e| tracing::error!(?e, "Failed to parse signature"))?;

    match verifying_key.verify(expected_sig.as_bytes(), &signature) {
        Ok(_) => {
            tracing::debug!("PayPal webhook signature verified successfully");
            Ok(true)
        }
        Err(e) => {
            println!("Failed to validate {e:?}");
            tracing::warn!("PayPal webhook signature verification failed: {}", e);
            Ok(false)
        }
    }
}

fn extract_verifying_key_from_pem(cert_pem: &str) -> Result<VerifyingKey<Sha256>, PayPalWebhookCertificateError> {
    let cert = pem::parse(cert_pem)?;

    let (_, cert) = x509_parser::certificate::X509Certificate::from_der(cert.contents())?;

    let spki = cert.public_key();

    // Parse as RSA public key
    let public_key = RsaPublicKey::from_pkcs1_der(&spki.subject_public_key.data)
        .inspect_err(|e| tracing::error!("Failed to extract RSA public key: {}", e))?;

    let verifying_key: VerifyingKey<Sha256> = rsa::pkcs1v15::VerifyingKey::new(public_key);

    Ok(verifying_key)
}

/// Fetches and parses PayPal's signing certificate to extract the RSA public key.
///
/// The certificate URL must be from a valid PayPal domain (`api.paypal.com` or
/// `api.sandbox.paypal.com`). This is enforced as a security measure to prevent
/// attackers from substituting their own certificates.
///
/// Consider using [`verify_paypal_webhook_signature`] instead, which handles
/// certificate caching automatically.
#[tracing::instrument]
pub async fn load_verification_key(cert_url: &str) -> Result<VerifyingKey<Sha256>, PayPalWebhookCertificateError> {
    // Verify cert URL is from PayPal
    if !cert_url.starts_with("https://api.paypal.com/") && !cert_url.starts_with("https://api.sandbox.paypal.com/") {
        return Err(PayPalWebhookCertificateError::InvalidCertificateUrl(
            cert_url.to_owned(),
        ));
    }

    // Fetch certificate from PayPal
    let response = reqwest::get(cert_url).await?;

    let cert_pem = response.text().await?;

    println!("{cert_pem}");

    let verifying_key = extract_verifying_key_from_pem(&cert_pem)?;

    Ok(verifying_key)
}

/// This is the size of LRU cache. E.g. the number of certificates that will be remembered.
pub const LRU_CACHE_SIZE: NonZeroUsize = NonZeroUsize::new(10).unwrap();

/// This uses an LRU cache, which may be massively overkill for this.
async fn fetch_or_load_verification_key(cert_url: &str) -> Result<VerifyingKey<Sha256>, PayPalWebhookCertificateError> {
    use std::sync::Arc;
    use std::sync::Mutex;

    static CERT_CACHE: std::sync::LazyLock<Arc<Mutex<LruCache<String, VerifyingKey<Sha256>>>>> =
        std::sync::LazyLock::new(|| Arc::new(Mutex::new(LruCache::new(LRU_CACHE_SIZE))));

    {
        let mut cache = CERT_CACHE.lock().unwrap_or_else(|err| err.into_inner());

        if let Some(cert_pem) = cache.get(cert_url) {
            return Ok(cert_pem.clone());
        }
    }
    let verification_key = load_verification_key(&cert_url).await?;

    let mut cache = CERT_CACHE.lock().unwrap_or_else(|err| err.into_inner());

    cache.put(cert_url.to_owned(), verification_key.clone());

    Ok(verification_key)
}

/// Verifies a PayPal webhook signature, automatically fetching and caching the certificate.
///
/// This is the recommended entry point for webhook verification. It handles:
/// - Fetching the signing certificate from PayPal (URL from `paypal-cert-url` header)
/// - Caching certificates in an LRU cache (up to 10 entries)
/// - Verifying the RSA signature matches the expected message
///
/// # Arguments
/// * `params` - Header values from the webhook request
/// * `cert_url` - Certificate URL from the `paypal-cert-url` header
/// * `body` - The raw request body (must be the exact bytes received, not re-serialized)
/// * `webhook_id` - Your webhook's ID from PayPal's Application Management dashboard
///
/// # Note
/// When using PayPal's Webhook Simulator for testing, use the literal string `"WEBHOOK_ID"`
/// as the webhook ID, not your actual webhook ID.
#[tracing::instrument(skip_all)]
pub async fn verify_paypal_webhook_signature(
    params: WebhookParams,
    cert_url: &str,
    body: &str,
    webhook_id: &str,
) -> Result<bool, PayPalWebhookValidationCertError> {
    let key = fetch_or_load_verification_key(cert_url).await?;

    let verified = verify_paypal_webhook_signature_with_key(params, body, webhook_id, &key)?;

    Ok(verified)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PEM: &str = r#"-----BEGIN CERTIFICATE-----
    MIIHXTCCBkWgAwIBAgIQDki0JdJMoAIx2jGAwTcTiTANBgkqhkiG9w0BAQsFADB1
    MQswCQYDVQQGEwJVUzEVMBMGA1UEChMMRGlnaUNlcnQgSW5jMRkwFwYDVQQLExB3
    d3cuZGlnaWNlcnQuY29tMTQwMgYDVQQDEytEaWdpQ2VydCBTSEEyIEV4dGVuZGVk
    IFZhbGlkYXRpb24gU2VydmVyIENBMB4XDTI1MDEzMTAwMDAwMFoXDTI2MDMwMzIz
    NTk1OVowgdsxEzARBgsrBgEEAYI3PAIBAxMCVVMxGTAXBgsrBgEEAYI3PAIBAhMI
    RGVsYXdhcmUxHTAbBgNVBA8MFFByaXZhdGUgT3JnYW5pemF0aW9uMRAwDgYDVQQF
    EwczMDE0MjY3MQswCQYDVQQGEwJVUzETMBEGA1UECBMKQ2FsaWZvcm5pYTERMA8G
    A1UEBxMIU2FuIEpvc2UxFTATBgNVBAoTDFBheVBhbCwgSW5jLjEsMCoGA1UEAxMj
    bWVzc2FnZXZlcmlmaWNhdGlvbmNlcnRzLnBheXBhbC5jb20wggEiMA0GCSqGSIb3
    DQEBAQUAA4IBDwAwggEKAoIBAQCKsBDJSiEyFRDXhgYqSqGcRSlZ44O7iVHNjd3P
    QiBc00kI4YwT4bZIGEa08QGRB+5xRLQDtmvTnQkz60YOFxwPaSZVdUEjybUCbbTu
    TNJ117mK2V6G3KrMsXo4OZIv/oG8ayf9T6+ocRFB4s1IDHGGZJcbjgFjkq+5+3+N
    aLATY9RHF3/qkq2RMFxCPqVQ/LSFdsEdkN4Q6FKWMYPTlScdTP1dg2YY6RdBXABP
    M6DFEC0c+plO0RG4UsrATsnLQ0b4gN2cTb4JOwZaJsG2BSlpQCJWoX4gCP3Pkl2I
    eLxjpaUDsfkdOMY78kUugZ60CPu29WYHlrLCBs5tSJ+rCHYxAgMBAAGjggOAMIID
    fDAfBgNVHSMEGDAWgBQ901Cl1qCt7vNKYApl0yHU+PjWDzAdBgNVHQ4EFgQUIeCu
    2GrxDXFuVkmB3JWC2psKP7swLgYDVR0RBCcwJYIjbWVzc2FnZXZlcmlmaWNhdGlv
    bmNlcnRzLnBheXBhbC5jb20wSgYDVR0gBEMwQTALBglghkgBhv1sAgEwMgYFZ4EM
    AQEwKTAnBggrBgEFBQcCARYbaHR0cDovL3d3dy5kaWdpY2VydC5jb20vQ1BTMA4G
    A1UdDwEB/wQEAwIFoDAdBgNVHSUEFjAUBggrBgEFBQcDAQYIKwYBBQUHAwIwdQYD
    VR0fBG4wbDA0oDKgMIYuaHR0cDovL2NybDMuZGlnaWNlcnQuY29tL3NoYTItZXYt
    c2VydmVyLWczLmNybDA0oDKgMIYuaHR0cDovL2NybDQuZGlnaWNlcnQuY29tL3No
    YTItZXYtc2VydmVyLWczLmNybDCBiAYIKwYBBQUHAQEEfDB6MCQGCCsGAQUFBzAB
    hhhodHRwOi8vb2NzcC5kaWdpY2VydC5jb20wUgYIKwYBBQUHMAKGRmh0dHA6Ly9j
    YWNlcnRzLmRpZ2ljZXJ0LmNvbS9EaWdpQ2VydFNIQTJFeHRlbmRlZFZhbGlkYXRp
    b25TZXJ2ZXJDQS5jcnQwDAYDVR0TAQH/BAIwADCCAX0GCisGAQQB1nkCBAIEggFt
    BIIBaQFnAHYADleUvPOuqT4zGyyZB7P3kN+bwj1xMiXdIaklrGHFTiEAAAGUu81a
    zwAABAMARzBFAiEA4FRk0fnMLk50tTYCr1yXMwBDP1/7VZ/8xAALIYpW7WwCICYM
    e99wI7JAzkzMyzoqwJm5/vFOrb1VKQqpyWpFHquKAHYAZBHEbKQS7KeJHKICLgC8
    q08oB9QeNSer6v7VA8l9zfAAAAGUu81a9gAABAMARzBFAiEA217qTUrcveLoXTGe
    LeI8glW2dalUr2GzbPnfwycRE4wCIAXNh7RdTW50P2zHQGkmak7NYJ7ZxvPBkiNj
    Fz2RGtY9AHUASZybad4dfOz8Nt7Nh2SmuFuvCoeAGdFVUvvp6ynd+MMAAAGUu81b
    DQAABAMARjBEAiA0YxvSEpLzpohHg6zH1RC15xXTYn0Ik2SZ+R+v1XRScgIgfo5x
    Og2qNi101qUvcuYUl9fFFol2aurZ/K23bWWUr50wDQYJKoZIhvcNAQELBQADggEB
    AJDE7+ZogRtnY/SyNPOQDKSoowrDN6PE8eVXf2AIROyMdayOIaX74FRf2bTrUxIc
    J3Dkdk1aFY/sqCq52ACB15iBAiDvamS4XuYYy0mbbZX8iQeQ0uvuPA/D2sH4gpEv
    sBHHcLTfmkxL3BUTRh0JaTWhuGY9OSf5Vtl+Vt6JKEARw8br7SSc0SIz03NH9aKc
    S3fVuCsbw1tbiqMtBHgPJ60EHWbbWzae9bqFPfTCAXvDpCi33vj4l1Am6i0kOmp4
    2/CV4XomE/7JPPm+5odijca0+/6jQpVg9z/W12mOn08ykrL7lS7IpaSjiC3xMeeR
    DMCtxZDITPKCPnbzgzl2Q/I=
    -----END CERTIFICATE-----"#;

    #[tokio::test]
    async fn test_paypal_webhook_event_validation() {
        //"paypal-transmission-time": "2025-11-28T10:00:24Z", "paypal-auth-version": "v2", "paypal-cert-url": "https://api.paypal.com/v1/notifications/certs/CERT-360caa42-fca2a594-b0d12406", "paypal-auth-algo": "SHA256withRSA", "paypal-transmission-sig": "De1vvm+9LQDFQgKZ7leyYaVaAbkuXzYJOmH5FuHFxUFF+BP3DUiNwF7IF/tWhdC0SQ1EZgsRmGmlO9+5uk6UWP5i7O7jaiwNOdHbb878uOhTKL0KhWMillfQi096lrM7oZL6R/HmSZcKfBfnkH0TN2g0gHcw8NhM82tBdRsc9lbzhmIlWXoz5lZc5N9YVcaC62hQNLPCJFPYMTE4qE3qQB8jOFDW2/QGOnM4FvwwL+6rfIOdNPSqarsw3Wgh3ByIFrkBO5kbxo7uyd4Rvce4lyHmkqnschdRtFdScjxiQrmf7akmX1qWv2Y68ht69j/De7De/MOVZ/JA1t9RP+ysIA==", "paypal-transmission-id": "0f14627d-cc41-11f0-9ad0-21cf84660aee", "correlation-id": "a8f0305f4a08a"}

        let body = r#"{"id":"WH-58D329510W468432D-8HN650336L201105X","event_version":"1.0","create_time":"2019-02-14T21:50:07.940Z","resource_type":"capture","resource_version":"2.0","event_type":"PAYMENT.CAPTURE.COMPLETED","summary":"Payment completed for $ 30.0 USD","resource":{"id":"12A34567BC123456S","amount":{"currency_code":"USD","value":"30.00"},"final_capture":true,"seller_protection":{"status":"ELIGIBLE","dispute_categories":["ITEM_NOT_RECEIVED","UNAUTHORIZED_TRANSACTION"]},"disbursement_mode":"INSTANT","seller_receivable_breakdown":{"gross_amount":{"currency_code":"USD","value":"30.00"},"paypal_fee":{"currency_code":"USD","value":"1.54"},"platform_fees":[{"amount":{"currency_code":"USD","value":"2.00"},"payee":{"merchant_id":"ABCDEFGHIJKL1"}}],"net_amount":{"currency_code":"USD","value":"26.46"}},"invoice_id":"5840243-146","status":"COMPLETED","supplementary_data":{"related_ids":{"order_id":"1AB234567A1234567"}},"create_time":"2022-08-23T18:29:50Z","update_time":"2022-08-23T18:29:50Z","links":[{"href":"https://api.paypal.com/v2/payments/captures/12A34567BC123456S","rel":"self","method":"GET"},{"href":"https://api.paypal.com/v2/payments/captures/12A34567BC123456S/refund","rel":"refund","method":"POST"},{"href":"https://api.paypal.com/v2/checkout/orders/1AB234567A1234567","rel":"up","method":"GET"}]},"links":[{"href":"https://api.paypal.com/v1/notifications/webhooks-events/WH-58D329510W468432D-8HN650336L201105X","rel":"self","method":"GET"},{"href":"https://api.paypal.com/v1/notifications/webhooks-events/WH-58D329510W468432D-8HN650336L201105X/resend","rel":"resend","method":"POST"}]}"#;

        let verifying_key = extract_verifying_key_from_pem(TEST_PEM).unwrap();

        // Note: PayPal's Webhook Simulator uses the literal string "WEBHOOK_ID" as the webhook ID
        // when generating signatures, not your actual webhook ID. This is documented at:
        // https://developer.paypal.com/api/rest/webhooks/rest/
        let x = verify_paypal_webhook_signature_with_key(
            WebhookParams {
                transmission_id: "0f14627d-cc41-11f0-9ad0-21cf84660aee".into(),
                transmission_time: "2025-11-28T10:00:24Z".into(),
                transmission_sig: "De1vvm+9LQDFQgKZ7leyYaVaAbkuXzYJOmH5FuHFxUFF+BP3DUiNwF7IF/tWhdC0SQ1EZgsRmGmlO9+5uk6UWP5i7O7jaiwNOdHbb878uOhTKL0KhWMillfQi096lrM7oZL6R/HmSZcKfBfnkH0TN2g0gHcw8NhM82tBdRsc9lbzhmIlWXoz5lZc5N9YVcaC62hQNLPCJFPYMTE4qE3qQB8jOFDW2/QGOnM4FvwwL+6rfIOdNPSqarsw3Wgh3ByIFrkBO5kbxo7uyd4Rvce4lyHmkqnschdRtFdScjxiQrmf7akmX1qWv2Y68ht69j/De7De/MOVZ/JA1t9RP+ysIA==".into(),
                auth_algo: "SHA256withRSA".into(),
            },
            body,
            "WEBHOOK_ID",
            &verifying_key,
        )
        .unwrap();

        assert!(x, "certificate is not valid");
    }
}
