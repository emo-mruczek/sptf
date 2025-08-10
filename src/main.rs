#![allow(clippy::assigning_clones)]

use rspotify::{prelude::*, scopes, AuthCodePkceSpotify, Credentials, OAuth};

#[tokio::main]
async fn main() {
    
    let creds = Credentials::from_env().unwrap();

    let oauth = OAuth::from_env(scopes!("user-read-playback-state")).unwrap();

    let mut spotify = AuthCodePkceSpotify::new(creds.clone(), oauth.clone());

    let url = spotify.get_authorize_url(None).unwrap();
    spotify.prompt_for_token(&url).await.unwrap();

    let history = spotify.current_playback(None, None::<Vec<_>>).await;
    println!("Response: {:#?}", history);
}
