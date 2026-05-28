#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code, unused_imports))]

use std::collections::BTreeMap;

use domain::{
    ApplicationType, BusinessValueDetails, DoraMetrics, HeatBand, SourceCoverage, SourceFreshness,
    SourceSystem,
};
use serde::Deserialize;

const API_BASE: &str = "http://127.0.0.1:3000";

#[derive(Debug, Deserialize)]
struct MapResponse {
    period: String,
    domains: Vec<DomainNode>,
}

#[derive(Debug, Deserialize)]
struct DomainNode {
    id: String,
    name: String,
    capabilities: Vec<CapabilityNode>,
}

#[derive(Debug, Deserialize)]
struct CapabilityNode {
    id: String,
    name: String,
    applications: Vec<ApplicationNode>,
}

#[derive(Debug, Deserialize)]
struct ApplicationNode {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize)]
struct HeatmapResponse {
    period: String,
    items: Vec<HeatmapItem>,
}

#[derive(Debug, Clone, Deserialize)]
struct HeatmapItem {
    entity_id: String,
    entity_name: String,
    business_value: HeatBand,
    technical_health: HeatBand,
    business_value_details: BusinessValueDetails,
    technical_health_details: DoraMetrics,
    colors: HeatmapColors,
}

#[derive(Debug, Clone, Deserialize)]
struct HeatmapColors {
    business_value: String,
    technical_health: String,
}

#[derive(Debug, Deserialize)]
struct SourcesResponse {
    period: String,
    items: Vec<ApplicationSourceNode>,
}

#[derive(Debug, Clone, Deserialize)]
struct ApplicationSourceNode {
    application_id: String,
    application_name: String,
    application_type: ApplicationType,
    coverage: SourceCoverage,
    freshness: SourceFreshness,
    systems: Vec<SourceSystem>,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn start() {
    wasm_bindgen_futures::spawn_local(async {
        if let Err(error) = render_app().await {
            render_error(&error);
        }
    });
}

#[cfg(not(target_arch = "wasm32"))]
pub fn start() {}

#[cfg(target_arch = "wasm32")]
async fn render_app() -> Result<(), String> {
    let map_response = gloo_net::http::Request::get(&format!("{API_BASE}/map"))
        .send()
        .await
        .map_err(|err| format!("failed to fetch map: {err}"))?
        .json::<MapResponse>()
        .await
        .map_err(|err| format!("failed to decode map payload: {err}"))?;

    let heatmap_response = gloo_net::http::Request::get(&format!("{API_BASE}/heatmap"))
        .send()
        .await
        .map_err(|err| format!("failed to fetch heatmap: {err}"))?
        .json::<HeatmapResponse>()
        .await
        .map_err(|err| format!("failed to decode heatmap payload: {err}"))?;

    let sources_response = gloo_net::http::Request::get(&format!("{API_BASE}/sources"))
        .send()
        .await
        .map_err(|err| format!("failed to fetch source context: {err}"))?
        .json::<SourcesResponse>()
        .await
        .map_err(|err| format!("failed to decode source context payload: {err}"))?;

    render_map_view(map_response, heatmap_response, sources_response)
}

#[cfg(target_arch = "wasm32")]
fn render_map_view(
    map_data: MapResponse,
    heatmap_data: HeatmapResponse,
    sources_data: SourcesResponse,
) -> Result<(), String> {
    let document = web_sys::window()
        .and_then(|window| window.document())
        .ok_or_else(|| "document is not available".to_owned())?;
    let body = document
        .body()
        .ok_or_else(|| "document body is not available".to_owned())?;

    let heatmap_by_entity: BTreeMap<String, HeatmapItem> = heatmap_data
        .items
        .into_iter()
        .map(|item| (item.entity_id.clone(), item))
        .collect();
    let sources_by_app: BTreeMap<String, ApplicationSourceNode> = sources_data
        .items
        .into_iter()
        .map(|item| (item.application_id.clone(), item))
        .collect();

    let mut html = String::new();
    html.push_str("<main style='font-family: Inter, Arial, sans-serif; max-width: 1280px; margin: 24px auto; padding: 0 16px;'>");
    html.push_str("<h1 style='margin-bottom: 8px;'>Ecosystem Map</h1>");
    html.push_str(&format!(
        "<p style='margin-top: 0; color: #475569;'>Monthly snapshot: <strong>{}</strong> | Heatmap snapshot: <strong>{}</strong> | Source snapshot: <strong>{}</strong></p>",
        escape_html(&map_data.period),
        escape_html(&heatmap_data.period),
        escape_html(&sources_data.period),
    ));
    html.push_str(&legend_html());
    html.push_str("<section style='display:grid;grid-template-columns:repeat(auto-fit,minmax(360px,1fr));gap:16px;'>");

    for domain in map_data.domains {
        html.push_str("<article style='border: 1px solid #CBD5E1; border-radius: 10px; padding: 14px; background:#FFFFFF; box-shadow:0 1px 3px rgba(15,23,42,0.08);'>");
        html.push_str(&format!(
            "<h2 style='margin: 0 0 12px 0; font-size: 20px; color:#0F172A;'>{}</h2>",
            escape_html(&domain.name)
        ));
        html.push_str("<div style='display:grid;grid-template-columns:1fr;gap:10px;'>");

        for capability in domain.capabilities {
            html.push_str(
                "<section style='border:1px solid #E2E8F0;border-radius:8px;padding:10px;background:#FFFFFF;'>",
            );
            html.push_str(&format!(
                "<h3 style='margin:0 0 8px 0;font-size:15px;color:#1E293B;'>{}</h3>",
                escape_html(&capability.name),
            ));
            html.push_str("<div style='display:flex;flex-wrap:wrap;gap:8px;'>");

            for app in capability.applications {
                let app_heat = heatmap_by_entity.get(&app.id);
                let app_sources = sources_by_app.get(&app.id);
                html.push_str(&application_tile_html(&app, app_heat, app_sources));
            }
            html.push_str("</div></section>");
        }
        html.push_str("</div></article>");
    }
    html.push_str("</section></main>");

    body.set_inner_html(&html);
    Ok(())
}

fn application_tile_html(
    app: &ApplicationNode,
    item: Option<&HeatmapItem>,
    source: Option<&ApplicationSourceNode>,
) -> String {
    let name = escape_html(&app.name);
    let width_style = "min-width:140px;max-width:220px;flex:1 1 160px;";
    let label_style = "display:block;font-size:12px;line-height:1.25;font-weight:600;";
    let source_badge = source_summary_badge(source);

    match item {
        Some(item) => {
            let title = escape_html(&details_title(item, source));
            format!(
                "<div title='{title}' style='{width_style}padding:10px;border-radius:8px;background:#FFFFFF;border:1px solid #CBD5E1;color:#0F172A;box-shadow:0 1px 2px rgba(15,23,42,0.08);'>
                    <span style='{label_style}'>{name}</span>
                    <div style='display:grid;gap:6px;margin-top:8px;'>
                        {bv_band}
                        {th_band}
                        {source_badge}
                    </div>
                 </div>",
                bv_band = metric_band("BV", &item.colors.business_value, heat_band_label(&item.business_value)),
                th_band = metric_band("TH", &item.colors.technical_health, heat_band_label(&item.technical_health)),
                source_badge = source_badge,
            )
        }
        None => format!(
            "<div title='No application heatmap data' style='{width_style}padding:10px;border-radius:8px;background:#E2E8F0;border:1px solid #CBD5E1;color:#334155;'>
                <span style='{label_style}'>{name}</span>
                <span style='display:block;font-size:11px;opacity:0.9;margin-top:4px;'>No heatmap data</span>
                {source_badge}
             </div>"
        ),
    }
}

fn details_title(item: &HeatmapItem, source: Option<&ApplicationSourceNode>) -> String {
    let base = format!(
        "{} | BV: {} | TH: {} | OA: {} | CPC: {} | OC: {} | PPI: {} | DF/mo: {} | LT(h): {} | CFR%: {} | MTTR(h): {}",
        item.entity_name,
        heat_band_label(&item.business_value),
        heat_band_label(&item.technical_health),
        item.business_value_details.operational_adoption_score,
        item.business_value_details.cross_plant_coverage_score,
        item.business_value_details.operational_criticality_score,
        item.business_value_details.process_performance_impact_score,
        item.technical_health_details.deployment_frequency_per_month,
        item.technical_health_details.lead_time_for_changes_hours,
        item.technical_health_details.change_failure_rate_pct,
        item.technical_health_details.mean_time_to_restore_hours
    );
    match source {
        Some(source) => format!(
            "{base} | Sources: {} ({}) | Type: {} | Reporting plants: {}/{} | Freshness stale: {}",
            source.systems.len(),
            source.application_name,
            application_type_label(&source.application_type),
            source.coverage.reporting_plants,
            source.coverage.targeted_plants,
            source.freshness.stale
        ),
        None => base,
    }
}

fn heat_band_label(band: &HeatBand) -> &'static str {
    match band {
        HeatBand::Low => "Low",
        HeatBand::Medium => "Medium",
        HeatBand::High => "High",
    }
}

fn badge_html(label: &str, token: &str) -> String {
    format!(
        "<span style='display:inline-block;padding:2px 8px;border-radius:999px;font-size:12px;font-weight:600;color:white;background:{};'>{}</span>",
        color_hex(token),
        label
    )
}

fn metric_band(metric: &str, token: &str, value: &str) -> String {
    format!(
        "<span style='display:block;padding:4px 8px;border-radius:6px;font-size:11px;font-weight:700;background:{};color:{};'>
            {}: {}
         </span>",
        color_hex(token),
        text_color_for_token(token),
        metric,
        value
    )
}

fn source_summary_badge(source: Option<&ApplicationSourceNode>) -> String {
    match source {
        Some(source) => format!(
            "<span style='display:block;padding:4px 8px;border-radius:6px;font-size:11px;font-weight:700;background:#E2E8F0;color:#0F172A;'>
                SRC: {} | {} / {} plants
             </span>",
            source.systems.len(),
            source.coverage.reporting_plants,
            source.coverage.targeted_plants
        ),
        None => "<span style='display:block;padding:4px 8px;border-radius:6px;font-size:11px;font-weight:700;background:#E2E8F0;color:#475569;'>SRC: n/a</span>".to_owned(),
    }
}

fn application_type_label(application_type: &ApplicationType) -> &'static str {
    match application_type {
        ApplicationType::Custom => "Custom",
        ApplicationType::OffTheShelf => "Off-the-shelf",
        ApplicationType::SaaS => "SaaS",
        ApplicationType::Legacy => "Legacy",
        ApplicationType::Platform => "Platform",
        ApplicationType::ShadowIT => "Shadow IT",
    }
}

fn legend_html() -> String {
    format!(
        "        <div style='display:flex;align-items:center;gap:8px;flex-wrap:wrap;margin-bottom:14px;padding:10px;border:1px solid #E2E8F0;border-radius:8px;background:#F8FAFC;'>
            <strong>Legend:</strong>
            {}
            {}
            {}
            <span style='color:#475569;'>Green = OK, Red = Bad, Amber = Medium (BV/TH) + SRC for connected source coverage</span>
         </div>",
        badge_html("Green", "green"),
        badge_html("Amber", "amber"),
        badge_html("Red", "red"),
    )
}

fn color_hex(token: &str) -> &'static str {
    match token {
        "blue" => "#16A34A",
        "green" => "#16A34A",
        "amber" => "#D97706",
        "red" => "#DC2626",
        "slate" => "#475569",
        _ => "#64748B",
    }
}

fn text_color_for_token(token: &str) -> &'static str {
    match token {
        "amber" => "#111827",
        _ => "#FFFFFF",
    }
}

fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(target_arch = "wasm32")]
fn render_error(error: &str) {
    if let Some(document) = web_sys::window().and_then(|window| window.document()) {
        if let Some(body) = document.body() {
            body.set_inner_html(&format!(
                "<main style='font-family: Arial, sans-serif; max-width: 800px; margin: 40px auto; color: #B91C1C;'>
                    <h1>Failed to load ecosystem map</h1>
                    <p>{}</p>
                 </main>",
                escape_html(error)
            ));
        }
    }
}
