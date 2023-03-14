use junit_report::{Duration, ReportBuilder, TestCase, TestSuite};
use serde_json::Result as SerdeResult;
use std::io;

use crate::*;

impl VulnerabilityOrViolationRow {
    fn to_testcase(&self) -> TestCase {
        TestCase::failure(
            &format!("{} ({})", &self.issue_id, &self.severity),
            Duration::seconds(0),
            "Vulnerability",
            &format!("[{} (version: {})]: {}", &self.impacted_dependency_name, &self.impacted_dependency_version, &self.summary),
        )
    }
}

impl LicenseViolationRow {
    fn to_testcase(&self) -> TestCase {
        TestCase::failure(
            &format!("{} ({})", &self.license_key, &self.severity),
            Duration::seconds(0),
            "License Violation",
            &format!("[{} (version: {})]: {}", &self.impacted_dependency_name, &self.impacted_dependency_version, &self.license_key),
        )
    }
}

impl OperationalRiskViolationRow {
    fn to_testcase(&self) -> TestCase {
        TestCase::failure(
            &format!("{} ({})", &self.impacted_dependency_name, &self.severity),
            Duration::seconds(0),
            "Operational Risk",
            &self.risk_reason,
        )
    }
}

impl SimpleJsonResults {
    pub fn new(r: impl io::Read) -> SerdeResult<SimpleJsonResults> {
        Ok(serde_json::from_reader(r)?)
    }

    pub fn export(&self, output: impl io::Write) -> Result<(), Box<dyn std::error::Error>> {
        let mut vulnerabilities = TestSuite::new("Vulnerabilities");

        match &self.vulnerabilities {
            Some(vulns) => {
                for vuln in vulns {
                    vulnerabilities.add_testcase(vuln.to_testcase());
                }
            }
            None => vulnerabilities.add_testcase(TestCase::success(
                "No vulnerabilities found.",
                Duration::seconds(0),
            )),
        }

        let mut security_violations = TestSuite::new("Security Violations");

        match &self.security_violations {
            Some(secs) => {
                for sec_vuln in secs {
                    security_violations.add_testcase(sec_vuln.to_testcase());
                }
            }
            None => security_violations.add_testcase(TestCase::success(
                "No security violations found.",
                Duration::seconds(0),
            )),
        }

        let mut license_violations = TestSuite::new("License Violations");

        match &self.licenses_violations {
            Some(lics) => {
                for lic_vio in lics {
                    license_violations.add_testcase(lic_vio.to_testcase());
                }
            }
            None => license_violations.add_testcase(TestCase::success(
                "No license violations found.",
                Duration::seconds(0),
            )),
        }

        let mut op_risk_violations = TestSuite::new("Operational Risk Violations");

        match &self.operational_risk_violations {
            Some(ops) => {
                for op in ops {
                    op_risk_violations.add_testcase(op.to_testcase());
                }
            }
            None => op_risk_violations.add_testcase(TestCase::success(
                "No operational risk violations found.",
                Duration::seconds(0),
            )),
        }

        let mut report = ReportBuilder::new().build();

        report.add_testsuite(vulnerabilities);
        report.add_testsuite(security_violations);
        report.add_testsuite(license_violations);
        report.add_testsuite(op_risk_violations);

        Ok(report.write_xml(output)?)
    }
}
