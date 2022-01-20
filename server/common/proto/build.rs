fn main() {
    tonic_build::configure()
        .out_dir("./src/target")
        .compile(
            &["proto/auth.proto", "proto/user.proto", "proto/core.proto"],
            &["proto"],
        )
        .unwrap();
}
