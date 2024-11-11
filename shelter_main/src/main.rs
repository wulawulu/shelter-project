use clap::{Arg, Command};
use dotenv::dotenv;
use tracing::Level;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;
use shelter_main::{commands, settings};

fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let mut command = Command::new("Dog Shelter smaple application")
        .version("1.0")
        .author("wu <wutuobangbang@gmail.com>")
        .about("A sample application to experiment with Rust-based microservices")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .help("Configuration file location")
                .default_value("config.json"),
        );

    command = commands::configure(command);

    let matches = command.get_matches();

    let config_location = matches
        .get_one::<String>("config")
        .map(|s| s.as_str())
        .unwrap_or("");

    let settings = settings::Settings::new(config_location, "SHELTER")?;

    let subscriber = Registry::default()
        .with(LevelFilter::from_level(Level::DEBUG))
        .with(tracing_subscriber::fmt::Layer::new().with_writer(std::io::stderr));

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    commands::handle(&matches, &settings)?;

    Ok(())
}
