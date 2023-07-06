
use std::env;
use reqwest;
use serde::{Deserialize, Serialize};

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};


const TOKEN_URL &str=  "https://www.warcraftlogs.com/oauth/token";
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
const PARSE_COMMAND: &str = "!Parse";


async fn get_token(client_id &str, client_secret: &str) -> Result<TokenResponse, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .post(TOKEN_URL)
        .basic_auth(client_id, Some(client_secret))
        .form(&[("grant_type", "client_credentials")])
        .send()
        .await?;
    
    let token_response: TokenResponse = response.json().await?;
    Ok(token_response)
}

struct Handler;

async fn get_parse_data(parse_url: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let api_key = env::var("WLC_KEY").expect("API key not found in environment variables");
    let response = client
        .get(parse_url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await?;
    let parse_data: ParseData = response.json().await?;
    Ok(parse_data)
}

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
        } else if msg.content.starts_with(PARSE_COMMAND) {
            let mut parts = msg.content.splitn(2, ' ');
            parts.next();
            if let Some(parse_url) = parts.next() {
                // let parse_data = get_parse_data(parse_url).await;
                // match parse_data {
                //    Ok(data) => {
                        let client_id = env::var("CLIENT_ID")
                        let client_secret = env::var("CLIENT_SECRET")
                        let access_token= get_token(&client_id, &client_secret).await;
                        // Process and format the received data
                        println!("Received parse data: {:?}", data);
                        println!("WLC_TOKEN: {}", access_token);
                   // },
                   // Err(error) => {
                        // Handle the error
                        println!("Error occurred: {:?}", error);
                    }
             else {
                // Handle the case where the parse URL is missing
                println!("Parse URL is missing");
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


