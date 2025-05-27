//! Spotify Web API data model.

pub mod albums;
pub mod artists;
pub mod audiobooks;
pub mod categories;
pub mod chapters;
pub mod episodes;
pub mod genres;
pub mod id;
pub mod markets;
pub mod misc;
pub mod player;
pub mod playlists;
pub mod search;
pub mod shows;
pub mod token;
pub mod tracks;
pub mod users;

pub use albums::*;
pub use artists::*;
pub use audiobooks::*;
pub use categories::*;
pub use chapters::*;
pub use episodes::*;
pub use genres::*;
pub use id::*;
pub use markets::*;
pub use misc::*;
pub use player::*;
pub use playlists::*;
pub use search::*;
pub use shows::*;
pub use token::*;
pub use tracks::*;
pub use users::*;
