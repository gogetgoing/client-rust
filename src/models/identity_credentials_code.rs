/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.15.7
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// IdentityCredentialsCode : CredentialsCode represents a one time login/registration code
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IdentityCredentialsCode {
    #[serde(rename = "addresses", skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<models::IdentityCredentialsCodeAddress>>,
}

impl IdentityCredentialsCode {
    /// CredentialsCode represents a one time login/registration code
    pub fn new() -> IdentityCredentialsCode {
        IdentityCredentialsCode {
            addresses: None,
        }
    }
}

