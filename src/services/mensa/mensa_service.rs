
use crate::mensa::mensaplan_grabber::MensaPlanGrabber;
use crate::mensa_menu::{menu_service_server::MenuService, MenuRequest, MenuResponse, Location};

use log::info;
use tonic::{Request, Response, Status};

#[derive(Debug, Clone)]
pub struct MensaDataService{
    mensaplan_grabber : &'static MensaPlanGrabber,
}

impl MensaDataService {
    pub fn new(mensaplan_grabber : &'static MensaPlanGrabber) -> Self {
        Self {
            mensaplan_grabber: mensaplan_grabber,
        }
    }
}

#[tonic::async_trait]
impl MenuService for MensaDataService {

    async fn get_menu(&self, request: Request<MenuRequest>) -> Result<Response<MenuResponse>, Status> {
        info!("Got a request: {:?}", request);
        let location: i32 = request.into_inner().location;
        let location : Location = match Location::try_from(location) {
            Ok(loc) => loc,
            Err(_) => return Err(Status::invalid_argument("Invalid location")),
        };

        let menu = match self.mensaplan_grabber.get_mensa_plan(location).await {
            Ok(menu) => menu,
            Err(err) => return Err(Status::unavailable(err.to_string())),
        };

        let menu_proto_response = MenuResponse {
            menu: Some(menu.into()),
        };
        Ok(Response::new(menu_proto_response))
    }

}
