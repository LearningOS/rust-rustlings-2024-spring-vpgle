//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
    let your_command = format!(
        "{:?}",
        timestamp
    );
    println!("cargo::rustc-env=TEST_FOO={}", your_command);

    // In tests8, we should enable "pass" feature to make the
    // testcase return early. Fill in the command to tell
    // Cargo about that.
    let your_command = "pass";
        println!("cargo::rustc-cfg=CFG=\"{}\"", your_command);
}
