/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.130
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SubmitSelfServiceLoginFlowWithLookupSecretMethodBody {
    /// Sending the anti-csrf token is only required for browser login flows.
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// The lookup secret.
    #[serde(rename = "lookup_secret")]
    pub lookup_secret: String,
    /// Method should be set to \"lookup_secret\" when logging in using the lookup_secret strategy.
    #[serde(rename = "method")]
    pub method: String,
}

impl SubmitSelfServiceLoginFlowWithLookupSecretMethodBody {
    pub fn new(lookup_secret: String, method: String) -> SubmitSelfServiceLoginFlowWithLookupSecretMethodBody {
        SubmitSelfServiceLoginFlowWithLookupSecretMethodBody {
            csrf_token: None,
            lookup_secret,
            method,
        }
    }
}


