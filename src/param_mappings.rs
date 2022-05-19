use keter_media_model::{*, media::*, userinfo::*};
use rocket::form::{self, FromFormField, ValueField};

pub enum AuthorParam {
    Id(UserKey),
    Alias(String),
}

#[rocket::async_trait]
impl<'r> FromFormField<'r> for AuthorParam {
    fn from_value(field: ValueField<'r>) -> form::Result<'r, Self> {
        if let Ok(id) = str::parse::<UserKey>(field.value) {
            Ok(AuthorParam::Id(id))
        } else {
            Ok(AuthorParam::Alias(field.value.to_owned()))
        }
    }
}

#[derive(FromFormField)]
pub enum MediaKindParam {
    #[field(value = "a")]
    #[field(value = "audio")]
    Audio,
    #[field(value = "v")]
    #[field(value = "video")]
    Video,
    #[field(value = "i")]
    #[field(value = "image")]
    Image,
}

impl From<MediaKindParam> for MediaKind {
    fn from(other: MediaKindParam) -> MediaKind {
        match other {
            MediaKindParam::Audio => MediaKind::Audio,
            MediaKindParam::Video => MediaKind::Video,
            MediaKindParam::Image => MediaKind::Image,
        }
    }
}

#[derive(FromFormField)]
pub enum FilterOrderingParam {
    #[field(value = "asc")]
    #[field(value = "ascending")]
    Ascending,
    #[field(value = "desc")]
    #[field(value = "descending")]
    Descending,
}

impl From<FilterOrderingParam> for FilterOrdering {
    fn from(other: FilterOrderingParam) -> FilterOrdering {
        match other {
            FilterOrderingParam::Ascending => FilterOrdering::Ascending,
            FilterOrderingParam::Descending => FilterOrdering::Descending,
        }
    }
}

use keter_media_model::RangeFilter;
pub fn parse_range_filter(
    min: Option<i64>,
    max: Option<i64>,
    ordering: Option<FilterOrdering>,
) -> Option<RangeFilter> {
    if min.is_some() || max.is_some() || ordering.is_some() {
        Some(RangeFilter {
            ordering,
            limits: Limits { min, max },
        })
    } else {
        None
    }
}