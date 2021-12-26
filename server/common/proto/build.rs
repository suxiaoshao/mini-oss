fn main() {
    tonic_build::configure()
        .compile(&["proto/auth.proto", "proto/user_manage.proto"], &["proto"])
        .unwrap();
}
