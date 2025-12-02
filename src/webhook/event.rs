//! This is the event types of PayPal webhook
//!
//! Related: [PayPal documentation](https://developer.paypal.com/api/rest/webhooks/event-names/)
//! Also related: [PayPal documentation](https://docs.paypal.ai/reference/webhook-events/webhook-format)

use serde::{Deserialize, Serialize};

/// Presently this is all of the event types we accept
///
/// Related: [PayPal documentation](https://developer.paypal.com/api/rest/webhooks/event-names/)
/// Also related: [PayPal documentation](https://docs.paypal.ai/reference/webhook-events/webhook-format)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PayPalEventType {
    // === Payments ===
    /// A payment authorization is created, approved, executed, or a future payment authorization is created.
    ///
    /// Related: [Capture authorized payment](https://developer.paypal.com/docs/api/payments/v2/#authorizations_capture)
    #[serde(rename = "PAYMENT.AUTHORIZATION.CREATED")]
    PaymentAuthorizationCreated,
    /// A payment authorization is voided either due to authorization reaching its 30 day validity period
    /// or authorization was manually voided using the Void Authorized Payment API.
    ///
    /// Related: [Show details for authorized payment](https://developer.paypal.com/docs/api/payments/v2/#authorizations_get)
    #[serde(rename = "PAYMENT.AUTHORIZATION.VOIDED")]
    PaymentAuthorizationVoided,
    /// A payment capture completes.
    ///
    /// Related: [Capture authorized payment](https://developer.paypal.com/docs/api/payments/v2/#authorizations_capture)
    #[serde(rename = "PAYMENT.CAPTURE.COMPLETED")]
    PaymentCaptureCompleted,
    /// A payment capture is declined.
    ///
    /// Related: [Capture authorized payment](https://developer.paypal.com/docs/api/payments/v2/#authorizations_capture)
    #[serde(rename = "PAYMENT.CAPTURE.DECLINED")]
    PaymentCaptureDeclined,
    /// A payment capture is denied.
    ///
    /// Related: [Show captured payment details](https://developer.paypal.com/docs/api/payments/v2/#captures_get)
    #[serde(rename = "PAYMENT.CAPTURE.DENIED")]
    PaymentCaptureDenied,
    /// The state of a payment capture changes to pending.
    ///
    /// Related: [Show details for authorized payment](https://developer.paypal.com/docs/api/payments/v2/#authorizations_get)
    #[serde(rename = "PAYMENT.CAPTURE.PENDING")]
    PaymentCapturePending,
    /// A merchant refunds a payment capture.
    ///
    /// Related: [Refund captured payment](https://developer.paypal.com/docs/api/payments/v2/#captures_refund)
    #[serde(rename = "PAYMENT.CAPTURE.REFUNDED")]
    PaymentCaptureRefunded,
    /// PayPal reverses a payment capture.
    ///
    /// Related: [Refund captured payment](https://developer.paypal.com/docs/api/payments/v2/#captures_refund)
    #[serde(rename = "PAYMENT.CAPTURE.REVERSED")]
    PaymentCaptureReversed,

    // === Batch Payouts ===
    /// A batch payout payment is denied.
    ///
    /// Related: [Show payout details](https://developer.paypal.com/docs/api/payments.payouts-batch/v1/#payouts_get)
    #[serde(rename = "PAYMENT.PAYOUTSBATCH.DENIED")]
    PaymentPayoutsBatchDenied,
    /// The state of a batch payout payment changes to processing.
    ///
    /// Related: [Show payout details](https://developer.paypal.com/docs/api/payments.payouts-batch/v1/#payouts_get)
    #[serde(rename = "PAYMENT.PAYOUTSBATCH.PROCESSING")]
    PaymentPayoutsBatchProcessing,
    /// A batch payout payment completes successfully.
    ///
    /// Related: [Show payout details](https://developer.paypal.com/docs/api/payments.payouts-batch/v1/#payouts_get)
    #[serde(rename = "PAYMENT.PAYOUTSBATCH.SUCCESS")]
    PaymentPayoutsBatchSuccess,
    /// A payouts item is blocked.
    ///
    /// Related: [Show payout item details](https://developer.paypal.com/docs/api/payments.payouts-batch/v1/#payouts-item_get)
    #[serde(rename = "PAYMENT.PAYOUTS-ITEM.BLOCKED")]
    PaymentPayoutsItemBlocked,
    /// A payouts item is canceled.
    ///
    /// Related: [Cancel unclaimed payout item](https://developer.paypal.com/docs/api/payments.payouts-batch/v1/#payouts-item_cancel)
    #[serde(rename = "PAYMENT.PAYOUTS-ITEM.CANCELED")]
    PaymentPayoutsItemCanceled,
    /// A payouts item is denied.
    ///
    /// Related: [Show payout item details](https://developer.paypal.com/docs/api/payments.payouts-batch/v1/#payouts-item_get)
    #[serde(rename = "PAYMENT.PAYOUTS-ITEM.DENIED")]
    PaymentPayoutsItemDenied,
    /// A payouts item fails.
    ///
    /// Related: [Show payout item details](https://developer.paypal.com/docs/api/payments.payouts-batch/v1/#payouts-item_get)
    #[serde(rename = "PAYMENT.PAYOUTS-ITEM.FAILED")]
    PaymentPayoutsItemFailed,
    /// A payouts item is held.
    ///
    /// Related: [Show payout item details](https://developer.paypal.com/docs/api/payments.payouts-batch/v1/#payouts-item_get)
    #[serde(rename = "PAYMENT.PAYOUTS-ITEM.HELD")]
    PaymentPayoutsItemHeld,
    /// A payouts item is refunded.
    ///
    /// Related: [Show payout item details](https://developer.paypal.com/docs/api/payments.payouts-batch/v1/#payouts-item_get)
    #[serde(rename = "PAYMENT.PAYOUTS-ITEM.REFUNDED")]
    PaymentPayoutsItemRefunded,
    /// A payouts item is returned.
    ///
    /// Related: [Show payout item details](https://developer.paypal.com/docs/api/payments.payouts-batch/v1/#payouts-item_get)
    #[serde(rename = "PAYMENT.PAYOUTS-ITEM.RETURNED")]
    PaymentPayoutsItemReturned,
    /// A payouts item succeeds.
    ///
    /// Related: [Show payout item details](https://developer.paypal.com/docs/api/payments.payouts-batch/v1/#payouts-item_get)
    #[serde(rename = "PAYMENT.PAYOUTS-ITEM.SUCCEEDED")]
    PaymentPayoutsItemSucceeded,
    /// A payouts item is unclaimed.
    ///
    /// Related: [Show payout item details](https://developer.paypal.com/docs/api/payments.payouts-batch/v1/#payouts-item_get)
    #[serde(rename = "PAYMENT.PAYOUTS-ITEM.UNCLAIMED")]
    PaymentPayoutsItemUnclaimed,

    // === Billing Plans & Subscriptions ===
    /// A billing plan is created.
    ///
    /// Related: [Create plan](https://developer.paypal.com/docs/api/subscriptions/v1/#plans_create)
    #[serde(rename = "BILLING.PLAN.CREATED")]
    BillingPlanCreated,
    /// A billing plan is updated.
    ///
    /// Related: [Update plan](https://developer.paypal.com/docs/api/subscriptions/v1/#plans_patch)
    #[serde(rename = "BILLING.PLAN.UPDATED")]
    BillingPlanUpdated,
    /// A billing plan is activated.
    ///
    /// Related: [Activate plan](https://developer.paypal.com/docs/api/subscriptions/v1/#plans_activate)
    #[serde(rename = "BILLING.PLAN.ACTIVATED")]
    BillingPlanActivated,
    /// A billing plan is deactivated.
    ///
    /// Related: [Deactivate plan](https://developer.paypal.com/docs/api/subscriptions/v1/#plans_deactivate)
    #[serde(rename = "BILLING.PLAN.DEACTIVATED")]
    BillingPlanDeactivated,
    /// A price change for the plan is activated.
    ///
    /// Related: [Update pricing](https://developer.paypal.com/docs/api/subscriptions/v1/#plans_update-pricing-schemes)
    #[serde(rename = "BILLING.PLAN.PRICING-CHANGE.ACTIVATED")]
    BillingPlanPricingChangeActivated,
    /// A subscription is created.
    ///
    /// Related: [Create subscription](https://developer.paypal.com/docs/api/subscriptions/v1/#subscriptions_create)
    #[serde(rename = "BILLING.SUBSCRIPTION.CREATED")]
    BillingSubscriptionCreated,
    /// A subscription is activated.
    ///
    /// Related: [Activate subscription](https://developer.paypal.com/docs/api/subscriptions/v1/#subscriptions_activate)
    #[serde(rename = "BILLING.SUBSCRIPTION.ACTIVATED")]
    BillingSubscriptionActivated,
    /// A subscription is updated.
    ///
    /// Related: [Update subscription](https://developer.paypal.com/docs/api/subscriptions/v1/#subscriptions_patch)
    #[serde(rename = "BILLING.SUBSCRIPTION.UPDATED")]
    BillingSubscriptionUpdated,
    /// A subscription expires.
    ///
    /// Related: [Show subscription details](https://developer.paypal.com/docs/api/subscriptions/v1/#subscriptions_get)
    #[serde(rename = "BILLING.SUBSCRIPTION.EXPIRED")]
    BillingSubscriptionExpired,
    /// A subscription is cancelled.
    ///
    /// Related: [Cancel subscription](https://developer.paypal.com/docs/api/subscriptions/v1/#subscriptions_cancel)
    #[serde(rename = "BILLING.SUBSCRIPTION.CANCELLED")]
    BillingSubscriptionCancelled,
    /// A subscription is suspended.
    ///
    /// Related: [Suspend subscription](https://developer.paypal.com/docs/api/subscriptions/v1/#subscriptions_suspend)
    #[serde(rename = "BILLING.SUBSCRIPTION.SUSPENDED")]
    BillingSubscriptionSuspended,
    /// A billing agreement is re-activated.
    ///
    /// Related: [Activate subscription](https://developer.paypal.com/docs/api/subscriptions/v1/#subscriptions_activate)
    #[serde(rename = "BILLING.SUBSCRIPTION.RE-ACTIVATED")]
    BillingSubscriptionReActivated,
    /// Payment failed on subscription.
    ///
    /// Related: [Show subscription details](https://developer.paypal.com/docs/api/subscriptions/v1/#subscriptions_get)
    #[serde(rename = "BILLING.SUBSCRIPTION.PAYMENT.FAILED")]
    BillingSubscriptionPaymentFailed,

    // === Catalog (Products) ===
    /// A product is created.
    ///
    /// Related: [Create product](https://developer.paypal.com/docs/api/catalog-products/v1/#products_create)
    #[serde(rename = "CATALOG.PRODUCT.CREATED")]
    CatalogProductCreated,
    /// A product is updated.
    ///
    /// Related: [Update product](https://developer.paypal.com/docs/api/catalog-products/v1/#products_patch)
    #[serde(rename = "CATALOG.PRODUCT.UPDATED")]
    CatalogProductUpdated,

    // === Checkout & Orders ===
    /// A buyer approved a checkout order.
    ///
    /// Related: [Orders API](https://developer.paypal.com/docs/api/orders/v2/)
    #[serde(rename = "CHECKOUT.ORDER.APPROVED")]
    CheckoutOrderApproved,
    /// A checkout order is completed. For use by marketplaces and platforms only.
    ///
    /// Related: [Orders API](https://developer.paypal.com/docs/api/orders/v2/)
    #[serde(rename = "CHECKOUT.ORDER.COMPLETED")]
    CheckoutOrderCompleted,
    /// A checkout order is processed.
    ///
    /// Related: [Orders API](https://developer.paypal.com/docs/api/orders/v2/)
    #[serde(rename = "CHECKOUT.ORDER.PROCESSED")]
    CheckoutOrderProcessed,
    /// Express checkout payment is created and approved by buyer.
    #[serde(rename = "CHECKOUT.CHECKOUT.BUYER-APPROVED")]
    CheckoutBuyerApproved,
    /// A problem occurred after the buyer approved the order but before you captured the payment.
    ///
    /// Related: [Handle uncaptured payments](https://developer.paypal.com/docs/checkout/standard/customize/handle-uncaptured-payments/)
    #[serde(rename = "CHECKOUT.PAYMENT-APPROVAL.REVERSED")]
    CheckoutPaymentApprovalReversed,
    /// Checkout payment is created and approved by buyer.
    #[serde(rename = "PAYMENTS.PAYMENT.CREATED")]
    PaymentsPaymentCreated,
    /// A payment order is canceled.
    ///
    /// Related: [Void order](https://developer.paypal.com/docs/api/orders/v2/#orders_void)
    #[serde(rename = "PAYMENT.ORDER.CANCELLED")]
    PaymentOrderCancelled,
    /// A payment order is created.
    ///
    /// Related: [Create order](https://developer.paypal.com/docs/api/orders/v2/#orders_create)
    #[serde(rename = "PAYMENT.ORDER.CREATED")]
    PaymentOrderCreated,

    // === Sales ===
    /// A sale completes.
    ///
    /// Related: [Show sale details](https://developer.paypal.com/docs/api/payments/v1/#sale_get)
    #[serde(rename = "PAYMENT.SALE.COMPLETED")]
    PaymentSaleCompleted,
    /// The state of a sale changes from pending to denied.
    ///
    /// Related: [Show sale details](https://developer.paypal.com/docs/api/payments/v1/#sale_get)
    #[serde(rename = "PAYMENT.SALE.DENIED")]
    PaymentSaleDenied,
    /// The state of a sale changes to pending.
    ///
    /// Related: [Show sale details](https://developer.paypal.com/docs/api/payments/v1/#sale_get)
    #[serde(rename = "PAYMENT.SALE.PENDING")]
    PaymentSalePending,
    /// A merchant refunds a sale.
    ///
    /// Related: [Refund sale](https://developer.paypal.com/docs/api/payments/v1/#sale_refund)
    #[serde(rename = "PAYMENT.SALE.REFUNDED")]
    PaymentSaleRefunded,
    /// PayPal reverses a sale.
    ///
    /// Related: [Refund sale](https://developer.paypal.com/docs/api/payments/v1/#sale_refund)
    #[serde(rename = "PAYMENT.SALE.REVERSED")]
    PaymentSaleReversed,

    // === Referenced Payouts ===
    /// Funds are disbursed to the seller and partner.
    ///
    /// Related: [Create referenced payout item](https://developer.paypal.com/docs/api/referenced-payouts/v1/#referenced-payouts-items_create)
    #[serde(rename = "PAYMENT.REFERENCED-PAYOUT-ITEM.COMPLETED")]
    PaymentReferencedPayoutItemCompleted,
    /// Attempt to disburse funds fails.
    ///
    /// Related: [Create referenced payout item](https://developer.paypal.com/docs/api/referenced-payouts/v1/#referenced-payouts-items_create)
    #[serde(rename = "PAYMENT.REFERENCED-PAYOUT-ITEM.FAILED")]
    PaymentReferencedPayoutItemFailed,

    // === Disputes ===
    /// A dispute is created.
    ///
    /// Related: [Disputes API](https://developer.paypal.com/docs/api/customer-disputes/v1/)
    #[serde(rename = "CUSTOMER.DISPUTE.CREATED")]
    CustomerDisputeCreated,
    /// A dispute is resolved.
    ///
    /// Related: [Settle dispute](https://developer.paypal.com/docs/api/customer-disputes/v1/#disputes_settle)
    #[serde(rename = "CUSTOMER.DISPUTE.RESOLVED")]
    CustomerDisputeResolved,
    /// A dispute is updated.
    ///
    /// Related: [Partially update dispute](https://developer.paypal.com/docs/api/customer-disputes/v1/#disputes_patch)
    #[serde(rename = "CUSTOMER.DISPUTE.UPDATED")]
    CustomerDisputeUpdated,
    /// A risk dispute is created.
    ///
    /// Deprecated: Use [`CustomerDisputeCreated`] instead.
    #[serde(rename = "RISK.DISPUTE.CREATED")]
    #[deprecated(note = "Use CustomerDisputeCreated instead")]
    RiskDisputeCreated,

    // === Invoicing ===
    /// A merchant or customer cancels an invoice.
    ///
    /// Related: [Cancel invoice](https://developer.paypal.com/docs/api/invoicing/v2/#invoices_cancel)
    #[serde(rename = "INVOICING.INVOICE.CANCELLED")]
    InvoicingInvoiceCancelled,
    /// An invoice is created.
    ///
    /// Related: [Create draft invoice](https://developer.paypal.com/docs/api/invoicing/v2/#invoices_create)
    #[serde(rename = "INVOICING.INVOICE.CREATED")]
    InvoicingInvoiceCreated,
    /// An invoice is paid, partially paid, or payment is made and is pending.
    ///
    /// Related: [Record payment for invoice](https://developer.paypal.com/docs/api/invoicing/v2/#invoices_payments)
    #[serde(rename = "INVOICING.INVOICE.PAID")]
    InvoicingInvoicePaid,
    /// An invoice is refunded or partially refunded.
    ///
    /// Related: [Record refund for invoice](https://developer.paypal.com/docs/api/invoicing/v2/#invoices_refunds)
    #[serde(rename = "INVOICING.INVOICE.REFUNDED")]
    InvoicingInvoiceRefunded,
    /// An invoice is scheduled.
    ///
    /// Related: [Send invoice](https://developer.paypal.com/docs/api/invoicing/v2/#invoices_send)
    #[serde(rename = "INVOICING.INVOICE.SCHEDULED")]
    InvoicingInvoiceScheduled,
    /// An invoice is updated.
    ///
    /// Related: [Fully update invoice](https://developer.paypal.com/docs/api/invoicing/v2/#invoices_update)
    #[serde(rename = "INVOICING.INVOICE.UPDATED")]
    InvoicingInvoiceUpdated,

    // === Identity ===
    /// A user's consent token is revoked.
    ///
    /// Related: [Identity API](https://developer.paypal.com/docs/api/identity/v1/)
    #[serde(rename = "IDENTITY.AUTHORIZATION-CONSENT.REVOKED")]
    IdentityAuthorizationConsentRevoked,

    // === Merchant Integration & Onboarding ===
    /// A merchant completes setup.
    ///
    /// Related: [Partner Referrals API](https://developer.paypal.com/docs/api/partner-referrals/v2/)
    #[serde(rename = "MERCHANT.ONBOARDING.COMPLETED")]
    MerchantOnboardingCompleted,
    /// The consents for a merchant account setup are revoked or an account is closed.
    ///
    /// Related: [Partner Referrals API](https://developer.paypal.com/docs/api/partner-referrals/v2/)
    #[serde(rename = "MERCHANT.PARTNER-CONSENT.REVOKED")]
    MerchantPartnerConsentRevoked,
    /// PayPal must enable the merchant's account as PPCP for this webhook to work.
    ///
    /// Related: [Create partner referral](https://developer.paypal.com/docs/api/partner-referrals/v2/#partner-referrals_create)
    #[serde(rename = "CUSTOMER.MERCHANT-INTEGRATION.CAPABILITY-UPDATED")]
    CustomerMerchantIntegrationCapabilityUpdated,
    /// The products available to the merchant have changed.
    ///
    /// Related: [Create partner referral](https://developer.paypal.com/docs/api/partner-referrals/v2/#partner-referrals_create)
    #[serde(rename = "CUSTOMER.MERCHANT-INTEGRATION.PRODUCT-SUBSCRIPTION-UPDATED")]
    CustomerMerchantIntegrationProductSubscriptionUpdated,
    /// Merchant onboards again to a partner.
    ///
    /// Related: [Create partner referral](https://developer.paypal.com/docs/api/partner-referrals/v2/#partner-referrals_create)
    #[serde(rename = "CUSTOMER.MERCHANT-INTEGRATION.SELLER-ALREADY-INTEGRATED")]
    CustomerMerchantIntegrationSellerAlreadyIntegrated,
    /// PayPal creates a merchant account from the partner's onboarding link.
    ///
    /// Related: [Create partner referral](https://developer.paypal.com/docs/api/partner-referrals/v2/#partner-referrals_create)
    #[serde(rename = "CUSTOMER.MERCHANT-INTEGRATION.SELLER-ONBOARDING-INITIATED")]
    CustomerMerchantIntegrationSellerOnboardingInitiated,
    /// Merchant grants consents to a partner.
    ///
    /// Related: [Create partner referral](https://developer.paypal.com/docs/api/partner-referrals/v2/#partner-referrals_create)
    #[serde(rename = "CUSTOMER.MERCHANT-INTEGRATION.SELLER-CONSENT-GRANTED")]
    CustomerMerchantIntegrationSellerConsentGranted,
    /// Merchant confirms the email and consents are granted.
    ///
    /// Related: [Create partner referral](https://developer.paypal.com/docs/api/partner-referrals/v2/#partner-referrals_create)
    #[serde(rename = "CUSTOMER.MERCHANT-INTEGRATION.SELLER-EMAIL-CONFIRMED")]
    CustomerMerchantIntegrationSellerEmailConfirmed,

    // === Managed Accounts ===
    /// Managed account has been created.
    ///
    /// Related: [Managed Accounts API](https://developer.paypal.com/docs/api/managed-accounts/v1/)
    #[serde(rename = "CUSTOMER.MANAGED-ACCOUNT.ACCOUNT-CREATED")]
    CustomerManagedAccountCreated,
    /// Managed account creation failed.
    ///
    /// Related: [Managed Accounts API](https://developer.paypal.com/docs/api/managed-accounts/v1/)
    #[serde(rename = "CUSTOMER.MANAGED-ACCOUNT.CREATION-FAILED")]
    CustomerManagedAccountCreationFailed,
    /// Managed account has been updated.
    ///
    /// Related: [Managed Accounts API](https://developer.paypal.com/docs/api/managed-accounts/v1/)
    #[serde(rename = "CUSTOMER.MANAGED-ACCOUNT.ACCOUNT-UPDATED")]
    CustomerManagedAccountUpdated,
    /// Capabilities and/or process status has been changed on a managed account.
    ///
    /// Related: [Managed Accounts API](https://developer.paypal.com/docs/api/managed-accounts/v1/)
    #[serde(rename = "CUSTOMER.MANAGED-ACCOUNT.ACCOUNT-STATUS-CHANGED")]
    CustomerManagedAccountStatusChanged,
    /// Managed account has been risk assessed or the risk assessment has been changed.
    ///
    /// Related: [Managed Accounts API](https://developer.paypal.com/docs/api/managed-accounts/v1/)
    #[serde(rename = "CUSTOMER.MANAGED-ACCOUNT.RISK-ASSESSED")]
    CustomerManagedAccountRiskAssessed,
    /// Negative balance debit has been notified on a managed account.
    ///
    /// Related: [Update managed account](https://developer.paypal.com/docs/api/managed-accounts/v1/#managed-accounts_update)
    #[serde(rename = "CUSTOMER.MANAGED-ACCOUNT.NEGATIVE-BALANCE-NOTIFIED")]
    CustomerManagedAccountNegativeBalanceNotified,
    /// Negative balance debit has been initiated on a managed account.
    ///
    /// Related: [Update managed account](https://developer.paypal.com/docs/api/managed-accounts/v1/#managed-accounts_update)
    #[serde(rename = "CUSTOMER.MANAGED-ACCOUNT.NEGATIVE-BALANCE-DEBIT-INITIATED")]
    CustomerManagedAccountNegativeBalanceDebitInitiated,

    // === Account Limitations ===
    /// A limitation is added for a partner's managed account.
    ///
    /// Related: [Update managed account](https://developer.paypal.com/docs/api/managed-accounts/v1/#managed-accounts_update)
    #[serde(rename = "CUSTOMER.ACCOUNT-LIMITATION.ADDED")]
    CustomerAccountLimitationAdded,
    /// A limitation is escalated for a partner's managed account.
    ///
    /// Related: [Update managed account](https://developer.paypal.com/docs/api/managed-accounts/v1/#managed-accounts_update)
    #[serde(rename = "CUSTOMER.ACCOUNT-LIMITATION.ESCALATED")]
    CustomerAccountLimitationEscalated,
    /// A limitation is lifted for a partner's managed account.
    ///
    /// Related: [Update managed account](https://developer.paypal.com/docs/api/managed-accounts/v1/#managed-accounts_update)
    #[serde(rename = "CUSTOMER.ACCOUNT-LIMITATION.LIFTED")]
    CustomerAccountLimitationLifted,
    /// A limitation is updated for a partner's managed account.
    ///
    /// Related: [Update managed account](https://developer.paypal.com/docs/api/managed-accounts/v1/#managed-accounts_update)
    #[serde(rename = "CUSTOMER.ACCOUNT-LIMITATION.UPDATED")]
    CustomerAccountLimitationUpdated,

    // === Payment Method Tokens (Vault) ===
    /// A payment token is created to save a payment method. Supports Cards and PayPal.
    ///
    /// Related: [Payment Method Tokens API](https://developer.paypal.com/docs/api/payment-tokens/v3/)
    #[serde(rename = "VAULT.PAYMENT-TOKEN.CREATED")]
    VaultPaymentTokenCreated,
    /// A payment token is deleted. The payer's payment method is no longer saved to the PayPal vault.
    /// Supports Cards and PayPal.
    ///
    /// Related: [Delete payment token](https://developer.paypal.com/docs/api/payment-tokens/v3/#payment-tokens_delete)
    #[serde(rename = "VAULT.PAYMENT-TOKEN.DELETED")]
    VaultPaymentTokenDeleted,
    /// A request to delete a payment token has been submitted to the Payment Method Tokens API.
    /// Supports PayPal only.
    ///
    /// Related: [Payment Method Tokens API](https://developer.paypal.com/docs/api/payment-tokens/v3/)
    #[serde(rename = "VAULT.PAYMENT-TOKEN.DELETION-INITIATED")]
    VaultPaymentTokenDeletionInitiated,
}

/// PayPal webhook event wrapper
///
/// # Example
///
/// ```rust
/// use tune_hub_backend::api::paypal::types::{PayPalEventType, PayPalWebhookEvent};
///
/// let json = r#"{
///     "id": "8PT597110X687430LKGECATA",
///     "create_time": "2013-06-25T21:41:28Z",
///     "resource_type": "authorization",
///     "event_version": "1.0",
///     "event_type": "PAYMENT.AUTHORIZATION.CREATED",
///     "summary": "A payment authorization was created",
///     "resource": {
///         "id": "2DC87612EK520411B",
///         "create_time": "2013-06-25T21:39:15Z",
///         "update_time": "2013-06-25T21:39:17Z",
///         "state": "authorized",
///         "amount": {
///             "total": "7.47",
///             "currency": "USD",
///             "details": {
///                 "subtotal": "7.47"
///             }
///         },
///         "parent_payment": "PAY-36246664YD343335CKHFA4AY",
///         "valid_until": "2013-07-24T21:39:15Z",
///         "links": [
///             {
///                 "href": "https://api-m.paypal.com/v1/payments/authorization/2DC87612EK520411B",
///                 "rel": "self",
///                 "method": "GET"
///             }
///         ]
///     },
///     "links": [
///         {
///             "href": "https://api-m.paypal.com/v1/notfications/webhooks-events/8PT597110X687430LKGECATA",
///             "rel": "self",
///             "method": "GET"
///         }
///     ]
/// }"#;
///
/// let event: PayPalWebhookEvent = serde_json::from_str(json).unwrap();
/// assert_eq!(event.id, "8PT597110X687430LKGECATA");
/// assert_eq!(event.resource_type, "authorization");
/// assert!(matches!(event.event_type, PayPalEventType::PaymentAuthorizationCreated));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalWebhookEvent {
    /// Unique event identifier
    pub id: String,
    /// Event type (e.g., "PAYMENT.CAPTURE.COMPLETED")
    pub event_type: PayPalEventType,
    /// The resource object related to the event - type varies by event_type!
    pub resource: serde_json::Value,
    /// Event version
    pub event_version: String,
    /// Summary description of the event
    pub summary: String,
    /// Resource type (e.g., "capture", "refund", "dispute")
    pub resource_type: String,
    /// When the event was created
    pub create_time: String,
}

/// PayPal capture resource - represents a completed payment capture
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalCapture {
    /// The PayPal-generated ID for the captured payment
    pub id: String,
    /// The status of the captured payment
    /// Values: COMPLETED, DECLINED, PARTIALLY_REFUNDED, PENDING, REFUNDED
    pub status: String,
    /// The amount for this captured payment
    pub amount: PayPalAmount,
    /// Indicates whether you can make additional captures against the authorized payment
    #[serde(default)]
    pub final_capture: bool,
    /// The level of protection offered for the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_protection: Option<PayPalSellerProtection>,
    /// The detailed breakdown of the capture amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_receivable_breakdown: Option<PayPalSellerReceivableBreakdown>,
    /// The API caller-provided external invoice number for this order
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// The API caller-provided external ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_id: Option<String>,
    /// An array of related HATEOAS links
    #[serde(default)]
    pub links: Vec<PayPalLink>,
    /// The date and time when the transaction was created
    pub create_time: String,
    /// The date and time when the transaction was last updated
    pub update_time: String,
    /// Additional payment related data - THIS CONTAINS THE ORDER_ID!
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supplementary_data: Option<PayPalSupplementaryData>,
}

/// Represents a monetary amount with currency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalAmount {
    /// The three-character ISO-4217 currency code
    pub currency_code: String,
    /// The value as a decimal string (e.g., "10.00")
    /// NOT in cents like Stripe!
    pub value: String,
}

/// Seller protection details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalSellerProtection {
    /// Indicates whether the transaction is eligible for seller protection
    /// Values: ELIGIBLE, PARTIALLY_ELIGIBLE, NOT_ELIGIBLE
    pub status: String,
    /// An array of conditions that are covered for the transaction
    #[serde(default)]
    pub dispute_categories: Vec<String>,
}

/// Breakdown of the seller receivable amount
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalSellerReceivableBreakdown {
    /// The amount for this captured payment in the currency of the transaction
    pub gross_amount: PayPalAmount,
    /// The applicable fee for this captured payment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal_fee: Option<PayPalAmount>,
    /// The net amount that the payee receives for this captured payment
    pub net_amount: PayPalAmount,
    /// An array of platform or partner fees, commissions, or brokerage fees
    #[serde(default)]
    pub platform_fees: Vec<PayPalPlatformFee>,
}

/// Platform or partner fee details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalPlatformFee {
    /// The fee amount
    pub amount: PayPalAmount,
    /// The merchant account that receives the fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payee: Option<PayPalPayee>,
}

/// Payee account details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalPayee {
    /// The PayPal-assigned merchant account ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    /// The email address of the merchant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_address: Option<String>,
}

/// Additional payment-related data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalSupplementaryData {
    /// Related IDs for the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_ids: Option<PayPalRelatedIds>,
}

/// Related transaction identifiers - CRITICAL for linking webhook to your DB!
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalRelatedIds {
    /// The order ID associated with this capture
    /// This is what you use to find the paypal_transaction record!
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// The authorization ID associated with this capture
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_id: Option<String>,
}

/// HATEOAS link for API navigation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalLink {
    /// The complete target URL
    pub href: String,
    /// The link relation type
    pub rel: String,
    /// The HTTP method required to make the related call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}

/// PayPal refund resource (for PAYMENT.CAPTURE.REFUNDED events)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalRefund {
    /// The PayPal-generated ID for the refund
    pub id: String,
    /// The status of the refund
    /// Values: CANCELLED, PENDING, COMPLETED
    pub status: String,
    /// The amount being refunded
    pub amount: PayPalAmount,
    /// The reason for the refund
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note_to_payer: Option<String>,
    /// The breakdown of the refund
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_payable_breakdown: Option<PayPalSellerPayableBreakdown>,
    /// The API caller-provided external invoice number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    /// Links for HATEOAS navigation
    #[serde(default)]
    pub links: Vec<PayPalLink>,
    /// The date and time when the transaction was created
    pub create_time: String,
    /// The date and time when the transaction was last updated
    pub update_time: String,
}

/// Breakdown of the seller payable amount for refunds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayPalSellerPayableBreakdown {
    /// The amount that the payee refunded to the payer
    pub gross_amount: PayPalAmount,
    /// The net amount that the payee's account is debited
    pub net_amount: PayPalAmount,
    /// The PayPal fee that is refunded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal_fee: Option<PayPalAmount>,
    /// An array of platform or partner fees
    #[serde(default)]
    pub platform_fees: Vec<PayPalPlatformFee>,
    /// The net amount debited from the merchant's PayPal account
    pub total_refunded_amount: PayPalAmount,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paypal_event_type_serialization() {
        let event_type = PayPalEventType::PaymentCaptureCompleted;
        let json = serde_json::to_string(&event_type).unwrap();
        assert_eq!(json, r#""PAYMENT.CAPTURE.COMPLETED""#);

        let deserialized: PayPalEventType = serde_json::from_str(r#""PAYMENT.CAPTURE.COMPLETED""#).unwrap();
        assert!(matches!(deserialized, PayPalEventType::PaymentCaptureCompleted));
    }

    #[test]
    fn test_paypal_amount_roundtrip() {
        let amount = PayPalAmount {
            currency_code: "USD".to_string(),
            value: "99.99".to_string(),
        };

        let json = serde_json::to_string(&amount).unwrap();
        let deserialized: PayPalAmount = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.currency_code, "USD");
        assert_eq!(deserialized.value, "99.99");
    }

    #[test]
    fn test_paypal_webhook_event_deserialization() {
        let json = r#"{
            "id": "WH-123",
            "event_type": "PAYMENT.CAPTURE.COMPLETED",
            "resource": {"capture_id": "CAP-456"},
            "event_version": "1.0",
            "summary": "Payment completed",
            "resource_type": "capture",
            "create_time": "2024-01-15T10:00:00Z"
        }"#;

        let event: PayPalWebhookEvent = serde_json::from_str(json).unwrap();

        assert_eq!(event.id, "WH-123");
        assert_eq!(event.event_type, PayPalEventType::PaymentCaptureCompleted);
        assert_eq!(event.resource_type, "capture");
    }
}
