use axum::extract::{State, WebSocketUpgrade};
use axum::response::IntoResponse;
use state::WebAppState;

mod ws;

pub async fn handler(
    ws: WebSocketUpgrade,
    // Path(market_id): Path<u32>,
    State(state): State<WebAppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |ws| ws::handle_connection(state, ws, 1))
}
