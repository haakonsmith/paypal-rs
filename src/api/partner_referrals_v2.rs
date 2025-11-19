//! Partner Referrals v2 API endpoints.
//!
//! Use the Partner Referrals API to add PayPal seller accounts to PayPal Complete Payments Platform
//! for Marketplaces and Platforms.
//!
//! ## Overview
//!
//! The Partner Referrals API enables platforms and marketplaces to onboard merchants to PayPal.
//! This API provides two main operations:
//!
//! 1. **Create Partner Referral** - Create a referral to onboard a new merchant
//! 2. **Show Partner Referral Details** - Retrieve details of an existing referral
//!
//! ## Usage Example
//!
//! ```rust,no_run
//! use paypal_rs::{Client, HeaderParams, PaypalEnv};
//! use paypal_rs::api::partner_referrals_v2::CreatePartnerReferral;
//! use paypal_rs::data::partner_referrals_v2::*;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Initialize the PayPal client
//! let mut client = Client::new(
//!     "client_id".to_string(),
//!     "secret".to_string(),
//!     PaypalEnv::Sandbox,
//! );
//!
//! // Get access token
//! client.get_access_token().await?;
//!
//! // Create referral data
//! let referral_data = ReferralData {
//!     email: Some("merchant@example.com".to_string()),
//!     preferred_language_code: Some("en-US".to_string()),
//!     tracking_id: Some("partner-tracking-123".to_string()),
//!     operations: vec![
//!         Operation {
//!             operation: OperationType::ApiIntegration,
//!             api_integration_preference: Some(ApiIntegrationPreference {
//!                 rest_api_integration: Some(RestApiIntegration {
//!                     integration_method: IntegrationMethod::Paypal,
//!                     integration_type: IntegrationType::ThirdParty,
//!                     first_party_details: None,
//!                     third_party_details: Some(ThirdPartyDetails {
//!                         features: Some(vec![
//!                             RestEndpointFeature::Payment,
//!                             RestEndpointFeature::Refund,
//!                         ]),
//!                     }),
//!                 }),
//!             }),
//!         },
//!     ],
//!     products: Some(vec![Product::ExpressCheckout]),
//!     legal_consents: Some(vec![
//!         LegalConsent {
//!             consent_type: LegalConsentType::ShareDataConsent,
//!             granted: true,
//!         },
//!     ]),
//!     ..Default::default()
//! };
//!
//! // Create the referral
//! let create_referral = CreatePartnerReferral::new(referral_data);
//! let response = client.execute(&create_referral).await?;
//!
//! // Extract action URL from links
//! if let Some(links) = response.links {
//!     for link in links {
//!         if link.rel == Some("action_url".to_string()) {
//!             println!("Redirect merchant to: {}", link.href);
//!         }
//!     }
//! }
//!
//! # Ok(())
//! # }
//! ```
//!
//! ## Retrieving Referral Details
//!
//! ```rust,no_run
//! use paypal_rs::api::partner_referrals_v2::ShowPartnerReferralDetails;
//! # use paypal_rs::Client;
//!
//! # async fn example(client: &mut Client) -> Result<(), Box<dyn std::error::Error>> {
//! let show_referral = ShowPartnerReferralDetails::new("REFERRAL-ID-123");
//! let referral_details = client.execute(&show_referral).await?;
//!
//! println!("Referral ID: {:?}", referral_details.partner_referral_id);
//! println!("Submitter Payer ID: {:?}", referral_details.submitter_payer_id);
//! # Ok(())
//! # }
//! ```
//!
//! ## Reference
//!
//! <https://developer.paypal.com/docs/api/partner-referrals/v2/>

use std::borrow::Cow;

use crate::{
    data::partner_referrals_v2::{CreateReferralDataResponse, ReferralData, ReferralDataResponse},
    endpoint::Endpoint,
};

/// Creates a partner referral.
///
/// Creates a partner referral that onboards sellers to PayPal.
/// The call returns a partner referral ID and an array of link objects that
/// contains `action_url`, which redirects the seller to PayPal to approve the referral.
///
/// # Example
///
/// ```rust,no_run
/// use paypal_rs::api::partner_referrals_v2::CreatePartnerReferral;
/// use paypal_rs::data::partner_referrals_v2::{ReferralData, Operation, OperationType};
///
/// let referral_data = ReferralData {
///     operations: vec![Operation {
///         operation: OperationType::ApiIntegration,
///         api_integration_preference: None,
///     }],
///     ..Default::default()
/// };
///
/// let endpoint = CreatePartnerReferral::new(referral_data);
/// ```
#[derive(Debug, Clone)]
pub struct CreatePartnerReferral {
    /// The referral data payload.
    pub referral_data: ReferralData,
}

impl CreatePartnerReferral {
    /// Creates a new CreatePartnerReferral endpoint.
    pub fn new(referral_data: ReferralData) -> Self {
        Self { referral_data }
    }
}

impl Endpoint for CreatePartnerReferral {
    type Query = ();
    type Body = ReferralData;
    type Response = CreateReferralDataResponse;

    fn relative_path(&self) -> Cow<str> {
        Cow::Borrowed("/v2/customer/partner-referrals")
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::POST
    }

    fn body(&self) -> Option<Self::Body> {
        Some(self.referral_data.clone())
    }
}

/// Shows details for a partner referral by ID.
///
/// Gets partner referral details by partner referral ID.
///
/// # Example
///
/// ```rust,no_run
/// use paypal_rs::api::partner_referrals_v2::ShowPartnerReferralDetails;
///
/// let endpoint = ShowPartnerReferralDetails::new("REFERRAL-123");
/// ```
#[derive(Debug, Clone)]
pub struct ShowPartnerReferralDetails {
    /// The partner referral ID.
    pub partner_referral_id: String,
}

impl ShowPartnerReferralDetails {
    /// Creates a new ShowPartnerReferralDetails endpoint.
    pub fn new(partner_referral_id: impl Into<String>) -> Self {
        Self {
            partner_referral_id: partner_referral_id.into(),
        }
    }
}

impl Endpoint for ShowPartnerReferralDetails {
    type Query = ();
    type Body = ();
    type Response = ReferralDataResponse;

    fn relative_path(&self) -> Cow<str> {
        Cow::Owned(format!("/v2/customer/partner-referrals/{}", self.partner_referral_id))
    }

    fn method(&self) -> reqwest::Method {
        reqwest::Method::GET
    }
}
