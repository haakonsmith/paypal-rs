//! This module has a little implementation of hateoas to make life a tad bit easier

use crate::data::common::LinkDescription;

/// This is a extension trait for Vec<LinkDescription> to make query Hateoas easier
pub trait HateoasExt {
    /// Retrieves a link from the collection by its `rel` attribute.
    ///
    /// # Arguments
    ///
    /// * `rel` - The relationship type to search for
    ///
    /// # Returns
    ///
    /// Returns `Some(href)` if a link with the matching `rel` is found, `None` otherwise.
    fn get_link(&mut self, rel: impl AsRef<str>) -> Option<String>;

    /// Gets the link with rel == 'self'
    fn get_self(&mut self) -> Option<String> {
        self.get_link("self")
    }

    /// Gets the link with rel == 'up'
    fn get_up(&mut self) -> Option<String> {
        self.get_link("up")
    }
}

impl<'a, T: Iterator<Item = &'a LinkDescription>> HateoasExt for T {
    fn get_link(&mut self, rel: impl AsRef<str>) -> Option<String> {
        let rel: &str = rel.as_ref();
        let link = self.find(|l| l.rel.as_deref() == Some(rel));

        link.map(|e| e.href.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_finding_links() {
        let links = vec![LinkDescription {
            rel: Some("test".into()),
            ..Default::default()
        }];

        assert_eq!(links.iter().get_link("test"), Some("".into()));

        let links = vec![LinkDescription {
            rel: Some("up".into()),
            ..Default::default()
        }];

        assert_eq!(links.iter().get_up(), Some("".into()));
    }
}
