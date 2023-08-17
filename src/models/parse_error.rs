/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.48
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ParseError {
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<Box<crate::models::SourcePosition>>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<Box<crate::models::SourcePosition>>,
}

impl Default for ParseError {
    fn default() -> Self {
        Self::new()
    }
}

impl ParseError {
    pub fn new() -> ParseError {
        ParseError {
                end: None,
                message: None,
                start: None,
        }
    }
}


