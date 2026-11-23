// Parses JSONL logs from various AI coding tool providers

pub fn parse_claude_log(path: &std::path::Path) -> anyhow::Result<Vec<super::UsageRecord>> {
    log::debug!("Parsing Claude log: {:?}", path);
    // TODO: Implement Claude log parsing
    Ok(Vec::new())
}

pub fn parse_codex_log(path: &std::path::Path) -> anyhow::Result<Vec<super::UsageRecord>> {
    log::debug!("Parsing Codex log: {:?}", path);
    Ok(Vec::new())
}

pub fn parse_cursor_log(path: &std::path::Path) -> anyhow::Result<Vec<super::UsageRecord>> {
    log::debug!("Parsing Cursor log: {:?}", path);
    Ok(Vec::new())
}
