use crate::generic::{CustomField, ListResponse, ProjectSimple, ServiceSimple};
use crate::traits::{GetMany, Post};
use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct HourType {
    pub id: String,
    pub label: String,
}

#[derive(Serialize, Deserialize)]
pub struct HourPost {
    pub employee_id: String,
    pub project_id: String,
    pub projectservice_id: String,
    pub type_id: String,
    pub hours: f64,
    pub start_date: String,
    pub note: String,
    pub custom_fields: Option<Vec<CustomField>>,
}

#[derive(Serialize, Deserialize)]
pub struct Hours {
    pub id: String,
    pub hours: f64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub note: Option<String>,
    pub project: Option<ProjectSimple>,
    pub projectservice: Option<ServiceSimple>
}

type HourtypesListResponse = ListResponse<HourType>;
type HoursListResponse = ListResponse<Hours>;

impl GetMany<HourType> for HourType {
    fn url_suffix() -> String {
        "hours/hourstype".to_string()
    }

    fn process_response(mut response: Response) -> Vec<HourType> {
        let r: HourtypesListResponse = response.json().expect("Could not parse response");
        r.data
    }
}

impl Post<HourPost> for HourPost {
    fn url_suffix() -> String {
        "hours/hours".to_string()
    }

    fn process_response(mut response: Response) {
        let success = &response.status().is_success();
        let content = &response.text();
        match (success, content) {
            (true, _) => (),
            (false, Ok(x)) => {
                println!("{}", x);
                panic!("Post failed")
            },
            (_, _) => panic!("Got no response")
        }
    }
}

impl GetMany<Hours> for Hours {
    fn url_suffix() -> String {
        "hours/hours".to_string()
    }

    fn process_response(mut response: Response) -> Vec<Hours> {
        let r: HoursListResponse = response.json().expect("Could not parse hours response");
        r.data
    }
}