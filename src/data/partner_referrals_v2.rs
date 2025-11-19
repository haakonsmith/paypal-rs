//! Partner Referrals v2 data structures.
//!
//! Reference: <https://developer.paypal.com/docs/api/partner-referrals/v2/>

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::data::common::{LinkDescription, Money};

/// Name information
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Name {
    /// The prefix, or title, to the party's name.
    pub prefix: Option<String>,
    /// The given name or first name
    pub given_name: Option<String>,
    /// The surname or last name
    pub surname: Option<String>,
    /// The middle name
    pub middle_name: Option<String>,
    /// The suffix for the party's name.
    pub suffix: Option<String>,
    /// The full name representation
    pub full_name: Option<String>,
}

/// Person name type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PersonNameType {
    /// Legal name
    Legal,
}

/// Person name with type
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonName {
    /// The name information
    #[serde(flatten)]
    pub name: Name,
    /// The name type
    #[serde(rename = "type")]
    pub name_type: PersonNameType,
}

/// Business name
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BusinessName {
    /// The business name
    pub business_name: Option<String>,
}

/// Business name type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessNameType {
    /// Legal name
    Legal,
    /// Doing business as name
    DoingBusinessAs,
}

/// Business name with type
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessNameDetail {
    /// The business name information
    #[serde(flatten)]
    pub name: BusinessName,
    /// The name type
    #[serde(rename = "type")]
    pub name_type: BusinessNameType,
}

/// Birth details
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirthDetails {
    /// Date of birth in YYYY-MM-DD format
    pub date_of_birth: String,
}

/// Portable address
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AddressPortable {
    /// The first line of the address. For example, number or street.
    pub address_line_1: Option<String>,
    /// The second line of the address. For example, suite or apartment number.
    pub address_line_2: Option<String>,
    /// The third line of the address.
    pub address_line_3: Option<String>,
    /// The neighborhood, ward, or district.
    pub admin_area_4: Option<String>,
    /// A sub-locality, suburb, neighborhood, or district.
    pub admin_area_3: Option<String>,
    /// A city, town, or village. Smaller than admin_area_level_1.
    pub admin_area_2: Option<String>,
    /// The highest level sub-division in a country, which is usually a province, state, or ISO-3166-2 subdivision.
    pub admin_area_1: Option<String>,
    /// The postal code.
    pub postal_code: Option<String>,
    /// The two-character ISO 3166-1 code that identifies the country or region.
    pub country_code: String,
}

/// Person address type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PersonAddressType {
    /// Home address
    Home,
}

/// Address with type for persons
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonAddressDetail {
    /// The address information
    #[serde(flatten)]
    pub address: AddressPortable,
    /// The address type
    #[serde(rename = "type")]
    pub address_type: PersonAddressType,
}

/// Business address type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessAddressType {
    /// Work address
    Work,
}

/// Address with type for businesses
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessAddressDetail {
    /// The address information
    #[serde(flatten)]
    pub address: AddressPortable,
    /// The address type
    #[serde(rename = "type")]
    pub address_type: BusinessAddressType,
}

/// Phone detail
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PhoneDetail {
    /// The country calling code (CC), in its canonical international E.164 numbering plan format.
    pub country_code: String,
    /// The national number, in its canonical international E.164 numbering plan format.
    pub national_number: String,
    /// The extension number.
    pub extension_number: Option<String>,
}

/// Phone type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PhoneType {
    /// Home phone
    Home,
    /// Mobile phone
    Mobile,
    /// Fax
    Fax,
    /// Other
    Other,
    /// Pager
    Pager,
}

/// Business phone type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessPhoneType {
    /// Work phone
    Work,
    /// Fax
    Fax,
}

/// Phone with type for persons
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonPhoneDetail {
    /// The phone information
    #[serde(flatten)]
    pub phone: PhoneDetail,
    /// The phone type
    #[serde(rename = "type")]
    pub phone_type: PhoneType,
}

/// Phone with type for businesses
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessPhoneDetail {
    /// The phone information
    #[serde(flatten)]
    pub phone: PhoneDetail,
    /// The phone type
    #[serde(rename = "type")]
    pub phone_type: BusinessPhoneType,
}

/// Email type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EmailType {
    /// Work email
    Work,
}

/// Email
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    /// The email address type
    #[serde(rename = "type")]
    pub email_type: EmailType,
    /// The email address
    pub email: String,
}

/// Individual owner type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IndividualOwnerType {
    /// Primary owner
    Primary,
}

/// Individual owner
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualOwner {
    /// List of names
    pub names: Option<Vec<PersonName>>,
    /// Citizenship country code
    pub citizenship: Option<String>,
    /// List of addresses
    pub addresses: Option<Vec<PersonAddressDetail>>,
    /// List of phone numbers
    pub phones: Option<Vec<PersonPhoneDetail>>,
    /// Birth details
    pub birth_details: Option<BirthDetails>,
    /// The owner type
    #[serde(rename = "type")]
    pub owner_type: Option<IndividualOwnerType>,
}

/// Business type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessType {
    /// Individual
    Individual,
    /// Proprietorship
    Proprietorship,
    /// Partnership
    Partnership,
    /// Corporation
    Corporation,
    /// Nonprofit
    Nonprofit,
    /// Government
    Government,
    /// Public company
    PublicCompany,
    /// Private corporation
    PrivateCorporation,
    /// Limited liability partnership
    LimitedLiabilityPartnership,
    /// Private partnership
    PrivatePartnership,
    /// Public partnership
    PublicPartnership,
    /// Limited liability private corporation
    LimitedLiabilityPrivateCorporation,
    /// Association
    Association,
    /// Limited liability proprietorship
    LimitedLiabilityProprietorship,
}

/// Business sub type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessSubType {
    /// AISBL
    Aisbl,
    /// Limited partnership
    LimitedPartnership,
    /// SCCV
    Sccv,
    /// Sole proprietorship
    SoleProprietorship,
    /// Trust
    Trust,
    /// Other
    Other,
}

/// Business type info
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessTypeInfo {
    /// The business type
    #[serde(rename = "type")]
    pub business_type: Option<BusinessType>,
    /// The business sub type
    pub subtype: Option<BusinessSubType>,
}

/// Business industry
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessIndustry {
    /// The business category
    pub category: String,
    /// The MCC code
    pub mcc_code: Option<String>,
    /// The business subcategory
    pub subcategory: Option<String>,
}

/// Business incorporation
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessIncorporation {
    /// The country code where the business is incorporated
    pub incorporation_country_code: Option<String>,
    /// The date of incorporation in YYYY-MM-DD format
    pub incorporation_date: Option<String>,
}

/// Currency range
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyRange {
    /// The minimum amount
    pub minimum_amount: Option<Money>,
    /// The maximum amount
    pub maximum_amount: Option<Money>,
}

/// Purpose code enum
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PurposeCode {
    /// Advertising
    Advertising,
    /// Business related expenses
    BusinessRelatedExpenses,
    /// Consulting
    Consulting,
    /// Education
    Education,
    /// Freight
    Freight,
    /// Gift
    Gift,
    /// Hotel
    Hotel,
    /// Investment
    Investment,
    /// Medical
    Medical,
    /// Other
    Other,
    /// Royalty
    Royalty,
    /// Software
    Software,
    /// Tourism
    Tourism,
    /// Utility
    Utility,
}

/// Individual beneficial owner
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndividualBeneficialOwner {
    /// List of names
    pub names: Option<Vec<PersonName>>,
    /// Citizenship country code
    pub citizenship: Option<String>,
    /// List of addresses
    pub addresses: Option<Vec<PersonAddressDetail>>,
    /// List of phone numbers
    pub phones: Option<Vec<PersonPhoneDetail>>,
    /// Birth details
    pub birth_details: Option<BirthDetails>,
    /// Percentage of ownership
    pub percentage_of_ownership: Option<String>,
}

/// Business beneficial owner
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessBeneficialOwner {
    /// Business type
    pub business_type: Option<BusinessTypeInfo>,
    /// Business industry
    pub business_industry: Option<BusinessIndustry>,
    /// Business incorporation details
    pub business_incorporation: Option<BusinessIncorporation>,
    /// List of business names
    pub names: Option<Vec<BusinessNameDetail>>,
    /// List of email addresses
    pub emails: Option<Vec<Email>>,
    /// Website
    pub website: Option<String>,
    /// List of addresses
    pub addresses: Option<Vec<BusinessAddressDetail>>,
    /// List of phone numbers
    pub phones: Option<Vec<BusinessPhoneDetail>>,
    /// Percentage of ownership
    pub percentage_of_ownership: Option<String>,
}

/// Beneficial owners
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BeneficialOwners {
    /// Individual beneficial owners
    pub individual_beneficial_owners: Option<Vec<IndividualBeneficialOwner>>,
    /// Business beneficial owners
    pub business_beneficial_owners: Option<Vec<BusinessBeneficialOwner>>,
}

/// Office bearer role
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OfficeBearerRole {
    /// Director
    Director,
    /// Secretary
    Secretary,
    /// Other
    Other,
}

/// Office bearer
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfficeBearer {
    /// List of names
    pub names: Option<Vec<PersonName>>,
    /// Citizenship country code
    pub citizenship: Option<String>,
    /// List of addresses
    pub addresses: Option<Vec<PersonAddressDetail>>,
    /// List of phone numbers
    pub phones: Option<Vec<PersonPhoneDetail>>,
    /// Birth details
    pub birth_details: Option<BirthDetails>,
    /// The role
    pub role: Option<OfficeBearerRole>,
}

/// Business entity
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessEntity {
    /// Business type
    pub business_type: Option<BusinessTypeInfo>,
    /// Business industry
    pub business_industry: Option<BusinessIndustry>,
    /// Business incorporation details
    pub business_incorporation: Option<BusinessIncorporation>,
    /// List of business names
    pub names: Option<Vec<BusinessNameDetail>>,
    /// List of email addresses
    pub emails: Option<Vec<Email>>,
    /// Website
    pub website: Option<String>,
    /// List of addresses
    pub addresses: Option<Vec<BusinessAddressDetail>>,
    /// List of phone numbers
    pub phones: Option<Vec<BusinessPhoneDetail>>,
    /// Beneficial owners
    pub beneficial_owners: Option<BeneficialOwners>,
    /// Office bearers
    pub office_bearers: Option<Vec<OfficeBearer>>,
    /// Annual sales volume range
    pub annual_sales_volume_range: Option<CurrencyRange>,
    /// Average monthly volume range
    pub average_monthly_volume_range: Option<CurrencyRange>,
    /// Purpose codes
    pub purpose_code: Option<Vec<PurposeCode>>,
    /// Business description
    pub business_description: Option<String>,
}

/// Account information
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    /// Individual owners
    pub individual_owners: Option<Vec<IndividualOwner>>,
    /// Business entity
    pub business_entity: Option<BusinessEntity>,
}

/// Account identifier type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountIdentifierType {
    /// IBAN
    Iban,
    /// BBAN
    Bban,
    /// BIC
    Bic,
    /// CLABE
    Clabe,
    /// BSB
    Bsb,
    /// Sort code
    SortCode,
    /// Routing number
    RoutingNumber,
    /// Bank code
    BankCode,
    /// Branch code
    BranchCode,
}

/// Account identifier
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountIdentifier {
    /// The identifier type
    #[serde(rename = "type")]
    pub identifier_type: AccountIdentifierType,
    /// The identifier value
    pub value: String,
}

/// Bank account type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BankAccountType {
    /// Checking account
    Checking,
    /// Savings account
    Savings,
}

/// Mandate
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mandate {
    /// Whether the mandate is accepted
    pub accepted: bool,
}

/// Bank
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bank {
    /// The bank nickname
    pub nick_name: Option<String>,
    /// The account number
    pub account_number: String,
    /// The account type
    pub account_type: BankAccountType,
    /// The currency code
    pub currency_code: String,
    /// List of identifiers
    pub identifiers: Option<Vec<AccountIdentifier>>,
    /// Branch location
    pub branch_location: Option<AddressPortable>,
    /// Mandate
    pub mandate: Option<Mandate>,
}

/// Financial instruments
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialInstruments {
    /// List of banks
    pub banks: Option<Vec<Bank>>,
}

/// Operation type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OperationType {
    /// API integration
    ApiIntegration,
    /// Bank addition
    BankAddition,
    /// Vetting
    Vetting,
    /// Subscription addition
    SubscriptionAddition,
}

/// Integration method
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IntegrationMethod {
    /// PayPal
    Paypal,
    /// Direct credit card
    DirectCreditCard,
    /// Both
    Both,
}

/// Integration type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IntegrationType {
    /// First party
    FirstParty,
    /// Third party
    ThirdParty,
}

/// REST endpoint features
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RestEndpointFeature {
    /// Payment
    Payment,
    /// Refund
    Refund,
    /// Partner fee
    PartnerFee,
    /// Delay funds disbursement
    DelayFundsDisbursement,
    /// Advanced transactions search
    AdvancedTransactionsSearch,
    /// Dispute management
    DisputeManagement,
    /// Invoice management
    InvoiceManagement,
    /// Payment restrictions
    PaymentRestrictions,
    /// Vault management
    VaultManagement,
}

/// First party details
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirstPartyDetails {
    /// List of features
    pub features: Option<Vec<RestEndpointFeature>>,
    /// Seller nonce
    pub seller_nonce: String,
}

/// Third party details
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThirdPartyDetails {
    /// List of features
    pub features: Option<Vec<RestEndpointFeature>>,
}

/// REST API integration
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestApiIntegration {
    /// Integration method
    pub integration_method: IntegrationMethod,
    /// Integration type
    pub integration_type: IntegrationType,
    /// First party details
    pub first_party_details: Option<FirstPartyDetails>,
    /// Third party details
    pub third_party_details: Option<ThirdPartyDetails>,
}

/// API integration preference
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiIntegrationPreference {
    /// REST API integration
    pub rest_api_integration: Option<RestApiIntegration>,
}

/// Operation
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    /// The operation type
    pub operation: OperationType,
    /// API integration preference
    pub api_integration_preference: Option<ApiIntegrationPreference>,
}

/// Product
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Product {
    /// Express checkout
    ExpressCheckout,
    /// PayPal Commerce Platform
    #[serde(rename = "PPCP")]
    PaypalCommercePlatform,
    /// Virtual terminal
    VirtualTerminal,
    /// Payment Pro
    PaymentPro,
}

/// Capability
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Capability {
    /// Apple Pay
    ApplePay,
    /// Google Pay
    GooglePay,
}

/// Legal consent type
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LegalConsentType {
    /// Share data consent
    ShareDataConsent,
}

/// Legal consent
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegalConsent {
    /// The consent type
    #[serde(rename = "type")]
    pub consent_type: LegalConsentType,
    /// Whether consent is granted
    pub granted: bool,
}

/// Partner config override
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartnerConfigOverride {
    /// Partner logo URL
    pub partner_logo_url: Option<String>,
    /// Return URL
    pub return_url: Option<String>,
    /// Return URL description
    pub return_url_description: Option<String>,
    /// Action renewal URL
    pub action_renewal_url: Option<String>,
    /// Whether to show add credit card
    pub show_add_credit_card: Option<bool>,
}

/// Referral data payload
#[skip_serializing_none]
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ReferralData {
    /// Individual owners
    pub individual_owners: Option<Vec<IndividualOwner>>,
    /// Business entity
    pub business_entity: Option<BusinessEntity>,
    /// Customer email address
    pub email: Option<String>,
    /// Preferred language code
    pub preferred_language_code: Option<String>,
    /// Tracking ID
    pub tracking_id: Option<String>,
    /// Partner config override
    pub partner_config_override: Option<PartnerConfigOverride>,
    /// Operations
    pub operations: Vec<Operation>,
    /// Products
    pub products: Option<Vec<Product>>,
    /// Capabilities
    pub capabilities: Option<Vec<Capability>>,
    /// Legal consents
    pub legal_consents: Option<Vec<LegalConsent>>,
    /// Financial instruments
    pub financial_instruments: Option<FinancialInstruments>,
    /// Legal country code
    pub legal_country_code: Option<String>,
}

/// Create referral data response
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateReferralDataResponse {
    /// Links
    pub links: Option<Vec<LinkDescription>>,
}

/// Referral data response
#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralDataResponse {
    /// Partner referral ID
    pub partner_referral_id: Option<String>,
    /// Submitter payer ID
    pub submitter_payer_id: Option<String>,
    /// Submitter client ID
    pub submitter_client_id: Option<String>,
    /// Referral data
    pub referral_data: Option<ReferralData>,
    /// Links
    pub links: Option<Vec<LinkDescription>>,
}
