/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v0.0.1-alpha.27
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProvisionProjectPayload {
    /// The stripe subscription ID to use for provisioning the project.
    #[serde(rename = "subscription_id")]
    pub subscription_id: String,
}

impl ProvisionProjectPayload {
    pub fn new(subscription_id: String) -> ProvisionProjectPayload {
        ProvisionProjectPayload {
            subscription_id,
        }
    }
}


