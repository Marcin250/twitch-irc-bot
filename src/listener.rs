use twitch_irc::message::{ServerMessage};
use twitch_irc::PlainWSTransport;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::TwitchIRCClient;

use crate::utils::{ConsoleInput};
use crate::configuration::loadConfiguration;

pub async fn listen() -> ()
{
    let channelName = ConsoleInput::ask_for_input("channel name".to_string());

    let (user, token) = loadConfiguration().accounts().first().unwrap().data();
    let config = ClientConfig::new_simple(
        StaticLoginCredentials::new(
            user.to_owned(),
            Some(token.to_owned())
        )
    );

    let (mut incoming_messages, client) =
        TwitchIRCClient::<PlainWSTransport , StaticLoginCredentials>::new(config);

    let handler = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await { messageHandler(message) }
    });

    let _ = client.join(channelName.to_owned());
    handler.await.unwrap();
}

fn messageHandler(message: ServerMessage) -> ()
{
    match message {
        ServerMessage::Privmsg(msg) => {
            println!("@{}> {}: {}", msg.channel_login, msg.sender.name, msg.message_text);
        },
        ServerMessage::Whisper(msg) => {
            println!("(w) {}: {}", msg.sender.name, msg.message_text);
        },
        _ => ()
    }
}