use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum IdError {
    #[error("The ID is not in correct format.")]
    InvalidFormat,

    #[error("The ID is not the correct length. Got {got}, expected {expected}.")]
    InvalidLength { got: usize, expected: usize },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IdType {
    User,
    Album,
    Artist,
    Playlist,
    Track,
    Show,
    Episode,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextType {
    Album(AlbumId),
    Artist(ArtistId),
    Playlist(PlaylistId),
    Show(ShowId),
}

impl ContextType {
    pub fn uri(&self) -> String {
        match self {
            Self::Album(id) => id.uri(),
            Self::Artist(id) => id.uri(),
            Self::Playlist(id) => id.uri(),
            Self::Show(id) => id.uri(),
        }
    }
}

impl std::fmt::Display for IdType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::User => "user",
            Self::Album => "album",
            Self::Artist => "artist",
            Self::Playlist => "playlist",
            Self::Track => "track",
            Self::Show => "show",
            Self::Episode => "episode",
        };
        write!(f, "{s}")
    }
}

macro_rules! impl_ids {
    ($(($struct_name:ident, $id_type:ident, $type_name:expr)),* $(,)?) => {
        $(
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct $struct_name(String);

            impl $struct_name {
                pub fn from_id<S>(id: S) -> Result<Self, IdError> where S: Into<String> {
                    let id = id.into();

                    if $type_name == "user" {
                        return Ok($struct_name(id.into()));
                    }

                    let id_len = id.len();

                    match id_len {
                        22 if is_base62(&id) => Ok($struct_name(id.into())),
                        22 => Err(IdError::InvalidFormat),
                        _ => Err(IdError::InvalidLength {
                        	got: id_len,
                         	expected: 22,
                        }),
                    }
                }

                pub fn from_uri<S>(uri: S) -> Result<Self, IdError> where S: Into<String> {
					let uri = uri.into();
					let prefix = format!("spotify:{}:", $type_name);

					let id = uri.strip_prefix(&prefix).ok_or(IdError::InvalidFormat)?;

					if $type_name == "user" {
                        return Ok($struct_name(id.into()));
                    }

					let id_len = id.len();

					match id_len {
						22 if is_base62(&id) => Ok($struct_name(id.into())),
						22 => Err(IdError::InvalidFormat),
						_ => Err(IdError::InvalidLength {
							got: id_len,
							expected: 22,
						}),
					}
				}

                /// The base-62 identifier found at the end of the Spotify URI (see above) for an artist, track, album, playlist, etc.
                /// Unlike a Spotify URI, a Spotify ID does not clearly identify the type of resource; that information is provided elsewhere in the call.
                pub fn id(&self) -> &str {
					&self.0
				}

				/// The type of the resource.
				pub fn _type(&self) -> IdType {
					IdType::$id_type
				}

				/// The resource identifier of, for example, an artist, album or track.
				pub fn uri(&self) -> String {
        			format!("spotify:{}:{}", self._type(), self.id())
    			}
            }
        )*
    }
}

impl_ids![
    (PlaylistId, Playlist, "playlist"),
    (TrackId, Track, "track"),
    (AlbumId, Album, "album"),
    (ArtistId, Artist, "artist"),
    (ShowId, Show, "show"),
    (EpisodeId, Episode, "episode"),
    (UserId, User, "user"),
];

#[inline(always)]
fn is_base62(s: &str) -> bool {
    s.chars().all(|c| c.is_ascii_alphanumeric())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let id = "6rqhFgbbKwnb9MLmUQDhG6";
        let track_id = TrackId::from_id(id).unwrap();
        assert_eq!(track_id.id(), id);

        let bad_id = "6rqhFgbbKwnb9MLmUQDhG";
        let track_id = TrackId::from_id(bad_id);
        assert_eq!(
            track_id,
            Err(IdError::InvalidLength {
                got: 21,
                expected: 22
            })
        );
    }

    #[test]
    fn test_id_from_uri() {
        let uri = "spotify:track:6rqhFgbbKwnb9MLmUQDhG6";
        let track_id = TrackId::from_uri(uri).unwrap();
        assert_eq!(track_id.id(), "6rqhFgbbKwnb9MLmUQDhG6");

        let bad_uri = "spotify:track:6rqhFgbbKwnb9MLmUQDhG";
        let track_id = TrackId::from_uri(bad_uri);
        assert_eq!(
            track_id,
            Err(IdError::InvalidLength {
                got: 21,
                expected: 22
            })
        );
    }
}
