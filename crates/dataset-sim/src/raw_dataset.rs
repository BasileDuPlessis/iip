use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use domain::{ApplicationType, Capability, Domain};
use serde::{Deserialize, Serialize};

const RAW_ENTERPRISE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/enterprise.json");
const RAW_APPLICATIONS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/raw/applications");
const RAW_METRICS_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/raw/metrics");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MetricTheme {
    Adoption,
    NorthStar,
    Technical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RawApplicationMetricReference {
    pub id: String,
    pub theme: MetricTheme,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RawEnterpriseFixture {
    pub name: String,
    pub domains: Vec<Domain>,
    pub capabilities: Vec<Capability>,
    pub applications: Vec<RawEnterpriseApplicationSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RawEnterpriseApplicationSummary {
    pub id: String,
    pub name: String,
    pub application_type: ApplicationType,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RawApplicationFixture {
    pub id: String,
    pub name: String,
    pub application_type: ApplicationType,
    pub story: String,
    pub capability_ids: Vec<String>,
    pub targeted_plants: u8,
    pub metrics: Vec<RawApplicationMetricReference>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RawMetricFixture {
    pub id: String,
    pub application_id: String,
    pub theme: MetricTheme,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RawDataset {
    pub enterprise: RawEnterpriseFixture,
    pub applications: Vec<RawApplicationFixture>,
    pub metrics: Vec<RawMetricFixture>,
}

pub fn load_raw_dataset() -> Result<RawDataset, String> {
    let enterprise = serde_json::from_str::<RawEnterpriseFixture>(
        &fs::read_to_string(RAW_ENTERPRISE_PATH)
            .map_err(|err| format!("invalid enterprise fixture path: {err}"))?,
    )
    .map_err(|err| format!("invalid enterprise fixture: {err}"))?;

    let applications = load_json_files::<RawApplicationFixture>(RAW_APPLICATIONS_DIR)?;
    let metrics = load_json_files::<RawMetricFixture>(RAW_METRICS_DIR)?;

    let dataset = RawDataset {
        enterprise,
        applications,
        metrics,
    };
    validate_raw_dataset(&dataset)?;
    Ok(dataset)
}

fn validate_raw_dataset(dataset: &RawDataset) -> Result<(), String> {
    let enterprise_app_ids = dataset
        .enterprise
        .applications
        .iter()
        .map(|application| application.id.as_str())
        .collect::<BTreeSet<_>>();
    let application_app_ids = dataset
        .applications
        .iter()
        .map(|application| application.id.as_str())
        .collect::<BTreeSet<_>>();
    if enterprise_app_ids != application_app_ids {
        return Err(format!(
            "enterprise application ids do not match raw application fixtures: enterprise={enterprise_app_ids:?} raw={application_app_ids:?}"
        ));
    }

    let metric_fixtures_by_app = dataset
        .metrics
        .iter()
        .fold(BTreeMap::<&str, BTreeMap<&str, MetricTheme>>::new(), |mut acc, metric| {
            acc.entry(metric.application_id.as_str())
                .or_default()
                .insert(metric.id.as_str(), metric.theme.clone());
            acc
        });

    for application in &dataset.applications {
        let Some(summary) = dataset
            .enterprise
            .applications
            .iter()
            .find(|summary| summary.id == application.id)
        else {
            return Err(format!(
                "missing enterprise summary for raw application {}",
                application.id
            ));
        };
        if summary.name != application.name
            || summary.application_type != application.application_type
        {
            return Err(format!(
                "raw application summary mismatch for {}",
                application.id
            ));
        }

        if application.metrics.len() != 3 {
            return Err(format!(
                "{} must define exactly 3 metrics, found {}",
                application.id,
                application.metrics.len()
            ));
        }

        let mut themes = BTreeSet::new();
        for metric_ref in &application.metrics {
            themes.insert(metric_ref.theme.clone());
            let Some(metric_fixtures) = metric_fixtures_by_app.get(application.id.as_str())
            else {
                return Err(format!(
                    "{} references metric {} but no raw metric file exists for the application",
                    application.id, metric_ref.id
                ));
            };
            let Some(metric_theme) = metric_fixtures.get(metric_ref.id.as_str()) else {
                return Err(format!(
                    "{} references metric {} but no raw metric file exists for that metric",
                    application.id, metric_ref.id
                ));
            };
            if metric_theme != &metric_ref.theme {
                return Err(format!(
                    "{} metric reference {} does not match raw metric fixture",
                    application.id, metric_ref.id
                ));
            }
        }

        if themes.len() != 3 {
            return Err(format!(
                "{} must include adoption, northstar, and technical metrics",
                application.id
            ));
        }
    }

    if dataset.metrics.len() != dataset.applications.len() * 3 {
        return Err(format!(
            "expected {} raw metrics but found {}",
            dataset.applications.len() * 3,
            dataset.metrics.len()
        ));
    }

    Ok(())
}

fn load_json_files<T>(dir: &str) -> Result<Vec<T>, String>
where
    T: for<'de> Deserialize<'de>,
{
    let mut paths = Vec::new();
    collect_json_files(Path::new(dir), &mut paths)?;
    paths.sort();

    paths
        .into_iter()
        .map(|path| {
            serde_json::from_str::<T>(
                &fs::read_to_string(&path)
                    .map_err(|err| format!("failed to read {}: {err}", path.display()))?,
            )
            .map_err(|err| format!("invalid raw fixture {}: {err}", path.display()))
        })
        .collect()
}

fn collect_json_files(dir: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(dir)
        .map_err(|err| format!("failed to read directory {}: {err}", dir.display()))?
    {
        let entry = entry.map_err(|err| format!("failed to read entry in {}: {err}", dir.display()))?;
        let path = entry.path();
        if path.is_dir() {
            collect_json_files(&path, paths)?;
        } else if path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
        {
            paths.push(path);
        }
    }
    Ok(())
}
