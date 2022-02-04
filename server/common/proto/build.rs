fn main() {
    println!("cargo:rerun-if-changed=proto");
    println!("cargo:rerun-if-changed=build.rs");
    std::fs::create_dir_all("./src/pre_gen").unwrap();
    tonic_build::configure()
        .out_dir("./src/pre_gen")
        .compile(
            &["proto/auth.proto", "proto/user.proto", "proto/core.proto"],
            &["proto"],
        )
        .unwrap();
}
