mod server_management;

use self::{
    proto::server_management_server::ServerManagementServer,
    server_management::ServerManagementController,
};
use crate::daemon::Daemon;
use std::sync::Arc;
use tonic::transport::Server;

mod proto {
    tonic::include_proto!("db_cluster.proto.daemon");
}

pub async fn start_grpc_server(daemon: Arc<Daemon>) {
    let addr = "[::1]:7888".parse().unwrap();
    let server_management = ServerManagementServer::new(ServerManagementController::new(daemon));
    let server = Server::builder()
        .add_service(server_management)
        .serve(addr)
        .await;
    if let Err(err) = server {
        panic!("Error during start of db-cluster-daemon: {}", err);
    }
}
