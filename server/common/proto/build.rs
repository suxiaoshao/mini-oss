fn main() {
    tonic_build::configure()
        .type_attribute("core.Access", "#[derive(async_graphql::Enum)]")
        .type_attribute(
            "core.UpdateBucketRequest",
            "#[derive(async_graphql::InputObject)]",
        )
        .type_attribute(
            "core.DeleteBucketRequest",
            "#[derive(async_graphql::InputObject)]",
        )
        .type_attribute(
            "core.CreateBucketRequest",
            "#[derive(async_graphql::InputObject)]",
        )
        .type_attribute("auth.LoginRequest", "#[derive(async_graphql::InputObject)]")
        .type_attribute("auth.CheckRequest", "#[derive(async_graphql::InputObject)]")
        .type_attribute(".user", "#[derive(async_graphql::InputObject)]")
        .compile(
            &["proto/auth.proto", "proto/user.proto", "proto/core.proto"],
            &["proto"],
        )
        .unwrap();
}
