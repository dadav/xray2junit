use std::io;
use serde_json::Result as SerdeResult;
use junit_report::{TestSuite,TestCase,ReportBuilder,Duration};

use crate::*;

impl SimpleJsonResults {
    pub fn new(r: impl io::Read) -> SerdeResult<SimpleJsonResults> {
        Ok(serde_json::from_reader(r)?)
    }

    pub fn export(&self, output: impl io::Write) -> Result<(), Box<dyn std::error::Error>> {

        let mut vulnerabilities = TestSuite::new("Vulnerabilities");

        for vuln in &self.vulnerabilities {
            vulnerabilities.add_testcase(
                TestCase::failure(
                    &vuln.issue_id,
                    Duration::seconds(0),
                    &vuln.severity,
                    &vuln.summary,
                )
            );
        }

        let mut security_violations = TestSuite::new("Security Violations");

        for sec_vuln in &self.security_violations {
            security_violations.add_testcase(
                TestCase::failure(
                    &sec_vuln.issue_id,
                    Duration::seconds(0),
                    &sec_vuln.severity,
                    &sec_vuln.summary,
                )
            );
        }

        let mut license_violations = TestSuite::new("License Violations");

        for lic_vio in &self.security_violations {
            license_violations.add_testcase(
                TestCase::failure(
                    &lic_vio.issue_id,
                    Duration::seconds(0),
                    &lic_vio.severity,
                    &lic_vio.summary,
                )
            );
        }

        let mut report = ReportBuilder::new()
            .build();

        report.add_testsuite(vulnerabilities);
        report.add_testsuite(security_violations);
        report.add_testsuite(license_violations);

        Ok(report.write_xml(output)?)
    }
}
