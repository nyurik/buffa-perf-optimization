fn main() -> Result<(), Box<dyn std::error::Error>> {
    buffa_build::Config::new()
        .files(&["src/perf.proto"])
        .includes(&["src"])
        .out_dir("src/generated")
        .include_file("mod.rs")
        .preserve_unknown_fields(false)
        .compile()
}
