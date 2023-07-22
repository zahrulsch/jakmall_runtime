#[macro_use]
extern crate log;

use anyhow::Result;
use log::LevelFilter;
use runtime::{ws_pool::WsPool, Runtime};
use warp::{hyper::Response, ws::Ws, Filter};
use zilog::Zilog;

mod prerun;
mod runtime;
mod utils;

#[tokio::main]
async fn main() -> Result<()> {
    let args = prerun::run().await?;

    Zilog::builder()
        .set_app_name("JAKMALL:")
        .set_file_log_path("process.log")
        .init(LevelFilter::Info)
        .unwrap();

    let ws_pool_instance = WsPool::new();
    let runtime = Runtime::new(&ws_pool_instance, &args);

    let runtime_service = warp::any().map(move || runtime.clone());

    let ws = runtime_service
        .clone()
        .and(warp::path("ws").and(warp::ws()))
        .map(|rt: Runtime, ws: Ws| {
            ws.on_upgrade(|socket| async move { rt.ws_pool.insert_from_socket(socket).await })
        });

    let get_categories = runtime_service
        .clone()
        .and(warp::get().and(warp::path("get-categories")))
        .then(|mut rt: Runtime| async move {
            let rt = rt.run_get_categories().await;

            let mut res = Response::builder().header("Content-Type", "application/json");

            let response = match &rt {
                Ok(res) => serde_json::json!(res).to_string(),
                Err(e) => {
                    res = res.status(403);
                    serde_json::json!(e).to_string()
                }
            };

            res.body(response)
        });

    let all_services = ws.or(get_categories);

    warp::serve(all_services).run(([127, 0, 0, 1], 8000)).await;
    Ok(())
}
