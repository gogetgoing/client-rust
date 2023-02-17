/*
 * Ory APIs
 *
 * Documentation for all public and administrative Ory APIs. Administrative APIs can only be accessed with a valid Personal Access Token. Public APIs are mostly used in browsers. 
 *
 * The version of the OpenAPI document: v1.1.14
 * Contact: support@ory.sh
 * Generated by: https://openapi-generator.tech
 */

/// OAuth2RedirectTo : Contains a redirect URL used to complete a login, consent, or logout request.



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OAuth2RedirectTo {
    /// RedirectURL is the URL which you should redirect the user's browser to once the authentication process is completed.
    #[serde(rename = "redirect_to")]
    pub redirect_to: String,
}


impl OAuth2RedirectTo {
    /// Contains a redirect URL used to complete a login, consent, or logout request.
    pub fn new(redirect_to: String) -> OAuth2RedirectTo {
        OAuth2RedirectTo {
                redirect_to,
        }
    }
}


