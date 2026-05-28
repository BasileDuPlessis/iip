use dataset_sim::{build_seed_dataset, latest_month_projection};

fn main() {
    let dataset = build_seed_dataset();
    let latest_projection = latest_month_projection(&dataset);

    println!(
        "{}",
        serde_json::json!({
            "domains": dataset.domains.len(),
            "capabilities": dataset.capabilities.len(),
            "applications": dataset.applications.len(),
            "monthly_indicators": dataset.monthly_indicators.len(),
            "source_contexts": dataset.source_contexts.len(),
            "latest_projection_count": latest_projection.len(),
            "indicator_periods_per_application": dataset.monthly_indicators.len() / dataset.applications.len().max(1),
            "latest_period": latest_projection
                .iter()
                .map(|projection| projection.snapshot.period.as_str())
                .max()
                .unwrap_or("unknown")
        })
    );
}
