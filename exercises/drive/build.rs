fn main() {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // Set the environment variable
    std::env::set_var("TEST_FOO", (timestamp).to_string());

    // Add code here to enable the "pass" feature
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
