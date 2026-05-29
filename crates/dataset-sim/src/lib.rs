use std::collections::{BTreeMap, BTreeSet};

use domain::{
    Application, ApplicationSourceContext, ApplicationType, BusinessValueDetails, Capability,
    Domain, DoraMetrics, HeatBand, IndicatorSnapshot, SourceCadence, SourceCategory,
    SourceCompleteness, SourceCoverage, SourceEvidenceSnapshot, SourceFreshness, SourceReliability,
    SourceSystem,
};
use serde::{Deserialize, Serialize};

pub const PERIOD_COUNT: usize = 24;
pub const LATEST_PERIOD: &str = "2026-05";

const ENTERPRISE_FIXTURE: &str = include_str!("../data/enterprise.json");
const APPLICATION_FIXTURES: &[&str] = &[
    include_str!("../data/applications/sap-s4-erp.json"),
    include_str!("../data/applications/mes-alpha.json"),
    include_str!("../data/applications/plm-core.json"),
    include_str!("../data/applications/cad-vault.json"),
    include_str!("../data/applications/maintenance-hub.json"),
    include_str!("../data/applications/predictive-insight.json"),
    include_str!("../data/applications/plant-scheduler.json"),
    include_str!("../data/applications/operator-portal.json"),
    include_str!("../data/applications/quality-tracker.json"),
    include_str!("../data/applications/compliance-hub.json"),
    include_str!("../data/applications/inventory-pro.json"),
    include_str!("../data/applications/energy-monitor.json"),
    include_str!("../data/applications/industrial-data-platform-app.json"),
    include_str!("../data/applications/legacy-reporting-cube.json"),
    include_str!("../data/applications/excel-macros-local-ops.json"),
];
const METRIC_FIXTURES: &[&str] = &[
    include_str!("../data/metrics/sap-s4-erp/service-now.json"),
    include_str!("../data/metrics/mes-alpha/service-now.json"),
    include_str!("../data/metrics/plm-core/service-now.json"),
    include_str!("../data/metrics/cad-vault/legacy-incidents.json"),
    include_str!("../data/metrics/maintenance-hub/github-actions.json"),
    include_str!("../data/metrics/predictive-insight/application-logs.json"),
    include_str!("../data/metrics/plant-scheduler/github-actions.json"),
    include_str!("../data/metrics/operator-portal/github-actions.json"),
    include_str!("../data/metrics/quality-tracker/github-actions.json"),
    include_str!("../data/metrics/compliance-hub/application-logs.json"),
    include_str!("../data/metrics/inventory-pro/application-logs.json"),
    include_str!("../data/metrics/energy-monitor/application-logs.json"),
    include_str!("../data/metrics/industrial-data-platform-app/jenkins.json"),
    include_str!("../data/metrics/legacy-reporting-cube/legacy-incidents.json"),
    include_str!("../data/metrics/excel-macros-local-ops/shared-spreadsheets.json"),
];

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ApplicationMonthlyIndicator {
    pub application_id: String,
    pub snapshot: IndicatorSnapshot,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SeedDataset {
    pub domains: Vec<Domain>,
    pub capabilities: Vec<Capability>,
    pub applications: Vec<Application>,
    pub monthly_indicators: Vec<ApplicationMonthlyIndicator>,
    pub source_contexts: Vec<ApplicationSourceContext>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct LatestApplicationProjection {
    pub application_id: String,
    pub snapshot: IndicatorSnapshot,
}

#[derive(Debug, Clone, Deserialize)]
struct EnterpriseFixture {
    #[allow(dead_code)]
    name: String,
    domains: Vec<Domain>,
    capabilities: Vec<Capability>,
    applications: Vec<ApplicationSummaryFixture>,
}

#[derive(Debug, Clone, Deserialize)]
struct ApplicationSummaryFixture {
    id: String,
    name: String,
    application_type: ApplicationType,
}

#[derive(Debug, Clone, Deserialize)]
struct ApplicationFixture {
    id: String,
    name: String,
    application_type: ApplicationType,
    #[allow(dead_code)]
    story: String,
    capability_ids: Vec<String>,
    targeted_plants: u8,
    source_systems: Vec<SourceSystemFixture>,
}

#[derive(Debug, Clone, Deserialize)]
struct SourceSystemFixture {
    id: String,
    name: String,
    source_kind: String,
    category: SourceCategory,
    cadence: SourceCadence,
    reliability: SourceReliability,
    completeness: SourceCompleteness,
    url: String,
}

#[derive(Debug, Clone, Deserialize)]
struct MetricSourceFixture {
    id: String,
    application_id: String,
    source_url: String,
    #[allow(dead_code)]
    kind: String,
    #[allow(dead_code)]
    description: String,
}

#[derive(Debug, Clone)]
struct LoadedFixtures {
    enterprise: EnterpriseFixture,
    applications: Vec<ApplicationFixture>,
    metrics: Vec<MetricSourceFixture>,
}

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
enum SimulationProfile {
    Legacy,
    Modern,
    Platform,
    ShadowIT,
    SaaS,
    OffTheShelf,
}

pub fn build_seed_dataset() -> SeedDataset {
    let fixtures = load_fixtures().expect("dataset-sim fixtures must be valid");
    let periods = monthly_periods_ending(LATEST_PERIOD, PERIOD_COUNT);
    let applications = fixtures
        .applications
        .iter()
        .map(|fixture| Application {
            id: fixture.id.clone(),
            name: fixture.name.clone(),
            application_type: fixture.application_type.clone(),
            capability_ids: fixture.capability_ids.clone(),
        })
        .collect::<Vec<_>>();

    let mut monthly_indicators = Vec::new();
    let mut source_contexts = Vec::new();

    for (index, fixture) in fixtures.applications.iter().enumerate() {
        let seed = (index as i32) + metric_seed(&fixtures.metrics, &fixture.id);
        monthly_indicators.extend(build_application_indicators(fixture, seed, &periods));
        source_contexts.push(build_application_source_context(fixture, seed, &periods));
    }

    SeedDataset {
        domains: fixtures.enterprise.domains,
        capabilities: fixtures.enterprise.capabilities,
        applications,
        monthly_indicators,
        source_contexts,
    }
}

pub fn latest_month_projection(dataset: &SeedDataset) -> Vec<LatestApplicationProjection> {
    let mut by_application: BTreeMap<String, &ApplicationMonthlyIndicator> = BTreeMap::new();
    for indicator in &dataset.monthly_indicators {
        match by_application.get(&indicator.application_id) {
            Some(existing) if existing.snapshot.period >= indicator.snapshot.period => {}
            _ => {
                by_application.insert(indicator.application_id.clone(), indicator);
            }
        }
    }

    by_application
        .into_values()
        .map(|indicator| LatestApplicationProjection {
            application_id: indicator.application_id.clone(),
            snapshot: indicator.snapshot.clone(),
        })
        .collect()
}

fn load_fixtures() -> Result<LoadedFixtures, String> {
    let enterprise = serde_json::from_str::<EnterpriseFixture>(ENTERPRISE_FIXTURE)
        .map_err(|err| format!("invalid enterprise fixture: {err}"))?;
    let applications = APPLICATION_FIXTURES
        .iter()
        .map(|fixture| {
            serde_json::from_str::<ApplicationFixture>(fixture)
                .map_err(|err| format!("invalid application fixture: {err}"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let metrics = METRIC_FIXTURES
        .iter()
        .map(|fixture| {
            serde_json::from_str::<MetricSourceFixture>(fixture)
                .map_err(|err| format!("invalid metric fixture: {err}"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let loaded = LoadedFixtures {
        enterprise,
        applications,
        metrics,
    };
    validate_fixtures(&loaded)?;
    Ok(loaded)
}

fn validate_fixtures(fixtures: &LoadedFixtures) -> Result<(), String> {
    let enterprise_app_ids = fixtures
        .enterprise
        .applications
        .iter()
        .map(|app| app.id.as_str())
        .collect::<BTreeSet<_>>();
    let detail_app_ids = fixtures
        .applications
        .iter()
        .map(|app| app.id.as_str())
        .collect::<BTreeSet<_>>();
    if enterprise_app_ids != detail_app_ids {
        return Err(format!(
            "enterprise application ids do not match detail fixtures: enterprise={enterprise_app_ids:?} detail={detail_app_ids:?}"
        ));
    }

    let capability_ids = fixtures
        .enterprise
        .capabilities
        .iter()
        .map(|capability| capability.id.as_str())
        .collect::<BTreeSet<_>>();
    for application in &fixtures.applications {
        let Some(summary) = fixtures
            .enterprise
            .applications
            .iter()
            .find(|summary| summary.id == application.id)
        else {
            return Err(format!("missing enterprise summary for {}", application.id));
        };
        if summary.name != application.name
            || summary.application_type != application.application_type
        {
            return Err(format!(
                "application summary mismatch for {}",
                application.id
            ));
        }
        for capability_id in &application.capability_ids {
            if !capability_ids.contains(capability_id.as_str()) {
                return Err(format!(
                    "{} references unknown capability {}",
                    application.id, capability_id
                ));
            }
        }
        for source in &application.source_systems {
            if !source.url.starts_with("/sources/") {
                return Err(format!(
                    "{} source {} has non-relative source URL {}",
                    application.id, source.id, source.url
                ));
            }
        }
        validate_required_source_categories(application)?;
    }

    let source_urls_by_application = fixtures
        .applications
        .iter()
        .map(|application| {
            (
                application.id.as_str(),
                application
                    .source_systems
                    .iter()
                    .map(|source| source.url.as_str())
                    .collect::<BTreeSet<_>>(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    for metric in &fixtures.metrics {
        let Some(source_urls) = source_urls_by_application.get(metric.application_id.as_str())
        else {
            return Err(format!(
                "metric {} references unknown application {}",
                metric.id, metric.application_id
            ));
        };
        if !source_urls.contains(metric.source_url.as_str()) {
            return Err(format!(
                "metric {} references source URL not defined on {}",
                metric.id, metric.application_id
            ));
        }
    }

    Ok(())
}

fn validate_required_source_categories(application: &ApplicationFixture) -> Result<(), String> {
    let has_technical = has_source_category(application, SourceCategory::TechnicalDelivery);
    let has_adoption = has_source_category(application, SourceCategory::AdoptionDiffusion);
    let has_outcome = has_source_category(application, SourceCategory::OperationalOutcome);

    let profile = simulation_profile(application);
    let valid = match profile {
        SimulationProfile::ShadowIT => has_adoption && has_outcome,
        SimulationProfile::SaaS | SimulationProfile::OffTheShelf => {
            has_technical && has_adoption && has_outcome
        }
        SimulationProfile::Legacy | SimulationProfile::Modern | SimulationProfile::Platform => {
            has_technical && has_adoption && has_outcome
        }
    };

    if valid {
        Ok(())
    } else {
        Err(format!(
            "{} does not define source coverage required for {:?}",
            application.id, profile
        ))
    }
}

fn simulation_profile(application: &ApplicationFixture) -> SimulationProfile {
    match application.application_type {
        ApplicationType::Custom => SimulationProfile::Modern,
        ApplicationType::OffTheShelf => SimulationProfile::OffTheShelf,
        ApplicationType::SaaS => SimulationProfile::SaaS,
        ApplicationType::Legacy => SimulationProfile::Legacy,
        ApplicationType::Platform => SimulationProfile::Platform,
        ApplicationType::ShadowIT => SimulationProfile::ShadowIT,
    }
}

fn has_source_category(application: &ApplicationFixture, category: SourceCategory) -> bool {
    application
        .source_systems
        .iter()
        .any(|source| source.category == category)
}

fn monthly_periods_ending(latest_period: &str, count: usize) -> Vec<String> {
    let (latest_year, latest_month) = parse_period(latest_period);
    let latest_index = latest_year * 12 + (latest_month - 1);
    let first_index = latest_index - (count as i32) + 1;
    (0..count)
        .map(|offset| {
            let month_index = first_index + offset as i32;
            let year = month_index.div_euclid(12);
            let month = month_index.rem_euclid(12) + 1;
            format!("{year:04}-{month:02}")
        })
        .collect()
}

fn parse_period(period: &str) -> (i32, i32) {
    period
        .split_once('-')
        .and_then(|(y, m)| Some((y.parse::<i32>().ok()?, m.parse::<i32>().ok()?)))
        .expect("period must be in YYYY-MM format")
}

fn build_application_indicators(
    application: &ApplicationFixture,
    seed: i32,
    periods: &[String],
) -> Vec<ApplicationMonthlyIndicator> {
    periods
        .iter()
        .enumerate()
        .map(|(month_idx, period)| {
            let trend = month_idx as i32;
            let profile = simulation_profile(application);
            let business_value_details = business_value_details(profile, seed, trend);
            let business_value_score = weighted_avg(
                &[
                    (business_value_details.operational_adoption_score, 30),
                    (business_value_details.cross_plant_coverage_score, 20),
                    (business_value_details.operational_criticality_score, 30),
                    (business_value_details.process_performance_impact_score, 20),
                ],
                100,
            );

            let technical_health_details =
                dora_metrics(profile, seed, trend, &application.source_systems);
            let technical_health_risk_score = weighted_avg(
                &[
                    (
                        reverse_risk_score(
                            technical_health_details.deployment_frequency_per_month as u16,
                            1,
                            40,
                        ),
                        25,
                    ),
                    (
                        risk_score(technical_health_details.lead_time_for_changes_hours, 2, 336),
                        25,
                    ),
                    (
                        risk_score(
                            technical_health_details.change_failure_rate_pct as u16,
                            2,
                            45,
                        ),
                        25,
                    ),
                    (
                        risk_score(technical_health_details.mean_time_to_restore_hours, 1, 120),
                        25,
                    ),
                ],
                100,
            );

            ApplicationMonthlyIndicator {
                application_id: application.id.clone(),
                snapshot: IndicatorSnapshot {
                    period: period.clone(),
                    business_value: band_from_score(business_value_score),
                    technical_health: band_from_score(technical_health_risk_score),
                    business_value_details,
                    technical_health_details,
                },
            }
        })
        .collect()
}

fn business_value_details(
    profile: SimulationProfile,
    seed: i32,
    trend: i32,
) -> BusinessValueDetails {
    let (adoption, coverage, criticality, impact, trend_weight, volatility) = match profile {
        SimulationProfile::Legacy => (38, 36, 76, 45, -1, 7),
        SimulationProfile::Modern => (54, 60, 66, 64, 3, 5),
        SimulationProfile::Platform => (64, 78, 82, 72, 2, 4),
        SimulationProfile::ShadowIT => (34, 28, 42, 38, 1, 18),
        SimulationProfile::SaaS => (66, 62, 58, 67, 2, 6),
        SimulationProfile::OffTheShelf => (58, 66, 74, 62, 1, 5),
    };

    BusinessValueDetails {
        operational_adoption_score: shaped_score(adoption, seed, trend, trend_weight, volatility),
        cross_plant_coverage_score: shaped_score(
            coverage,
            seed + 2,
            trend,
            trend_weight / 2,
            volatility,
        ),
        operational_criticality_score: shaped_score(
            criticality,
            seed + 4,
            trend,
            0,
            volatility / 2,
        ),
        process_performance_impact_score: shaped_score(
            impact,
            seed + 6,
            trend,
            trend_weight,
            volatility,
        ),
    }
}

fn dora_metrics(
    profile: SimulationProfile,
    seed: i32,
    trend: i32,
    sources: &[SourceSystemFixture],
) -> DoraMetrics {
    let telemetry_bonus = sources
        .iter()
        .filter(|source| {
            source.category == SourceCategory::TechnicalDelivery
                && source.reliability == SourceReliability::High
                && source.completeness != SourceCompleteness::Sparse
        })
        .count()
        .min(4) as i32;

    match profile {
        SimulationProfile::Legacy => DoraMetrics {
            deployment_frequency_per_month: ranged(seed, trend, 1, 5, 1) as u8,
            lead_time_for_changes_hours: ranged(seed + 2, trend, 190, 336, -2) as u16,
            change_failure_rate_pct: ranged(seed + 3, trend, 26, 45, -1) as u8,
            mean_time_to_restore_hours: ranged(seed + 4, trend, 76, 120, -1) as u16,
        },
        SimulationProfile::Modern => DoraMetrics {
            deployment_frequency_per_month: ranged(seed, trend, 16 + telemetry_bonus, 38, 2) as u8,
            lead_time_for_changes_hours: ranged(seed + 2, trend, 8, 76, -3) as u16,
            change_failure_rate_pct: ranged(seed + 3, trend, 3, 14, -1) as u8,
            mean_time_to_restore_hours: ranged(seed + 4, trend, 2, 28, -1) as u16,
        },
        SimulationProfile::Platform => DoraMetrics {
            deployment_frequency_per_month: ranged(seed, trend, 24 + telemetry_bonus, 40, 2) as u8,
            lead_time_for_changes_hours: ranged(seed + 2, trend, 4, 48, -2) as u16,
            change_failure_rate_pct: ranged(seed + 3, trend, 2, 9, -1) as u8,
            mean_time_to_restore_hours: ranged(seed + 4, trend, 1, 16, -1) as u16,
        },
        SimulationProfile::ShadowIT => DoraMetrics {
            deployment_frequency_per_month: ranged(seed, trend, 1, 8, 0) as u8,
            lead_time_for_changes_hours: ranged(seed + 2, trend, 80, 260, 3) as u16,
            change_failure_rate_pct: ranged(seed + 3, trend, 18, 42, 2) as u8,
            mean_time_to_restore_hours: ranged(seed + 4, trend, 48, 110, 2) as u16,
        },
        SimulationProfile::SaaS => DoraMetrics {
            deployment_frequency_per_month: ranged(seed, trend, 6, 18, 1) as u8,
            lead_time_for_changes_hours: ranged(seed + 2, trend, 24, 144, -1) as u16,
            change_failure_rate_pct: ranged(seed + 3, trend, 4, 18, 0) as u8,
            mean_time_to_restore_hours: ranged(seed + 4, trend, 6, 60, -1) as u16,
        },
        SimulationProfile::OffTheShelf => DoraMetrics {
            deployment_frequency_per_month: ranged(seed, trend, 3, 14, 1) as u8,
            lead_time_for_changes_hours: ranged(seed + 2, trend, 48, 190, -1) as u16,
            change_failure_rate_pct: ranged(seed + 3, trend, 8, 26, 0) as u8,
            mean_time_to_restore_hours: ranged(seed + 4, trend, 12, 80, -1) as u16,
        },
    }
}

fn build_application_source_context(
    application: &ApplicationFixture,
    seed: i32,
    periods: &[String],
) -> ApplicationSourceContext {
    let systems = application
        .source_systems
        .iter()
        .map(|source| SourceSystem {
            id: source.id.clone(),
            name: source.name.clone(),
            source_kind: source.source_kind.clone(),
            url: source.url.clone(),
            category: source.category.clone(),
            cadence: source.cadence.clone(),
            reliability: source.reliability.clone(),
            completeness: source.completeness.clone(),
        })
        .collect::<Vec<_>>();
    let profile = simulation_profile(application);
    let reporting_plants = reporting_plant_count(profile, application.targeted_plants, seed);
    let completeness_pct =
        source_completeness_pct(reporting_plants, application.targeted_plants, &systems);
    let freshness = source_freshness(profile, seed, periods, &systems);
    let monthly_evidence = periods
        .iter()
        .enumerate()
        .map(|(month_idx, period)| {
            source_evidence_snapshot(profile, seed, month_idx as i32, period, &systems)
        })
        .collect();

    ApplicationSourceContext {
        application_id: application.id.clone(),
        systems,
        coverage: SourceCoverage {
            targeted_plants: application.targeted_plants,
            reporting_plants,
            completeness_pct,
        },
        freshness,
        monthly_evidence,
    }
}

fn reporting_plant_count(profile: SimulationProfile, targeted_plants: u8, seed: i32) -> u8 {
    let base: u8 = match profile {
        SimulationProfile::Platform => targeted_plants,
        SimulationProfile::Modern | SimulationProfile::OffTheShelf | SimulationProfile::SaaS => 2,
        SimulationProfile::Legacy | SimulationProfile::ShadowIT => 1,
    };
    let variance = ((seed * 7 + 3).rem_euclid(2)) as u8;
    base.saturating_add(variance).min(targeted_plants).max(1)
}

fn source_completeness_pct(
    reporting_plants: u8,
    targeted_plants: u8,
    systems: &[SourceSystem],
) -> u8 {
    let plant_score = (reporting_plants as i32 * 100) / (targeted_plants as i32).max(1);
    let source_score = systems
        .iter()
        .map(|system| match system.completeness {
            SourceCompleteness::Complete => 100,
            SourceCompleteness::Partial => 65,
            SourceCompleteness::Sparse => 30,
        })
        .sum::<i32>()
        / (systems.len() as i32).max(1);
    ((plant_score + source_score) / 2).clamp(15, 100) as u8
}

fn source_freshness(
    profile: SimulationProfile,
    seed: i32,
    periods: &[String],
    systems: &[SourceSystem],
) -> SourceFreshness {
    let worst_cadence_hours = systems
        .iter()
        .map(|system| match system.cadence {
            SourceCadence::NearRealTime => 2,
            SourceCadence::Daily => 24,
            SourceCadence::Weekly => 168,
            SourceCadence::Monthly => 720,
        })
        .max()
        .unwrap_or(24);
    let staleness_offset = match profile {
        SimulationProfile::Legacy | SimulationProfile::ShadowIT => {
            1 + ((seed + 1).rem_euclid(2)) as usize
        }
        SimulationProfile::SaaS | SimulationProfile::OffTheShelf => (seed.rem_euclid(2)) as usize,
        SimulationProfile::Modern | SimulationProfile::Platform => 0,
    };
    let latest_index = periods.len().saturating_sub(1);
    let period_index = latest_index.saturating_sub(staleness_offset);
    let profile_latency = match profile {
        SimulationProfile::Platform | SimulationProfile::Modern => 0,
        SimulationProfile::SaaS | SimulationProfile::OffTheShelf => 12,
        SimulationProfile::Legacy => 96,
        SimulationProfile::ShadowIT => 144,
    };
    let latency_hours =
        worst_cadence_hours + profile_latency + ((seed * 5 + 9).rem_euclid(18)) as u16;

    SourceFreshness {
        last_successful_period: periods
            .get(period_index)
            .cloned()
            .unwrap_or_else(|| "unknown".to_owned()),
        latency_hours,
        stale: latency_hours > 72 || staleness_offset > 1,
    }
}

fn source_evidence_snapshot(
    profile: SimulationProfile,
    seed: i32,
    month_idx: i32,
    period: &str,
    systems: &[SourceSystem],
) -> SourceEvidenceSnapshot {
    let has_technical = systems
        .iter()
        .any(|system| system.category == SourceCategory::TechnicalDelivery);
    let has_adoption = systems
        .iter()
        .any(|system| system.category == SourceCategory::AdoptionDiffusion);
    let has_erp_mes = systems
        .iter()
        .any(|system| system.source_kind.eq_ignore_ascii_case("ERP/MES events"));
    let operational_feeds = systems
        .iter()
        .filter(|system| system.category == SourceCategory::OperationalOutcome)
        .count()
        .max(1)
        .min(3) as u8;
    let (deployment_base, incident_base, login_base, workflow_base, erp_base) = match profile {
        SimulationProfile::Legacy => (2, 9, 900, 250, 900),
        SimulationProfile::Modern => (24, 4, 3600, 1400, 2400),
        SimulationProfile::Platform => (32, 3, 5200, 2100, 3600),
        SimulationProfile::ShadowIT => (1, 7, 500, 160, 260),
        SimulationProfile::SaaS => (10, 3, 4300, 900, 900),
        SimulationProfile::OffTheShelf => (7, 5, 3300, 1100, 1800),
    };

    SourceEvidenceSnapshot {
        period: period.to_owned(),
        deployment_events: if has_technical {
            (deployment_base + ranged(seed, month_idx, 0, 9, 1)) as u16
        } else {
            0
        },
        incident_events: if has_technical {
            (incident_base + ranged(seed + 2, month_idx, 0, 4, 0)) as u16
        } else {
            0
        },
        login_events: if has_adoption {
            (login_base + ranged(seed + 3, month_idx, 0, 1200, 90)) as u32
        } else {
            0
        },
        workflow_events: if has_adoption {
            (workflow_base + ranged(seed + 4, month_idx, 0, 600, 45)) as u32
        } else {
            0
        },
        erp_mes_events: if has_erp_mes {
            (erp_base + ranged(seed + 5, month_idx, 0, 1400, 55)) as u32
        } else {
            0
        },
        kpi_feeds_expected: operational_feeds,
        kpi_feeds_available: operational_feeds
            .saturating_sub(((seed + month_idx * 3 + 1).rem_euclid(2)) as u8),
    }
}

fn metric_seed(metrics: &[MetricSourceFixture], application_id: &str) -> i32 {
    metrics
        .iter()
        .filter(|metric| metric.application_id == application_id)
        .count() as i32
}

fn shaped_score(base: i32, seed: i32, trend: i32, trend_weight: i32, volatility: i32) -> u8 {
    let noise = (seed * 17 + trend * 11 + 23).rem_euclid((volatility * 2 + 1).max(1)) - volatility;
    (base + trend * trend_weight + noise).clamp(0, 100) as u8
}

fn ranged(seed: i32, trend: i32, min: i32, max: i32, trend_weight: i32) -> i32 {
    let span = (max - min + 1).max(1);
    (min + (seed * 19 + trend * 7 + 31).rem_euclid(span) + trend * trend_weight).clamp(min, max)
}

fn weighted_avg(scores: &[(u8, u8)], denominator: u32) -> i32 {
    let total: u32 = scores
        .iter()
        .map(|(value, weight)| (*value as u32) * (*weight as u32))
        .sum();
    (total / denominator) as i32
}

fn risk_score(value: u16, min: u16, max: u16) -> u8 {
    let bounded = value.clamp(min, max);
    let span = max - min;
    if span == 0 {
        return 0;
    }
    (((bounded - min) as u32 * 100) / (span as u32)) as u8
}

fn reverse_risk_score(value: u16, min: u16, max: u16) -> u8 {
    100u8.saturating_sub(risk_score(value, min, max))
}

fn band_from_score(score: i32) -> HeatBand {
    match score {
        0..=34 => HeatBand::Low,
        35..=69 => HeatBand::Medium,
        _ => HeatBand::High,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        LATEST_PERIOD, PERIOD_COUNT, build_seed_dataset, latest_month_projection, load_fixtures,
    };
    use domain::{ApplicationType, SourceCategory};

    #[test]
    fn generated_dataset_has_expected_structure() {
        let dataset = build_seed_dataset();
        assert_eq!(dataset.domains.len(), 6);
        assert_eq!(dataset.capabilities.len(), 17);
        assert_eq!(dataset.applications.len(), 15);
        assert_eq!(dataset.source_contexts.len(), dataset.applications.len());
        assert_eq!(
            dataset.monthly_indicators.len(),
            dataset.applications.len() * PERIOD_COUNT
        );
    }

    #[test]
    fn latest_projection_has_one_entry_per_application() {
        let dataset = build_seed_dataset();
        let latest = latest_month_projection(&dataset);
        assert_eq!(latest.len(), dataset.applications.len());
        assert!(
            latest
                .iter()
                .all(|item| item.snapshot.period == LATEST_PERIOD)
        );
    }

    #[test]
    fn indicator_details_are_in_expected_ranges() {
        let dataset = build_seed_dataset();
        assert!(!dataset.monthly_indicators.is_empty());
        for item in &dataset.monthly_indicators {
            let bv = &item.snapshot.business_value_details;
            let dora = &item.snapshot.technical_health_details;

            assert!(bv.operational_adoption_score <= 100);
            assert!(bv.cross_plant_coverage_score <= 100);
            assert!(bv.operational_criticality_score <= 100);
            assert!(bv.process_performance_impact_score <= 100);

            assert!((1..=40).contains(&dora.deployment_frequency_per_month));
            assert!((2..=336).contains(&dora.lead_time_for_changes_hours));
            assert!((2..=45).contains(&dora.change_failure_rate_pct));
            assert!((1..=120).contains(&dora.mean_time_to_restore_hours));
        }
    }

    #[test]
    fn source_contexts_are_deterministic_and_categorized() {
        let first = build_seed_dataset();
        let second = build_seed_dataset();
        assert_eq!(first.source_contexts, second.source_contexts);

        assert!(first.source_contexts.iter().all(|context| {
            !context.systems.is_empty()
                && context.monthly_evidence.len() == PERIOD_COUNT
                && context
                    .systems
                    .iter()
                    .any(|source| source.category == SourceCategory::AdoptionDiffusion)
        }));
    }

    #[test]
    fn source_systems_expose_relative_source_urls() {
        let dataset = build_seed_dataset();
        for context in &dataset.source_contexts {
            for source in &context.systems {
                assert!(
                    source.url.starts_with("/sources/"),
                    "{}:{} has invalid URL {}",
                    context.application_id,
                    source.id,
                    source.url
                );
            }
        }
    }

    #[test]
    fn source_coverage_and_evidence_are_in_expected_ranges() {
        let dataset = build_seed_dataset();
        for context in &dataset.source_contexts {
            assert!((1..=3).contains(&context.coverage.reporting_plants));
            assert_eq!(context.coverage.targeted_plants, 3);
            assert!((15..=100).contains(&context.coverage.completeness_pct));

            for snapshot in &context.monthly_evidence {
                assert!(snapshot.kpi_feeds_available <= snapshot.kpi_feeds_expected);
                assert!((1..=3).contains(&snapshot.kpi_feeds_expected));
            }
        }
    }

    #[test]
    fn fixtures_validate_cross_references() {
        let fixtures = load_fixtures().expect("fixtures should validate");
        assert_eq!(
            fixtures.enterprise.applications.len(),
            fixtures.applications.len()
        );
        assert_eq!(fixtures.metrics.len(), fixtures.applications.len());
    }

    #[test]
    fn each_application_has_indicator_and_evidence_snapshots_for_every_period() {
        let dataset = build_seed_dataset();
        for application in &dataset.applications {
            assert_eq!(
                dataset
                    .monthly_indicators
                    .iter()
                    .filter(|item| item.application_id == application.id)
                    .count(),
                PERIOD_COUNT
            );
            assert_eq!(
                dataset
                    .source_contexts
                    .iter()
                    .find(|context| context.application_id == application.id)
                    .expect("source context exists")
                    .monthly_evidence
                    .len(),
                PERIOD_COUNT
            );
        }
    }

    #[test]
    fn all_reference_applications_are_present() {
        let dataset = build_seed_dataset();
        let application_ids = dataset
            .applications
            .iter()
            .map(|application| application.id.as_str())
            .collect::<std::collections::BTreeSet<_>>();

        let expected = [
            "mes-alpha",
            "plant-scheduler",
            "operator-portal",
            "energy-monitor",
            "quality-tracker",
            "compliance-hub",
            "maintenance-hub",
            "predictive-insight",
            "sap-s4-erp",
            "inventory-pro",
            "plm-core",
            "cad-vault",
            "legacy-reporting-cube",
            "industrial-data-platform-app",
            "excel-macros-local-ops",
        ];

        for application_id in expected {
            assert!(
                application_ids.contains(application_id),
                "missing reference application {application_id}"
            );
        }
    }

    #[test]
    fn key_application_capability_mappings_match_reference() {
        let dataset = build_seed_dataset();
        let capabilities_by_app = dataset
            .applications
            .iter()
            .map(|application| {
                (
                    application.id.as_str(),
                    application
                        .capability_ids
                        .iter()
                        .map(String::as_str)
                        .collect::<Vec<_>>(),
                )
            })
            .collect::<std::collections::BTreeMap<_, _>>();

        assert_eq!(
            capabilities_by_app["mes-alpha"],
            ["shopfloor-execution", "traceability"]
        );
        assert_eq!(
            capabilities_by_app["energy-monitor"],
            ["energy-optimization"]
        );
        assert_eq!(
            capabilities_by_app["sap-s4-erp"],
            ["inventory-management", "shipment-planning"]
        );
        assert_eq!(
            capabilities_by_app["industrial-data-platform-app"],
            [
                "industrial-data-platform",
                "kpi-management",
                "operational-reporting"
            ]
        );
        assert_eq!(
            capabilities_by_app["legacy-reporting-cube"],
            ["operational-reporting", "kpi-management"]
        );
        assert_eq!(
            capabilities_by_app["compliance-hub"],
            ["product-compliance", "traceability"]
        );
        assert_eq!(
            capabilities_by_app["predictive-insight"],
            ["predictive-maintenance", "asset-reliability"]
        );
        assert_eq!(
            capabilities_by_app["cad-vault"],
            ["product-specification-management"]
        );
    }

    #[test]
    fn every_metric_fixture_references_declared_application_source_url() {
        let fixtures = load_fixtures().expect("fixtures should validate");
        let source_urls_by_application = fixtures
            .applications
            .iter()
            .map(|application| {
                (
                    application.id.as_str(),
                    application
                        .source_systems
                        .iter()
                        .map(|source| source.url.as_str())
                        .collect::<std::collections::BTreeSet<_>>(),
                )
            })
            .collect::<std::collections::BTreeMap<_, _>>();

        for metric in &fixtures.metrics {
            let urls = source_urls_by_application
                .get(metric.application_id.as_str())
                .expect("metric application is declared");
            assert!(
                urls.contains(metric.source_url.as_str()),
                "{} references undeclared source URL {}",
                metric.id,
                metric.source_url
            );
        }
    }

    #[test]
    fn simulation_profiles_shape_dora_health() {
        let dataset = build_seed_dataset();
        let latest = latest_month_projection(&dataset);

        let by_id = latest
            .iter()
            .map(|projection| {
                (
                    projection.application_id.as_str(),
                    &projection.snapshot.technical_health_details,
                )
            })
            .collect::<std::collections::BTreeMap<_, _>>();
        let legacy = by_id.get("legacy-reporting-cube").expect("legacy app");
        let modern = by_id.get("operator-portal").expect("modern app");
        let platform = by_id
            .get("industrial-data-platform-app")
            .expect("platform app");

        assert!(legacy.lead_time_for_changes_hours > modern.lead_time_for_changes_hours);
        assert!(legacy.change_failure_rate_pct > platform.change_failure_rate_pct);
        assert!(platform.deployment_frequency_per_month > modern.deployment_frequency_per_month);
    }

    #[test]
    fn public_application_catalog_stays_stable() {
        let dataset = build_seed_dataset();
        assert_eq!(dataset.applications[0].id, "sap-s4-erp");
        assert_eq!(
            dataset.applications[0].application_type,
            ApplicationType::OffTheShelf
        );
        assert_eq!(dataset.applications[14].id, "excel-macros-local-ops");
        assert_eq!(
            dataset.applications[14].application_type,
            ApplicationType::ShadowIT
        );
    }
}
