/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.4.3
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// Organization : B2B SSO Organization



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Organization {
    /// The organization's creation date.
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "domains")]
    pub domains: Vec<String>,
    /// The organization's ID.
    #[serde(rename = "id")]
    pub id: String,
    /// The organization's human-readable label.
    #[serde(rename = "label")]
    pub label: String,
    /// The project's ID.
    #[serde(rename = "project_id")]
    pub project_id: String,
    /// The last time the organization was updated.
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}


impl Organization {
    /// B2B SSO Organization
    pub fn new(created_at: String, domains: Vec<String>, id: String, label: String, project_id: String, updated_at: String) -> Organization {
        Organization {
                created_at,
                domains,
                id,
                label,
                project_id,
                updated_at,
        }
    }
}


