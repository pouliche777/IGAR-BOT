use serde::{Deserialize, Serialize};
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
