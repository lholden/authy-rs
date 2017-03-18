use serde_json;
use std::collections::HashMap;

use error::AuthyError;
use client::{Client, Status};

const PREFIX: &'static str = "onetouch";

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ApprovalRequest {
    uuid: String,
}

pub fn request(client: &Client, id: u32, message: &str, details: Option<&HashMap<&str, String>>, hidden_details: Option<&HashMap<&str, String>>, logos: Option<&HashMap<&str, String>>, seconds_to_expire: Option<u32>) -> Result<(Status, ApprovalRequest), AuthyError> {
    let mut params: Vec<(String, String)> = vec![];
    params.push(("message".into(), message.into()));

    if let Some(seconds_to_expire) = seconds_to_expire {
        params.push(("seconds_to_expire".into(), seconds_to_expire.to_string()));
    }

    if let Some(details) = details {
        for (k, v) in details {
            params.push((format!("details[{}]", k), v.clone()));
        }
    }

    if let Some(hidden_details) = hidden_details {
        for (k, v) in hidden_details {
            params.push((format!("hidden_details[{}]", k), v.clone()));
        }
    }
    
    if let Some(logos) = logos {
        for (k, v) in logos {
            params.push(("logos[][res]".into(), k.clone().into()));
            params.push(("logos[][res]".into(), v.clone()));
        }
    }

    let (status, res) = client.post(PREFIX, &format!("users/{}/approval_requests", id), None, Some(params))?;

    let approval_request = serde_json::from_value(res["approval_request"].clone())?;

    Ok((status, approval_request))
}
