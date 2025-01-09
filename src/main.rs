use api::get_routes;
use common::logs::init_logs;
use common::server::init_server;

#[tokio::main]
async fn main() {
    let _guards = init_logs();
    init_server(get_routes()).await
}
