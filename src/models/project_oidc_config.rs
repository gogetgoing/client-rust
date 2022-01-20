/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.55
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectOidcConfig {
    /// AuthURL is the authorize url, typically something like: https://example.org/oauth2/auth Should only be used when the OAuth2 / OpenID Connect server is not supporting OpenID Connect Discovery and when `provider` is set to `generic`.
    #[serde(rename = "auth_url", skip_serializing_if = "Option::is_none")]
    pub auth_url: Option<String>,
    /// ClientID is the application's Client ID.
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    /// ClientSecret is the application's secret.
    #[serde(rename = "client_secret", skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    /// ID is the provider's ID
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// IssuerURL is the OpenID Connect Server URL. You can leave this empty if `provider` is not set to `generic`. If set, neither `auth_url` nor `token_url` are required.
    #[serde(rename = "issuer_url", skip_serializing_if = "Option::is_none")]
    pub issuer_url: Option<String>,
    /// Label represents an optional label which can be used in the UI generation.
    #[serde(rename = "label", skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Mapper specifies the JSONNet code snippet which uses the OpenID Connect Provider's data (e.g. GitHub or Google profile information) to hydrate the identity's data.  It can be either a URL (file://, http(s)://, base64://) or an inline JSONNet code snippet.
    #[serde(rename = "mapper_url", skip_serializing_if = "Option::is_none")]
    pub mapper_url: Option<String>,
    /// Provider is either \"generic\" for a generic OAuth 2.0 / OpenID Connect Provider or one of: generic google github gitlab microsoft discord slack facebook vk yandex
    #[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    /// RequestedClaims string encoded json object that specifies claims and optionally their properties which should be included in the id_token or returned from the UserInfo Endpoint.  More information: https://openid.net/specs/openid-connect-core-1_0.html#ClaimsParameter
    #[serde(rename = "requested_claims", skip_serializing_if = "Option::is_none")]
    pub requested_claims: Option<serde_json::Value>,
    /// Scope specifies optional requested permissions.
    #[serde(rename = "scope", skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<String>>,
    #[serde(rename = "string", skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    /// Tenant is the Azure AD Tenant to use for authentication, and must be set when `provider` is set to `microsoft`. Can be either `common`, `organizations`, `consumers` for a multitenant application or a specific tenant like `8eaef023-2b34-4da1-9baa-8bc8c9d6a490` or `contoso.onmicrosoft.com`.
    #[serde(rename = "tenant", skip_serializing_if = "Option::is_none")]
    pub tenant: Option<String>,
    /// TokenURL is the token url, typically something like: https://example.org/oauth2/token Should only be used when the OAuth2 / OpenID Connect server is not supporting OpenID Connect Discovery and when `provider` is set to `generic`.
    #[serde(rename = "token_url", skip_serializing_if = "Option::is_none")]
    pub token_url: Option<String>,
}

impl ProjectOidcConfig {
    pub fn new() -> ProjectOidcConfig {
        ProjectOidcConfig {
            auth_url: None,
            client_id: None,
            client_secret: None,
            id: None,
            issuer_url: None,
            label: None,
            mapper_url: None,
            provider: None,
            requested_claims: None,
            scope: None,
            string: None,
            tenant: None,
            token_url: None,
        }
    }
}


