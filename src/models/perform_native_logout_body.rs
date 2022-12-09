/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.0
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// PerformNativeLogoutBody : nolint:deadcode,unused



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PerformNativeLogoutBody {
    /// The Session Token  Invalidate this session token.
    #[serde(rename = "session_token")]
    pub session_token: String,
}


impl PerformNativeLogoutBody {
    /// nolint:deadcode,unused
    pub fn new(session_token: String) -> PerformNativeLogoutBody {
        PerformNativeLogoutBody {
                session_token,
        }
    }
}

