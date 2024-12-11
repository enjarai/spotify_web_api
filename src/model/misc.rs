use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ItemType {
    User,
    Album,
    Artist,
    Playlist,
    Track,
    Show,
    Episode,
    Audiobook,
    Unknown,
    Chapter,
    Collection,
}

impl std::fmt::Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::User => "user",
            Self::Album => "album",
            Self::Artist => "artist",
            Self::Playlist => "playlist",
            Self::Track => "track",
            Self::Show => "show",
            Self::Episode => "episode",
            Self::Audiobook => "audiobook",
            Self::Unknown => "unknown",
            Self::Chapter => "chapter",
            Self::Collection => "collection",
        };

        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    /// The source URL of the image.
    pub url: String,

    /// The image height in pixels.
    pub height: Option<u16>,

    /// The image width in pixels.
    pub width: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Icon {
    /// The source URL of the image.
    pub url: String,

    /// The image height in pixels.
    pub height: Option<u16>,

    /// The image width in pixels.
    pub width: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoThumbnail {
    // The source URL of the image.
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalUrls {
    /// The [Spotify URL](https://developer.spotify.com/documentation/web-api/concepts/spotify-uris-ids) for the object.
    pub spotify: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalIds {
    /// [International Standard Recording Code](http://en.wikipedia.org/wiki/International_Standard_Recording_Code)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isrc: Option<String>,

    /// [International Article Number](http://en.wikipedia.org/wiki/International_Article_Number_%28EAN%29)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ean: Option<String>,

    /// [Universal Product Code](http://en.wikipedia.org/wiki/Universal_Product_Code)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upc: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Followers {
    /// This will always be set to null, as the Web API does not support it at the moment.
    pub href: Option<String>,

    /// The total number of followers.
    pub total: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseDatePrecision {
    Year,
    Month,
    Day,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Restrictions {
    /// The reason for the restriction.
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CopyrightType {
    /// The copyright.
    C,

    /// The sound recording (performance) copyright.
    P,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Copyright {
    /// The copyright text for this content.
    pub text: String,

    /// The type of copyright: C = the copyright, P = the sound recording (performance) copyright.
    #[serde(rename = "type")]
    pub type_: CopyrightType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page<T> {
    /// A link to the Web API endpoint returning the full result of the request.
    pub href: String,

    /// The maximum number of items in the response (as set in the query or by default).
    pub limit: usize,

    /// URL to the next page of items
    pub next: Option<String>,

    /// The offset of the items returned (as set in the query or by default).
    pub offset: usize,

    /// URL to the previous page of items.
    pub previous: Option<String>,

    /// The total number of items available to return.
    pub total: usize,

    pub items: Vec<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cursors {
    /// The cursor to use as key to find the next page of items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,

    /// The cursor to use as key to find the previous page of items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResumePoint {
    /// Whether or not the episode has been fully played by the user.
    pub fully_played: bool,

    /// The user's most recent position in the episode in milliseconds.
    pub resume_position_ms: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum TrackItem {
    Track(super::Track),
    Episode(super::Episode),
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TimeRange {
    /// Calculated from ~1 year of data and including all new data as it becomes available.
    LongTerm,

    /// Approximately last 6 months.
    #[default]
    MediumTerm,

    /// Approximately last 4 weeks
    ShortTerm,
}

impl std::fmt::Display for TimeRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LongTerm => write!(f, "long_term"),
            Self::MediumTerm => write!(f, "medium_term"),
            Self::ShortTerm => write!(f, "short_term"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TopItemType {
    Artists,
    Tracks,
}

impl std::fmt::Display for TopItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Artists => write!(f, "artists"),
            Self::Tracks => write!(f, "tracks"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[allow(clippy::large_enum_variant)]
pub enum TopItem {
    Artist(super::Artist),
    Track(super::Track),
}

pub type TopItems = Page<TopItem>;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FollowedArtistsType {
    Artist,
}

impl std::fmt::Display for FollowedArtistsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Artist => write!(f, "artist"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum FollowType {
    Artist,
    User,
}

impl std::fmt::Display for FollowType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Artist => "artist",
            Self::User => "user",
        };

        write!(f, "{s}")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum IncludeExternalType {
    Audio,
}

impl std::fmt::Display for IncludeExternalType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Audio => "audio",
        };

        write!(f, "{s}")
    }
}
