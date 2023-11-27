/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.4.3
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// SuccessfulCodeExchangeResponse : The Response for Registration Flows via API



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuccessfulCodeExchangeResponse {
    #[serde(rename = "session")]
    pub session: Box<crate::models::Session>,
    /// The Session Token  A session token is equivalent to a session cookie, but it can be sent in the HTTP Authorization Header:  Authorization: bearer ${session-token}  The session token is only issued for API flows, not for Browser flows!
    #[serde(rename = "session_token", skip_serializing_if = "Option::is_none")]
    pub session_token: Option<String>,
}


impl SuccessfulCodeExchangeResponse {
    /// The Response for Registration Flows via API
    pub fn new(session: crate::models::Session) -> SuccessfulCodeExchangeResponse {
        SuccessfulCodeExchangeResponse {
                session: Box::new(session),
                session_token: None,
        }
    }
}


