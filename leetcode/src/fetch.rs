extern crate reqwest;
extern crate serde_json;
use std::{
    collections::HashMap,
    fmt::{Display, Error, Formatter},
};

use serde_json::Value;

const PROBLEMS_URL: &str = "https://leetcode.com/api/problems/algorithms/";

const GRAPHQL_URL: &str = "https://leetcode.com/graphql";
const QUESTION_QUERY_STRING: &str = r#"
query questionData($titleSlug: String!) {
    question(titleSlug: $titleSlug) {
        content
        stats
        codeDefinition
        sampleTestCase
        metaData
    }
}"#;
const QUESTION_QUERY_OPERATION: &str = "questionData";

async fn api_get_problems() -> Problems {
    reqwest::get(PROBLEMS_URL)
        .await
        .expect("Failed to fetch problems")
        .json()
        .await
        .unwrap()
}

pub async fn get_problem(id: u32) -> Option<Problem> {
    let problems = api_get_problems().await;
    for i in problems.stat_status_pairs.iter() {
        if i.stat.question_id == id {
            if i.paid_only {
                return None;
            }
            let client: reqwest::Client = reqwest::Client::new();
            let resp = client
                .post(GRAPHQL_URL)
                .json(&Query::question_query(
                    &i.stat.question_title_slug.as_ref().unwrap()[..],
                ))
                .send()
                .await
                .unwrap()
                .json::<RawProblem>()
                .await
                .unwrap();
            return Some(Problem {
                title: i.stat.question_title.clone().unwrap(),
                title_slug: i.stat.question_title_slug.clone().unwrap(),
                code_definition: serde_json::from_str(&resp.data.question.code_definition).unwrap(),
                content: resp.data.question.content,
                sample_test_case: resp.data.question.sample_test_case,
                difficulty: i.difficulty.to_string(),
                question_id: i.stat.frontend_question_id,
                return_type: {
                    let v: Value = serde_json::from_str(&resp.data.question.meta_data).unwrap();
                    v["return"]["type"].to_string().replace("\"", "")
                },
            });
        }
    }
    None
}

#[derive(Debug, Serialize, Deserialize)]
struct RawProblem {
    data: Data,
}

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    question: Question,
}

#[derive(Debug, Serialize, Deserialize)]
struct Question {
    content: String,
    stats: String,
    #[serde(rename = "codeDefinition")]
    code_definition: String,
    #[serde(rename = "sampleTestCase")]
    sample_test_case: String,
    #[serde(rename = "metaData")]
    meta_data: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Query {
    #[serde(rename = "operationName")]
    operation_name: String,
    variables: serde_json::Value,
    query: String,
}

impl Query {
    fn question_query(title_slug: &str) -> Query {
        Query {
            operation_name: QUESTION_QUERY_OPERATION.to_owned(),
            variables: json!({ "titleSlug": title_slug }),
            query: QUESTION_QUERY_STRING.to_owned(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Problems {
    user_name: String,
    num_solved: u32,
    num_total: u32,
    ac_easy: u32,
    ac_medium: u32,
    ac_hard: u32,
    pub stat_status_pairs: Vec<StatWithStatus>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatWithStatus {
    pub stat: Stat,
    difficulty: Difficulty,
    paid_only: bool,
    is_favor: bool,
    frequency: u32,
    progress: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Stat {
    question_id: u32,
    #[serde(rename = "question__article__slug")]
    question_article_slug: Option<String>,
    #[serde(rename = "question__title")]
    question_title: Option<String>,
    #[serde(rename = "question__title_slug")]
    question_title_slug: Option<String>,
    #[serde(rename = "question__hide")]
    question_hide: bool,
    total_acs: u32,
    total_submitted: u32,
    pub frontend_question_id: u32,
    is_new_question: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Difficulty {
    level: u32,
}

impl Display for Difficulty {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.level {
            1 => f.write_str("Easy"),
            2 => f.write_str("Medium"),
            3 => f.write_str("Hard"),
            _ => f.write_str("Unknown"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Problem {
    pub title: String,
    pub title_slug: String,
    pub content: String,
    #[serde(rename = "codeDefinition")]
    pub code_definition: Vec<CodeDefinition>,
    #[serde(rename = "sampleTestCase")]
    pub sample_test_case: String,
    pub difficulty: String,
    pub question_id: u32,
    pub return_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CodeDefinition {
    pub value: String,
    pub text: String,
    #[serde(rename = "defaultCode")]
    pub default_code: String,
}
