use axum::{
    routing::{delete, get, post, put},
    Router,
};
use rustic_server::{
    routes::{charge_code_routes::*, time_entry_routes::*},
    utils,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    env_logger::init();
    let pool = utils::connections::get_connection().await;

    let app = Router::new()
        .route("/full_state", get(get_everything_request))
        .route("/time_entries/day/:day", post(create_time_entry_request))
        .route(
            "/time_entries/:id/charge_code/:code_id",
            put(update_time_entry_charge_code_request),
        )
        .route(
            "/time_entries/:id/time/:total_time",
            put(update_time_entry_time_request),
        )
        .route(
            "/time_entries/:id/note",
            put(update_time_entry_note_request),
        )
        .route("/time_entries/:id/play", put(play_time_entry_request))
        .route("/time_entries/:id/pause", put(pause_time_entry_request))
        .route("/time_entries/:id", delete(delete_time_entry_request))
        .route("/charge_codes", get(get_charge_codes))
        .layer(axum::extract::Extension(pool));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
