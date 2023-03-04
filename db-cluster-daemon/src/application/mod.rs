use tonic::transport::Server;

use crate::AppSender;

use self::{
    proto::server_management_server::ServerManagementServer,
    server_management::ServerManagementController,
};

pub mod server_management;

mod proto {
    tonic::include_proto!("db_cluster.proto.daemon");
}

pub async fn start_grpc_server(sender: AppSender) {
    let addr = "[::1]:7888".parse().unwrap();
    let server_management = ServerManagementServer::new(ServerManagementController::new(sender));
    let server = Server::builder()
        .add_service(server_management)
        .serve(addr)
        .await;
    if let Err(err) = server {
        panic!("Error during start of db-cluster-daemon: {}", err);
    }
}
