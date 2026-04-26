use boltffi::*;
use chrono::{DateTime, FixedOffset};

custom_type!(
  pub  DateTimeWithTimeZone,
    remote = DateTime<FixedOffset>,
    repr = i64,
    into_ffi = |dt: &DateTime<FixedOffset>| dt.timestamp_millis(),
    try_from_ffi = |millis: i64| {
        DateTime::from_timestamp_millis(millis)
            .map(|dt_utc| dt_utc.fixed_offset())
            .ok_or(CustomTypeConversionError)
    },
);

custom_type!(
  pub  Date,
    remote = chrono::NaiveDate,
    repr = i64,
    into_ffi = |dt: &chrono::NaiveDate| dt.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp_millis(),
    try_from_ffi = |millis: i64| {
        chrono::DateTime::from_timestamp_millis(millis)
            .map(|dt_utc| dt_utc.date_naive())
            .ok_or(CustomTypeConversionError)
    },
);

custom_type!(
  pub  Time,
    remote = chrono::NaiveTime,
    repr = String,
    into_ffi = |dt: &chrono::NaiveTime| dt.format("%H:%M:%S").to_string(),
    try_from_ffi = |millis: String| {
        chrono::DateTime:: parse_from_str(&millis, "%H:%M:%S"    )
            .map(|dt_utc| dt_utc.time())

            .map_err(|_| CustomTypeConversionError)
    },
);

custom_type!(
  pub  Json,
    remote = serde_json::Value,
    repr = String,
    into_ffi = |dt: &serde_json::Value| dt.to_string(),
    try_from_ffi = |millis: String| {
        serde_json::from_str(&millis)
            .map_err(|_| CustomTypeConversionError)
    },
);

#[cfg(feature = "postgres")]
pub mod tag_enum_ffi {
    use std::fmt::Display;

    use crate::entities::sea_orm_active_enums::Tag;
    use boltffi::*;

    impl Display for Tag {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = match self {
                Tag::Development => "development",
                Tag::Inspiration => "inspiration",
                Tag::Design => "design",
                Tag::Research => "research",
            };
            write!(f, "{}", s)
        }
    }

    impl TryFrom<String> for Tag {
        type Error = ();

        fn try_from(value: String) -> Result<Self, Self::Error> {
            match value.as_str() {
                "development" => Ok(Tag::Development),
                "inspiration" => Ok(Tag::Inspiration),
                "design" => Ok(Tag::Design),
                "research" => Ok(Tag::Research),
                _ => Err(()),
            }
        }
    }
    custom_type!(
      pub  Tag,
        remote = Tag,
        repr = String,
        into_ffi = |tag: &Tag| tag.to_string(),
        try_from_ffi = |s: String| {
            Tag::try_from(s)
                .map_err(|_| CustomTypeConversionError)
        },
    );
}

#[cfg(feature = "postgres")]
pub mod item_type_enum_ffi {
    use std::fmt::Display;

    use crate::entities::sea_orm_active_enums::ItemType;
    use boltffi::*;

    impl Display for ItemType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = match self {
                ItemType::Todo => "todo",
                ItemType::Note => "note",
                ItemType::Reminder => "reminder",
                ItemType::Snippet => "snippet",
                ItemType::Bookmark => "bookmark",
            };
            write!(f, "{}", s)
        }
    }

    impl TryFrom<String> for ItemType {
        type Error = ();

        fn try_from(value: String) -> Result<Self, Self::Error> {
            match value.as_str() {
                "todo" => Ok(ItemType::Todo),
                "note" => Ok(ItemType::Note),
                "reminder" => Ok(ItemType::Reminder),
                "snippet" => Ok(ItemType::Snippet),
                "bookmark" => Ok(ItemType::Bookmark),
                _ => Err(()),
            }
        }
    }
    custom_type!(
      pub  ItemType,
        remote = ItemType,
        repr = String,
        into_ffi = |item_type: &ItemType| item_type.to_string(),
        try_from_ffi = |s: String| {
            ItemType::try_from(s)
                .map_err(|_| CustomTypeConversionError)
        },
    );
}
