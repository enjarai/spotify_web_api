# spotify_web_api

[![Build Status](https://img.shields.io/github/actions/workflow/status/ry-sev/spotify_web_api/ci.yml?branch=main)](https://github.com/ry-sev/spotify_web_api/actions)

A wrapper for the [Spotify Web API](https://developer.spotify.com/documentation/web-api) written in Rust.

> Spotify Web API enables the creation of applications that can interact with Spotify's streaming service, such as retrieving content metadata, getting recommendations, creating and managing playlists, or controlling playback.

### Which OAuth flow should I use?

Choosing one flow over the rest depends on the application you are building:

- In scenarios where storing the client secret is not safe (e.g. desktop, mobile apps or JavaScript web apps running in the browser), you can use the [authorization code with PKCE](https://developer.spotify.com/documentation/web-api/tutorials/code-pkce-flow), as it provides protection against attacks where the authorization code may be intercepted.
- For some applications running on the backend, such as CLIs or daemons, the system authenticates and authorizes the app rather than a user. For these scenarios, [Client credentials](https://developer.spotify.com/documentation/web-api/tutorials/client-credentials-flow) is the typical choice. This flow does not include user authorization, so only endpoints that do not request user information (e.g. user profile data) can be accessed.

The following table summarizes the flows' behaviors:

| Flow | Access User Resources | Requires Secret Key (Server-Side) | Access Token Refresh |
| :--- | :--- | :--- | :--- |
| Authorization code with PKCE | Yes | 	No | Yes |
| Client credentials | No | Yes | No |


#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
