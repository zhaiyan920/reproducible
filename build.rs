use libbpf_cargo::SkeletonBuilder;
use std::path::PathBuf;

const SRC: &str = "src/bpf/test.bpf.c";

fn main() {
    let mut out = PathBuf::from("src/bpf");
    out.push("test.skel.rs");
    SkeletonBuilder::new()
        .source(SRC)
        .build_and_generate(&out)
        .unwrap();
    println!("cargo:rerun-if-changed={}", SRC);
}
