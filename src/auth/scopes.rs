use std::collections::HashSet;

/// Represents the various access scopes available in the Spotify Web API.
///
/// Scopes allow Spotify users to grant third-party apps specific permissions to access
/// their account data. By using scopes, users can have confidence that only the data they
/// agree to share will be accessed by the application.
///
/// For more details, see the [Spotify Scopes Guide](https://developer.spotify.com/documentation/web-api/concepts/scopes).
///
/// # Examples
/// Requesting scopes for accessing playlist information and playback control:
/// ```rust
/// use std::collections::HashSet;
/// use spotify_web_api::auth::{scopes::{self, Scope}};
///
/// let scopes = HashSet::from([
///     Scope::PlaylistReadPrivate,
///     Scope::UserModifyPlaybackState,
/// ]);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scope {
    /// Upload images to a user's profile.
    ///
    /// Allows the application to upload custom images to a user’s Spotify profile.
    UgcImageUpload,

    /// Read playback state information.
    ///
    /// Allows access to information about the user’s current playback state, such as
    /// whether something is playing and the device being used.
    UserReadPlaybackState,

    /// Modify playback state.
    ///
    /// Allows the application to control playback, including play, pause, skip, and
    /// other similar actions.
    UserModifyPlaybackState,

    /// Access information about currently playing content.
    ///
    /// Allows the application to retrieve information about the content currently
    /// being played by the user, including metadata.
    UserReadCurrentlyPlaying,

    /// Enable playback in the Web API player.
    ///
    /// Allows the application to play audio content directly using the Spotify Web API.
    Streaming,

    /// Access private playlists.
    ///
    /// Allows the application to read the user's private playlists.
    PlaylistReadPrivate,

    /// Access collaborative playlists.
    ///
    /// Allows the application to read collaborative playlists that the user is part of.
    PlaylistReadCollaborative,

    /// Modify private playlists.
    ///
    /// Allows the application to add, remove, or reorder tracks in the user’s private playlists.
    PlaylistModifyPrivate,

    /// Modify public playlists.
    ///
    /// Allows the application to add, remove, or reorder tracks in the user’s public playlists.
    PlaylistModifyPublic,

    /// Manage the user's followed artists and users.
    ///
    /// Allows the application to follow or unfollow artists and other Spotify users on behalf of the user.
    UserFollowModify,

    /// Read the user's followed artists and users.
    ///
    /// Allows the application to retrieve information about the artists and users the user is following.
    UserFollowRead,

    /// Access playback position in podcasts.
    ///
    /// Allows the application to read the user's playback position for podcast episodes.
    UserReadPlaybackPosition,

    /// Access the user's top artists and tracks.
    ///
    /// Allows the application to view information about the user’s most listened-to artists and tracks.
    UserTopRead,

    /// Access the user's recently played items.
    ///
    /// Allows the application to view information about the content the user has recently played.
    UserReadRecentlyPlayed,

    /// Manage the user's library.
    ///
    /// Allows the application to save or remove tracks and albums from the user’s Spotify library.
    UserLibraryModify,

    /// Access the user's library.
    ///
    /// Allows the application to retrieve the user's saved tracks and albums.
    UserLibraryRead,

    /// Access the user's email address.
    ///
    /// Allows the application to view the email address associated with the user’s Spotify account.
    UserReadEmail,

    /// Access the user's subscription details.
    ///
    /// Allows the application to view private information about the user’s Spotify account,
    /// such as their subscription type.
    UserReadPrivate,
}

/// Returns all playlist-related scopes.
///
/// This includes both read and modify permissions for private and public playlists.
///
/// # Scopes Included
/// - [`Scope::PlaylistReadPrivate`]
/// - [`Scope::PlaylistReadCollaborative`]
/// - [`Scope::PlaylistModifyPublic`]
/// - [`Scope::PlaylistModifyPrivate`]
pub fn playlist() -> HashSet<Scope> {
    vec![
        Scope::PlaylistReadPrivate,
        Scope::PlaylistReadCollaborative,
        Scope::PlaylistModifyPublic,
        Scope::PlaylistModifyPrivate,
    ]
    .into_iter()
    .collect()
}

/// Returns scopes for reading playlists.
///
/// This includes permissions to read private playlists and collaborative playlists.
///
/// # Scopes Included
/// - [`Scope::PlaylistReadPrivate`]
/// - [`Scope::PlaylistReadCollaborative`]
pub fn playlist_read() -> HashSet<Scope> {
    vec![Scope::PlaylistReadPrivate, Scope::PlaylistReadCollaborative]
        .into_iter()
        .collect()
}

/// Returns scopes for modifying playlists.
///
/// This includes permissions to modify both public and private playlists.
///
/// # Scopes Included
/// - [`Scope::PlaylistModifyPublic`]
/// - [`Scope::PlaylistModifyPrivate`]
pub fn playlist_modify() -> HashSet<Scope> {
    vec![Scope::PlaylistModifyPublic, Scope::PlaylistModifyPrivate]
        .into_iter()
        .collect()
}

/// Returns scopes for accessing user account details.
///
/// This includes permissions to read the user's private information and email address.
///
/// # Scopes Included
/// - [`Scope::UserReadPrivate`]
/// - [`Scope::UserReadEmail`]
pub fn user_details() -> HashSet<Scope> {
    vec![Scope::UserReadPrivate, Scope::UserReadEmail]
        .into_iter()
        .collect()
}

/// Returns scopes for managing the user's Spotify library.
///
/// This includes permissions to read and modify the user's saved tracks and albums.
///
/// # Scopes Included
/// - [`Scope::UserLibraryRead`]
/// - [`Scope::UserLibraryModify`]
pub fn user_library() -> HashSet<Scope> {
    vec![Scope::UserLibraryRead, Scope::UserLibraryModify]
        .into_iter()
        .collect()
}

/// Returns scopes for accessing the user's listening history.
///
/// This includes permissions to view the user's top artists and tracks, as well as their
/// recently played items.
///
/// # Scopes Included
/// - [`Scope::UserTopRead`]
/// - [`Scope::UserReadRecentlyPlayed`]
pub fn user_recents() -> HashSet<Scope> {
    vec![Scope::UserTopRead, Scope::UserReadRecentlyPlayed]
        .into_iter()
        .collect()
}

/// Returns scopes for managing the user's followed artists and users.
///
/// This includes permissions to read and modify the list of followed artists and users.
///
/// # Scopes Included
/// - [`Scope::UserFollowRead`]
/// - [`Scope::UserFollowModify`]
pub fn user_follow() -> HashSet<Scope> {
    vec![Scope::UserFollowRead, Scope::UserFollowModify]
        .into_iter()
        .collect()
}

/// Returns scopes for accessing and controlling user playback.
///
/// This includes permissions for reading playback state, controlling playback, and
/// enabling streaming via the Web API.
///
/// # Scopes Included
/// - [`Scope::UserReadPlaybackPosition`]
/// - [`Scope::UserReadPlaybackState`]
/// - [`Scope::UserReadCurrentlyPlaying`]
/// - [`Scope::UserModifyPlaybackState`]
/// - [`Scope::Streaming`]
pub fn user_playback() -> HashSet<Scope> {
    vec![
        Scope::UserReadPlaybackPosition,
        Scope::UserReadPlaybackState,
        Scope::UserReadCurrentlyPlaying,
        Scope::UserModifyPlaybackState,
        Scope::Streaming,
    ]
    .into_iter()
    .collect()
}

/// Returns a set containing all available Spotify API scopes.
///
/// This function provides a comprehensive set of permissions, granting access to all
/// features supported by the Spotify Web API. It is useful for applications that
/// require full access to a user's account and data.
///
/// # Scopes Included
/// This function includes all defined [`Scope`] variants.
pub fn all() -> HashSet<Scope> {
    vec![
        Scope::UgcImageUpload,
        Scope::UserReadPlaybackState,
        Scope::UserModifyPlaybackState,
        Scope::UserReadCurrentlyPlaying,
        Scope::Streaming,
        Scope::PlaylistReadPrivate,
        Scope::PlaylistReadCollaborative,
        Scope::PlaylistModifyPrivate,
        Scope::PlaylistModifyPublic,
        Scope::UserFollowModify,
        Scope::UserFollowRead,
        Scope::UserReadPlaybackPosition,
        Scope::UserTopRead,
        Scope::UserReadRecentlyPlayed,
        Scope::UserLibraryModify,
        Scope::UserLibraryRead,
        Scope::UserReadEmail,
        Scope::UserReadPrivate,
    ]
    .into_iter()
    .collect()
}

/// Converts a set of `Scope` values into a space-separated string.
///
/// This function takes a `HashSet` of `Scope` values and returns a string representation,
/// with each scope's name separated by a space. This format is commonly used in OAuth
/// authorization requests where scopes need to be specified as a single string.
///
/// # Parameters
/// - `set`: A reference to a `HashSet` of `Scope` values to be converted.
///
/// # Returns
/// A `String` containing the space-separated names of the scopes.
///
/// # Example
/// ```rust
/// use std::collections::HashSet;
/// use spotify_web_api::auth::scopes::{self, Scope};
///
/// let scopes = HashSet::from([Scope::UserReadEmail, Scope::UserReadPrivate]);
/// let result = scopes::to_string(&scopes);
///
/// assert!(result.contains("user-read-email"));
/// assert!(result.contains("user-read-private"));
/// ```
///
/// # Notes
/// The order of the scopes in the resulting string is not guaranteed, as `HashSet` does
/// not maintain insertion order.
pub fn to_string(set: &HashSet<Scope>) -> String {
    set.iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::UgcImageUpload => "ugc-image-upload",
            Self::UserReadPlaybackState => "user-read-playback-state",
            Self::UserModifyPlaybackState => "user-modify-playback-state",
            Self::UserReadCurrentlyPlaying => "user-read-currently-playing",
            Self::Streaming => "streaming",
            Self::PlaylistReadPrivate => "playlist-read-private",
            Self::PlaylistReadCollaborative => "playlist-read-collaborative",
            Self::PlaylistModifyPrivate => "playlist-modify-private",
            Self::PlaylistModifyPublic => "playlist-modify-public",
            Self::UserFollowModify => "user-follow-modify",
            Self::UserFollowRead => "user-follow-read",
            Self::UserReadPlaybackPosition => "user-read-playback-position",
            Self::UserTopRead => "user-top-read",
            Self::UserReadRecentlyPlayed => "user-read-recently-played",
            Self::UserLibraryModify => "user-library-modify",
            Self::UserLibraryRead => "user-library-read",
            Self::UserReadEmail => "user-read-email",
            Self::UserReadPrivate => "user-read-private",
        };

        write!(f, "{s}")
    }
}

impl TryFrom<&str> for Scope {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let x = match value {
            "ugc-image-upload" => Self::UgcImageUpload,
            "user-read-playback-state" => Self::UserReadPlaybackState,
            "user-modify-playback-state" => Self::UserModifyPlaybackState,
            "user-read-currently-playing" => Self::UserReadCurrentlyPlaying,
            "streaming" => Self::Streaming,
            "playlist-read-private" => Self::PlaylistReadPrivate,
            "playlist-read-collaborative" => Self::PlaylistReadCollaborative,
            "playlist-modify-private" => Self::PlaylistModifyPrivate,
            "playlist-modify-public" => Self::PlaylistModifyPublic,
            "user-follow-modify" => Self::UserFollowModify,
            "user-follow-read" => Self::UserFollowRead,
            "user-read-playback-position" => Self::UserReadPlaybackPosition,
            "user-top-read" => Self::UserTopRead,
            "user-read-recently-played" => Self::UserReadRecentlyPlayed,
            "user-library-modify" => Self::UserLibraryModify,
            "user-library-read" => Self::UserLibraryRead,
            "user-read-email" => Self::UserReadEmail,
            "user-read-private" => Self::UserReadPrivate,
            _ => return Err(()),
        };

        Ok(x)
    }
}

impl From<Scope> for HashSet<Scope> {
    fn from(val: Scope) -> Self {
        vec![val].into_iter().collect()
    }
}

impl From<Scope> for Option<HashSet<Scope>> {
    fn from(val: Scope) -> Self {
        Some(val.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_kebab_case() {
        let scope = Scope::UserReadPlaybackState;
        assert_eq!("user-read-playback-state", scope.to_string());
    }
}
