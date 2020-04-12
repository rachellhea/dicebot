#[macro_use] extern crate log;
extern crate simplelog;

use discord::model::Channel;
use discord::model::Event;
use discord::model::UserId;
use discord::Discord;
use rand::Rng;
use simplelog::*;

use std::env;
use std::fs::File;

fn main() {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed).unwrap(),
            WriteLogger::new(LevelFilter::Info,
                    ConfigBuilder::new()
                        .set_time_format_str("%F %T%.3f")
                        .build(),
                    File::create("dicebot.log").unwrap()
            ),
        ]
    ).unwrap();

    let token = &env::var("DISCORD_TOKEN").expect("Expected token");
    let bot_id = get_bot_id();
    let admin_id = get_admin_id();

    let discord = Discord::from_bot_token(token).expect("login failed");
    let (mut connection, _) = discord.connect().expect("connect failed");
    info!("Bot initialized with token [{}], user ID [{}], admin ID [{}]",
            token,
            bot_id,
            admin_id);

    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                // Ignore the bot's own messages to prevent log pollution
                if message.author.id == bot_id {
                    continue;
                }

                // Don't respond to any DMs or group chats
                // (not that you can invite it to a group chat anyways)
                let receipt_channel = discord.get_channel(message.channel_id)
                    .expect("failed to get channel from channel ID");
                match receipt_channel {
                    Channel::Group(_) | Channel::Private(_) => continue,
                    _ => (),
                }

                debug!("{} (user ID: {}) sent message: {}",
                        message.author.name,
                        message.author.id,
                        message.content);
                if message.content.starts_with("!random")
                        || message.content.starts_with("!roll") {
                    let rand = rand::thread_rng().gen_range(0, 1001);
                    info!("{} rolled {}",
                          message.author.name,
                          rand);

                    let reply = format!("🎲 - {} rolled: **{}**",
                            message.author.mention(),
                            rand);
                    let _ = discord.send_message(message.channel_id,
                            &reply,
                            "",
                            false);
                } else if message.content.starts_with("!rules") {
                    let _ = discord.send_message(message.channel_id,
                            &RULES,
                            "",
                            false);
                } else if message.content.starts_with("!help")
                        || message.content.starts_with("!commands")
                        || message.content.starts_with("!guide") {
                    let _ = discord.send_message(message.channel_id,
                            &HELP,
                            "",
                            false);
                } else if message.content == "!quit"
                        && message.author.id == admin_id {
                    info!("Quitting...");
                    break;
                }
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                error!("Gateway closed on us with code {:?}: {}", code, body);
                break;
            }
            Err(err) => error!("Received error: {:?}", err),
        }
    }
}

fn get_bot_id() -> UserId {
    let bot_id = &env::var("DISCORD_USER_ID").expect("Expected bot user ID");

    UserId(bot_id.parse::<u64>().unwrap())
}

fn get_admin_id() -> UserId {
    let admin_id = &env::var("DISCORD_ADMIN_ID").expect("Expected bot admin ID");

    UserId(admin_id.parse::<u64>().unwrap())
}

const RULES: &'static str = "📝🎲 ***__Dice Rules__*** 🎲📝

**1. Highest > Lowest**
The loser chooses between answering a truth or performing a dare. \
The highest winner will then picks an appropriate task.

**2. Minimum/Maximum Rolls**
In the case of 0 or 1000 rolls, the 0 will take a truth or dare \
from each player while the 1000 will give a truth or dare to each player.

**3. Double Jeopardy**
If the same player loses two rounds in a row, both them and the \
next lowest roll will take a truth or dare chosen by the two time loser.";

const HELP: &'static str = "📝💁🏻‍♀️ ***__Command List__*** 💁🏻‍♀️📝

`!random`/`!roll` - Roll a dice, from 0 to 1000!
`!rules` - Display the rules of the dice game!
`!help`/`!commands`/`!guide` - Get this nifty help text!";
