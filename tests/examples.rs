/// Integration tests — compile and run every _site/examples/*.hom file.
/// Each .hom is compiled via homunc (resolver + codegen), then rustc, then executed.
use std::fs;
use std::path::Path;
use std::process::Command;

fn compile_and_run(hom_path: &Path) -> String {
    let tmp = std::path::PathBuf::from(".tmp/homun_tests");
    fs::create_dir_all(&tmp).unwrap();

    let stem = hom_path.file_stem().unwrap().to_string_lossy();

    // Compile .hom → .rs via cargo run
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

    // Compile .rs → binary via rustc
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

    // Run binary
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

#[test]
fn test_example_fib() {
    let out = compile_and_run(Path::new("_site/examples/fib.hom"));
    assert!(out.contains("fib(0) = 0"));
    assert!(out.contains("fib(9) = 34"));
}

#[test]
fn test_example_fizzbuzz() {
    let out = compile_and_run(Path::new("_site/examples/fizzbuzz.hom"));
    assert!(out.contains("FizzBuzz"));
    assert!(out.contains("Hello, Aria!"));
}

#[test]
fn test_example_binary_search() {
    let out = compile_and_run(Path::new("_site/examples/binary_search.hom"));
    assert!(out.contains("index 3"));
    assert!(out.contains("index -1"));
}

#[test]
fn test_example_dfs() {
    let out = compile_and_run(Path::new("_site/examples/dfs.hom"));
    assert!(out.contains("DFS traversal:"));
}

#[test]
fn test_example_pipeline() {
    let out = compile_and_run(Path::new("_site/examples/pipeline.hom"));
    assert!(out.contains("220"));
}

#[test]
fn test_example_quicksort() {
    let out = compile_and_run(Path::new("_site/examples/quicksort.hom"));
    assert!(out.contains("[1, 2, 3, 4, 5, 6, 7, 8, 9]"));
}

#[test]
fn test_example_two_sum() {
    let out = compile_and_run(Path::new("_site/examples/two_sum.hom"));
    assert!(out.contains("Two Sum:"));
}

#[test]
fn test_example_mut_bind() {
    let out = compile_and_run(Path::new("_site/examples/mut_bind.hom"));
    assert!(
        out.contains("5"),
        "counter_test should print 5, got: {}",
        out
    );
    assert!(
        out.contains("a=20, b=10"),
        "swap_test should print a=20, b=10, got: {}",
        out
    );
}

#[test]
fn test_example_mut_param() {
    let out = compile_and_run(Path::new("_site/examples/mut_param.hom"));
    assert!(
        out.contains("7"),
        "mut_param should print 7 (0+1+1+5), got: {}",
        out
    );
}

#[test]
fn test_example_attr_derive() {
    let out = compile_and_run(Path::new("_site/examples/attr_derive.hom"));
    assert!(out.contains("p=(3,4) q=(3,4)"));
}

#[test]
fn test_example_attr_cfg() {
    let out = compile_and_run(Path::new("_site/examples/attr_cfg.hom"));
    assert!(out.contains("sq(7)=49"));
}

#[test]
fn test_example_enum_multi_payload() {
    let out = compile_and_run(Path::new("_site/examples/enum_multi_payload.hom"));
    assert!(
        out.contains("sum=42"),
        "enum_multi_payload should print sum=42, got: {}",
        out
    );
}

#[test]
fn test_example_match_or_pat() {
    let out = compile_and_run(Path::new("_site/examples/match_or_pat.hom"));
    assert!(
        out.contains("true")
            && out.contains("false")
            && out.contains("vertical")
            && out.contains("horizontal"),
        "match_or_pat should print true/false/vertical/horizontal, got: {}",
        out
    );
}

#[test]
fn test_example_derive_enum() {
    let out = compile_and_run(Path::new("_site/examples/derive_enum.hom"));
    assert!(
        out.contains("num=5"),
        "derive_enum should print num=5, got: {}",
        out
    );
}

#[test]
fn test_example_thread_local_state() {
    let out = compile_and_run(Path::new("_site/examples/thread_local_state.hom"));
    assert!(
        out.contains("steps=3") && out.contains("msg=hello"),
        "thread_local_state should print steps=3 msg=hello, got: {}",
        out
    );
}

#[test]
fn test_example_test_path_fs() {
    let out = compile_and_run(Path::new("_site/examples/test_path_fs.hom"));
    assert!(
        out.contains("joined=a/b.txt"),
        "test_path_fs: expected joined=a/b.txt, got: {}",
        out
    );
    assert!(
        out.contains("parent=a/b"),
        "test_path_fs: expected parent=a/b, got: {}",
        out
    );
    assert!(
        out.contains("stripped=q/r"),
        "test_path_fs: expected stripped=q/r, got: {}",
        out
    );
    assert!(
        out.contains("write=ok"),
        "test_path_fs: expected write=ok, got: {}",
        out
    );
    assert!(
        out.contains("content=hello_from_fs"),
        "test_path_fs: expected content=hello_from_fs, got: {}",
        out
    );
    assert!(
        out.contains("found=true"),
        "test_path_fs: expected found=true, got: {}",
        out
    );
    assert!(
        out.contains("is_dir=true"),
        "test_path_fs: expected is_dir=true, got: {}",
        out
    );
}

#[test]
fn test_example_explicit_generics() {
    let out = compile_and_run(Path::new("_site/examples/explicit_generics.hom"));
    assert!(
        out.contains("n=7")
            && out.contains("t=hello")
            && out.contains("r=99")
            && out.contains("w=42"),
        "explicit_generics should print n=7 t=hello r=99 w=42, got: {}",
        out
    );
}

#[test]
fn test_example_struct_destruct() {
    let out = compile_and_run(Path::new("_site/examples/struct_destruct.hom"));
    assert!(
        out.contains("px=3, py=4"),
        "struct_destruct should print px=3, py=4, got: {}",
        out
    );
    assert!(
        out.contains("dist=25"),
        "struct_destruct should print dist=25, got: {}",
        out
    );
    assert!(
        out.contains("rgb=255,128,0"),
        "struct_destruct should print rgb=255,128,0, got: {}",
        out
    );
}
