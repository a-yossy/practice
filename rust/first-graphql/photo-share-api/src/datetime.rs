use async_graphql::{InputValueError, Scalar, ScalarType, Value};
use chrono::{DateTime as ChronoDateTime, Utc};
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct DateTime(pub ChronoDateTime<Utc>);

#[Scalar]
impl ScalarType for DateTime {
    fn parse(value: Value) -> async_graphql::InputValueResult<Self> {
        if let Value::String(value) = &value {
            let date_time = value
                .parse::<ChronoDateTime<Utc>>()
                .map_err(|e| InputValueError::custom(format!("無効な DateTime: {}", e)))?;
            Ok(DateTime(date_time))
        } else {
            Err(InputValueError::expected_type(value))
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_rfc3339())
    }
}
