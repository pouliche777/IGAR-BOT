
use std::env;
use reqwest;
use colored::*;
use std::fmt;

use serde::{Deserialize, Serialize};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub report_data: ReportData,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportData {
    pub report: Report,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Report {
    pub rankings: Rankings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rankings {
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Daum {
    #[serde(rename = "fightID")]
    pub fight_id: i64,
    pub partition: i64,
    pub zone: i64,
    pub encounter: Encounter,
    pub difficulty: i64,
    pub size: i64,
    pub kill: i64,
    pub duration: i64,
    pub bracket_data: f64,
    pub deaths: i64,
    pub damage_taken_excluding_tanks: i64,
    pub roles: Roles,
    pub bracket: i64,
    pub guild: Guild,
    pub speed: Speed,
    pub execution: Execution,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Encounter {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Roles {
    pub tanks: Tanks,
    pub healers: Healers,
    pub dps: Dps,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tanks {
    pub name: String,
    pub characters: Vec<Character>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub id: i64,
    pub name: String,
    pub server: Server,
    pub class: String,
    pub spec: String,
    pub amount: f64,
    pub bracket_data: i64,
    pub bracket: i64,
    pub rank: String,
    pub best: String,
    pub total_parses: i64,
    pub bracket_percent: i64,
    pub rank_percent: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub id: i64,
    pub name: String,
    pub region: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Healers {
    pub name: String,
    pub characters: Vec<Character2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character2 {
    pub id: i64,
    pub name: String,
    pub server: Server2,
    pub class: String,
    pub spec: String,
    pub amount: f64,
    pub bracket_data: i64,
    pub bracket: i64,
    pub rank: String,
    pub best: String,
    pub total_parses: i64,
    pub bracket_percent: i64,
    pub rank_percent: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server2 {
    pub id: i64,
    pub name: String,
    pub region: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Dps {
    pub name: String,
    pub characters: Vec<Character3>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Character3 {
    pub id: i64,
    pub name: String,
    pub server: Server3,
    pub class: String,
    pub spec: String,
    pub amount: f64,
    pub bracket_data: i64,
    pub bracket: i64,
    pub rank: String,
    pub best: String,
    pub total_parses: i64,
    pub bracket_percent: i64,
    pub rank_percent: i64,
    pub exploit: Option<i64>,
    pub banned: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server3 {
    pub id: i64,
    pub name: String,
    pub region: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Guild {
    pub id: i64,
    pub name: String,
    pub faction: i64,
    pub server: Server4,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server4 {
    pub id: i64,
    pub name: String,
    pub region: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Speed {
    pub rank: String,
    pub best: String,
    pub total_parses: i64,
    pub rank_percent: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Execution {
    pub rank: String,
    pub best: String,
    pub total_parses: i64,
    pub rank_percent: i64,
}

//Mise en place de constantes
const TOKEN_URL: &str = "https://www.warcraftlogs.com/oauth/token";
const WCL_API_URL: &str = "https://www.warcraftlogs.com/api/v2/client";
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
const PARSE_COMMAND: &str = "!parse";

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
    // Include any other fields you expect to receive in the token response
}


impl fmt::Display for TokenResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Access Token: {}, Token Type: {}, Expires In: {}",
            self.access_token, self.token_type, self.expires_in
        )
    }
}



async fn get_token(client_id: &str, client_secret: &str) -> Result<TokenResponse, reqwest::Error> {
    let client = reqwest::Client::new();

    let response = client
        .post(TOKEN_URL)
        .basic_auth(client_id, Some(client_secret))
        .form(&[("grant_type", "client_credentials")])
        .send()
        .await?;

    let token_response = response.json::<TokenResponse>().await?;
    Ok(token_response)
}
async fn get_data(access_token: &str, code: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let query = r#"
        query Query($code: String) {
            reportData {
                report(code: $code ) {
                    rankings
                }
            }
        }
    "#;

    let variables = serde_json::json!({
        "code": code
    });

    let body = serde_json::json!({
        "query": query,
        "variables": variables
    });

    let response = client
        .post(WCL_API_URL)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", access_token))
        .body(body.to_string())
        .send()
        .await?;

    let status = response.status();
    println!("Response Status Code: {}", status);
    let headers = response.headers();
    println!("Response Headers: {:?}", headers);

    if response.status().is_success() {
        let data = response.json::<serde_json::Value>().await?;
        let message = parse_data(data).await?;
        Ok(message)
    } else {
        println!("Request failed with status code: {}", response.status());
        Ok(String::new())
    }
}

async fn parse_data(data: serde_json::Value) -> Result<String, Box<dyn std::error::Error>> {
    let root: Root = match serde_json::from_value(data) {
        Ok(root) => root,
        Err(err) => return Err(Box::new(err)),
    };

    let message = build_message(root);
    //println!("voici le message : {}", message);
    Ok(message)
}
fn build_message(root: Root) -> String {
    let mut message = String::from("Voici les parses du rapport :\n");
    println!("{:#?}", root);

    let report_data = root.data.report_data;
    let report = report_data.report;
    let ranking = report.rankings;
    let fights = ranking.data; 
                for fight in fights {
                    message.push('\n');
                    message.push('\n');
                    let encounter = format!("**{}**", fight.encounter.name);
                    message.push('\n');
                    message.push_str(&encounter);
                    message.push('\n');
                    message.push('\n');
                    let tanks= fight.roles.tanks.characters;
                    let dps= fight.roles.dps.characters;
                    let healers= fight.roles.healers.characters;
                    for tank in &tanks {
                        let player_name = &tank.name;
                        let rank_percent = &tank.rank_percent;
                        let damage_done = &tank.amount;
                        let damage_done_str = format!("{:.0}", damage_done);
                        let player_line = format!("{}: {}  ---   {}", player_name, rank_percent, damage_done_str);
                        message.push_str(&player_line);
                        message.push('\n');
                    }
                    for healer in &healers {
                        let player_name = &healer.name;
                        let rank_percent = &healer.rank_percent;
                        let damage_done = &healer.amount;
                        let damage_done_str = format!("{:.0}", damage_done);
                        let player_line = format!("{}: {}  ---   {}", player_name, rank_percent, damage_done_str);
                        message.push_str(&player_line);
                        message.push('\n');
                    }
                    for dps in &dps {
                        let player_name = &dps.name;
                        let rank_percent = &dps.rank_percent;
                        let damage_done = &dps.amount;
                        let damage_done_str = format!("{:.0}", damage_done);
                        let player_line = format!("{}: {}  ---   {}", player_name, rank_percent, damage_done_str);
                        message.push_str(&player_line);
                        message.push('\n');
                    }
                    message.push('\r');
                    message.push('\n');
                    
                    
                }



    message
}
struct Handler;
unsafe impl Send for Handler {}

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
            
            let mut parts = msg.content.splitn(2, ' ');

        
            parts.next();

          
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
            if let Some(report_code) = parts.next() {
                let client_id = env::var("CLIENT_ID").expect("CLIENT_ID not found in environment variables");
                let client_secret = env::var("CLIENT_SECRET").expect("CLIENT_SECRET not found in environment variables");
                let access_token = get_token(&client_id, &client_secret).await;
                if let Ok(token_response) = access_token {
                    println!("Access Token: {}", token_response);
                    let message = get_data(&token_response.access_token, report_code).await.unwrap();
                    println!("voici le message : {}", message);
                    
                    const MAX_MESSAGE_LENGTH: usize = 2000;
                    const CRLF: &str = "\r\n";

                    let message_chunks = message.split(CRLF).collect::<Vec<&str>>();

                    for chunk in message_chunks {
                        if !chunk.is_empty() {
                            let mut remaining_chunk = chunk;
                            while !remaining_chunk.is_empty() {
                                let chunk_part = remaining_chunk.chars().take(MAX_MESSAGE_LENGTH).collect::<String>();
                                remaining_chunk = &remaining_chunk[chunk_part.len()..];
                                if let Err(why) = msg.channel_id.say(&ctx.http, &chunk_part).await {
                                    println!("Error sending message: {:?}", why);
                                }
                            }
                        }
                    }
                
                } 
            } 
            else {
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
    let token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN not found in environment variables");
    println!("discordToken: {}", token);
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

