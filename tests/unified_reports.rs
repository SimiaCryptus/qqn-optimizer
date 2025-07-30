// Integration test for unified reporting functionality
use qqn_optimizer::experiment_runner::{
    Report, ReportCollection, ReportConfig, ReportFormat, UnifiedReportTestSuite
};
use qqn_optimizer::experiment_runner::reports::unified_summary_statistics::SummaryStatisticsReport;

#[test]
fn test_unified_report_trait() {
    let report = SummaryStatisticsReport::new();
    
    // Test basic trait functionality
    assert_eq!(report.name(), "summary_statistics");
    assert!(!report.description().is_empty());
    
    let formats = report.supported_formats();
    assert!(!formats.is_empty());
    assert!(formats.contains(&ReportFormat::Html));
    assert!(formats.contains(&ReportFormat::Latex));
    assert!(formats.contains(&ReportFormat::Markdown));
}

#[test]
fn test_unified_report_collection() {
    let collection = ReportCollection::new()
        .add_report(SummaryStatisticsReport::new());
    
    let names = collection.report_names();
    assert_eq!(names.len(), 1);
    assert_eq!(names[0], "summary_statistics");
}

#[test]
fn test_unified_test_suite() {
    let report = SummaryStatisticsReport::new();
    
    // Use the unified test suite to validate the report
    UnifiedReportTestSuite::test_basic_functionality(&report).unwrap();
    
    // Create test data
    let test_data = UnifiedReportTestSuite::create_test_data();
    assert!(!test_data.is_empty());
    
    for (problem_spec, benchmark_results) in &test_data {
        assert!(!problem_spec.get_name().is_empty());
        assert!(!benchmark_results.results.is_empty());
    }
}

#[test]
fn test_summary_statistics_report_content() {
    let report = SummaryStatisticsReport::new();
    let test_data = UnifiedReportTestSuite::create_test_data();
    let data_refs: Vec<_> = test_data.iter().map(|(p, r)| (p, r.clone())).collect();
    
    // Test HTML output
    let html_config = ReportConfig {
        format: ReportFormat::Html,
        ..Default::default()
    };
    let html_content = report.generate_content(&data_refs, &html_config).unwrap();
    assert!(html_content.contains("<!DOCTYPE html>"));
    assert!(html_content.contains("Summary Statistics Report"));
    
    // Test Markdown output
    let md_config = ReportConfig {
        format: ReportFormat::Markdown,
        ..Default::default()
    };
    let md_content = report.generate_content(&data_refs, &md_config).unwrap();
    assert!(md_content.contains("# Summary Statistics Report"));
    
    // Test CSV output
    let csv_config = ReportConfig {
        format: ReportFormat::Csv,
        ..Default::default()
    };
    let csv_content = report.generate_content(&data_refs, &csv_config).unwrap();
    assert!(csv_content.contains("Problem_Family,Optimizer"));
    assert!(csv_content.contains("Convex Unimodal"));
}