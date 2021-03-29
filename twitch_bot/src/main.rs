use twitch_irc::{login::StaticLoginCredentials, ClientConfig, TCPTransport, TwitchIRCClient, message::ServerMessage};
use std::fs::File;
use std::io::prelude::*;

#[tokio::main]
async fn main() {
    let mut bot_username = String::new();
    let mut oauth_token = String::new();

    match File::open("bot_username.txt") {
        Ok(mut file) => {
            file.read_to_string(&mut bot_username).unwrap();
        }
        Err(_) => {
            File::create("bot_username.txt").unwrap();
            println!("please fill the \"bot_username.txt\" file\n", );
        }
    }

    match File::open("oauth_token.txt") {
        Ok(mut file) => {
            file.read_to_string(&mut oauth_token).unwrap();
        }
        Err(_) => {
            File::create("oauth_token.txt").unwrap();
            println!("please fill the \"oauth_token.txt\" file\n", );
        }
    }

    //let bot_username = "5gxd".to_owned();
    //let oauth_token = "6flaivmsw6h2ytjyo08gzcmvu494ft".to_owned();

    let config = ClientConfig::new_simple(StaticLoginCredentials::new(bot_username, Some(oauth_token)));
    let (mut incoming_messages, client) = TwitchIRCClient::<TCPTransport, StaticLoginCredentials>::new(config);

    match File::open("channels.txt") {
        Ok(mut file) => {
            let mut channels = String::new();
            file.read_to_string(&mut channels).unwrap();
            let channels: Vec<&str> = channels.split_whitespace().collect();

            if channels.len() > 0 {
                for i in 0..channels.len() {
                    client.join(channels[i].to_owned());
                    println!("succesfully connected to twitch.tv/{}", channels[i]);
                }
            } else {
                println!("please fill the \"channels.txt\" file\n", );
            }
        },
        Err(_) => {
            File::create("channels.txt").unwrap();
            println!("please fill the \"channels.txt\" file\n", );
        }
    }

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    println!("(#{}) {}: {}", msg.channel_login, msg.sender.name, msg.message_text);
                    match msg.message_text.as_str() {
                        "PogChamp a Raffle has begun for 1000 doinks pokeU it will end in 20 Seconds. Enter by typing \"!join\" pokeFAT" => {
                            if msg.sender.name == "StreamElements".to_owned() {
                                client.say(msg.channel_login, "!join".to_owned()).await.unwrap();
                            }
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
