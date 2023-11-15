/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.4.0
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectApiKey {
    /// The token's creation date
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// The token's ID.
    #[serde(rename = "id")]
    pub id: String,
    /// The Token's Name  Set this to help you remember, for example, where you use the token.
    #[serde(rename = "name")]
    pub name: String,
    /// The token's owner
    #[serde(rename = "owner_id")]
    pub owner_id: String,
    /// The Token's Project ID
    #[serde(rename = "project_id", skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
    /// The token's last update date
    #[serde(rename = "updated_at", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// The token's value
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}


impl ProjectApiKey {
    pub fn new(id: String, name: String, owner_id: String) -> ProjectApiKey {
        ProjectApiKey {
                created_at: None,
                id,
                name,
                owner_id,
                project_id: None,
                updated_at: None,
                value: None,
        }
    }
}


