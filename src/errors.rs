//! Errors created by this crate.
use crate::data::common::LinkDescription;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

/// This comes from the ass backwards reality that is paypal rest api.
/// Basically they lie and say they will return a vec but instead send back a single item
///
/// This impls Default, the default implementation returns a Many(Vec::new()).
/// Which seems like a reasonable representation but :shrug:
///
/// Ultimately you probably wanna interface with this using the to_vec() method
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(bound = "T: Serialize + DeserializeOwned")]
#[serde(untagged)]
pub enum OneOrMany<T: DeserializeOwned> {
    /// A single value was returned from the api
    One(T),
    /// None or Many values were returned.
    Many(Vec<T>),
}

impl<T: DeserializeOwned + Clone> OneOrMany<T> {
    /// Flattens the types into a Vec representation. Will clone
    pub fn to_vec(&self) -> Vec<T> {
        match self {
            OneOrMany::One(x) => vec![x.clone()],
            OneOrMany::Many(items) => items.clone(),
        }
    }
}

impl<T: DeserializeOwned> Default for OneOrMany<T> {
    fn default() -> Self {
        Self::Many(vec![])
    }
}

/// A paypal api response error.
#[derive(Debug, Serialize, Deserialize)]
pub struct PaypalError {
    /// The error name.
    pub name: String,
    /// The error message.
    pub message: Option<String>,
    /// Paypal debug id
    pub debug_id: Option<String>,
    /// Error details
    #[serde(default)]
    pub details: OneOrMany<HashMap<String, String>>,
    /// Only available on Identity errors
    pub error: Option<String>,
    /// Only available on Identity errors
    pub error_description: Option<String>,
    /// Links with more information about the error.
    #[serde(default)]
    pub links: Vec<LinkDescription>,
}

impl fmt::Display for PaypalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for PaypalError {}

/// A response error, it may be paypal related or an error related to the http request itself.
#[derive(Debug, thiserror::Error)]
pub enum ResponseError {
    /// A paypal api error.
    #[error("PayPal error {0}")]
    ApiError(#[from] PaypalError),
    /// A serde error
    #[error("Serde error {0}")]
    Serde(#[from] serde_json::Error),
    /// A http error.
    #[error("Http error {0}")]
    HttpError(#[from] reqwest::Error),
}

/// When a currency is invalid.
#[derive(Debug)]
pub struct InvalidCurrencyError(pub String);

impl fmt::Display for InvalidCurrencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} is not a valid currency", self.0)
    }
}

impl Error for InvalidCurrencyError {}

/// When a country is invalid.
#[derive(Debug)]
pub struct InvalidCountryError(pub String);

impl fmt::Display for InvalidCountryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} is not a valid country", self.0)
    }
}

impl Error for InvalidCountryError {}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_decoding() {
        serde_json::from_value::<PaypalError>(json!({
                  "name":"INVALID_REQUEST",
                  "message":"Request is not well-formed, syntactically incorrect, or violates schema.",
                  "debug_id":"f896367ed3b42",
                  "information_link":"",
                  "details":{
                    "issue":" INVALID_PARAMETER_SYNTAX",
                    "description":"api_integration_preference is mandatory when operation is API_INTEGRATION.",
                    "field":"/operations/0",
                    "location":"body"
                  },
                  "links":[],
                  "sys":{
                    "links":{
                      "jsBaseUrl":"https://api-m.sandbox.paypal.com/js",
                      "cssBaseUrl":"https://api-m.sandbox.paypal.com/css",
                      "templateBaseUrl":"https://api-m.sandbox.paypal.com/templates",
                      "resourceBaseUrl":"https://api-m.sandbox.paypal.com"},
                    "pageInfo":{
                      "date":"Nov 19, 2025 02:30:08 -0 8:00",
                      "hostName":"rZJvnqaaQhLn/nmWT8cSUjOx898qoYZ0pO7iw8eqgZjo3CyMhqQpbsz9WA4gBCrqQHOe7b4a cg4",
                      "rlogId":"rZJvnqaaQhLn%2FnmWT8cSUvl%2BhVU1VdNAVDKJ5S9HXX%2Bw%2FCbJ%2BGKaXb5ubGp1hdQRK s4ug1gnxWPOd5unF8U%2FcWo%2FPSWe6ohg1RR9i45iMUY_19a9baa1b9a",
                      "script":"node",
                      "debug":null
                    },
                    "tracking":{
                      "fpti":{
                        "dataString":"pgrp=partnselleronbnodeserv%2F.dust&page=partnselleronbnodeserv%2F.dust&qual=&comp=partnselleronbnodeserv&tsrce=partnselleronbnodeserv&cu=0&ef_policy=&c_prefs=&pxpguid=&pgst=1763548208026&calc=f896367ed3b42&csci=f1fd1839da674009ac962140c62b2747&nsid=&pgtf=Nodejs&s=ci&env=sandbox",
                        "varName":"pta",
                        "name":"pta",
                        "jsURL":"https://www.paypalobjects.com",
                        "serverURL":"https://t.paypal.com/ts"
                      }
                    }
                  }
                }
        )).unwrap();
    }
}
