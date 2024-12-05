use actix_web::{web, HttpRequest, HttpResponse};
use cqrs::domain::command_bus::CommandBus;
use cti::{cves::{application::{create_one::create_cve_command::CreateCveCommand, cve_command_response::CveCommandResponse}, domain::repositories::cve_repository::CveRepository, infrastructure::dtos::cve_json_dto::{parse_to_domain, CveJsonDto}}, shared::domain::errors::DomainError};
use events::domain::event_bus::EventBus;

pub async fn controller<R: CveRepository, E: EventBus>(
    dto: web::Json<CveJsonDto>,
    _: HttpRequest,
    command_bus: web::Data<dyn CommandBus>,
) -> HttpResponse {
    let cmd = CreateCveCommand::new_boxed(
        dto.id.clone(), 
        dto.state.clone(), 
        dto.date_published.clone(), 
        dto.description.clone()
    );

    let res = command_bus.dispatch(cmd).await;
    let res = match res.as_any().downcast_ref::<CveCommandResponse>() {
        Some(res) => res,
        None => return HttpResponse::InternalServerError().finish(),
    };

    if res.is_ok() {
        return HttpResponse::Accepted().finish()
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
