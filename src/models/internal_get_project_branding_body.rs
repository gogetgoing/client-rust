/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.14
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// InternalGetProjectBrandingBody : Get Project Branding Request Body



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InternalGetProjectBrandingBody {
    #[serde(rename = "hostname", skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
}

impl Default for InternalGetProjectBrandingBody {
    fn default() -> Self {
        Self::new()
    }
}

impl InternalGetProjectBrandingBody {
    /// Get Project Branding Request Body
    pub fn new() -> InternalGetProjectBrandingBody {
        InternalGetProjectBrandingBody {
                hostname: None,
        }
    }
}


