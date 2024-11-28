use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};
use tracing::{self as logger};

pub fn router(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(controller));
}

async fn controller() -> HttpResponse {
    logger::debug!("Health check endpoint called.");
    HttpResponse::Ok().finish()
}

#[cfg(test)]
mod unit_tests {
    use actix_web::http::StatusCode;

    use super::controller;

    #[actix_rt::test]
    async fn health_check_works() {
        let res = controller().await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);
    }
}

#[cfg(test)]
mod integration_tests {
    use actix_web::{
        http::StatusCode,
        test::{call_service, init_service, TestRequest},
        App,
    };

    #[actix_rt::test]
    async fn health_check_works() {
        let app = App::new().configure(super::router);
        let app = init_service(app).await;
        let req = TestRequest::get().uri("/health").to_request();
        let res = call_service(&app, req).await;
        assert!(res.status().is_success());
        assert_eq!(res.status(), StatusCode::OK);
    }
}
