extern crate rspotify;
extern crate tokio;

use rspotify::client::Spotify;
use rspotify::model::offset::for_position;
use rspotify::oauth2::{SpotifyClientCredentials, SpotifyOAuth};
use rspotify::util::get_token;

#[tokio::main]
pub async fn init_spotify() Result<{
    let mut oauth = SpotifyOAuth::default()
        .scope("user-read-playback-state user-modify-playback-state")
        .build();

    match get_token(&mut oauth).await {
        Some(token_info) => token_info,
        None => println!("auth failed"),
    }
}

/*
#[tokio::main]
pub async fn pause() {
    let token = init_spotify().await?;

    let client_credential = SpotifyClientCredentials::default()
        .token_info(token)
        .build();
    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();
    // this is the example device_id from spotify website,
    // so it will raise a 403 error, just change this device_id to yours
    let device_id = String::from("romodoro");
    match spotify.pause_playback(Some(device_id)).await {
        Ok(_) => println!("pause playback successful"),
        Err(_) => eprintln!("pause playback failed"),
    }
}
*/
/*
#[tokio::main]
pub async fn continue_playback() {

    /*
    let token_info = init_spotify();
    let uris = vec!["spotify:playlist:3gT0ZSgrnuEmYFP6eEBrhv".to_owned()];
    let spotify = Spotify::default()
        .start_playback(Some(device_id), None, Some(uris), for_position(0), None)
        .await
    {
        Ok(_) => println!("start playback successful"),
        Err(_) => eprintln!("start playback failed"),
    }
    */
}
*/

pub async fn get_devices() -> rspotify::model::device::DevicePayload {
    let token = init_spotify().await;
    let client_credential = SpotifyClientCredentials::default()
        .token_info(token)
        .build();

    let spotify = Spotify::default()
        .client_credentials_manager(client_credential)
        .build();

    spotify.device().await.unwrap()
}

