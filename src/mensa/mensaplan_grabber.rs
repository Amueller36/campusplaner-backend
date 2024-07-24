use crate::mensa_menu::{Location, WeekMenu, DatesMenu,MenuItem};
use super::mensa_dtos::{MenuDto, Date, Item};
use super::error::MensaPlanError;

use std::error::Error;
use log::{info, error};
use serde_xml_rs;
use std::sync::OnceLock;




impl From<MenuDto> for WeekMenu {
    fn from(menue: MenuDto) -> Self {
        WeekMenu {
            location: match menue.location.as_str() {
                "Mensa Suedstadt" => Location::MensaSuedstadt as i32 ,
                "Mensa Claudiusstraße" => Location::MensaClaudiusstrasse as i32,
                "Bistro Deutz" => Location::BistroDeutz as i32,
                "Mensa Deutz" => Location::MensaDeutz as i32,
                "Mensa Zülpicher Straße" => Location::MensaZuelpicherStrasse as i32,
                "Mensa Gummersbach" => Location::MensaGummersbach as i32,
                _ => Location::MensaGummersbach as i32,
            },
            dates_menu: menue.dates.into_iter().map(DatesMenu::from).collect(),
        }
    }
}



impl From<Date> for DatesMenu {
    fn from(date: Date) -> Self {
        DatesMenu {
            timestamp: date.timestamp,
            menu_items: date.items.into_iter().map(MenuItem::try_from).filter_map(Result::ok).collect(),
        }
    }
}

impl TryFrom<Item> for MenuItem {

    type Error = Box<dyn Error>;

    fn try_from(item: Item) -> Result<Self, Self::Error> {
        Ok (MenuItem {
            category: item.category,
            meal: item.meal,
            description: item.description,
            foodicons: item.foodicons,
            price_students: Item::parse_float(&item.price_students)?,
            price_employees: Item::parse_float(&item.price_employees)?,
            price_internal_guests: Item::parse_float(&item.price_internal_guests)?,
            price_external_guests: Item::parse_float(&item.price_external_guests)?,
            weight_unit: item.weight_unit.unwrap_or_default(),
            foto: item.foto.unwrap_or_default(),
            uhrzeit: item.uhrzeit.unwrap_or_default(),
        })
    }
}

#[derive(Debug)]
pub struct MensaPlanGrabber {
    http_client : reqwest::Client,
    base_url : String,

}

impl MensaPlanGrabber {

    pub fn instance() -> &'static MensaPlanGrabber {
        static INSTANCE: OnceLock<MensaPlanGrabber> = OnceLock::new();
        INSTANCE.get_or_init(|| MensaPlanGrabber::new())
    }
    
    fn new() -> MensaPlanGrabber {
        MensaPlanGrabber {
            http_client : reqwest::Client::new(),
            base_url : "https://max-manager.de/daten-extern/sw-koeln/slsys-xml/extern/".to_string(),
        }
    }

    pub async fn get_mensa_plan(&self, mensa_location: Location) -> Result<WeekMenu, MensaPlanError> {
        let url = self.get_menu_url_by_location(mensa_location)?;
        let response = self.http_client.get(&url).send().await.map_err(MensaPlanError::from)?;
        let response_body = response.text().await.map_err(MensaPlanError::from)?;
        
        let weekly_menu: MenuDto = serde_xml_rs::from_str(&response_body).map_err(MensaPlanError::from)?;
        let week_menu = WeekMenu::from(weekly_menu);
        
        Ok(week_menu)
    }

    fn get_menu_url_by_location(&self, mensa_location: Location) -> Result<String, MensaPlanError> {
        let sub_url = match mensa_location {
            Location::BistroDeutz => "bistro-deutz",
            Location::MensaGummersbach => "mensa_gummersbach",
            Location::MensaDeutz => "mensa_deutz",
            Location::MensaClaudiusstrasse => "mensa_claudiusstrasse",
            Location::MensaSuedstadt => "mensa_suedstadt",
            Location::MensaZuelpicherStrasse => "mensa_zuelpicher_strasse",
        };
        return Ok(format!("{}{}.xml",self.base_url, sub_url));
    }
}