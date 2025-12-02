//! This module has a little implementation of hateoas to make life a tad bit easier

use crate::{data::common::LinkDescription, endpoint::Endpoint};

/// This is a extension trait for Vec<LinkDescription> to make query Hateoas easier
pub trait HateoasExt<'a> {
    /// Retrieves a link from the collection by its `rel` attribute.
    ///
    /// # Arguments
    ///
    /// * `rel` - The relationship type to search for
    ///
    /// # Returns
    ///
    /// Returns `Some(href)` if a link with the matching `rel` is found, `None` otherwise.
    fn get_link(&'a mut self, rel: impl AsRef<str>) -> Option<&'a LinkDescription>;

    /// Gets the link with rel == 'self'
    fn get_self(&'a mut self) -> Option<&'a LinkDescription> {
        self.get_link("self")
    }

    /// Gets the link with rel == 'up'
    fn get_up(&'a mut self) -> Option<&'a LinkDescription> {
        self.get_link("up")
    }
}

impl<'a, T: Iterator<Item = &'a LinkDescription>> HateoasExt<'a> for T {
    fn get_link(&'a mut self, rel: impl AsRef<str>) -> Option<&'a LinkDescription> {
        let rel: &str = rel.as_ref();
        let link = self.find(|l| l.rel.as_deref() == Some(rel));

        link
    }
}

impl Endpoint for LinkDescription {
    type Query = ();

    type Body = ();

    type Response = serde_json::Value;

    fn relative_path(&self) -> std::borrow::Cow<'_, str> {
        std::borrow::Cow::Borrowed(&self.href)
    }

    fn method(&self) -> reqwest::Method {
        match self.method.unwrap_or(super::common::LinkMethod::Get) {
            super::common::LinkMethod::Get => reqwest::Method::GET,
            super::common::LinkMethod::Post => reqwest::Method::POST,
            super::common::LinkMethod::Put => reqwest::Method::PUT,
            super::common::LinkMethod::Delete => reqwest::Method::DELETE,
            super::common::LinkMethod::Head => reqwest::Method::HEAD,
            super::common::LinkMethod::Connect => reqwest::Method::CONNECT,
            super::common::LinkMethod::Options => reqwest::Method::OPTIONS,
            super::common::LinkMethod::Patch => reqwest::Method::PATCH,
        }
    }
}
