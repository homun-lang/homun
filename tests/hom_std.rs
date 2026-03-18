/// Integration tests for hom-std runtime libraries (re, heap, chars).
/// Compiles and runs the test_*.hom files from _site/examples/.
use std::fs;
use std::path::Path;
use std::process::Command;

fn compile_and_run(hom_path: &Path) -> String {
    let tmp = std::env::temp_dir().join("homun_tests_std");
    fs::create_dir_all(&tmp).unwrap();

    let stem = hom_path.file_stem().unwrap().to_string_lossy();

    let rs_path = tmp.join(format!("{}.rs", stem));
    let compile_hom = Command::new(env!("CARGO_BIN_EXE_homunc"))
        .args([hom_path.to_str().unwrap(), "-o", rs_path.to_str().unwrap()])
        .output()
        .expect("failed to run homunc");

    assert!(
        compile_hom.status.success(),
        "{}: homunc failed:\n{}",
        stem,
        String::from_utf8_lossy(&compile_hom.stderr)
    );

    let bin_path = tmp.join(format!("{}_bin", stem));
    let compile_rs = Command::new("rustc")
        .args([rs_path.to_str().unwrap(), "-o", bin_path.to_str().unwrap()])
        .output()
        .expect("rustc not found");

    if !compile_rs.status.success() {
        let src = fs::read_to_string(&rs_path).unwrap_or_default();
        panic!(
            "{}: rustc failed:\n{}\n\nGenerated .rs:\n{}",
            stem,
            String::from_utf8_lossy(&compile_rs.stderr),
            src
        );
    }

    let run = Command::new(&bin_path)
        .output()
        .expect("failed to run binary");

    assert!(
        run.status.success(),
        "{}: binary exited with error:\n{}",
        stem,
        String::from_utf8_lossy(&run.stderr)
    );

    String::from_utf8(run.stdout).unwrap()
}

/// Compile a .hom file that depends on external crates via a temp Cargo project.
/// `deps` is a list of `(crate_name, version)` pairs to add to Cargo.toml.
fn compile_and_run_cargo(hom_path: &Path, deps: &[(&str, &str)]) -> String {
    let stem = hom_path.file_stem().unwrap().to_string_lossy();
    let tmp = std::env::temp_dir().join(format!("homun_tests_cargo_{}", stem));

    // Set up temp Cargo project
    let src_dir = tmp.join("src");
    fs::create_dir_all(&src_dir).unwrap();

    // Compile .hom → .rs
    let rs_path = src_dir.join("main.rs");
    let compile_hom = Command::new(env!("CARGO_BIN_EXE_homunc"))
        .args([hom_path.to_str().unwrap(), "-o", rs_path.to_str().unwrap()])
        .output()
        .expect("failed to run homunc");

    assert!(
        compile_hom.status.success(),
        "{}: homunc failed:\n{}",
        stem,
        String::from_utf8_lossy(&compile_hom.stderr)
    );

    // Write Cargo.toml
    let mut cargo_toml = String::from(
        "[package]\nname = \"hom_test\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[dependencies]\n",
    );
    for (name, ver) in deps {
        cargo_toml.push_str(&format!("{} = \"{}\"\n", name, ver));
    }
    fs::write(tmp.join("Cargo.toml"), &cargo_toml).unwrap();

    // cargo run
    let run = Command::new("cargo")
        .args(["run", "--quiet"])
        .current_dir(&tmp)
        .output()
        .expect("cargo not found");

    if !run.status.success() {
        let src = fs::read_to_string(&rs_path).unwrap_or_default();
        panic!(
            "{}: cargo run failed:\n{}\n\nGenerated .rs:\n{}",
            stem,
            String::from_utf8_lossy(&run.stderr),
            src
        );
    }

    String::from_utf8(run.stdout).unwrap()
}

#[test]
fn test_hom_std_re() {
    let out = compile_and_run_cargo(Path::new("_site/examples/test_re.hom"), &[("regex", "1")]);
    assert!(!out.is_empty(), "test_re should produce output");
}

#[test]
fn test_hom_std_heap() {
    let out = compile_and_run(Path::new("_site/examples/test_heap.hom"));
    assert!(!out.is_empty(), "test_heap should produce output");
}

#[test]
fn test_hom_std_chars() {
    let out = compile_and_run(Path::new("_site/examples/test_chars.hom"));
    assert!(!out.is_empty(), "test_chars should produce output");
}
