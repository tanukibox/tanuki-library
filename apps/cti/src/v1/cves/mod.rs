use actix_web::web::{self, ServiceConfig};
use cti::cves::domain::repositories::cve_repository::CveRepository;
use events::domain::event_bus::EventBus;

pub mod cve_post_controller;

pub fn router<R: CveRepository, E: EventBus>(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/v1/cves")
            //.configure(|cfg: &mut ServiceConfig| -> () { cfg.route("/{cve_id}", web::get().to(cve_get_controller::controller::<R>)); })
            .configure(|cfg: &mut ServiceConfig| -> () { cfg.route("/", web::post().to(cve_post_controller::controller::<R, E>)); })
    );
}