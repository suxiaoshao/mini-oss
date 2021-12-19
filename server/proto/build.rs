fn main() {
    tonic_build::configure()
        .compile(&["proto/auth.proto"], &["proto"])
        .unwrap();
}
