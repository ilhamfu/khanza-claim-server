#[derive(Debug, clap::Args)]
pub struct AppConfig {
    /// Hospital name, used in SPRI and SEP
    #[arg(
        long = "rs-name",
        env = "KHANZA_CLAIM__RS_NAME",
        default_value = "RS YANKES KHANZA"
    )]
    pub rs_name: String,
    /// Path to khanza resource, usually inside ./webapps folder. accept both file path or url
    #[arg(
        long = "resource-location",
        env = "KHANZA_CLAIM__RESOURCE_LOCATION",
        default_value = "http://localhost/webapps"
    )]
    pub resource_location: String,
}
#[derive(Debug, clap::Args)]
pub struct Config {
    /// Database connection string of khanza
    #[arg(
        long = "database-url",
        env = "KHANZA_CLAIM__DATABASE_URL",
        default_value = "mysql://root:12345678@localhost:3366/khanza"
    )]
    pub database_url: String,
    #[command(flatten)]
    pub app_config: AppConfig,
}
