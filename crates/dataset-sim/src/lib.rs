use std::collections::BTreeMap;

use domain::{
    Application, ApplicationSourceContext, ApplicationType, BusinessValueDetails, Capability, Domain,
    DoraMetrics, HeatBand, IndicatorSnapshot, SourceCadence, SourceCategory, SourceCompleteness,
    SourceCoverage, SourceEvidenceSnapshot, SourceFreshness, SourceReliability, SourceSystem,
};
use serde::Serialize;

const PERIOD_COUNT: usize = 24;
const TARGETED_PLANTS: u8 = 3;

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

pub fn build_seed_dataset() -> SeedDataset {
    let domains = vec![
        Domain {
            id: "manufacturing-operations".to_owned(),
            name: "Manufacturing Operations".to_owned(),
        },
        Domain {
            id: "quality-compliance".to_owned(),
            name: "Quality & Compliance".to_owned(),
        },
        Domain {
            id: "maintenance-reliability".to_owned(),
            name: "Maintenance & Reliability".to_owned(),
        },
        Domain {
            id: "supply-chain-logistics".to_owned(),
            name: "Supply Chain & Logistics".to_owned(),
        },
        Domain {
            id: "engineering-product-lifecycle".to_owned(),
            name: "Engineering & Product Lifecycle".to_owned(),
        },
        Domain {
            id: "data-reporting".to_owned(),
            name: "Data & Reporting".to_owned(),
        },
    ];

    let capabilities = vec![
        Capability {
            id: "production-scheduling".to_owned(),
            domain_id: "manufacturing-operations".to_owned(),
            name: "Production Scheduling".to_owned(),
        },
        Capability {
            id: "shopfloor-execution".to_owned(),
            domain_id: "manufacturing-operations".to_owned(),
            name: "Shopfloor Execution".to_owned(),
        },
        Capability {
            id: "quality-incident-management".to_owned(),
            domain_id: "quality-compliance".to_owned(),
            name: "Quality Incident Management".to_owned(),
        },
        Capability {
            id: "traceability".to_owned(),
            domain_id: "quality-compliance".to_owned(),
            name: "Traceability".to_owned(),
        },
        Capability {
            id: "maintenance-work-orders".to_owned(),
            domain_id: "maintenance-reliability".to_owned(),
            name: "Maintenance Work Orders".to_owned(),
        },
        Capability {
            id: "predictive-maintenance".to_owned(),
            domain_id: "maintenance-reliability".to_owned(),
            name: "Predictive Maintenance".to_owned(),
        },
        Capability {
            id: "inventory-management".to_owned(),
            domain_id: "supply-chain-logistics".to_owned(),
            name: "Inventory Management".to_owned(),
        },
        Capability {
            id: "engineering-change-management".to_owned(),
            domain_id: "engineering-product-lifecycle".to_owned(),
            name: "Engineering Change Management".to_owned(),
        },
        Capability {
            id: "operational-reporting".to_owned(),
            domain_id: "data-reporting".to_owned(),
            name: "Operational Reporting".to_owned(),
        },
        Capability {
            id: "industrial-data-platform".to_owned(),
            domain_id: "data-reporting".to_owned(),
            name: "Industrial Data Platform".to_owned(),
        },
    ];

    let applications = vec![
        Application {
            id: "sap-s4-erp".to_owned(),
            name: "SAP S/4 ERP".to_owned(),
            application_type: ApplicationType::OffTheShelf,
            capability_ids: vec!["inventory-management".to_owned()],
        },
        Application {
            id: "mes-alpha".to_owned(),
            name: "MES Alpha".to_owned(),
            application_type: ApplicationType::OffTheShelf,
            capability_ids: vec![
                "shopfloor-execution".to_owned(),
                "production-scheduling".to_owned(),
            ],
        },
        Application {
            id: "plm-core".to_owned(),
            name: "PLM Core".to_owned(),
            application_type: ApplicationType::OffTheShelf,
            capability_ids: vec!["engineering-change-management".to_owned()],
        },
        Application {
            id: "maintenance-hub".to_owned(),
            name: "Maintenance Hub".to_owned(),
            application_type: ApplicationType::Custom,
            capability_ids: vec!["maintenance-work-orders".to_owned()],
        },
        Application {
            id: "plant-scheduler".to_owned(),
            name: "Plant Scheduler".to_owned(),
            application_type: ApplicationType::Custom,
            capability_ids: vec!["production-scheduling".to_owned()],
        },
        Application {
            id: "operator-portal".to_owned(),
            name: "Operator Portal".to_owned(),
            application_type: ApplicationType::Custom,
            capability_ids: vec!["shopfloor-execution".to_owned()],
        },
        Application {
            id: "quality-tracker".to_owned(),
            name: "Quality Tracker".to_owned(),
            application_type: ApplicationType::Custom,
            capability_ids: vec![
                "quality-incident-management".to_owned(),
                "traceability".to_owned(),
            ],
        },
        Application {
            id: "inventory-pro".to_owned(),
            name: "Inventory Pro".to_owned(),
            application_type: ApplicationType::SaaS,
            capability_ids: vec!["inventory-management".to_owned()],
        },
        Application {
            id: "energy-monitor".to_owned(),
            name: "Energy Monitor".to_owned(),
            application_type: ApplicationType::SaaS,
            capability_ids: vec!["operational-reporting".to_owned()],
        },
        Application {
            id: "industrial-data-platform-app".to_owned(),
            name: "Industrial Data Platform".to_owned(),
            application_type: ApplicationType::Platform,
            capability_ids: vec!["industrial-data-platform".to_owned()],
        },
        Application {
            id: "legacy-reporting-cube".to_owned(),
            name: "Legacy Reporting Cube".to_owned(),
            application_type: ApplicationType::Legacy,
            capability_ids: vec!["operational-reporting".to_owned()],
        },
        Application {
            id: "excel-macros-local-ops".to_owned(),
            name: "Excel Macros Local Ops".to_owned(),
            application_type: ApplicationType::ShadowIT,
            capability_ids: vec![
                "shopfloor-execution".to_owned(),
                "operational-reporting".to_owned(),
            ],
        },
    ];

    let periods = monthly_periods("2024-06", PERIOD_COUNT);
    let mut monthly_indicators = Vec::new();
    let mut source_contexts = Vec::new();

    for (index, application) in applications.iter().enumerate() {
        monthly_indicators.extend(build_application_indicators(
            &application.id,
            index as i32,
            &periods,
        ));
        source_contexts.push(build_application_source_context(
            application,
            index as i32,
            &periods,
        ));
    }

    SeedDataset {
        domains,
        capabilities,
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

fn monthly_periods(start_period: &str, count: usize) -> Vec<String> {
    let (year, month) = start_period
        .split_once('-')
        .and_then(|(y, m)| Some((y.parse::<i32>().ok()?, m.parse::<i32>().ok()?)))
        .expect("start_period must be in YYYY-MM format");

    let mut periods = Vec::with_capacity(count);
    let mut y = year;
    let mut m = month;

    for _ in 0..count {
        periods.push(format!("{y:04}-{m:02}"));
        m += 1;
        if m > 12 {
            m = 1;
            y += 1;
        }
    }

    periods
}

fn build_application_indicators(
    application_id: &str,
    seed: i32,
    periods: &[String],
) -> Vec<ApplicationMonthlyIndicator> {
    periods
        .iter()
        .enumerate()
        .map(|(month_idx, period)| {
            let trend = month_idx as i32;

            let business_value_details = BusinessValueDetails {
                operational_adoption_score: score(seed, trend, 13, 5, 37),
                cross_plant_coverage_score: score(seed, trend, 11, 3, 53),
                operational_criticality_score: score(seed, trend, 7, 2, 61),
                process_performance_impact_score: score(seed, trend, 17, 4, 29),
            };

            let business_value_score = weighted_avg(
                &[
                    (business_value_details.operational_adoption_score, 30),
                    (business_value_details.cross_plant_coverage_score, 20),
                    (business_value_details.operational_criticality_score, 30),
                    (
                        business_value_details.process_performance_impact_score,
                        20,
                    ),
                ],
                100,
            );

            let technical_health_details = DoraMetrics {
                deployment_frequency_per_month: 1 + (score(seed, trend, 5, 3, 7) % 40),
                lead_time_for_changes_hours: 2 + ((seed * 19 + trend * 11 + 31).rem_euclid(335)) as u16,
                change_failure_rate_pct: 2 + (score(seed, trend, 9, 7, 19) % 44),
                mean_time_to_restore_hours: 1
                    + ((seed * 23 + trend * 13 + 17).rem_euclid(120)) as u16,
            };

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
                        risk_score(
                            technical_health_details.lead_time_for_changes_hours,
                            2,
                            336,
                        ),
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
                        risk_score(
                            technical_health_details.mean_time_to_restore_hours,
                            1,
                            120,
                        ),
                        25,
                    ),
                ],
                100,
            );

            ApplicationMonthlyIndicator {
                application_id: application_id.to_owned(),
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

fn build_application_source_context(
    application: &Application,
    seed: i32,
    periods: &[String],
) -> ApplicationSourceContext {
    let systems = source_systems_for_type(&application.application_type);
    let reporting_plants = reporting_plant_count(&application.application_type, seed);
    let completeness_pct = source_completeness_pct(reporting_plants, seed);
    let freshness = source_freshness(seed, periods, &systems);
    let monthly_evidence = periods
        .iter()
        .enumerate()
        .map(|(month_idx, period)| source_evidence_snapshot(seed, month_idx as i32, period, &systems))
        .collect();

    ApplicationSourceContext {
        application_id: application.id.clone(),
        systems,
        coverage: SourceCoverage {
            targeted_plants: TARGETED_PLANTS,
            reporting_plants,
            completeness_pct,
        },
        freshness,
        monthly_evidence,
    }
}

fn source_systems_for_type(application_type: &ApplicationType) -> Vec<SourceSystem> {
    match application_type {
        ApplicationType::Custom => vec![
            source_system(
                "github",
                "GitHub",
                "Source Control",
                SourceCategory::TechnicalDelivery,
                SourceCadence::Daily,
                SourceReliability::High,
                SourceCompleteness::Complete,
            ),
            source_system(
                "github-actions",
                "GitHub Actions",
                "CI/CD",
                SourceCategory::TechnicalDelivery,
                SourceCadence::Daily,
                SourceReliability::High,
                SourceCompleteness::Complete,
            ),
            source_system(
                "argocd",
                "ArgoCD",
                "Deployment",
                SourceCategory::TechnicalDelivery,
                SourceCadence::Daily,
                SourceReliability::Medium,
                SourceCompleteness::Partial,
            ),
            source_system(
                "datadog",
                "Datadog",
                "Observability",
                SourceCategory::TechnicalDelivery,
                SourceCadence::NearRealTime,
                SourceReliability::High,
                SourceCompleteness::Partial,
            ),
            source_system(
                "pagerduty",
                "PagerDuty",
                "Incident Management",
                SourceCategory::TechnicalDelivery,
                SourceCadence::NearRealTime,
                SourceReliability::Medium,
                SourceCompleteness::Partial,
            ),
            source_system(
                "application-logs",
                "Application Logs",
                "Application Logs",
                SourceCategory::AdoptionDiffusion,
                SourceCadence::NearRealTime,
                SourceReliability::Medium,
                SourceCompleteness::Partial,
            ),
            source_system(
                "okta",
                "Okta",
                "IAM/SSO",
                SourceCategory::AdoptionDiffusion,
                SourceCadence::Daily,
                SourceReliability::High,
                SourceCompleteness::Complete,
            ),
            source_system(
                "camunda",
                "Camunda",
                "Workflow",
                SourceCategory::AdoptionDiffusion,
                SourceCadence::Daily,
                SourceReliability::Medium,
                SourceCompleteness::Partial,
            ),
            source_system(
                "erp-mes-events",
                "ERP/MES Events",
                "ERP/MES events",
                SourceCategory::AdoptionDiffusion,
                SourceCadence::Daily,
                SourceReliability::Medium,
                SourceCompleteness::Partial,
            ),
            source_system(
                "mes-kpis",
                "MES KPI Feed",
                "Operational KPI",
                SourceCategory::OperationalOutcome,
                SourceCadence::Daily,
                SourceReliability::Medium,
                SourceCompleteness::Partial,
            ),
        ],
        ApplicationType::OffTheShelf => vec![
            source_system(
                "service-now",
                "ServiceNow",
                "Incident Management",
                SourceCategory::TechnicalDelivery,
                SourceCadence::Daily,
                SourceReliability::Medium,
                SourceCompleteness::Partial,
            ),
            source_system(
                "azure-ad",
                "Azure AD",
                "IAM/SSO",
                SourceCategory::AdoptionDiffusion,
                SourceCadence::Daily,
                SourceReliability::High,
                SourceCompleteness::Complete,
            ),
            source_system(
                "workflow-engine",
                "Workflow Engine",
                "Workflow",
                SourceCategory::AdoptionDiffusion,
                SourceCadence::Daily,
                SourceReliability::Medium,
                SourceCompleteness::Partial,
            ),
            source_system(
                "erp-mes-events",
                "ERP/MES Events",
                "ERP/MES events",
                SourceCategory::AdoptionDiffusion,
                SourceCadence::Daily,
                SourceReliability::Medium,
                SourceCompleteness::Partial,
            ),
            source_system(
                "erp-kpis",
                "ERP KPI Feed",
                "Operational KPI",
                SourceCategory::OperationalOutcome,
                SourceCadence::Weekly,
                SourceReliability::Medium,
                SourceCompleteness::Partial,
            ),
        ],
        ApplicationType::SaaS => vec![
            source_system(
                "vendor-status",
                "Vendor Status Feed",
                "Observability",
                SourceCategory::TechnicalDelivery,
                SourceCadence::Daily,
                SourceReliability::Medium,
                SourceCompleteness::Partial,
            ),
            source_system(
                "iam-sso",
                "IAM/SSO",
                "IAM/SSO",
                SourceCategory::AdoptionDiffusion,
                SourceCadence::Daily,
                SourceReliability::High,
                SourceCompleteness::Complete,
            ),
            source_system(
                "application-logs",
                "Application Logs",
                "Application Logs",
                SourceCategory::AdoptionDiffusion,
                SourceCadence::Daily,
                SourceReliability::Medium,
                SourceCompleteness::Partial,
            ),
            source_system(
                "reporting-platform",
                "Reporting Platform",
                "Reporting platform",
                SourceCategory::OperationalOutcome,
                SourceCadence::Weekly,
                SourceReliability::Medium,
                SourceCompleteness::Partial,
            ),
        ],
        ApplicationType::Legacy => vec![
            source_system(
                "legacy-incidents",
                "Legacy Incident Register",
                "Incident Management",
                SourceCategory::TechnicalDelivery,
                SourceCadence::Weekly,
                SourceReliability::Low,
                SourceCompleteness::Sparse,
            ),
            source_system(
                "legacy-logs",
                "Legacy Application Logs",
                "Application Logs",
                SourceCategory::AdoptionDiffusion,
                SourceCadence::Weekly,
                SourceReliability::Low,
                SourceCompleteness::Sparse,
            ),
            source_system(
                "erp-mes-events",
                "ERP/MES Events",
                "ERP/MES events",
                SourceCategory::AdoptionDiffusion,
                SourceCadence::Weekly,
                SourceReliability::Low,
                SourceCompleteness::Sparse,
            ),
            source_system(
                "excel-kpis",
                "Spreadsheet KPI Feed",
                "Operational KPI",
                SourceCategory::OperationalOutcome,
                SourceCadence::Monthly,
                SourceReliability::Low,
                SourceCompleteness::Sparse,
            ),
        ],
        ApplicationType::Platform => vec![
            source_system(
                "gitlab",
                "GitLab",
                "Source Control",
                SourceCategory::TechnicalDelivery,
                SourceCadence::Daily,
                SourceReliability::High,
                SourceCompleteness::Complete,
            ),
            source_system(
                "jenkins",
                "Jenkins",
                "CI/CD",
                SourceCategory::TechnicalDelivery,
                SourceCadence::Daily,
                SourceReliability::High,
                SourceCompleteness::Complete,
            ),
            source_system(
                "kubernetes",
                "Kubernetes",
                "Deployment",
                SourceCategory::TechnicalDelivery,
                SourceCadence::NearRealTime,
                SourceReliability::High,
                SourceCompleteness::Complete,
            ),
            source_system(
                "grafana",
                "Grafana",
                "Observability",
                SourceCategory::TechnicalDelivery,
                SourceCadence::NearRealTime,
                SourceReliability::High,
                SourceCompleteness::Partial,
            ),
            source_system(
                "keycloak",
                "Keycloak",
                "IAM/SSO",
                SourceCategory::AdoptionDiffusion,
                SourceCadence::Daily,
                SourceReliability::High,
                SourceCompleteness::Complete,
            ),
            source_system(
                "workflow-platform",
                "Workflow Platform",
                "Workflow",
                SourceCategory::AdoptionDiffusion,
                SourceCadence::Daily,
                SourceReliability::Medium,
                SourceCompleteness::Partial,
            ),
            source_system(
                "iot-telemetry",
                "Industrial IoT Telemetry",
                "Industrial IoT",
                SourceCategory::OperationalOutcome,
                SourceCadence::NearRealTime,
                SourceReliability::Medium,
                SourceCompleteness::Partial,
            ),
            source_system(
                "process-mining",
                "Process Mining",
                "Process mining",
                SourceCategory::OperationalOutcome,
                SourceCadence::Weekly,
                SourceReliability::Medium,
                SourceCompleteness::Partial,
            ),
        ],
        ApplicationType::ShadowIT => vec![
            source_system(
                "manual-incident-log",
                "Manual Incident Log",
                "Incident Management",
                SourceCategory::TechnicalDelivery,
                SourceCadence::Monthly,
                SourceReliability::Low,
                SourceCompleteness::Sparse,
            ),
            source_system(
                "local-logs",
                "Local Usage Logs",
                "Application Logs",
                SourceCategory::AdoptionDiffusion,
                SourceCadence::Weekly,
                SourceReliability::Low,
                SourceCompleteness::Sparse,
            ),
            source_system(
                "shared-spreadsheets",
                "Shared Spreadsheets",
                "Reporting platform",
                SourceCategory::OperationalOutcome,
                SourceCadence::Monthly,
                SourceReliability::Low,
                SourceCompleteness::Sparse,
            ),
        ],
    }
}

fn source_system(
    id: &str,
    name: &str,
    source_kind: &str,
    category: SourceCategory,
    cadence: SourceCadence,
    reliability: SourceReliability,
    completeness: SourceCompleteness,
) -> SourceSystem {
    SourceSystem {
        id: id.to_owned(),
        name: name.to_owned(),
        source_kind: source_kind.to_owned(),
        category,
        cadence,
        reliability,
        completeness,
    }
}

fn reporting_plant_count(application_type: &ApplicationType, seed: i32) -> u8 {
    let base: u8 = match application_type {
        ApplicationType::Platform => 3,
        ApplicationType::Custom => 2,
        ApplicationType::OffTheShelf => 2,
        ApplicationType::SaaS => 2,
        ApplicationType::Legacy => 1,
        ApplicationType::ShadowIT => 1,
    };
    let variance = ((seed * 7 + 3).rem_euclid(2)) as u8;
    base.saturating_add(variance).min(TARGETED_PLANTS)
}

fn source_completeness_pct(reporting_plants: u8, seed: i32) -> u8 {
    let base = (reporting_plants as i32 * 100) / (TARGETED_PLANTS as i32);
    let adjustment = (seed * 11 + 19).rem_euclid(23) - 11;
    (base + adjustment).clamp(15, 100) as u8
}

fn source_freshness(seed: i32, periods: &[String], systems: &[SourceSystem]) -> SourceFreshness {
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

    let staleness_offset = ((seed * 3 + 17).rem_euclid(3)) as usize;
    let latest_index = periods.len().saturating_sub(1);
    let period_index = latest_index.saturating_sub(staleness_offset);
    let latency_hours = worst_cadence_hours + ((seed * 5 + 9).rem_euclid(18)) as u16;

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

    let deployment_events = if has_technical {
        ((seed * 7 + month_idx * 5 + 19).rem_euclid(55) + 1) as u16
    } else {
        0
    };
    let incident_events = if has_technical {
        ((seed * 5 + month_idx * 3 + 11).rem_euclid(14) + 1) as u16
    } else {
        0
    };
    let login_events = if has_adoption {
        ((seed * 89 + month_idx * 211 + 1200).rem_euclid(9000) + 400) as u32
    } else {
        0
    };
    let workflow_events = if has_adoption {
        ((seed * 31 + month_idx * 47 + 300).rem_euclid(2600) + 80) as u32
    } else {
        0
    };
    let erp_mes_events = if has_erp_mes {
        ((seed * 43 + month_idx * 59 + 500).rem_euclid(5000) + 150) as u32
    } else {
        0
    };
    let kpi_feeds_available =
        operational_feeds.saturating_sub(((seed + month_idx * 3 + 1).rem_euclid(2)) as u8);

    SourceEvidenceSnapshot {
        period: period.to_owned(),
        deployment_events,
        incident_events,
        login_events,
        workflow_events,
        erp_mes_events,
        kpi_feeds_expected: operational_feeds,
        kpi_feeds_available,
    }
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

fn score(seed: i32, trend: i32, seed_factor: i32, trend_factor: i32, offset: i32) -> u8 {
    ((seed * seed_factor + trend * trend_factor + offset).rem_euclid(101)) as u8
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
    use super::{PERIOD_COUNT, build_seed_dataset, latest_month_projection};
    use domain::SourceCategory;

    #[test]
    fn generated_dataset_has_expected_structure() {
        let dataset = build_seed_dataset();
        assert_eq!(dataset.domains.len(), 6);
        assert_eq!(dataset.capabilities.len(), 10);
        assert_eq!(dataset.applications.len(), 12);
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
        assert!(latest.iter().all(|item| item.snapshot.period == "2026-05"));
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
}
