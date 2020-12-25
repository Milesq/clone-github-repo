use serde_json::Value;
#[derive(Debug)]
pub enum GraphqlError {
    VariablesAreNotAnArray,
    RequestError(isahc::Error),
    GraphqlApiError(Value),
    NoData(Value),
}

pub type GraphqlResult<T> = Result<T, GraphqlError>;
