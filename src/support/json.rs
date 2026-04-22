use serde_json::Value;

pub fn parse_number(value: Option<&Value>) -> Option<f64> {
    match value {
        None | Some(Value::Null) => None,
        Some(Value::Number(number)) => number.as_f64(),
        Some(Value::String(text)) => {
            let stripped = text.trim();
            if stripped.is_empty() {
                None
            } else {
                stripped.parse::<f64>().ok()
            }
        }
        _ => None,
    }
}

pub fn json_string(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".to_string())
}
