use crate::{Post, QueryMany};
use chrono::{NaiveDate, NaiveDateTime};
use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct ListResponse<T> {
    data: Vec<T>,
}

#[derive(Deserialize)]
pub struct ProjectStatus {
    pub id: String,
    pub label: String,
}

#[derive(Serialize, Deserialize)]
pub struct NamedReference {
    pub id: String,
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub project_status: Option<ProjectStatus>,
    pub organization: Option<NamedReference>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Deserialize)]
pub struct Service {
    pub id: String,
    pub name: Option<String>,
    pub project_id: String,
    pub status: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Deserialize)]
pub struct Hours {
    pub id: String,
    pub hours: f64,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub note: Option<String>,
    pub project: Option<NamedReference>,
    pub projectservice: Option<NamedReference>,
}

#[derive(Serialize)]
pub struct NewHours {
    pub hours: f64,
    pub project_id: String,
    pub projectservice_id: String,
    pub employee_id: String,
    pub type_id: String,
    pub start_date: NaiveDateTime,
    pub note: Option<String>,
}

#[derive(Deserialize)]
pub struct HourType {
    pub id: String,
    pub label: String,
}

type WrappedProjects = ListResponse<Project>;
type WrappedServices = ListResponse<Service>;
type WrappedHourTypes = ListResponse<HourType>;
type WrappedHours = ListResponse<Hours>;

impl QueryMany<Project> for Project {
    const ENDPOINT: &'static str = "projects/project";
    fn unwrap_response(mut response: Response) -> Vec<Project> {
        let r: WrappedProjects = response.json().expect("Could not parse data type");
        r.data
    }
}

impl QueryMany<Service> for Service {
    const ENDPOINT: &'static str = "projects/service";
    fn unwrap_response(mut response: Response) -> Vec<Service> {
        let r: WrappedServices = response.json().expect("Could not parse data type");
        r.data
    }
}

impl QueryMany<Hours> for Hours {
    const ENDPOINT: &'static str = "hours/hours";
    fn unwrap_response(mut response: Response) -> Vec<Hours> {
        // let msg = format!("Could not parse data type:\n{} ", response.text().unwrap());
        let r: WrappedHours = response.json().expect("Failed to parse hours");
        r.data
    }
}

impl QueryMany<HourType> for HourType {
    const ENDPOINT: &'static str = "hours/hourstype";
    fn unwrap_response(mut response: Response) -> Vec<HourType> {
        let r: WrappedHourTypes = response.json().expect("Could not parse data type");
        r.data
    }
}

impl Post<NewHours> for NewHours {
    const ENDPOINT: &'static str = "hours/hours";
}
