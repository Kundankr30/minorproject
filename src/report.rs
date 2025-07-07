use serde::Serialize;

pub fn generate_json_report<T: Serialize>(data: &T) -> String {
    serde_json::to_string_pretty(data).unwrap_or_else(|_| "{}".to_string())
}

pub fn generate_html_report<T: Serialize>(_data: &T) -> String {
    // TODO: Implement HTML report generation
    "<html><body><h1>Report</h1></body></html>".to_string()
} 