
use std::env;
use reqwest;

use serde::{Deserialize, Serialize};
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
    let root: Root = serde_json::from_value(data)?;
    println!("{:#?}", root);
    
    

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

