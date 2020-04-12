use discord::model::Event;
use discord::model::UserId;
use discord::Discord;
use rand::Rng;
use std::env;

fn main() {
    // Log into Discord using the bot token.
    let token = &env::var("DISCORD_TOKEN").expect("Expected token");
    let bot_id = &env::var("DISCORD_USER_ID").expect("Expected bot user ID");
    let bot_id = UserId(bot_id.parse::<u64>().unwrap());

    let discord = Discord::from_bot_token(token).expect("login failed");
    let (mut connection, _) = discord.connect().expect("connect failed");
    println!("Ready!");

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                // Ignore the bot's own messages to prevent log pollution
                if message.author.id == bot_id {
                    continue;
                }

                println!("[DEBUG] {} (user ID: {}) sent message: {}",
                        message.author.name,
                        message.author.id,
                        message.content);
                if message.content.starts_with("!random")
                        || message.content.starts_with("!roll") {
                    let rand = rand::thread_rng().gen_range(1, 1001);
                    let out = format!("🎲 - {} rolled: {}",
                            message.author.name,
                            rand);

                    println!("[INFO] {}", out);
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
