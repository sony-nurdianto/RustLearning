use actix_web::{HttpResponse, Responder};
use redis::FromRedisValue;
use serde_json::{json, Error as SerdeJsonError, Value};

use crate::utils::encryption_handlers::jwt_handlers::base64_to_json;

pub async fn send_messages() -> impl Responder {
    let return_response = base64_to_json(String::from("eyJpdiI6Im8vWXJ4RkkvYlFnV0dON2RmYVF1OWc9PSIsInZhbHVlIjoiS0tJUTZ0ZnVpYzNIWmxHVE5Ub2hUS1YwdStWVGdKN3g2QzBaSWFuMFRvNlBMTnlpTCtWT1V1U0s4TVRXU0pDSERidXNDUnM4aXNNQk5mVDNPWHBQNEtxYVhSSnRHb3FsYUxQMVdROGdKZm5DNXptc2pHK3hLUGNWa08zNHV5d2oiLCJtYWMiOiIxMmZjMzdiOWViMWVhYTg4NzY4YWJlZDQwYmQ5NTZiYzUwMjI1MTNkMjI4OTFjZmU3NGI4NTk2Zjg5YmYzMjE1IiwidGFnIjoiIn0"));

    match return_response {
        Ok(respone) => {
            let mut decoded_base64: String = String::new();
            if let Some(value) = String::from_byte_vec(&respone) {
                if let Some(first_value) = value.into_iter().next() {
                    decoded_base64 = first_value
                }
            }
            let result_json: Result<Value, SerdeJsonError> = serde_json::from_str(&decoded_base64);
            if let Ok(mut rj) = result_json {
                if let Some(data) = rj.as_object_mut() {
                    data.insert("alg".to_string(), json!("HS256"));
                };
            }
            // if let Some(value) = String::from_byte_vec(&respone) {
            //     // let mut json_value: Value = serde_json::from_str(&value[0]).unwrap();
            //     //
            //     // if let Some(data) = json_value.as_object_mut() {
            //     //     data.insert("alg".to_string(), json!("wrestling"));
            //     // }
            //     //
            //     // let new_json = serde_json::to_string_pretty(&json_value).unwrap();
            //     // println!("{}", new_json)
            // }
            return HttpResponse::Ok().body(respone);
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
