pub mod generic;
pub mod hours;
pub mod projects;
pub mod traits;
use hours::{HourType, Hours};
use projects::{Project, Service};
use traits::GetMany;

#[derive(Clone)]
pub struct SimplicateClient {
    pub api_key: String,
    pub api_secret: String,
    pub host: String,
}

impl SimplicateClient {
    pub fn get_projects(&self) -> Vec<Project> {
        Project::get_many(self.clone(), None, vec![])
    }

    pub fn get_projects_by_status(&self, status_id: String) -> Vec<Project> {
        let params = vec![("q[project_status.id]".to_string(), status_id)];
        Project::get_many(self.clone(), None, params)
    }

    pub fn get_services(&self) -> Vec<Service> {
        Service::get_many(self.clone(), None, vec![])
    }

    pub fn get_services_by_project(&self, project_id: &str) -> Vec<Service> {
        let params = vec![("q[project_id]".to_string(), project_id.to_string())];
        Service::get_many(self.clone(), None, params)
    }

    pub fn get_hourtypes(&self) -> Vec<HourType> {
        HourType::get_many(self.clone(), None, vec![])
    }

    pub fn get_employee_hours_for_daterange(
        &self,
        employee_id: String,
        start_date: Option<String>,
        end_date: Option<String>,
    ) -> Vec<Hours> {
        let mut params = vec![("q[employee.id]".to_string(), employee_id)];
        match start_date {
            Some(x) => params.push(("q[start_date][ge]".to_string(), x)),
            None => (),
        };
        match end_date {
            Some(x) => params.push(("q[start_date][le]".to_string(), x)),
            None => (),
        };
        Hours::get_many(self.clone(), None, params)
    }
}
