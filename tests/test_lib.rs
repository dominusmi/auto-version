// In order to completely test the package, a new test package is created in the temporary folder
// and filled with a dummy main and Cargo.toml.
// This is a bit extra, but since the macro is used for executables, typical unit tests won't work
// The other idea was to test using example binaries, but it defeats the purpose of testing on a
// completely independent binary.

#[cfg(test)]
mod tests {
    use std::process::Command;

    /// Used to remove the temporary package at the end of the test
    struct ResourceCleaner;

    impl Drop for ResourceCleaner {
        fn drop(&mut self) {
            let dir = std::env::temp_dir().join("test_auto_version");
            if dir.exists() {
                if let Err(e) = std::fs::remove_dir_all(&dir){
                    println!("Couldn't clean up test directory {:?}: {:?}", &dir, e);
                }
            }
        }
    }

    #[test]
    fn test_package(){
        let _setup = ResourceCleaner;
        let dir = std::env::temp_dir();
        let result = Command::new("cargo")
            .arg("init")
            .arg("test_auto_version")
            .current_dir(&dir)
            .output();

        if result.is_err() || !result.as_ref().unwrap().status.success() {
            println!("Error when trying to create temporary project in {:?}: {:?}", &dir, result)
        };

        let test_package_dir = dir.join("test_auto_version");

        if let Err(e) = std::fs::write(test_package_dir.join("src").join("main.rs"), MAIN){
            println!("Could not run test due to error when modifying test source: {:?}", e);
        }

        let curr_package_dir = std::env!("CARGO_MANIFEST_DIR");
        let cargo_content = CARGO.replace("$VERSION", "0.0.1").replace("$PATH", &curr_package_dir);
        if let Err(e) = std::fs::write(test_package_dir.join("Cargo.toml"), cargo_content){
            println!("Could not run test due to error when modifying test source: {:?}", e);
        }

        println!("Building temporary package");
        let output = Command::new("cargo")
            .current_dir(&test_package_dir)
            .arg("build").output().unwrap();

        println!("Testing -v");
        let output = Command::new("cargo")
            .current_dir(&test_package_dir)
            .arg("run").arg("--").arg("-v").output().unwrap();

        dbg!(&output);
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.starts_with("0.0.1"), "Expected stdout to be 0.0.1, got {}", stdout);

        println!("Testing --version");
        let output = Command::new("cargo")
            .current_dir(&test_package_dir)
            .arg("run").arg("--").arg("--version").output().unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.starts_with("0.0.1"), "Expected stdout to be 0.0.1, got {}", stdout);

        println!("Testing no args");
        let output = Command::new("cargo")
            .current_dir(&test_package_dir)
            .arg("run").output().unwrap();

        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(stdout.starts_with("Hello world!"), "Expected stdout to be 'Hello world!', got {}", stdout);
    }

    const CARGO: &str = r#"
[package]
name = "test_auto_version"
version = "$VERSION"
edition = "2018"
description = "Temporary package for testing of auto-version"

[dependencies]
auto-version = { path = "$PATH" }
"#;

    const MAIN: &str = r#"
use auto_version::auto_version;

#[auto_version]
fn main(){
    println!("Hello world!");
}
"#;
}
