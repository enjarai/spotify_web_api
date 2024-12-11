# Examples

To run the examples, you need to have a Spotify account and create a Spotify app. You can create a Spotify app [here](https://developer.spotify.com/dashboard/applications).

Then export the following environment variables:

```bash
export SPOTIFY_CLIENT_ID=your_client_id
export SPOTIFY_CLIENT_SECRET=your_client_secret
```

Note that only the examples using the Client Credentials Flow, prefaced with `creds_` in the filename, require the `SPOTIFY_CLIENT_SECRET` environment variable.

## Run Examples

```bash
cargo run --example example_name
```

where the `example_name` is the name of the example file without the `.rs` extension.
