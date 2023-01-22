#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    use axum::{
        extract::Extension,
        routing::{get, post},
        Router,
    };
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_start::{app::*, file::file_handler};
    use std::sync::Arc;

    let conf = get_configuration(None).await.unwrap();
    let addr = conf.leptos_options.site_address;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    let leptos_options = conf.leptos_options;

    // build our application with a route
    let app = Router::new()
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .route("/favicon.ico", get(file_handler))
        .leptos_routes(leptos_options.clone(), routes, |cx| view! { cx, <App/> })
        .fallback(file_handler)
        .layer(Extension(Arc::new(leptos_options)));
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    log!("listening on {}", &addr);
    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
