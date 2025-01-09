use configs::CFG;
use silent::prelude::{Route, Server};
use std::path::Path;
use tracing::info;

pub async fn init_route(route: Route) -> Route {
    let root_route = Route::new("");
    if !Path::new(CFG.web.upload_dir.clone().as_str()).exists() {
        std::fs::create_dir_all(CFG.web.upload_dir.clone())
            .expect("Failed to create upload directory");
        info!("创建上传目录: {}", CFG.web.upload_dir.clone().as_str());
    }
    root_route
        .append(Route::new(CFG.server.api_prefix.clone().as_str()).append(route))
        .with_static(CFG.web.dir.clone().as_str())
        .with_static_in_url(
            CFG.web.upload_url.clone().as_str(),
            CFG.web.upload_dir.clone().as_str(),
        )
}

pub async fn init_server(route: Route) {
    let route = init_route(route).await;
    Server::new()
        .bind(CFG.server.address.parse().expect("Invalid address"))
        .serve(route)
        .await;
}
