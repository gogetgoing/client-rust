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
pub struct ProjectHost {
    /// The project's host.
    #[serde(rename = "host")]
    pub host: String,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "project_id")]
    pub project_id: String,
}

impl ProjectHost {
    pub fn new(host: String, id: String, project_id: String) -> ProjectHost {
        ProjectHost {
            host,
            id,
            project_id,
        }
    }
}


