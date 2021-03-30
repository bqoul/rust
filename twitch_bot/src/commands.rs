pub mod help {
    use twitch_irc::{TwitchIRCClient, TCPTransport, login::StaticLoginCredentials};

    pub async fn run(client: &TwitchIRCClient::<TCPTransport, StaticLoginCredentials>, msg: twitch_irc::message::PrivmsgMessage, bot_username: &String) {
        client.say(msg.channel_login, format!("@{} {} is a bot created by @bqoul, you can find additional information here => https://github.com/bqoul/rust/tree/main/twitch_bot",
                    msg.sender.name,
                    bot_username)).await.unwrap();
    }
}
