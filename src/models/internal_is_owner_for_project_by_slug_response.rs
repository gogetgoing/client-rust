/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.4.3
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InternalIsOwnerForProjectBySlugResponse {
    /// ProjectID is the project's ID.
    #[serde(rename = "project_id")]
    pub project_id: String,
}


impl InternalIsOwnerForProjectBySlugResponse {
    pub fn new(project_id: String) -> InternalIsOwnerForProjectBySlugResponse {
        InternalIsOwnerForProjectBySlugResponse {
                project_id,
        }
    }
}


