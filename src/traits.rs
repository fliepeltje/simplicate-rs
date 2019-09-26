use super::SimplicateClient;
use reqwest::{Client, Response};
use serde::Serialize;

pub trait Get<T> {
    fn url_suffix() -> String;
    fn process_response(response: Response) -> Vec<T>;

    fn offset_params(iteration: u32) -> Vec<(String, String)> {
        let hundred: u32 = 100;
        let offset: u32 = hundred * iteration;
        vec![
            ("limit".to_string(), hundred.to_string()),
            ("offset".to_string(), offset.to_string()),
        ]
    }

    fn get_many(
        cli: SimplicateClient,
        iteration: Option<u32>,
        extra_params: Vec<(String, String)>,
    ) -> Vec<T> {
        let suffix = Self::url_suffix();
        let url = format!("https://{}.simplicate.nl/api/v2/{}", &cli.host, suffix);
        let count: u32 = iteration.unwrap_or(0);
        let n_count: u32 = count + 1;
        let mut params = Self::offset_params(count);
        for (k, v) in &extra_params {
            params.push((k.to_string(), v.to_string()));
        }
        let response = Client::new()
            .get(&url)
            .header("Authentication-Key", &cli.api_key)
            .header("Authentication-Secret", &cli.api_secret)
            .query(&params)
            .send()
            .expect("No response");
        let mut objects = Self::process_response(response);
        match objects.len() {
            100 => {
                let mut next_objects = Self::get_many(cli.clone(), Some(n_count), extra_params);
                objects.append(&mut next_objects);
                objects
            }
            _ => objects,
        }
    }

    fn get(cli: SimplicateClient, params: Option<Vec<(String, String)>>) -> Vec<T> {
        let extra_params = match params {
            Some(ls) => ls,
            None => vec![],
        };
        Self::get_many(cli, None, extra_params)
    }
}

pub trait Post<T>: Serialize {
    fn url_suffix() -> String;
    fn process_response(response: Response);
    fn post(&self, cli: SimplicateClient) {
        let suffix = Self::url_suffix();
        let url = format!("https://{}.simplicate.nl/api/v2/{}", &cli.host, suffix);
        let response = Client::new()
            .post(&url)
            .header("Authentication-Key", &cli.api_key)
            .header("Authentication-Secret", &cli.api_secret)
            .json(self)
            .send()
            .expect("No response");
        Self::process_response(response)
    }
}
