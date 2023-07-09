
use std::env;
use reqwest;
use regex::Regex;
use serde::{Deserialize};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

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
use std::fmt;

impl fmt::Display for TokenResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Access Token: {}, Token Type: {}, Expires In: {}",
            self.access_token, self.token_type, self.expires_in
        )
    }
}

struct Handler;

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
async fn get_data(access_token: &str, code: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    //
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
        //MOntre la reponse HTTP de  ma requete, enfin ca marche
     let status = response.status();
     println!("Response Status Code: {}", status);
     let headers = response.headers();
     println!("Response Headers: {:?}", headers);


        if response.status().is_success() {
    let data = response.json::<serde_json::Value>().await?;
    println!("testo");
    parse_data(data).await?;
    Ok(())
} else {
    println!("Request failed with status code: {}", response.status());
    // Handle the error case appropriately
    // You can return an error or take other actions
    // For now, let's return an empty result
    Ok(())
}
}

async fn parse_data(data: serde_json::Value) -> Result<(), Box<dyn std::error::Error>> {
    #[derive(Debug, Deserialize)]
struct Report {
    data: ReportData,
}

#[derive(Debug, Deserialize)]
struct ReportData {
    reportData: ReportInfo,
}

#[derive(Debug, Deserialize)]
struct ReportInfo {
    report: Rankings,
}

#[derive(Debug, Deserialize)]
struct Rankings {
    data: Vec<Ranking>,
}

#[derive(Debug, Deserialize)]
struct Ranking {
    affixes: Vec<u32>,
    bracket: u32,
    bracketData: u32,
    computedScore: f64,
    damageTakenExcludingTanks: u32,
    deaths: u32,
    difficulty: u32,
    duration: u32,
    encounter: Encounter,
    fightID: u32,
    kill: u32,
    medal: String,
    partition: u32,
    roles: Roles,
    score: Score,
    size: u32,
    speed: Speed,
    team: Vec<TeamMember>,
    zone: u32,
}

#[derive(Debug, Deserialize)]
struct Encounter {
    id: u32,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Roles {
    dps: RoleInfo,
    healers: RoleInfo,
    tanks: RoleInfo,
}

#[derive(Debug, Deserialize)]
struct RoleInfo {
    characters: Vec<Character>,
    name: String,
}

#[derive(Debug, Deserialize)]
struct Character {
    amount: f64,
    best: String,
    bracket: u32,
    bracketData: u32,
    bracketPercent: u32,
    class: String,
    id: u32,
    name: String,
    rank: String,
    rankPercent: u32,
    server: Server,
    spec: String,
    totalParses: u32,
}

#[derive(Debug, Deserialize)]
struct Server {
    id: u32,
    name: String,
    region: String,
}

#[derive(Debug, Deserialize)]
struct Score {
    best: String,
    rank: String,
    rankPercent: u32,
    totalParses: u32,
}

#[derive(Debug, Deserialize)]
struct Speed {
    best: String,
    rank: String,
    rankPercent: u32,
    totalParses: u32,
}

#[derive(Debug, Deserialize)]
struct TeamMember {
    class: String,
    id: u32,
    name: String,
    role: String,
    spec: String,
}

    println!("{:?}", data);
    let report_data: Result<ReportData, serde_json::Error> = serde_json::from_value(data);
    match report_data {
        Ok(report_data) => println!("{:?}", report_data),
        Err(error) => println!("Deserialization error: {}", error),
    }

    Ok(())
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
                    if let Ok(data) = get_data(&token_response.access_token, report_code).await {
                        
                    } else {
                        println!("Error getting data");
                }
            } 
            else if let Err(err) = access_token {
                println!("Error getting access token: {:?}", err);
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

