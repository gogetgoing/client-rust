/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.14.4
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// UpdateSettingsFlowWithTotpMethod : Update Settings Flow with TOTP Method
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateSettingsFlowWithTotpMethod {
    /// CSRFToken is the anti-CSRF token
    #[serde(rename = "csrf_token", skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<String>,
    /// Method  Should be set to \"totp\" when trying to add, update, or remove a totp pairing.
    #[serde(rename = "method")]
    pub method: String,
    /// ValidationTOTP must contain a valid TOTP based on the
    #[serde(rename = "totp_code", skip_serializing_if = "Option::is_none")]
    pub totp_code: Option<String>,
    /// UnlinkTOTP if true will remove the TOTP pairing, effectively removing the credential. This can be used to set up a new TOTP device.
    #[serde(rename = "totp_unlink", skip_serializing_if = "Option::is_none")]
    pub totp_unlink: Option<bool>,
    /// Transient data to pass along to any webhooks
    #[serde(rename = "transient_payload", skip_serializing_if = "Option::is_none")]
    pub transient_payload: Option<serde_json::Value>,
}

impl UpdateSettingsFlowWithTotpMethod {
    /// Update Settings Flow with TOTP Method
    pub fn new(method: String) -> UpdateSettingsFlowWithTotpMethod {
        UpdateSettingsFlowWithTotpMethod {
            csrf_token: None,
            method,
            totp_code: None,
            totp_unlink: None,
            transient_payload: None,
        }
    }
}

