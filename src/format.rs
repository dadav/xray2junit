// See: https://github.com/jfrog/jfrog-cli-core/blob/master/xray/formats/simplejsonapi.go

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

// This struct holds the sorted results of the simple-json output.
#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleJsonResults {
	pub vulnerabilities: Option<Vec<VulnerabilityOrViolationRow>>,

  #[serde(rename = "securityViolations")]
	pub security_violations: Option<Vec<VulnerabilityOrViolationRow>>,

  #[serde(rename = "licensesViolations")]
	pub licenses_violations: Option<Vec<LicenseViolationRow>>,

	pub licenses: Option<Vec<LicenseRow>>,

  #[serde(rename = "operationalRiskViolations")]
	pub operational_risk_violations: Option<Vec<OperationalRiskViolationRow>>,

	pub errors: Option<Vec<SimpleJsonError>>,
}

// Used for vulnerabilities and security violations
#[derive(Debug, Deserialize, Serialize)]
pub struct VulnerabilityOrViolationRow {
	pub summary: String,
	pub severity: String,

  #[serde(rename = "impactedPackageName")]
	pub impacted_dependency_name: String,

  #[serde(rename = "impactedPackageVersion")]
	pub impacted_dependency_version: String,

  #[serde(rename = "impactedPackageType")]
	pub impacted_dependency_type: String,

  #[serde(rename = "fixedVersions")]
	pub fixed_versions: Option<Vec<String>>,

	pub components: Vec<ComponentRow>,
	pub cves: Option<Vec<CveRow>>,

  #[serde(rename = "issueId")]
	pub issue_id: String,

	pub references: Vec<String>,

  #[serde(rename = "impactPaths")]
	pub impact_paths: Vec<Vec<ComponentRow>>,

  #[serde(rename = "jfrogResearchInformation")]
	pub jfrog_research_information: Option<JfrogResearchInformation>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LicenseRow {
  #[serde(rename = "licenseKey")]
	pub license_key: String,

  #[serde(rename = "impactedPackageName")]
	pub impacted_dependency_name: String,

  #[serde(rename = "impactedPackageVersion")]
	pub impacted_dependency_version: String,

  #[serde(rename = "impactedPackageType")]
	pub impacted_dependency_type: String,

	pub components: Vec<ComponentRow>,

  #[serde(rename = "impactPaths")]
	pub impact_paths: Vec<Vec<ComponentRow>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct LicenseViolationRow {
  #[serde(rename = "licenseKey")]
	pub license_key: String,

	pub severity: String,

  #[serde(rename = "impactedPackageName")]
	pub impacted_dependency_name: String,

  #[serde(rename = "impactedPackageVersion")]
	pub impacted_dependency_version: String,

  #[serde(rename = "impactedPackageType")]
	pub impacted_dependency_type: String,

	pub components: Vec<ComponentRow>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OperationalRiskViolationRow {
	pub severity: String,

  #[serde(rename = "impactedPackageName")]
	pub impacted_dependency_name: String,

  #[serde(rename = "impactedPackageVersion")]
	pub impacted_dependency_version: String,

  #[serde(rename = "impactedPackageType")]
	pub impacted_dependency_type: String,

	pub components: Vec<ComponentRow>,

  #[serde(rename = "riskReason")]
	pub risk_reason: String,

  #[serde(rename = "isEndOfLife")]
	pub is_eol: String,

  #[serde(rename = "endOfLifeMessage")]
	pub eol_message: String,

	pub cadence: String,
	pub commits: String,
	pub committers: String,

  #[serde(rename = "newerVersions")]
	pub newer_versions: String,

  #[serde(rename = "latestVersion")]
	pub latest_version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ComponentRow {
	pub name: String,
	pub version: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CveRow {
	pub id: String,

  #[serde(rename = "cvssV2")]
	pub cvss_v2: String,

  #[serde(rename = "cvssV3")]
	pub cvss_v3: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleJsonError {
  #[serde(rename = "filePath")]
	pub file_path: PathBuf,

  #[serde(rename = "errorMessage")]
	pub error_message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JfrogResearchInformation {
	pub summary: Option<String>,
	pub details: Option<String>,
	pub severity: Option<String>,

  #[serde(rename = "severityReasons")]
	pub severity_reasons: Option<Vec<JfrogResearchSeverityReason>>,

	pub remediation: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JfrogResearchSeverityReason {
	pub name: Option<String>,
	pub description: Option<String>,

  #[serde(rename = "isPositive")]
	pub is_positive: Option<bool>,
}
