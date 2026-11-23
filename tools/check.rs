use std::process::Command;

#[test]
fn run_check_script() {
    let tl = std::env::var("TL").unwrap_or_else(|_| "1".into());
    let ml = std::env::var("ML").unwrap_or_else(|_| "256".into());

    let status = Command::new("bash")
        .arg("tools/check.sh")
        .args([&tl, &ml])
        .env("CP_NO_BUILD", "1")
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .status()
        .expect("failed to run check.sh");

    assert!(status.success(), "some test cases failed");
}
