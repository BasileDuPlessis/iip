use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApplicationType {
    Custom,
    OffTheShelf,
    SaaS,
    Legacy,
    Platform,
    ShadowIT,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Domain {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Capability {
    pub id: String,
    pub domain_id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Application {
    pub id: String,
    pub name: String,
    pub application_type: ApplicationType,
    pub capability_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum HeatBand {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BusinessValueDetails {
    pub operational_adoption_score: u8,
    pub cross_plant_coverage_score: u8,
    pub operational_criticality_score: u8,
    pub process_performance_impact_score: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DoraMetrics {
    pub deployment_frequency_per_month: u8,
    pub lead_time_for_changes_hours: u16,
    pub change_failure_rate_pct: u8,
    pub mean_time_to_restore_hours: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IndicatorSnapshot {
    pub period: String,
    pub business_value: HeatBand,
    pub technical_health: HeatBand,
    pub business_value_details: BusinessValueDetails,
    pub technical_health_details: DoraMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SourceCategory {
    TechnicalDelivery,
    AdoptionDiffusion,
    OperationalOutcome,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SourceCadence {
    NearRealTime,
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SourceReliability {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SourceCompleteness {
    Complete,
    Partial,
    Sparse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceSystem {
    pub id: String,
    pub name: String,
    pub source_kind: String,
    pub category: SourceCategory,
    pub cadence: SourceCadence,
    pub reliability: SourceReliability,
    pub completeness: SourceCompleteness,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceCoverage {
    pub targeted_plants: u8,
    pub reporting_plants: u8,
    pub completeness_pct: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceFreshness {
    pub last_successful_period: String,
    pub latency_hours: u16,
    pub stale: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SourceEvidenceSnapshot {
    pub period: String,
    pub deployment_events: u16,
    pub incident_events: u16,
    pub login_events: u32,
    pub workflow_events: u32,
    pub erp_mes_events: u32,
    pub kpi_feeds_expected: u8,
    pub kpi_feeds_available: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ApplicationSourceContext {
    pub application_id: String,
    pub systems: Vec<SourceSystem>,
    pub coverage: SourceCoverage,
    pub freshness: SourceFreshness,
    pub monthly_evidence: Vec<SourceEvidenceSnapshot>,
}

impl HeatBand {
    pub const fn color_token(&self) -> &'static str {
        match self {
            HeatBand::Low => "blue",
            HeatBand::Medium => "amber",
            HeatBand::High => "red",
        }
    }
}
