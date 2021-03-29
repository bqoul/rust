use twitch_irc::{login::StaticLoginCredentials, ClientConfig, TCPTransport, TwitchIRCClient, message::ServerMessage};

#[tokio::main]
async fn main() {
    let bot_nickname = "your bot username".to_owned();
    let oauth_token = "your bot oauth_token".to_owned();

    let config = ClientConfig::new_simple(StaticLoginCredentials::new(bot_nickname, Some(oauth_token)));
    let (mut incoming_messages, client) = TwitchIRCClient::<TCPTransport, StaticLoginCredentials>::new(config);

    client.join("pokelawls".to_owned());

    let join_handle = tokio::spawn(async move {
            while let Some(message) = incoming_messages.recv().await {
                match message {
                    ServerMessage::Privmsg(msg) => {
                        println!("(#{}) {}: {}", msg.channel_login, msg.sender.name, msg.message_text);
                        match msg.message_text.as_str() {
                            "!help" => {
                                //client.say(msg.channel_login, "help yourself, loser".to_owned()).await.unwrap();
                            },
                            r#"PogChamp a Raffle has begun for 1000 doinks pokeU it will end in 20 Seconds. Enter by typing "!join" pokeFAT "# => {
                                client.say(msg.channel_login, "!join".to_owned()).await.unwrap();
                            }
                            _ => {},
                        }
                    }
                    ServerMessage::Whisper(msg) => {
                        println!("(w) {}: {}", msg.sender.name, msg.message_text);
                    }
                    _ => {}
                }
            }
        });

    join_handle.await.unwrap();
}
