pub mod v1;

use std::sync::{atomic::{AtomicU16, Ordering}, Arc};

use actix_web::{web::Data, App, HttpServer};
use cqrs::{domain::{command_bus::CommandBus, query_bus::QueryBus}, infrastructure::inmemory::{inmemory_command_bus::InMemoryCommandBus, inmemory_query_bus::InMemoryQueryBus}};
use cti::cves::{application::{create_one::{create_cve_command_handler::CreateCveCommandHandler, cve_creator::CveCreator}, delete_one::cve_deleter::CveDeleter, find_one::{cve_finder::CveFinder, find_cve_q_handler::FindCveQueryHandler}}, infrastructure::sqlx::sqlx_postgres_cve_repository::SqlxPostgresCveRepository};
use events::infrastructure::inmemory::inmemory_event_bus::InMemoryEventBus;
use tracing::{self as logger};
use v1::health::health_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_path("./apps/cti/.env").ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        //.pretty()
        .with_timer(tracing_subscriber::fmt::time::ChronoUtc::rfc3339())
        .init();

    let host = "0.0.0.0";
    let port = std::env::var("PORT").expect("PORT must be set");
    let address = format!("{}:{}", host, port);
    logger::info!("Starting server at http://{}:{}/.", host, port);

    let thread_counter = Arc::new(AtomicU16::new(0));

    let event_bus = InMemoryEventBus::new();
    let event_bus_ref = Arc::new(event_bus);

    let mut query_bus = InMemoryQueryBus::new();
    let mut command_bus = InMemoryCommandBus::new();

    let cve_repository = SqlxPostgresCveRepository::from_env().await;
    let cve_repository_ref = Arc::new(cve_repository);

    let cve_finder = CveFinder::new(cve_repository_ref.clone());
    let find_cve_q_handler = FindCveQueryHandler::new(cve_finder);
    let find_cve_q_handler_ref = Arc::new(find_cve_q_handler);
    query_bus.register(find_cve_q_handler_ref);

    let cve_creator = CveCreator::new(cve_repository_ref.clone(), event_bus_ref.clone());
    let create_cve_cmd_handler = CreateCveCommandHandler::new(cve_creator);
    let create_cve_cmd_handler_ref = Arc::new(create_cve_cmd_handler);
    command_bus.register(create_cve_cmd_handler_ref);

    let cve_deleter = CveDeleter::new(cve_repository_ref.clone(), event_bus_ref.clone());
    let cve_deleter_ref = Data::new(cve_deleter);

    let query_bus_ref: Data<Arc<dyn QueryBus>> = Data::new(Arc::new(query_bus));
    let command_bus_ref: Data<Arc<dyn CommandBus>> = Data::new(Arc::new(command_bus));
    
    HttpServer::new(move || {
        let thread_counter = thread_counter.fetch_add(1, Ordering::SeqCst);
        logger::info!("Thread {} started.", thread_counter);

        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(query_bus_ref.clone())
            .app_data(command_bus_ref.clone())
            .configure(health_controller::router)
    })
    .bind(&address)
    .unwrap_or_else(|err| {
        panic!("🔥🔥🔥 Can not bind to address: {} 🔥🔥🔥", err);
    })
    .run()
    .await
}