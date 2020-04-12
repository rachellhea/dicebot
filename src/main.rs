use discord::model::Event;
use discord::Discord;
use rand::Rng;
use std::env;

fn main() {
    // Log into Discord using the bot token.
    let token = &env::var("DISCORD_TOKEN").expect("Expected token");
    let discord = Discord::from_bot_token(token)
        .expect("login failed");

    let (mut connection, _) = discord.connect().expect("connect failed");
    println!("Ready!");

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                println!("[NEW MSG] {} - {}", message.author.name, message.content);

                if message.content.starts_with("!random")
                        || message.content.starts_with("!roll") {
                    let rand = rand::thread_rng().gen_range(1, 1001);
                    let out = format!("🎲 - {} rolled: {}",
                            message.author.name,
                            rand);

                    println!("{}", out);
                    let _ = discord.send_message(message.channel_id,
                            &out,
                            "",
                            false);
                } else if message.content == "!quit" {
                    println!("Quitting...");
                    break;
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break;
            }
            Err(err) => println!("Received error: {:?}", err),
        }
    }
}
