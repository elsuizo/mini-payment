use actix_web::HttpResponse;
// NOTE(elsuizo: 2024-04-12): endpoint que no hace nada practicamente
pub async fn client_creation() -> HttpResponse {
    HttpResponse::Ok().finish()
}
