
use std::collections::{HashMap, HashSet};
use rand::Rng;

/// All possible errors of the [`UrlShortenerService`].
#[derive(Debug, PartialEq)]
pub enum ShortenerError {
    /// This error occurs when an invalid [`Url`] is provided for shortening.
    InvalidUrl,

    /// This error occurs when an attempt is made to use a slug (custom alias)
    /// that already exists.
    SlugAlreadyInUse,

    /// This error occurs when the provided [`Slug`] does not map to any existing
    /// short link.
    SlugNotFound,
}

/// A unique string (or alias) that represents the shortened version of the
/// URL.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Slug(pub String);


/// The original URL that the short link points to.
#[derive(Clone, Debug, PartialEq)]
pub struct Url(pub String);


/// Shortened URL representation.
#[derive(Debug, Clone, PartialEq)]
pub struct ShortLink {
    /// A unique string (or alias) that represents the shortened version of the
    /// URL.
    pub slug: Slug,

    /// The original URL that the short link points to.
    pub url: Url,
}

/// Statistics of the [`ShortLink`].
#[derive(Debug, Clone, PartialEq)]
pub struct Stats {
    /// [`ShortLink`] to which this [`Stats`] are related.
    pub link: ShortLink,

    /// Count of redirects of the [`ShortLink`].
    pub redirects: u64,
}

/// Commands for CQRS.
pub mod commands {
    use super::{ShortLink, ShortenerError, Slug, Url};

    /// Trait for command handlers.
    pub trait CommandHandler {
        /// Creates a new short link. It accepts the original url and an
        /// optional [`Slug`]. If a [`Slug`] is not provided, the service will generate
        /// one. Returns the newly created [`ShortLink`].
        ///
        /// ## Errors
        ///
        /// See [`ShortenerError`].
        fn handle_create_short_link(
            &mut self,
            url: Url,
            slug: Option<Slug>,
        ) -> Result<ShortLink, ShortenerError>;

        /// Processes a redirection by [`Slug`], returning the associated
        /// [`ShortLink`] or a [`ShortenerError`].
        fn handle_redirect(
            &mut self,
            slug: Slug,
        ) -> Result<ShortLink, ShortenerError>;
    }
}

/// Queries for CQRS
pub mod queries {
    use super::{ShortenerError, Slug, Stats};

    /// Trait for query handlers.
    pub trait QueryHandler {
        /// Returns the [`Stats`] for a specific [`ShortLink`], such as the
        /// number of redirects (clicks).
        ///
        /// [`ShortLink`]: super::ShortLink
        fn get_stats(&self, slug: Slug) -> Result<Stats, ShortenerError>;
    }
}

/// CQRS and Event Sourcing-based service implementation
pub struct UrlShortenerService {
    links: HashMap<Slug, ShortLink>,
    stats: HashMap<Slug, Stats>,
    slugs: HashSet<String>,
}

impl UrlShortenerService {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890";
    pub fn new() -> Self {
        Self {
            links: HashMap::new(),
            stats: HashMap::new(),
            slugs: HashSet::new(),
        }
    }
    fn generate_slug() -> Slug {
        
        let mut rng = rand::thread_rng();
        let ch =  || Self::CHARSET[rng.gen_range(0..Self::CHARSET.len())] as char;
        let slug: String = std::iter::repeat_with(ch).take(6).collect();
        Slug(slug)
    }
    fn valid_url(url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }
}

impl commands::CommandHandler for UrlShortenerService {
    fn handle_create_short_link(
        &mut self,
        url: Url,
        slug: Option<Slug>,
    ) -> Result<ShortLink, ShortenerError> {
        if !Self::valid_url(&url.0) {
            return Err(ShortenerError::InvalidUrl);
        }

        let gen_slug = match slug {
            Some(s) => {
                if self.slugs.contains(&s.0) {
                    return Err(ShortenerError::SlugAlreadyInUse);
                }
                self.slugs.insert(s.clone().0);
                s
            }
            None => {
                let unique_slug = loop {
                    //wait unique link and break
                    let slug = Self::generate_slug();
                    if !self.links.contains_key(&slug) {
                        self.slugs.insert(slug.clone().0);
                        break slug;
                    }
                };
                unique_slug
            }
        };

        let short_link = ShortLink {
            slug: gen_slug.clone(),
            url: url.clone(),
        };
        self.links.insert(gen_slug.clone(), short_link.clone());

        let stat = Stats {
            link: short_link,
            redirects: 0,
        };
        self.stats.insert(gen_slug, stat.clone());

        Ok(stat.link)
    }

    fn handle_redirect(
        &mut self,
        slug: Slug,
    ) -> Result<ShortLink, ShortenerError> {
        if let Some(link) = self.links.get(&slug) {
            if let Some(stat) = self.stats.get_mut(&slug) {
                stat.redirects += 1;
            }
            return Ok(link.clone());
        }
        Err(ShortenerError::SlugNotFound)
    }
}

impl queries::QueryHandler for UrlShortenerService {
    fn get_stats(&self, slug: Slug) -> Result<Stats, ShortenerError> {
        if let Some(stats) = self.stats.get(&slug) {
            return Ok(stats.clone());
        }
        Err(ShortenerError::SlugNotFound)
    }
}
#[cfg(test)]
mod tests
{
    use commands::CommandHandler;
    use queries::QueryHandler;
    use super::*;

    #[test]
    pub fn test()
    {
        let mut service = UrlShortenerService::new();

        let short_link = service.handle_create_short_link(Url("https://ya.ru".to_string()), None).unwrap();
        let slug = short_link.slug.clone();
        let err_link = service.handle_create_short_link(Url("http://ya.ru".to_string()), Some(slug));
        // ошибка, уже есть
        assert_eq!(err_link.err().unwrap(), ShortenerError::SlugAlreadyInUse);
        
        let _redirected_link = service.handle_redirect(short_link.slug.clone());
        let err_redirected_link = service.handle_redirect(Slug("123321".to_owned()));
        //ошибка не найдена
        assert_eq!(err_redirected_link.err().unwrap(), ShortenerError::SlugNotFound);
        let stats = service.get_stats(short_link.slug.clone()).unwrap();
        //редирект увеличился на 1
        assert_eq!(stats.redirects, 1);
        println!("{:?} {:?}", stats,  service.slugs);
    }
}
