use test_harness::megalo_roundtrip;

fn main() {
    println!("Running megalo roundtrip tests");
    let report = megalo_roundtrip::run_all();

    println!();
    println!("passed: {}", report.passed);
    println!("failed: {}", report.failures.len());

    if !report.success() {
        for failure in &report.failures {
            eprintln!("  {} — {}", failure.path.display(), failure.message);
        }
        std::process::exit(1);
    }

    println!("all megalo roundtrip tests passed");
}
