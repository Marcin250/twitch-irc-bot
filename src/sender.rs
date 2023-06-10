use std::time::{Duration};

use redis::Commands;
use redis::RedisError;
use tokio::runtime::Runtime;
use twitch_irc::PlainWSTransport;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::TwitchIRCClient;

use crate::utils::{ConsoleInput, RedisConnection, key};
use crate::configuration::loadConfiguration;

pub async fn sendAsync() -> Result<(), RedisError>
{
    let runtime = Runtime::new().unwrap();
    let mut connection = RedisConnection::initialize();
    let mut pubsubConnection = connection.as_pubsub();
    pubsubConnection.subscribe(pubSubChannel());

    let channelName = ConsoleInput::ask_for_input("channel name".to_string());
    let accounts = loadConfiguration().accounts();

    loop {
        let message = pubsubConnection.get_message()?;
        let payload : String = message.get_payload()?;

        for account in accounts.clone() {
            let cChannelName = channelName.clone();
            let cMessage = payload.clone();

            runtime.spawn (async move {
                let (user, token) = account.data();
                let config = ClientConfig::new_simple(
                    StaticLoginCredentials::new(
                        user.to_owned(),
                        Some(token.to_owned())
                    )
                );

                let (mut receiver, client) = TwitchIRCClient::<PlainWSTransport , StaticLoginCredentials>::new(config);

                match client.me(cChannelName.to_owned(), cMessage.to_owned()).await {
                    Ok(_) => (),
                    Err(err) => eprintln!("Error sending message: {:?}", err),
                }

                receiver.close();
            });
        }

        std::thread::sleep(Duration::from_secs(1));
    }
}

pub fn queueMessage() -> ()
{
    let mut connection = RedisConnection::initialize();

    let message = ConsoleInput::ask_for_input("message".to_string());

    let result: Result<(), _> = connection.publish(pubSubChannel(), message);

    if let Err(err) = result {
        panic!("Unable to queue message: {}", err);
    }
}

fn pubSubChannel() -> String
{
    key("message_queue".to_string())
}