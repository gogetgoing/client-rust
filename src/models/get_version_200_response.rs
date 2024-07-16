/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.14.0
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetVersion200Response {
    /// The version of Ory Kratos.
    #[serde(rename = "version")]
    pub version: String,
}

impl GetVersion200Response {
    pub fn new(version: String) -> GetVersion200Response {
        GetVersion200Response {
            version,
        }
    }
}

