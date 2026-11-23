use serde_json::json;

/// Retrieves a summary of AI usage across all providers.
///
/// Returns a placeholder JSON with empty usage data for now.
/// TODO: Phase 2 - Implement actual usage metrics calculation from the database.
#[tauri::command]
pub async fn get_usage_summary() -> Result<serde_json::Value, String> {
    // TODO: Phase 2 - Query persistent storage to aggregate real token counts and costs
    Ok(json!({
        "total_tokens": 0,
        "total_cost": 0.0,
        "today_tokens": 0,
        "today_cost": 0.0,
        "providers": {}
    }))
}

/// Retrieves historical usage data for the specified number of days.
///
/// Returns an empty vector for now.
/// TODO: Phase 2 - Implement usage history retrieval and filtering by days.
#[tauri::command]
pub async fn get_usage_history(days: Option<u32>) -> Result<Vec<serde_json::Value>, String> {
    let _ = days;
    // TODO: Phase 2 - Fetch historical usage records from database based on `days` parameter
    Ok(vec![])
}
