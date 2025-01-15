pub mod v1;

use std::sync::{
    atomic::{AtomicU16, Ordering},
    Arc,
};

use actix_web::{web::Data, App, HttpServer};
use cqrs::{
    domain::{command_bus::CommandBus, query_bus::QueryBus},
    infrastructure::inmemory::{
        inmemory_command_bus::InMemoryCommandBus, inmemory_query_bus::InMemoryQueryBus,
    },
};
use cti::{breaches::{application::{create_one::{breach_creator::BreachCreator, create_breach_command_handler::CreateBreachCommandHandler}, find_one::{breach_finder::BreachFinder, find_breach_q_handler::FindBreachQueryHandler}}, infrastructure::sqlx::sqlx_postgres_breach_repository::SqlxPostgresBreachRepository}, cves::{
    application::{
        create_one::{
            create_cve_command_handler::CreateCveCommandHandler, cve_creator::CveCreator,
        },
        find_one::{cve_finder::CveFinder, find_cve_q_handler::FindCveQueryHandler},
        update_one::{
            cve_updater::CveUpdater, update_cve_command_handler::UpdateCveCommandHandler,
        },
    },
    infrastructure::sqlx::sqlx_postgres_cve_repository::SqlxPostgresCveRepository,
}};
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

    let query_bus = InMemoryQueryBus::new();
    let query_bus_ref = Arc::new(query_bus);
    let command_bus = InMemoryCommandBus::new();
    let command_bus_ref = Arc::new(command_bus);

    let cve_repository = SqlxPostgresCveRepository::from_env().await;
    let cve_repository_ref = Arc::new(cve_repository);

    let breach_repository = SqlxPostgresBreachRepository::from_env().await;
    let breach_repository_ref = Arc::new(breach_repository);

    let cve_finder = CveFinder::new(cve_repository_ref.clone());
    let find_cve_q_handler = FindCveQueryHandler::new(cve_finder);
    let find_cve_q_handler_ref = Arc::new(find_cve_q_handler);
    query_bus_ref.register(find_cve_q_handler_ref).await;

    let cve_creator = CveCreator::new(cve_repository_ref.clone(), event_bus_ref.clone());
    let create_cve_cmd_handler = CreateCveCommandHandler::new(cve_creator);
    let create_cve_cmd_handler_ref = Arc::new(create_cve_cmd_handler);
    command_bus_ref.register(create_cve_cmd_handler_ref).await;

    let cve_updater = CveUpdater::new(cve_repository_ref.clone(), event_bus_ref.clone());
    let update_cve_cmd_handler = UpdateCveCommandHandler::new(cve_updater);
    let update_cve_cmd_handler_ref = Arc::new(update_cve_cmd_handler);
    command_bus_ref.register(update_cve_cmd_handler_ref).await;

    let breach_finder = BreachFinder::new(breach_repository_ref.clone());
    let find_breach_q_handler = FindBreachQueryHandler::new(breach_finder);
    let find_breach_q_handler_ref = Arc::new(find_breach_q_handler);
    query_bus_ref.register(find_breach_q_handler_ref).await;

    let breach_creator = BreachCreator::new(breach_repository_ref.clone(), query_bus_ref.clone(), event_bus_ref.clone());
    let create_breach_cmd_handler = CreateBreachCommandHandler::new(breach_creator);
    let create_breach_cmd_handler_ref = Arc::new(create_breach_cmd_handler);
    command_bus_ref.register(create_breach_cmd_handler_ref).await;

    let query_bus_ref: Data<Arc<dyn QueryBus>> = Data::new(query_bus_ref);
    let command_bus_ref: Data<Arc<dyn CommandBus>> = Data::new(command_bus_ref);

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
            .configure(v1::cves::router::<SqlxPostgresCveRepository, InMemoryEventBus>)
    })
    .bind(&address)
    .unwrap_or_else(|err| {
        panic!("🔥🔥🔥 Can not bind to address: {} 🔥🔥🔥", err);
    })
    .run()
    .await
}
