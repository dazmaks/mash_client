use chrono::Date;
use chrono::Utc;
use serde::de::{Deserialize, Deserializer};
use serde::Serialize;
use serde_derive::Deserialize;

use crate::types::Homework;

const USERAGENT: &str = "Mozilla/5.0 (X11; Ubuntu; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.87 Safari/537.36 RuxitSynthetic/1.0 v8662719366318635631 t6281935149377429786 ";

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[derive(Deserialize)]
pub struct Attachment {}

#[derive(Deserialize)]
pub struct RemoteAttachment {}

#[allow(unused)]
#[derive(Deserialize)]
pub struct Subject {
    pub id: u64,
    pub name: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub exam_name: String,
}

#[allow(unused)]
#[derive(Deserialize)]
pub struct HomeworkData {
    pub id: u64,
    pub created_at: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub updated_at: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub deleted_at: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub deleted_by: String,
    pub teacher_id: u64,
    pub subject_id: u32,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub is_required: bool,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub mark_required: bool,
    pub group_id: u64,
    pub date_assigned_on: String,
    pub date_prepared_for: String,
    pub subject: Subject,
}

#[allow(unused)]
#[derive(Deserialize)]
pub struct EomUrls {
    pub url_type: String,
    pub url: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub profile_type: String,
}

#[allow(unused)]
#[derive(Deserialize)]
pub struct EomUrl {
    pub material_id: u64,
    pub r#type: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub content_type: String,
    pub urls: Vec<EomUrls>,
}

#[allow(unused)]
#[derive(Deserialize)]
pub struct HomeworkEntry {
    pub id: u64,
    pub created_at: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub updated_at: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub deleted_at: String,
    pub homework_id: u64,
    pub description: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub duration: u32,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub no_duration: bool,
    pub homework: HomeworkData,
    pub attachments: Vec<Attachment>,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub homework_entry_student_answer: String,
    pub controllable_items: Vec<u64>,
    pub homework_entry_comments: Vec<String>,
    pub student_ids: Vec<u64>,
    pub attachment_ids: Vec<u64>,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub controllable_item_ids: Vec<u64>,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub books: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub tests: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub scripts: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub data: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub update_comment: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub game_apps: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub atomic_objects: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub related_materials: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub eom_urls: Vec<EomUrl>,
    pub long_term: bool,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub is_digital_homework: bool,
}

#[allow(unused)]
#[derive(Deserialize)]
pub struct ClientHomework {
    pub id: u64,
    pub created_at: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub updated_at: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub deleted_at: String,
    pub student_id: u64,
    pub homework_entry_id: u64,
    pub student_name: String,
    #[serde(deserialize_with = "deserialize_null_default")]
    pub comment: String,
    pub is_ready: bool,
    pub attachments: Vec<Attachment>,
    pub remote_attachments: Vec<RemoteAttachment>,
    pub homework_entry: HomeworkEntry,
    pub attachment_ids: Vec<u64>,
}

#[derive(Clone)]
pub struct Client {
    token: String,
    profile_id: String,
}

impl Client {
    pub fn new(token: &str, profile_id: &str) -> Client {
        Client {
            token: token.to_owned(),
            profile_id: profile_id.to_owned(),
        }
    }

    pub fn request<T: Serialize>(&self, method: &str, query: T) -> String {
        let client = reqwest::blocking::Client::new();

        let res = client
            .get(format!("https://dnevnik.mos.ru{}", method))
            .header("Auth-Token", self.token.clone())
            .header("Content-Type", "application/json")
            .header("Profile-Id", self.profile_id.clone())
            .header("User-Agent", USERAGENT)
            .header("Accept", "*/*")
            .query(&query)
            .send()
            .unwrap();

        res.text().unwrap()
    }

    pub fn get_homework(&self, from: Option<Date<Utc>>, to: Option<Date<Utc>>) -> Vec<Homework> {
        let result_from = from.unwrap_or(Utc::today()).format("%d.%m.%Y").to_string();
        let result_to = to.unwrap_or(Utc::today()).format("%d.%m.%Y").to_string();

        let response = self.request(
            "/core/api/student_homeworks",
            &[
                ("begin_prepared_date", &result_from),
                ("end_prepared_date", &result_to),
            ],
        );

        let client_homework: Vec<ClientHomework> = serde_json::from_str(&response).unwrap();

        let homework = client_homework
            .into_iter()
            .map(|i| Homework {
                date: i.homework_entry.homework.date_prepared_for,
                created_at: i.created_at,
                subject_name: i.homework_entry.homework.subject.name,
                task: i.homework_entry.description,
                test_urls: i
                    .homework_entry
                    .eom_urls
                    .into_iter()
                    .map(|j| j.urls.into_iter().map(|k| Some(k.url)).collect())
                    .collect(),
            })
            .collect();

        homework
    }

    pub fn get_homework_at(&self, at: Date<Utc>) -> Vec<Homework> {
        self.get_homework(Some(at), Some(at))
    }
}
