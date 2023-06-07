use std::sync::Arc;

use app::*;
use axum::extract::Extension;
use axum::routing::{any, get};
use axum::Router;
use fileserv::file_and_error_handler;
use leptos::leptos_server::server_fns_by_path;
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tracing_subscriber::prelude::*;

pub mod fileserv;

#[tokio::main]
async fn main() {
    init_tracing();

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;
    app::register_server_functions();

    println!("{:?}", server_fns_by_path());
    let (state, handle) = state::spawn_actix_rt();
    // build our application with a route
    let app = Router::new()
        .route("/ws/:id", get(live_connection::handler))
        .with_state(state)
        .route("/api/*fn_name", any(leptos_axum::handle_server_fns))
        .leptos_routes(leptos_options.clone(), routes, |cx| view! { cx, <App/> })
        .fallback(file_and_error_handler)
        .layer(Extension(Arc::new(leptos_options)));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    tracing::info!("listening on http://{}", &addr);

    let server = axum::Server::bind(&addr).serve(app.into_make_service());
    let _ = server.await;
    handle.join().unwrap().unwrap();
}

fn init_tracing() {
    // construct a subscriber that prints formatted traces to stdout
    // use that subscriber to process traces emitted after this point
    // TODO add openetelemtry support for tracing - https://tokio.rs/tokio/topics/tracing-next-steps
    println!("---- SERVER {:} ----", env!("CARGO_PKG_VERSION"));
    let env = tracing_subscriber::EnvFilter::new("DEBUG");

    let fmt_layer = tracing_subscriber::fmt::layer()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(false);
    tracing_subscriber::registry().with(fmt_layer).with(env).init();
}
