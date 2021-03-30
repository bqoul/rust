mod commands;

use twitch_irc::{login::StaticLoginCredentials, ClientConfig, TCPTransport, TwitchIRCClient, message::ServerMessage};
use std::{fs::File, io::prelude::*, time, thread};
//use serde_derive::Deserialize;

#[tokio::main]
async fn main() {
    let bot_username = get_file("bot_username.txt");
    let oauth_token = get_file("oauth_token.txt");

    let config = ClientConfig::new_simple(StaticLoginCredentials::new(bot_username.clone(), Some(oauth_token)));
    let (mut incoming_messages, client) = TwitchIRCClient::<TCPTransport, StaticLoginCredentials>::new(config);
    
    let channels = get_file("channels.txt");
    let channels: Vec<&str> = channels.as_str().split_whitespace().collect();

    for i in 0..channels.len() {
        client.join(channels[i].to_owned());
        println!("succesfully connected to twitch.tv/{}\n", channels[i]);
    }

    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                ServerMessage::Privmsg(msg) => {
                    println!("(#{}) {}: {}", msg.channel_login, msg.sender.name, msg.message_text);

                    match msg.message_text.as_str() {
                        "&help" => {
                            commands::help::run(&client, msg, &bot_username).await;
                            gtp();
                        }
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


//async fn mod_check(channel: &String) -> Vec<String> {
//    let link = format!("https://tmi.twitch.tv/group/user/{}/chatters", channel);
//    let mut responce = reqwest::get(&link).await.unwrap().json::<Responce>().await.unwrap();
//
//    let mut chatters: Vec<String> = Vec::new();
//    chatters.append(&mut responce.chatters.broadcaster);
//    chatters.append(&mut responce.chatters.vips);
//    chatters.append(&mut responce.chatters.moderators);
//
//    #[derive(Deserialize)]
//    struct Responce {
//        chatters: Chatters,
//    }
//
//    #[derive(Deserialize)]
//    struct Chatters {
//        broadcaster: Vec<String>,
//        vips: Vec<String>,
//        moderators: Vec<String>,
//    }
//
//    chatters
//}

fn get_file(file_name: &str) -> String {
    let mut data = String::new();

    match File::open(format!("{}", file_name)) {
        Ok(mut file) => {
            file.read_to_string(&mut data).unwrap();
        }
        Err(_) => {
            File::create(format!("{}", file_name)).unwrap();
            println!("please fill the \"{}\" file\n", file_name);
        }
    }
    data
}

fn gtp() {
    thread::sleep(time::Duration::from_secs_f32(1.5));
}
