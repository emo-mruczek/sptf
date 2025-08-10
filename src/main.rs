#![allow(clippy::assigning_clones)]

use rspotify::{prelude::*, scopes, AuthCodePkceSpotify, Credentials, OAuth};

#[tokio::main]
async fn main() {
    
    let creds = Credentials::from_env().unwrap();

    let oauth = OAuth::from_env(scopes!("user-library-read")).unwrap();

    let mut spotify = AuthCodePkceSpotify::new(creds.clone(), oauth.clone());

    let url = spotify.get_authorize_url(None).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    let tracks = spotify.current_user_saved_tracks_manual(None, None, None).await.unwrap();

    println!("Response: {:#?}", tracks);
}
