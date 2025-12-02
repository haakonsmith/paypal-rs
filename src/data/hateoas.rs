//! This module has a little implementation of hateoas to make life a tad bit easier

use crate::data::common::LinkDescription;

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
