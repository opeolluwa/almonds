use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBookmark {
    pub title: String,
    pub url: String,
    pub tag: String,
    pub workspace_identifier: Option<Uuid>,
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
            workspace_identifier: b.workspace_identifier,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateBookmark {
    pub title: Option<String>,
    pub url: Option<String>,
    pub tag: Option<String>,
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
