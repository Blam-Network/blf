use test_harness::megalo_roundtrip;

#[test]
fn megalo_roundtrip_all_fixtures() {
    let report = megalo_roundtrip::run_all();
    assert!(
        report.success(),
        "{} roundtrip failures ({} passed)",
        report.failures.len(),
        report.passed
    );
}
