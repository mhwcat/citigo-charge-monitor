use backtrace::Backtrace;
use citigo_charge_monitor_api_server::{config::Config, run};
use log::{error, info};
use sqlx::{mysql::MySqlPoolOptions, Executor};
use std::ops::Deref;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    log4rs::init_file("log4rs.yml", Default::default()).expect("cannot initialize logging");

    std::panic::set_hook(Box::new(|panic_info| {
        let (filename, line) = panic_info
            .location()
            .map(|loc| (loc.file(), loc.line()))
            .unwrap_or(("<unknown>", 0));

        let cause = panic_info
            .payload()
            .downcast_ref::<String>()
            .map(String::deref);
        let cause = cause.unwrap_or_else(|| {
            panic_info
                .payload()
                .downcast_ref::<&str>()
                .copied()
                .unwrap_or("<cause unknown>")
        });

        error!(
            "A panic occurred at {}:{}: {}\n{:?}",
            filename,
            line,
            cause,
            Backtrace::new()
        );
    }));

    let config = Config::default();

    info!(
        "Starting {} v{}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION")
    );

    info!("Base addr: {}", config.api_base_addr);

    let db_connection_pool = MySqlPoolOptions::new()
        .after_connect(|conn, _meta| {
            Box::pin(async move {
                // Force local timezone on DB connection, because sqlx sets it explicitly to +00:00 for each connection
                // https://github.com/launchbadge/sqlx/issues/329
                conn.execute("SET time_zone = SYSTEM")
                    .await
                    .expect("Failed setting SYSTEM timezone for DB connection");

                Ok(())
            })
        })
        .connect(&config.database_url)
        .await
        .expect("Failed connecting to DB");
    let redis_connection_pool = deadpool_redis::Config::from_url(&config.redis_url)
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .expect("Failed connecting to Redis");

    // Run sqlx migrations on target DB
    sqlx::migrate!()
        .run(&db_connection_pool)
        .await
        .expect("Failed performing DB migration");

    run(
        &config.api_base_addr,
        db_connection_pool,
        redis_connection_pool,
    )?
    .await
}
