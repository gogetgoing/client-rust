/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.12.1
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorBrowserLocationChangeRequired {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<models::ErrorGeneric>>,
    /// Points to where to redirect the user to next.
    #[serde(rename = "redirect_browser_to", skip_serializing_if = "Option::is_none")]
    pub redirect_browser_to: Option<String>,
}

impl ErrorBrowserLocationChangeRequired {
    pub fn new() -> ErrorBrowserLocationChangeRequired {
        ErrorBrowserLocationChangeRequired {
            error: None,
            redirect_browser_to: None,
        }
    }
}

