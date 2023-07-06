
use std::env;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

const HELP_MESSAGE: &str = "
Bonjour, je suis Igar-bot!
Voici la liste de mes fonctionnalités:

!insult ---
!love ---


";

const INSULT_MESSAGE: &str = "
Sale paysan!!
";

const LOVE_MESSAGE: &str = "
Igar #les mains baladeuses, vous agrippe la fesse gauche!
";

const HELP_COMMAND: &str = "!help";
const INSULT_COMMAND: &str = "!insult";
const LOVE_COMMAND: &str = "!love";
const TELL_COMMAND: &str = "!tell";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content == INSULT_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, INSULT_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content == LOVE_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, LOVE_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        } else if msg.content.starts_with(TELL_COMMAND) {
            // Split the message into two parts: the command and the user/message arguments
            let mut parts = msg.content.splitn(2, ' ');

            // Skip the command itself
            parts.next();

            // Extract the user argument
            let user_arg = parts.next().unwrap_or("");

            let user = if let Some(user_id) = user_arg.strip_prefix("<@!") {
                if let Ok(user_id) = user_id.trim_end_matches('>').parse::<u64>() {
                    if let Ok(user) = ctx.http.get_user(user_id).await {
                        Some(user)
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            };

            if let Some(user) = user {
                if let Err(why) = user.dm(&ctx.http, |m| m.content("Your message here")).await {
                    println!("Error sending message: {:?}", why);
                }
            } else {
                if let Err(why) = msg
                    .channel_id
                    .say(&ctx.http, "Could not find the specified user.")
                    .await
                {
                    println!("Error sending message: {:?}", why);
                }
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        }
}



#[tokio::main]

async fn main() {
  let token = env::var("DISCORD_TOKEN")
  .expect("Expected a token in the environment");

  let mut client = Client::builder(&token)
  .event_handler(Handler)
  .await
  .expect("Err creating client");

  if let Err(why) = client.start().await {
      println!("Client error: {:?}", why);
  }
}


