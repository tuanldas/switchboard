//! Phase 2 cost calculation module.
//!
//! Provides data structures and functions for computing USD costs of AI model usage
//! based on input, output, cache read, and cache write token counts and per-model pricing rates.

use serde::{Deserialize, Serialize};

/// Pricing rates for an AI model per million tokens.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelPricing {
    pub model: String,
    pub input_cost_per_million: f64,
    pub output_cost_per_million: f64,
    pub cache_read_cost_per_million: f64,
    pub cache_write_cost_per_million: f64,
}

impl ModelPricing {
    /// Creates a new `ModelPricing` entry.
    pub fn new(
        model: impl Into<String>,
        input_cost_per_million: f64,
        output_cost_per_million: f64,
        cache_read_cost_per_million: f64,
        cache_write_cost_per_million: f64,
    ) -> Self {
        Self {
            model: model.into(),
            input_cost_per_million,
            output_cost_per_million,
            cache_read_cost_per_million,
            cache_write_cost_per_million,
        }
    }
}

/// Returns a list of known default model prices per million tokens.
pub fn get_default_pricing() -> Vec<ModelPricing> {
    vec![
        // Anthropic models
        ModelPricing::new("claude-sonnet-4-20250514", 3.0, 15.0, 0.3, 3.75),
        ModelPricing::new("claude-opus-4-20250514", 15.0, 75.0, 1.5, 18.75),
        ModelPricing::new("claude-3-7-sonnet", 3.0, 15.0, 0.3, 3.75),
        ModelPricing::new("claude-3-5-sonnet-20241022", 3.0, 15.0, 0.3, 3.75),
        ModelPricing::new("claude-3-5-haiku-20241022", 0.8, 4.0, 0.08, 1.0),
        ModelPricing::new("claude-3-opus-20240229", 15.0, 75.0, 1.5, 18.75),
        // OpenAI models
        ModelPricing::new("gpt-4.1", 2.0, 8.0, 0.5, 2.0),
        ModelPricing::new("gpt-4o", 2.5, 10.0, 1.25, 2.5),
        ModelPricing::new("gpt-4o-mini", 0.15, 0.6, 0.075, 0.15),
        ModelPricing::new("o1", 15.0, 60.0, 7.5, 15.0),
        ModelPricing::new("o3-mini", 1.1, 4.4, 0.55, 1.1),
        // DeepSeek models
        ModelPricing::new("deepseek-chat", 0.14, 0.28, 0.014, 0.14),
        ModelPricing::new("deepseek-coder", 0.14, 0.28, 0.014, 0.14),
    ]
}

/// Computes the USD cost of an AI usage record given a pricing list.
///
/// Matches the pricing by model name (exact match first, then case-insensitive,
/// and fallback substring match). If no pricing matches or model is empty, returns 0.0.
///
/// Formula:
/// `(input * input_rate + output * output_rate + cache_read * cache_read_rate + cache_write * cache_write_rate) / 1_000_000`
pub fn calculate_cost(record: &super::UsageRecord, pricing: &[ModelPricing]) -> f64 {
    let model_str = match record.model.as_deref() {
        Some(m) if !m.trim().is_empty() => m.trim(),
        _ => return 0.0,
    };

    let matched = pricing
        .iter()
        .find(|p| p.model == model_str)
        .or_else(|| {
            pricing
                .iter()
                .find(|p| p.model.eq_ignore_ascii_case(model_str))
        })
        .or_else(|| {
            let lower = model_str.to_lowercase();
            pricing.iter().find(|p| {
                let p_lower = p.model.to_lowercase();
                lower.contains(&p_lower) || p_lower.contains(&lower)
            })
        });

    if let Some(rate) = matched {
        let input = record.input_tokens.max(0) as f64;
        let output = record.output_tokens.max(0) as f64;
        let cache_read = record.cache_read_tokens.max(0) as f64;
        let cache_write = record.cache_write_tokens.max(0) as f64;

        (input * rate.input_cost_per_million
            + output * rate.output_cost_per_million
            + cache_read * rate.cache_read_cost_per_million
            + cache_write * rate.cache_write_cost_per_million)
            / 1_000_000.0
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model_pricing_serde() {
        let pricing = ModelPricing::new("claude-sonnet-4-20250514", 3.0, 15.0, 0.3, 3.75);
        let serialized = serde_json::to_string(&pricing).expect("failed to serialize");
        let deserialized: ModelPricing =
            serde_json::from_str(&serialized).expect("failed to deserialize");
        assert_eq!(pricing, deserialized);
        assert_eq!(deserialized.model, "claude-sonnet-4-20250514");
        assert_eq!(deserialized.input_cost_per_million, 3.0);
        assert_eq!(deserialized.output_cost_per_million, 15.0);
        assert_eq!(deserialized.cache_read_cost_per_million, 0.3);
        assert_eq!(deserialized.cache_write_cost_per_million, 3.75);
    }

    #[test]
    fn test_default_pricing_required_models() {
        let default_pricing = get_default_pricing();

        // 1. claude-sonnet-4-20250514: input 3.0, output 15.0, cache_read 0.3, cache_write 3.75
        let sonnet = default_pricing
            .iter()
            .find(|p| p.model == "claude-sonnet-4-20250514")
            .expect("claude-sonnet-4-20250514 missing");
        assert_eq!(sonnet.input_cost_per_million, 3.0);
        assert_eq!(sonnet.output_cost_per_million, 15.0);
        assert_eq!(sonnet.cache_read_cost_per_million, 0.3);
        assert_eq!(sonnet.cache_write_cost_per_million, 3.75);

        // 2. claude-opus-4-20250514: input 15.0, output 75.0, cache_read 1.5, cache_write 18.75
        let opus = default_pricing
            .iter()
            .find(|p| p.model == "claude-opus-4-20250514")
            .expect("claude-opus-4-20250514 missing");
        assert_eq!(opus.input_cost_per_million, 15.0);
        assert_eq!(opus.output_cost_per_million, 75.0);
        assert_eq!(opus.cache_read_cost_per_million, 1.5);
        assert_eq!(opus.cache_write_cost_per_million, 18.75);

        // 3. gpt-4.1: input 2.0, output 8.0, cache_read 0.5, cache_write 2.0
        let gpt41 = default_pricing
            .iter()
            .find(|p| p.model == "gpt-4.1")
            .expect("gpt-4.1 missing");
        assert_eq!(gpt41.input_cost_per_million, 2.0);
        assert_eq!(gpt41.output_cost_per_million, 8.0);
        assert_eq!(gpt41.cache_read_cost_per_million, 0.5);
        assert_eq!(gpt41.cache_write_cost_per_million, 2.0);
    }

    #[test]
    fn test_calculate_cost() {
        let pricing = get_default_pricing();
        let record = super::super::UsageRecord {
            id: None,
            account_id: None,
            provider: None,
            model: Some("claude-sonnet-4-20250514".to_string()),
            input_tokens: 1_000_000,
            output_tokens: 100_000,
            cache_read_tokens: 500_000,
            cache_write_tokens: 200_000,
            cost_usd: 0.0,
            session_id: None,
            project: None,
            timestamp: None,
        };

        // (1_000_000 * 3.0 + 100_000 * 15.0 + 500_000 * 0.3 + 200_000 * 3.75) / 1_000_000
        // = (3_000_000 + 1_500_000 + 150_000 + 750_000) / 1_000_000
        // = 5_400_000 / 1_000_000 = 5.4
        let cost = calculate_cost(&record, &pricing);
        assert!((cost - 5.4).abs() < 1e-9);
    }

    #[test]
    fn test_calculate_cost_unknown_model() {
        let pricing = get_default_pricing();
        let record = super::super::UsageRecord {
            id: None,
            account_id: None,
            provider: None,
            model: Some("non-existent-model-xyz".to_string()),
            input_tokens: 1_000_000,
            output_tokens: 100_000,
            cache_read_tokens: 0,
            cache_write_tokens: 0,
            cost_usd: 0.0,
            session_id: None,
            project: None,
            timestamp: None,
        };

        let cost = calculate_cost(&record, &pricing);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_calculate_cost_none_model() {
        let pricing = get_default_pricing();
        let record = super::super::UsageRecord {
            id: None,
            account_id: None,
            provider: None,
            model: None,
            input_tokens: 1_000_000,
            output_tokens: 100_000,
            cache_read_tokens: 0,
            cache_write_tokens: 0,
            cost_usd: 0.0,
            session_id: None,
            project: None,
            timestamp: None,
        };

        let cost = calculate_cost(&record, &pricing);
        assert_eq!(cost, 0.0);
    }

    #[test]
    fn test_calculate_cost_zero_tokens() {
        let pricing = get_default_pricing();
        let record = super::super::UsageRecord {
            id: None,
            account_id: None,
            provider: None,
            model: Some("gpt-4.1".to_string()),
            input_tokens: 0,
            output_tokens: 0,
            cache_read_tokens: 0,
            cache_write_tokens: 0,
            cost_usd: 0.0,
            session_id: None,
            project: None,
            timestamp: None,
        };

        let cost = calculate_cost(&record, &pricing);
        assert_eq!(cost, 0.0);
    }
}
