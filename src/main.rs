use tonic::service;
use tonic::{transport::Server, Request, Response, Status};
use mensa_menu::{Location, MenuRequest, MenuResponse};
use mensa_menu::menu_service_server::{MenuService, MenuServiceServer};
use log::info;

mod mensa;
mod services;
use services::service_provider::ServiceProvider;

pub mod mensa_menu {
    tonic::include_proto!("mensa_menu");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "INFO");
    env_logger::init();
    
    let addr = "[::1]:8081".parse()?;
    info!("Server listening on {}", addr);
    
    let mut service_provider = ServiceProvider::new();

    Server::builder()
        .add_service(service_provider.get_menu_service_server())
        .serve(addr)
        .await?;

    Ok(())
}