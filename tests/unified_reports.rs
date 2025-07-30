// Integration test for unified reporting functionality
use qqn_optimizer::experiment_runner::reports::unified_performance_table::PerformanceTableReport;
use qqn_optimizer::experiment_runner::reports::unified_summary_statistics::SummaryStatisticsReport;
use qqn_optimizer::experiment_runner::{
    Report, ReportCollection, ReportConfig, ReportFormat, UnifiedReportTestSuite,
};

#[test]
fn test_unified_report_trait() {
    let summary_report = SummaryStatisticsReport::new();

    // Test basic trait functionality
    assert_eq!(summary_report.name(), "summary_statistics");
    assert!(!summary_report.description().is_empty());

    let formats = summary_report.supported_formats();
    assert!(!formats.is_empty());
    assert!(formats.contains(&ReportFormat::Html));
    assert!(formats.contains(&ReportFormat::Latex));
    assert!(formats.contains(&ReportFormat::Markdown));

    let performance_report = PerformanceTableReport::new();
    assert_eq!(performance_report.name(), "performance_table");
    assert!(!performance_report.description().is_empty());
}

#[test]
fn test_unified_report_collection() {
    let collection = ReportCollection::new()
        .add_report(SummaryStatisticsReport::new())
        .add_report(PerformanceTableReport::new());

    let names = collection.report_names();
    assert_eq!(names.len(), 2);
    assert!(names.contains(&"summary_statistics"));
    assert!(names.contains(&"performance_table"));
}

#[test]
fn test_unified_test_suite() {
    let summary_report = SummaryStatisticsReport::new();
    let performance_report = PerformanceTableReport::new();

    // Use the unified test suite to validate both reports
    UnifiedReportTestSuite::test_basic_functionality(&summary_report).unwrap();
    UnifiedReportTestSuite::test_basic_functionality(&performance_report).unwrap();

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

#[test]
fn test_performance_table_report_content() {
    let report = PerformanceTableReport::new();
    let test_data = UnifiedReportTestSuite::create_test_data();
    let data_refs: Vec<_> = test_data.iter().map(|(p, r)| (p, r.clone())).collect();

    // Test HTML output
    let html_config = ReportConfig {
        format: ReportFormat::Html,
        ..Default::default()
    };
    let html_content = report.generate_content(&data_refs, &html_config).unwrap();
    assert!(html_content.contains("<!DOCTYPE html>"));
    assert!(html_content.contains("Performance Table Report"));
    assert!(html_content.contains("<table>"));

    // Test Markdown output
    let md_config = ReportConfig {
        format: ReportFormat::Markdown,
        ..Default::default()
    };
    let md_content = report.generate_content(&data_refs, &md_config).unwrap();
    assert!(md_content.contains("# Performance Table Report"));
    assert!(md_content.contains("| Optimizer |"));

    // Test CSV output
    let csv_config = ReportConfig {
        format: ReportFormat::Csv,
        ..Default::default()
    };
    let csv_content = report.generate_content(&data_refs, &csv_config).unwrap();
    assert!(csv_content.contains("Problem,Optimizer"));
}

#[test]
fn test_multiple_reports_different_content() {
    let summary_report = SummaryStatisticsReport::new();
    let performance_report = PerformanceTableReport::new();
    let test_data = UnifiedReportTestSuite::create_test_data();
    let data_refs: Vec<_> = test_data.iter().map(|(p, r)| (p, r.clone())).collect();

    let config = ReportConfig {
        format: ReportFormat::Html,
        ..Default::default()
    };

    let summary_content = summary_report
        .generate_content(&data_refs, &config)
        .unwrap();
    let performance_content = performance_report
        .generate_content(&data_refs, &config)
        .unwrap();

    // Different reports should produce different content
    assert_ne!(summary_content, performance_content);

    // But both should be valid HTML
    assert!(summary_content.contains("<!DOCTYPE html>"));
    assert!(performance_content.contains("<!DOCTYPE html>"));
}
