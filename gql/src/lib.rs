use {
    isahc::prelude::*,
    serde::Serialize,
    serde_json::{json, Value as JSONValue},
    std::convert::Into,
};

mod utils;
pub use utils::*;

mod result;
pub use result::*;

#[derive(Debug, Default)]
pub struct GraphqlClient {
    url: String,
    token: String,
    body: GraphqlRequestBody,
}
#[derive(Serialize, Debug, Default)]
pub struct GraphqlRequestBody {
    query: Option<String>,
    variables: Option<String>,
}

impl GraphqlClient {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            ..Default::default()
        }
    }

    pub fn auth(&mut self, token: impl Into<String>) -> &mut Self {
        self.token = token.into();
        self
    }

    pub fn query(&mut self, query: impl Into<String>) -> &mut Self {
        self.body.query = Some(query.into());
        self
    }

    pub fn send(&mut self, vars: Option<JSONValue>) -> GraphqlResult<JSONValue> {
        if let Some(vars) = &vars {
            if !vars.is_object() {
                return Err(GraphqlError::VariablesAreNotAnArray);
            }
        }

        self.body.variables = vars.map(|e| e.to_string());
        let body = json!(self.body).to_string();

        let client = Request::post(&self.url)
            .header("Authorization", format!("Bearer {}", self.token))
            .body(body)
            .unwrap();

        let mut resp = match client.send() {
            Ok(r) => r,
            Err(err) => {
                return Err(GraphqlError::RequestError(err));
            }
        };

        let data = resp.text().expect("server didnt respond with text");
        let data: JSONValue =
            serde_json::from_str(&data).expect("data returned by server is not correct JSON");
        let data = unwrap_json_object(data);

        if let Some(data) = data.get("data") {
            return Ok(data.clone());
        }

        if let Some(errors) = data.get("errors") {
            return Err(GraphqlError::GraphqlApiError(errors.clone()));
        }

        return Err(GraphqlError::NoData(JSONValue::Object(data)));
    }
}
