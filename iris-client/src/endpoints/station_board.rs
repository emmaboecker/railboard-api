mod response;
pub use response::*;
use serde::{Deserialize, Serialize};

use crate::{IrisClient, IrisError, IrisOrRequestError};

impl IrisClient {
    pub async fn realtime_station_board(
        &self,
        eva: String,
    ) -> Result<StationBoard, IrisOrRequestError> {
        let _permit = self.semaphore.acquire().await;

        let response = self
            .client
            .get(format!("{}/iris-tts/timetable/fchg/{}", self.base_url, eva))
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(IrisOrRequestError::IrisError(IrisError));
        }

        let response: String = response.text().await?;

        let response: TimeTable = serde_xml_rs::from_str(&response)?;

        Ok(response.into())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StationBoard {
    pub station_name: String,
    pub station_eva: String,
    pub items: Vec<StationBoardItem>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct StationBoardItem {
    pub id: String,
    pub eva: Option<String>,
    pub planned_stations: Option<Vec<String>>,
    pub real_stations: Option<Vec<String>>,
    pub cancelled: Option<bool>,
    pub messages: Vec<Message>,
    pub arrival: Option<ArrivalOrDeparture>,
    pub departure: Option<ArrivalOrDeparture>,
    pub planned_platform: Option<String>,
    pub real_platform: Option<String>,
    pub train_type: Option<String>,
    pub line_number: Option<String>,
    pub train_number: Option<String>,
    pub product_type: Option<String>,
    pub wings: Option<String>,
    pub replaces: Vec<Replaces>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Replaces {
    pub train_number: String,
    pub train_type: String,
    pub product_type: String,
    pub t: String,
    pub o: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Message {
    pub id: String,
    pub code: Option<String>,
    pub heading: Option<String>,
    pub priority: Option<String>,
    pub time: String,
    pub timestamp: String,
    pub from: Option<String>,
    pub to: Option<String>,
}

impl StationBoardItem {
    pub fn new(id: String, eva: Option<String>) -> Self {
        Self {
            id,
            eva,
            planned_stations: None,
            real_stations: None,
            cancelled: None,
            messages: Vec::new(),
            arrival: None,
            departure: None,
            planned_platform: None,
            real_platform: None,
            train_type: None,
            line_number: None,
            train_number: None,
            product_type: None,
            wings: None,
            replaces: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArrivalOrDeparture {
    planned_time: Option<String>,
    real_time: Option<String>,
    messages: Vec<Message>,
}

impl From<TimeTable> for StationBoard {
    fn from(timetable: TimeTable) -> Self {
        let station_name = timetable.station;
        let station_eva = timetable.eva;

        for _train in timetable.items {

        }

        Self {
            station_name,
            station_eva,
            items: Vec::new(),
        }
    }
}

// trait IntoStationBoardItem {
//     fn insert_data(&mut self, trains: Vec<TrainData>, id: String, eva: Option<String>);
// }

// impl IntoStationBoardItem for Vec<StationBoardItem> {
//     fn insert_data(&mut self, trains: Vec<TrainData>, id: String, eva: Option<String>) {
//         let mut item = StationBoardItem::new(id, eva);

//         for information in trains {
//             match information {
//                 TrainData::Message(message) => {
//                     item.messages.push(message.into());
//                 }
//                 TrainData::Arrival(arrival) => {
//                     item.arrival = Some(ArrivalOrDeparture {
//                         planned_time: arrival.planned_time,
//                         real_time: arrival.realtime_time,
//                         messages: arrival
//                             .messages
//                             .map(|messages| {
//                                 messages.into_iter().map(|message| message.into()).collect()
//                             })
//                             .unwrap_or_default(),
//                     });
//                     item.planned_platform = arrival.planned_platform;
//                     item.real_platform = arrival.realtime_platform;
//                     item.cancelled = arrival.cancelled.map(|cancelled| cancelled == "c");
//                     item.line_number = arrival.line_number;
//                     item.wings = arrival.wings;
//                 }
//                 TrainData::Departure(departure) => {
//                     item.departure = Some(ArrivalOrDeparture {
//                         planned_time: departure.planned_time,
//                         real_time: departure.realtime_time,
//                         messages: departure
//                             .messages
//                             .map(|messages| {
//                                 messages.into_iter().map(|message| message.into()).collect()
//                             })
//                             .unwrap_or_default(),
//                     });
//                     item.planned_platform = departure.planned_platform;
//                     item.real_platform = departure.realtime_platform;
//                     item.cancelled = departure.cancelled.map(|cancelled| cancelled == "c");
//                     item.line_number = departure.line_number;
//                     item.wings = departure.wings;
//                 }
//                 TrainData::TrainInformation(information) => {
//                     item.train_type = information.train_name;
//                     item.train_number = Some(information.train_number);
//                     item.product_type = information.product_type;
//                 }
//                 TrainData::Replaces(replaces) => item.replaces.push(Replaces {
//                     train_number: replaces.train_information.train_number,
//                     train_type: replaces.train_information.train_name.unwrap(),
//                     product_type: replaces.train_information.product_type.unwrap(),
//                     t: replaces.train_information.t,
//                     o: replaces.train_information.o,
//                 }),
//             }
//         }

//         self.push(item);
//     }
// }

// impl From<response::Message> for Message {
//     fn from(message: response::Message) -> Self {
//         Self {
//             id: message.id,
//             code: message.message_code,
//             heading: message.text,
//             priority: message.priority,
//             time: message.short_time,
//             timestamp: message.long_time,
//             from: message.from,
//             to: message.to,
//         }
//     }
// }
