use std::sync::Arc;

use actix_web::{web, HttpRequest, HttpResponse};
use cqrs::domain::query_bus::QueryBus;
use cti::{cves::{application::{cve_query_response::CveQueryResponse, find_one::find_cve_query::FindCveQuery}, domain::repositories::cve_repository::CveRepository, infrastructure::dtos::cve_json_dto::parse_to_dto}, shared::domain::errors::DomainError};
use tracing::{self as logger};

pub async fn controller<R: CveRepository>(
    cve_id: web::Path<String>,
    _: HttpRequest,
    query_bus: web::Data<Arc<dyn QueryBus>>,
) -> HttpResponse {
    logger::debug!("GET /api/v1/cves/{} called.", cve_id);
    let q = FindCveQuery::new_boxed(
        Some(cve_id.clone()),
    );

    let res = query_bus.ask(q).await;
    let res = match res.as_any().downcast_ref::<CveQueryResponse>() {
        Some(res) => res,
        None => return HttpResponse::InternalServerError().finish(),
    };

    if res.is_ok() {
        let cve= res.cve.as_ref().unwrap();
        let json_cve = parse_to_dto(&cve);
        return HttpResponse::Ok().json(json_cve);
    }

    match &res.error {
        None => HttpResponse::InternalServerError().finish(),
        Some(err) => {
            match err {
                DomainError::CveNotFound { id } => {
                    HttpResponse::NotFound().body(format!("CVE with id <{}>, not found..", id))
                },
                _ => HttpResponse::InternalServerError().finish()
            }
        }
    }
}
