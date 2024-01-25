use reqwest::blocking::{Client, Request};
use serde_json::Value;

#[derive(Clone)]
pub struct Pipeline {
	pub client: Client,
}

impl Pipeline {
	pub fn new() -> Pipeline {
		Self { client: Client::new() }
	}

	pub fn send(&self, request: Request) {
		println!("URL: {:?}", request.url().as_str());
		println!("Headers: {:?}", request.headers());
		println!("Method: {:?}", request.method());
		println!("Body: {:?}", request.body());

		let response = self.client.execute(request).unwrap();
		let value: Value = response.json().unwrap();

		println!("response: {:?}", value);

		// self.client.post(endpoint)
		// .header("Content-Type", "application/json")
		// .header("AUTHORIZATION", bearer_token)
		// .json(&request_body);
	}
}
