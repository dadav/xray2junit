mod cli;
mod format;
mod convert;

// use junit_report::{datetime, Duration, ReportBuilder, TestCase, TestCaseBuilder, TestSuite, TestSuiteBuilder};
pub use {
    format::*,
    cli::*,
    convert::*,
};

