use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subcriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subcriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Address already in use");
    run(listener, connection_pool)?.await
}
