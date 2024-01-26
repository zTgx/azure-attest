use reqwest::blocking::{Client, Request, Response};

#[derive(Clone)]
pub struct Pipeline {
	pub client: Client,
}

impl Pipeline {
	pub fn new() -> Pipeline {
		Self { client: Client::new() }
	}

	pub fn send(&self, request: Request) -> Result<Response, String> {
		// println!("URL: {:?}", request.url().as_str());
		// println!("Headers: {:?}", request.headers());
		// println!("Method: {:?}", request.method());
		// println!("Body: {:?}", request.body());

		let response = self.client.execute(request).unwrap();
		Ok(response)
	}
}
