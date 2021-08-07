#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;
#[macro_use]
extern crate juniper;

mod packages;
mod schema;

use anyhow::{Result, Context as _};
use schema::{Query, Schema};
use clap::Clap;
use dotenv::dotenv;
use juniper::{EmptyMutation, EmptySubscription};
use log::LevelFilter;
use scylla::{Session, SessionBuilder};
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;
use warp::Filter;

#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"))]
struct Opts {
    #[clap(short, long, default_value = "127.0.0.1:8080", env = "ADDRESS")]
    /// Service address
    address: String,
    /// Scylla endpoint
    #[clap(short, long, default_value = "127.0.0.1:9042", env = "SCYLLA")]
    scylla: String,
    /// Print debug info
    #[clap(short, long, env = "DEBUG", takes_value = false)]
    debug: bool,
    /// Print debug info
    #[clap(short, long, default_value = "registry:5000", env = "REGISTRY")]
    registry: String,
}

#[derive(Clone)]
pub struct Context {
    pub registry: String,
    pub scylla: Arc<Session>,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let opts = Opts::parse();

    // Configure logger.
    let mut logger = env_logger::builder();
    logger.format_module_path(false);

    if opts.debug {
        logger.filter_level(LevelFilter::Debug).init();
    } else {
        logger.filter_level(LevelFilter::Info).init();
    }

    // Configure Scylla.
    let scylla = SessionBuilder::new()
        .known_node(&opts.scylla)
        .connection_timeout(Duration::from_secs(3))
        .build()
        .await?;

    ensure_db_keyspace(&scylla).await?;
    packages::ensure_db_table(&scylla).await?;

    let scylla = Arc::new(scylla);
    let registry = opts.registry.clone();

    // Configure Juniper.
    let context = warp::any().map(move || Context {
        registry: registry.clone(),
        scylla: scylla.clone(),
    });

    let schema = Schema::new(Query {}, EmptyMutation::new(), EmptySubscription::new());
    let graphql_filter = juniper_warp::make_graphql_filter(schema, context.clone().boxed());
    let graphql = warp::path("graphql").and(graphql_filter);

    // Configure Warp.
    let package_upload = warp::path("packages")
        .and(warp::post())
        .and(warp::filters::header::headers_cloned())
        .and(warp::filters::body::bytes())
        .and(context)
        .and_then(packages::upload);

    let routes = graphql.or(package_upload).with(warp::log("brane-api"));
    let address: SocketAddr = opts.address.clone().parse()?;
    warp::serve(routes).run(address).await;

    Ok(())
}

///
///
///
pub async fn ensure_db_keyspace(scylla: &Session) -> Result<()> {
    let query = r#"
        CREATE KEYSPACE IF NOT EXISTS brane
        WITH replication = {'class': 'SimpleStrategy', 'replication_factor' : 3};
    "#;

    scylla
        .query(query, &[])
        .await
        .map(|_| Ok(()))
        .map_err(|e| anyhow!("{:?}", e))
        .context("Failed to create 'brane' keyspace.")?
}