# Endpoints

Format: `[x]` `[Title]` `[Method]` `[Endpoint]` `[Spotify Docs]`

## Albums

- [x] Get Album `GET` `/albums/{id}` [get-an-album](https://developer.spotify.com/documentation/web-api/reference/get-an-album)
- [x] Get Several Albums `GET` `/albums` [get-multiple-albums](https://developer.spotify.com/documentation/web-api/reference/get-multiple-albums)
- [x] Get Album Tracks `GET` `/albums/{id}/tracks` [get-an-albums-tracks](https://developer.spotify.com/documentation/web-api/reference/get-an-albums-tracks)
- [ ] Get User's Saved Albums `GET` `me/albums` [get-users-saved-albums](https://developer.spotify.com/documentation/web-api/reference/get-users-saved-albums)
- [ ] Save Albums for Current User `PUT` `me/albums` [save-albums-user](https://developer.spotify.com/documentation/web-api/reference/save-albums-user)
- [ ] Remove User's Saved Albums `DELETE` `me/albums` [remove-albums-user](https://developer.spotify.com/documentation/web-api/reference/remove-albums-user)
- [x] Check User's Saved Albums `GET` `me/albums/contains` [check-users-saved-albums](https://developer.spotify.com/documentation/web-api/reference/check-users-saved-albums)
- [ ] Get New Releases `GET` `/browse/new-releases` [get-new-releases](https://developer.spotify.com/documentation/web-api/reference/get-new-releases)

## Artists

- [x] Get Artist `GET` `/artists/{id}` [get-an-artist](https://developer.spotify.com/documentation/web-api/reference/get-an-artist)
- [ ] Get Several Artists `GET` `/artists` [get-multiple-artists](https://developer.spotify.com/documentation/web-api/reference/get-multiple-artists)
- [ ] Get Artist's Albums `GET` `/artists/{id}/albums` [get-an-artists-albums](https://developer.spotify.com/documentation/web-api/reference/get-an-artists-albums)
- [ ] Get Artist's Top Tracks `GET` `/artists/{id}/top-tracks` [get-an-artists-top-tracks](https://developer.spotify.com/documentation/web-api/reference/get-an-artists-top-tracks)
- [ ] Get Artist's Related Artists `GET` `/artists/{id}/related-artists` [get-an-artists-related-artists](https://developer.spotify.com/documentation/web-api/reference/get-an-artists-related-artists)

## Audiobooks

- [ ] Get an Audiobook `GET` `/audiobooks/{id}` [get-an-audiobook](https://developer.spotify.com/documentation/web-api/reference/get-an-audiobook)
- [ ] Get Several Audiobooks `GET` `/audiobooks` [get-multiple-audiobooks](https://developer.spotify.com/documentation/web-api/reference/get-multiple-audiobooks)
- [ ] Get Audiobook Chapters `GET` `/audiobooks/{id}/chapters` [get-audiobook-chapters](https://developer.spotify.com/documentation/web-api/reference/get-audiobook-chapters)
- [ ] Get User's Saved Audiobooks `GET` `me/audiobooks` [get-users-saved-audiobooks](https://developer.spotify.com/documentation/web-api/reference/get-users-saved-audiobooks)
- [ ] Save Audiobooks for Current User `PUT` `me/audiobooks` [save-audiobooks-user](https://developer.spotify.com/documentation/web-api/reference/save-audiobooks-user)
- [ ] Remove User's Saved Audiobooks `DELETE` `me/audiobooks` [remove-audiobooks-user](https://developer.spotify.com/documentation/web-api/reference/remove-audiobooks-user)
- [ ] Check User's Saved Audiobooks `GET` `me/audiobooks/contains` [check-users-saved-audiobooks](https://developer.spotify.com/documentation/web-api/reference/check-users-saved-audiobooks)

## Categories

- [ ] Get Several Browse Categories `GET` `/browse/categories` [get-categories](https://developer.spotify.com/documentation/web-api/reference/get-categories)
- [ ] Get Single Browse Category `GET` `/browse/categories/{category_id}` [get-a-category](https://developer.spotify.com/documentation/web-api/reference/get-a-category)

## Chapters

- [ ] Get a Chapter `GET` `/chapters/{id}` [get-a-chapter](https://developer.spotify.com/documentation/web-api/reference/get-a-chapter)
- [ ] Get Several Chapters `GET` `/chapters` [get-several-chapters](https://developer.spotify.com/documentation/web-api/reference/get-several-chapters)

## Episodes

- [ ] Get Episode `GET` `/episodes/{id}` [get-an-episode](https://developer.spotify.com/documentation/web-api/reference/get-an-episode)
- [ ] Get Several Episodes `GET` `/episodes` [get-multiple-episodes](https://developer.spotify.com/documentation/web-api/reference/get-multiple-episodes)
- [ ] Get User's Saved Episodes `GET` `me/episodes` [get-users-saved-episodes](https://developer.spotify.com/documentation/web-api/reference/get-users-saved-episodes)
- [ ] Save Episodes for Current User `PUT` `me/episodes` [save-episodes-user](https://developer.spotify.com/documentation/web-api/reference/save-episodes-user)
- [ ] Remove User's Saved Episodes `DELETE` `me/episodes` [remove-episodes-user](https://developer.spotify.com/documentation/web-api/reference/remove-episodes-user)
- [ ] Check User's Saved Episodes `GET` `me/episodes/contains` [check-users-saved-episodes](https://developer.spotify.com/documentation/web-api/reference/check-users-saved-episodes)

## Genres

- [ ] Get Available Genre Seeds `GET` `/recommendations/available-genre-seeds` [get-recommendation-genres](https://developer.spotify.com/documentation/web-api/reference/get-recommendation-genres)

## Markets

- [ ] Get Available Markets `GET` `/markets` [get-available-markets](https://developer.spotify.com/documentation/web-api/reference/get-available-markets)

## Player

- [ ] Get Playback State `GET` `/me/player` [get-information-about-the-users-current-playback](https://developer.spotify.com/documentation/web-api/reference/get-information-about-the-users-current-playback)
- [ ] Transfer Playback `PUT` `/me/player` [transfer-a-users-playback](https://developer.spotify.com/documentation/web-api/reference/transfer-a-users-playback)
- [ ] Get Available Devices `GET` `/me/player/devices` [get-a-users-available-devices](https://developer.spotify.com/documentation/web-api/reference/get-a-users-available-devices)
- [ ] Get Currently Playing Track `GET` `/me/player/currently-playing` [get-the-users-currently-playing-track](https://developer.spotify.com/documentation/web-api/reference/get-the-users-currently-playing-track)
- [ ] Start/Resume Playback `PUT` `/me/player/play` [start-a-users-playback](https://developer.spotify.com/documentation/web-api/reference/start-a-users-playback)
- [ ] Pause Playback `PUT` `/me/player/pause` [pause-a-users-playback](https://developer.spotify.com/documentation/web-api/reference/pause-a-users-playback)
- [ ] Skip To Next `POST` `/me/player/next` [skip-users-playback-to-next-track](https://developer.spotify.com/documentation/web-api/reference/skip-users-playback-to-next-track)
- [ ] Skip To Previous `POST` `/me/player/previous` [skip-users-playback-to-previous-track](https://developer.spotify.com/documentation/web-api/reference/skip-users-playback-to-previous-track)
- [ ] Seek To Position `PUT` `/me/player/seek` [seek-to-position-in-currently-playing-track](https://developer.spotify.com/documentation/web-api/reference/seek-to-position-in-currently-playing-track)
- [ ] Set Repeat Mode `PUT` `/me/player/repeat` [set-repeat-mode-on-users-playback](https://developer.spotify.com/documentation/web-api/reference/set-repeat-mode-on-users-playback)
- [ ] Set Playback Volume `PUT` `/me/player/volume` [set-volume-for-users-playback](https://developer.spotify.com/documentation/web-api/reference/set-volume-for-users-playback)
- [ ] Toggle Playback Shuffle `PUT` `/me/player/shuffle` [toggle-shuffle-for-users-playback](https://developer.spotify.com/documentation/web-api/reference/toggle-shuffle-for-users-playback)
- [ ] Get Recently Played Tracks `GET` `/me/player/recently-played` [get-recently-played](https://developer.spotify.com/documentation/web-api/reference/get-recently-played)
- [ ] Get the User's Queue `GET` `/me/player/queue` [get-queue](https://developer.spotify.com/documentation/web-api/reference/get-queue)
- [ ] Add Item to Playback Queue `POST` `/me/player/queue` [add-to-queue](https://developer.spotify.com/documentation/web-api/reference/add-to-queue)

## Playlists

- [ ] Get Playlist `GET` `/playlists/{playlist_id}` [get-playlist](https://developer.spotify.com/documentation/web-api/reference/get-playlist)
- [ ] Change Playlist Details `PUT` `/playlists/{playlist_id}` [change-playlist-details](https://developer.spotify.com/documentation/web-api/reference/change-playlist-details)
- [ ] Get Playlist Items `GET` `/playlists/{playlist_id}/tracks` [get-playlists-tracks](https://developer.spotify.com/documentation/web-api/reference/get-playlists-tracks)
- [ ] Update Playlist Items `PUT` `/playlists/{playlist_id}/tracks` [reorder-or-replace-playlists-tracks](https://developer.spotify.com/documentation/web-api/reference/reorder-or-replace-playlists-tracks)
- [ ] Add Items to Playlist `POST` `/playlists/{playlist_id}/tracks` [add-tracks-to-playlist](https://developer.spotify.com/documentation/web-api/reference/add-tracks-to-playlist)
- [ ] Remove Playlist Items `DELETE` `/playlists/{playlist_id}/tracks` [remove-tracks-playlist](https://developer.spotify.com/documentation/web-api/reference/remove-tracks-playlist)
- [ ] Get Current User's Playlists `GET` `/me/playlists` [get-a-list-of-current-users-playlists](https://developer.spotify.com/documentation/web-api/reference/get-a-list-of-current-users-playlists)
- [ ] Get User's Playlists `GET` `/users/{user_id}/playlists` [get-list-users-playlists](https://developer.spotify.com/documentation/web-api/reference/get-list-users-playlists)
- [ ] Create Playlist `POST` `/users/{user_id}/playlists` [create-playlist](https://developer.spotify.com/documentation/web-api/reference/create-playlist)
- [ ] Get Playlist Cover Image `GET` `/playlists/{playlist_id}/images` [get-playlist-cover](https://developer.spotify.com/documentation/web-api/reference/get-playlist-cover)
- [ ] Add Custom Playlist Cover Image `PUT` `/playlists/{playlist_id}/images` [upload-custom-playlist-cover](https://developer.spotify.com/documentation/web-api/reference/upload-custom-playlist-cover)

## Search

- [ ] Search for Item `GET` `/search` [search](https://developer.spotify.com/documentation/web-api/reference/search)

## Shows

- [ ] Get Show `GET` `/shows/{id}` [get-a-show](https://developer.spotify.com/documentation/web-api/reference/get-a-show)
- [ ] Get Several Shows `GET` `/shows` [get-multiple-shows](https://developer.spotify.com/documentation/web-api/reference/get-multiple-shows)
- [ ] Get Show Episodes `GET` `/shows/{id}/episodes` [get-a-shows-episodes](https://developer.spotify.com/documentation/web-api/reference/get-a-shows-episodes)
- [ ] Get User's Saved Shows `GET` `me/shows` [get-users-saved-shows](https://developer.spotify.com/documentation/web-api/reference/get-users-saved-shows)
- [ ] Save Shows for Current User `PUT` `me/shows` [save-shows-user](https://developer.spotify.com/documentation/web-api/reference/save-shows-user)
- [ ] Remove User's Saved Shows `DELETE` `me/shows` [remove-shows-user](https://developer.spotify.com/documentation/web-api/reference/remove-shows-user)
- [ ] Check User's Saved Shows `GET` `me/shows/contains` [check-users-saved-shows](https://developer.spotify.com/documentation/web-api/reference/check-users-saved-shows)

## Tracks

- [x] Get Track `GET` `/tracks/{id}` [get-track](https://developer.spotify.com/documentation/web-api/reference/get-track)
- [ ] Get Several Tracks `GET` `/tracks` [get-several-tracks](https://developer.spotify.com/documentation/web-api/reference/get-several-tracks)
- [ ] Get User's Saved Tracks `GET` `me/tracks` [get-users-saved-tracks](https://developer.spotify.com/documentation/web-api/reference/get-users-saved-tracks)
- [ ] Save Tracks for Current User `PUT` `me/tracks` [save-tracks-user](https://developer.spotify.com/documentation/web-api/reference/save-tracks-user)
- [ ] Remove User's Saved Tracks `DELETE` `me/tracks` [remove-tracks-user](https://developer.spotify.com/documentation/web-api/reference/remove-tracks-user)
- [ ] Check User's Saved Tracks `GET` `me/tracks/contains` [check-users-saved-tracks](https://developer.spotify.com/documentation/web-api/reference/check-users-saved-tracks)

## Users

- [x] Get Current User's Profile `GET` `/me` [get-current-users-profile](https://developer.spotify.com/documentation/web-api/reference/get-current-users-profile)
- [ ] Get User's Top Items `GET` `/me/top/{type}` [get-users-top-artists-and-tracks](https://developer.spotify.com/documentation/web-api/reference/get-users-top-artists-and-tracks)
- [ ] Get User's Profile `GET` `/users/{user_id}` [get-users-profile](https://developer.spotify.com/documentation/web-api/reference/get-users-profile)
- [ ] Follow Playlist `PUT` `/playlists/{playlist_id}/followers` [follow-playlist](https://developer.spotify.com/documentation/web-api/reference/follow-playlist)
- [ ] Unfollow Playlist `DELETE` `/playlists/{playlist_id}/followers` [unfollow-playlist](https://developer.spotify.com/documentation/web-api/reference/unfollow-playlist)
- [ ] Get Followed Artists `GET` `/me/following` [get-followed](https://developer.spotify.com/documentation/web-api/reference/get-followed)
- [ ] Follow Artists or Users `PUT` `/me/following` [follow-artists-users](https://developer.spotify.com/documentation/web-api/reference/follow-artists-users)
- [ ] Unfollow Artists or Users `DELETE` `/me/following` [unfollow-artists-users](https://developer.spotify.com/documentation/web-api/reference/unfollow-artists-users)
- [ ] Check If User Follows Artists or Users `GET` `/me/following/contains` [check-current-user-follows](https://developer.spotify.com/documentation/web-api/reference/check-current-user-follows)
- [ ] Check if Current User Follows Playlist `GET` `/playlists/{playlist_id}/followers/contains` [check-if-user-follows-playlist](https://developer.spotify.com/documentation/web-api/reference/check-if-user-follows-playlist)
