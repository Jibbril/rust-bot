use std::collections::HashMap;

#[allow(dead_code)]
pub fn params_to_query_str(params: &HashMap<String,String>) -> String {
    params.iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("&")
}

