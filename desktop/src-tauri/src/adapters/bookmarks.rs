use almond_kernel::adapters::meta::RequestMeta;
use sanitizer::prelude::*;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Sanitizer, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookmark {
    #[sanitizer(trim)]
    pub title: String,
    #[sanitizer(trim)]
    #[validate(url)]
    pub url: String,
    #[sanitizer(trim)]
    pub tag: String,
    pub meta: Option<RequestMeta>,
}

impl From<CreateBookmark> for almond_kernel::adapters::bookmarks::CreateBookmark {
    fn from(b: CreateBookmark) -> Self {
        use almond_kernel::adapters::bookmarks::BookmarkTag;

        let tag = match b.tag.as_str() {
            "development" => BookmarkTag::Development,
            "inspiration" => BookmarkTag::Inspiration,
            "design" => BookmarkTag::Design,
            _ => BookmarkTag::Research,
        };

        Self {
            title: b.title,
            url: b.url,
            tag,
        }
    }
}

#[derive(Debug, Deserialize, Sanitizer, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBookmark {
    #[sanitizer(trim)]
    pub title: Option<String>,
    #[sanitizer(trim)]
    #[validate(url)]
    pub url: Option<String>,
    #[sanitizer(trim)]
    pub tag: Option<String>,
    pub meta: Option<RequestMeta>,
}

impl From<UpdateBookmark> for almond_kernel::adapters::bookmarks::UpdateBookmark {
    fn from(b: UpdateBookmark) -> Self {
        use almond_kernel::adapters::bookmarks::BookmarkTag;

        let tag = b.tag.as_deref().map(|t| match t {
            "development" => BookmarkTag::Development,
            "inspiration" => BookmarkTag::Inspiration,
            "design" => BookmarkTag::Design,
            _ => BookmarkTag::Research,
        });

        Self {
            title: b.title,
            url: b.url,
            tag,
        }
    }
}
