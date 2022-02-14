/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.92
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// SelfServiceLoginFlow : This object represents a login flow. A login flow is initiated at the \"Initiate Login API / Browser Flow\" endpoint by a client.  Once a login flow is completed successfully, a session cookie or session token will be issued.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SelfServiceLoginFlow {
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<crate::models::IdentityCredentialsType>,
    /// CreatedAt is a helper struct field for gobuffalo.pop.
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// ExpiresAt is the time (UTC) when the flow expires. If the user still wishes to log in, a new flow has to be initiated.
    #[serde(rename = "expires_at")]
    pub expires_at: String,
    #[serde(rename = "id")]
    pub id: String,
    /// IssuedAt is the time (UTC) when the flow started.
    #[serde(rename = "issued_at")]
    pub issued_at: String,
    /// Refresh stores whether this login flow should enforce re-authentication.
    #[serde(rename = "refresh", skip_serializing_if = "Option::is_none")]
    pub refresh: Option<bool>,
    /// RequestURL is the initial URL that was requested from Ory Kratos. It can be used to forward information contained in the URL's path or query for example.
    #[serde(rename = "request_url")]
    pub request_url: String,
    #[serde(rename = "requested_aal", skip_serializing_if = "Option::is_none")]
    pub requested_aal: Option<crate::models::AuthenticatorAssuranceLevel>,
    /// ReturnTo contains the requested return_to URL.
    #[serde(rename = "return_to", skip_serializing_if = "Option::is_none")]
    pub return_to: Option<String>,
    /// The flow type can either be `api` or `browser`.
    #[serde(rename = "type")]
    pub _type: String,
    #[serde(rename = "ui")]
    pub ui: Box<crate::models::UiContainer>,
    /// UpdatedAt is a helper struct field for gobuffalo.pop.
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl SelfServiceLoginFlow {
    /// This object represents a login flow. A login flow is initiated at the \"Initiate Login API / Browser Flow\" endpoint by a client.  Once a login flow is completed successfully, a session cookie or session token will be issued.
    pub fn new(expires_at: String, id: String, issued_at: String, request_url: String, _type: String, ui: crate::models::UiContainer) -> SelfServiceLoginFlow {
        SelfServiceLoginFlow {
            active: None,
            created_at: None,
            expires_at,
            id,
            issued_at,
            refresh: None,
            request_url,
            requested_aal: None,
            return_to: None,
            _type,
            ui: Box::new(ui),
            updated_at: None,
        }
    }
}


