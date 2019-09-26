pub mod generic;
pub mod hours;
pub mod projects;
pub mod traits;
use hours::HourType;
use projects::{Project, Service};
use traits::Get;

#[derive(Clone)]
pub struct SimplicateClient {
    pub api_key: String,
    pub api_secret: String,
    pub host: String,
}

impl SimplicateClient {
    pub fn get_projects(&self) -> Vec<Project> {
        Project::get(self.clone(), None)
    }

    pub fn get_services(&self) -> Vec<Service> {
        Service::get(self.clone(), None)
    }

    pub fn get_services_by_project(&self, project_id: &str) -> Vec<Service> {
        let params = vec![("q[project_id]".to_string(), project_id.to_string())];
        Service::get(self.clone(), Some(params))
    }

    pub fn get_hourtypes(&self) -> Vec<HourType> {
        HourType::get(self.clone(), None)
    }
}
