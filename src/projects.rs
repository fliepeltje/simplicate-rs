use crate::generic::{Budget, ListResponse};
use crate::traits::Get;
use reqwest::Response;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectStatus {
    pub id: String,
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub budget: Option<Budget>,
    pub project_status: Option<ProjectStatus>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub url: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Service {
    pub id: String,
    pub name: Option<String>,
    pub project_id: String,
    pub status: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

type ProjectListResponse = ListResponse<Project>;
type ServiceListResponse = ListResponse<Service>;

impl Get<Project> for Project {
    fn url_suffix() -> String {
        "projects/project".to_string()
    }

    fn process_response(mut response: Response) -> Vec<Project> {
        let r: ProjectListResponse = response.json().expect("Could not parse response");
        r.data
    }
}

impl Get<Service> for Service {
    fn url_suffix() -> String {
        "projects/service".to_string()
    }

    fn process_response(mut response: Response) -> Vec<Service> {
        let r: ServiceListResponse = response.json().expect("Could not parse response");
        r.data
    }
}
