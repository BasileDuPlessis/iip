use std::{collections::BTreeMap, sync::Arc};

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use dataset_sim::{
    LatestApplicationProjection, SeedDataset, build_seed_dataset, latest_month_projection,
};
use domain::{
    ApplicationType, BusinessValueDetails, DoraMetrics, HeatBand, SourceCoverage, SourceFreshness,
    SourceSystem,
};
use serde::Serialize;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState {
    dataset: Arc<SeedDataset>,
    latest_projection: Arc<Vec<LatestApplicationProjection>>,
}

#[derive(Debug, Serialize)]
struct MapResponse {
    period: String,
    domains: Vec<DomainNode>,
}

#[derive(Debug, Serialize)]
struct DomainNode {
    id: String,
    name: String,
    capabilities: Vec<CapabilityNode>,
}

#[derive(Debug, Serialize)]
struct CapabilityNode {
    id: String,
    name: String,
    applications: Vec<ApplicationNode>,
}

#[derive(Debug, Clone, Serialize)]
struct ApplicationNode {
    id: String,
    name: String,
}

#[derive(Debug, Serialize)]
struct HeatmapResponse {
    period: String,
    items: Vec<HeatmapItem>,
}

#[derive(Debug, Serialize)]
struct HeatmapItem {
    entity_id: String,
    entity_name: String,
    business_value: HeatBand,
    technical_health: HeatBand,
    business_value_details: BusinessValueDetails,
    technical_health_details: DoraMetrics,
    colors: HeatmapColors,
}

#[derive(Debug, Serialize)]
struct HeatmapColors {
    business_value: &'static str,
    technical_health: &'static str,
}

#[derive(Debug, Serialize)]
struct SourcesResponse {
    period: String,
    items: Vec<ApplicationSourceNode>,
}

#[derive(Debug, Clone, Serialize)]
struct ApplicationSourceNode {
    application_id: String,
    application_name: String,
    application_type: ApplicationType,
    coverage: SourceCoverage,
    freshness: SourceFreshness,
    systems: Vec<SourceSystem>,
}

#[tokio::main]
async fn main() {
    let dataset = build_seed_dataset();
    let latest_projection = latest_month_projection(&dataset);
    let app_state = AppState {
        dataset: Arc::new(dataset),
        latest_projection: Arc::new(latest_projection),
    };

    let app = Router::new()
        .route("/map", get(get_map))
        .route("/heatmap", get(get_heatmap))
        .route("/sources", get(get_sources))
        .route("/applications/{id}/sources", get(get_application_sources))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind API listener");
    println!("API listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.expect("API server failed");
}

async fn get_map(State(state): State<AppState>) -> Json<MapResponse> {
    let applications_by_capability = applications_by_capability(&state.dataset);

    let domains = state
        .dataset
        .domains
        .iter()
        .map(|domain| {
            let capabilities = state
                .dataset
                .capabilities
                .iter()
                .filter(|capability| capability.domain_id == domain.id)
                .map(|capability| {
                    let applications = applications_by_capability
                        .get(&capability.id)
                        .cloned()
                        .unwrap_or_default();
                    CapabilityNode {
                        id: capability.id.clone(),
                        name: capability.name.clone(),
                        applications,
                    }
                })
                .collect();
            DomainNode {
                id: domain.id.clone(),
                name: domain.name.clone(),
                capabilities,
            }
        })
        .collect();

    Json(MapResponse {
        period: latest_period(&state.latest_projection),
        domains,
    })
}

async fn get_heatmap(State(state): State<AppState>) -> Json<HeatmapResponse> {
    let names = entity_names(&state.dataset);
    let items = state
        .latest_projection
        .iter()
        .map(|entry| HeatmapItem {
            entity_id: entry.application_id.clone(),
            entity_name: names
                .get(&entry.application_id)
                .cloned()
                .unwrap_or_else(|| entry.application_id.clone()),
            business_value: entry.snapshot.business_value.clone(),
            technical_health: entry.snapshot.technical_health.clone(),
            business_value_details: entry.snapshot.business_value_details.clone(),
            technical_health_details: entry.snapshot.technical_health_details.clone(),
            colors: HeatmapColors {
                business_value: business_value_color_token(&entry.snapshot.business_value),
                technical_health: technical_health_color_token(&entry.snapshot.technical_health),
            },
        })
        .collect();

    Json(HeatmapResponse {
        period: latest_period(&state.latest_projection),
        items,
    })
}

async fn get_sources(State(state): State<AppState>) -> Json<SourcesResponse> {
    Json(build_sources_response(&state))
}

async fn get_application_sources(
    Path(application_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ApplicationSourceNode>, StatusCode> {
    let name_and_type = state
        .dataset
        .applications
        .iter()
        .find(|app| app.id == application_id)
        .map(|app| (app.name.clone(), app.application_type.clone()))
        .ok_or(StatusCode::NOT_FOUND)?;

    let context = state
        .dataset
        .source_contexts
        .iter()
        .find(|context| context.application_id == application_id)
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(ApplicationSourceNode {
        application_id: context.application_id.clone(),
        application_name: name_and_type.0,
        application_type: name_and_type.1,
        coverage: context.coverage.clone(),
        freshness: context.freshness.clone(),
        systems: context.systems.clone(),
    }))
}

fn build_sources_response(state: &AppState) -> SourcesResponse {
    let app_names: BTreeMap<String, (String, ApplicationType)> = state
        .dataset
        .applications
        .iter()
        .map(|app| {
            (
                app.id.clone(),
                (app.name.clone(), app.application_type.clone()),
            )
        })
        .collect();

    let items = state
        .dataset
        .source_contexts
        .iter()
        .map(|context| {
            let (application_name, application_type) = app_names
                .get(&context.application_id)
                .cloned()
                .unwrap_or_else(|| ("Unknown".to_owned(), ApplicationType::Legacy));
            ApplicationSourceNode {
                application_id: context.application_id.clone(),
                application_name,
                application_type,
                coverage: context.coverage.clone(),
                freshness: context.freshness.clone(),
                systems: context.systems.clone(),
            }
        })
        .collect();

    SourcesResponse {
        period: latest_source_period(&state.dataset),
        items,
    }
}

fn applications_by_capability(dataset: &SeedDataset) -> BTreeMap<String, Vec<ApplicationNode>> {
    let mut map: BTreeMap<String, Vec<ApplicationNode>> = BTreeMap::new();
    for app in &dataset.applications {
        for capability_id in &app.capability_ids {
            map.entry(capability_id.clone())
                .or_default()
                .push(ApplicationNode {
                    id: app.id.clone(),
                    name: app.name.clone(),
                });
        }
    }
    map
}

fn entity_names(dataset: &SeedDataset) -> BTreeMap<String, String> {
    let mut names = BTreeMap::new();
    for domain in &dataset.domains {
        names.insert(domain.id.clone(), domain.name.clone());
    }
    for capability in &dataset.capabilities {
        names.insert(capability.id.clone(), capability.name.clone());
    }
    for application in &dataset.applications {
        names.insert(application.id.clone(), application.name.clone());
    }
    names
}

fn latest_period(latest_projection: &[LatestApplicationProjection]) -> String {
    latest_projection
        .first()
        .map(|item| item.snapshot.period.clone())
        .unwrap_or_else(|| "unknown".to_owned())
}

fn latest_source_period(dataset: &SeedDataset) -> String {
    dataset
        .source_contexts
        .first()
        .and_then(|context| context.monthly_evidence.last())
        .map(|snapshot| snapshot.period.clone())
        .unwrap_or_else(|| "unknown".to_owned())
}

fn business_value_color_token(band: &HeatBand) -> &'static str {
    match band {
        HeatBand::Low => "red",
        HeatBand::Medium => "amber",
        HeatBand::High => "green",
    }
}

#[cfg(test)]
mod tests {
    use super::{AppState, build_sources_response};
    use dataset_sim::{build_seed_dataset, latest_month_projection};
    use std::sync::Arc;

    #[test]
    fn sources_response_contains_one_item_per_application() {
        let dataset = build_seed_dataset();
        let state = AppState {
            latest_projection: Arc::new(latest_month_projection(&dataset)),
            dataset: Arc::new(dataset.clone()),
        };
        let response = build_sources_response(&state);
        assert_eq!(response.items.len(), dataset.applications.len());
    }

    #[test]
    fn sources_response_exposes_systems_and_period() {
        let dataset = build_seed_dataset();
        let state = AppState {
            latest_projection: Arc::new(latest_month_projection(&dataset)),
            dataset: Arc::new(dataset),
        };
        let response = build_sources_response(&state);
        assert_ne!(response.period, "unknown");
        assert!(response.items.iter().all(|item| !item.systems.is_empty()));
        assert!(response.items.iter().all(|item| {
            item.systems
                .iter()
                .all(|system| system.url.starts_with("/sources/"))
        }));
    }
}

fn technical_health_color_token(band: &HeatBand) -> &'static str {
    match band {
        HeatBand::Low => "green",
        HeatBand::Medium => "amber",
        HeatBand::High => "red",
    }
}
