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
pub struct SetProject {
    #[serde(rename = "cors_admin")]
    pub cors_admin: Box<crate::models::ProjectCors>,
    #[serde(rename = "cors_public")]
    pub cors_public: Box<crate::models::ProjectCors>,
    /// The name of the project.
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "services")]
    pub services: Box<crate::models::ProjectServices>,
}


impl SetProject {
    pub fn new(cors_admin: crate::models::ProjectCors, cors_public: crate::models::ProjectCors, name: String, services: crate::models::ProjectServices) -> SetProject {
        SetProject {
                cors_admin: Box::new(cors_admin),
                cors_public: Box::new(cors_public),
                name,
                services: Box::new(services),
        }
    }
}


