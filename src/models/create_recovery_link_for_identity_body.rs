/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.11
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// CreateRecoveryLinkForIdentityBody : Create Recovery Link for Identity Request Body



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateRecoveryLinkForIdentityBody {
    /// Link Expires In  The recovery link will expire after that amount of time has passed. Defaults to the configuration value of `selfservice.methods.code.config.lifespan`.
    #[serde(rename = "expires_in", skip_serializing_if = "Option::is_none")]
    pub expires_in: Option<String>,
    /// Identity to Recover  The identity's ID you wish to recover.
    #[serde(rename = "identity_id")]
    pub identity_id: String,
}


impl CreateRecoveryLinkForIdentityBody {
    /// Create Recovery Link for Identity Request Body
    pub fn new(identity_id: String) -> CreateRecoveryLinkForIdentityBody {
        CreateRecoveryLinkForIdentityBody {
                expires_in: None,
                identity_id,
        }
    }
}


