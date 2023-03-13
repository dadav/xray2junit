use std::io;
use serde_json::Result as SerdeResult;
use junit_report::{TestSuite,TestCase,ReportBuilder,Duration};

use crate::*;

impl VulnerabilityOrViolationRow {
    fn to_testcase(&self) -> TestCase {
        TestCase::failure(
            &self.issue_id,
            Duration::seconds(0),
            &self.severity,
            &self.summary,
        )
    }
}

impl LicenseViolationRow {
    fn to_testcase(&self) -> TestCase {
        TestCase::failure(
            &self.license_key,
            Duration::seconds(0),
            &self.severity,
            "foo",
        )
    }
}

impl OperationalRiskViolationRow {
    fn to_testcase(&self) -> TestCase {
        TestCase::failure(
            &self.risk_reason,
            Duration::seconds(0),
            &self.severity,
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
            },
            None => (),
        }

        let mut security_violations = TestSuite::new("Security Violations");

        match &self.security_violations {
            Some(secs) => {
                for sec_vuln in secs {
                    security_violations.add_testcase(sec_vuln.to_testcase());
                }
            },
            None => (),
        }


        let mut license_violations = TestSuite::new("License Violations");

        match &self.licenses_violations {
            Some(lics) => {
                for lic_vio in lics {
                    license_violations.add_testcase(lic_vio.to_testcase());
                }
            },
            None => (),
        }

        let mut op_risk_violations = TestSuite::new("Operational Risk Violations");

        match &self.operational_risk_violations {
            Some(ops) => {
                for op in ops {
                    op_risk_violations.add_testcase(op.to_testcase());
                }
            },
            None => (),
        }

        let mut report = ReportBuilder::new()
            .build();

        report.add_testsuite(vulnerabilities);
        report.add_testsuite(security_violations);
        report.add_testsuite(license_violations);
        report.add_testsuite(op_risk_violations);

        Ok(report.write_xml(output)?)
    }
}
