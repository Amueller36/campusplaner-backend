use super::mensa::mensa_service::MensaDataService;

use crate::{mensa::mensaplan_grabber::MensaPlanGrabber, mensa_menu::menu_service_server::MenuServiceServer};

pub struct ServiceProvider {
    mensa_plan_service: MensaDataService,
}

impl ServiceProvider {
    pub fn new() -> Self {
        ServiceProvider {
            mensa_plan_service: MensaDataService::new(&MensaPlanGrabber::instance()),
        }
    }

    pub fn get_menu_service_server(&mut self) -> MenuServiceServer<MensaDataService> {
        MenuServiceServer::new(self.mensa_plan_service.clone())
    }
}