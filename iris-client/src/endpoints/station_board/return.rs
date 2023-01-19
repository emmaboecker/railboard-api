use std::collections::HashSet;

use chrono::{DateTime, FixedOffset, Offset};
use serde::{Deserialize, Serialize};

pub mod message;
pub mod stop;

pub use message::*;
pub use stop::*;

use crate::helpers::parse_iris_date;

use super::response::{EventStatus, TimetableStop};

use wu_diff::*;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IrisStationBoard {
    pub station_name: String,
    pub station_eva: String,
    pub disruptions: Vec<Message>,
    pub stops: Vec<StationBoardStop>,
}

pub fn from_iris_timetable(
    station_eva: &str,
    station_name: &str,
    stop: TimetableStop,
    realtime: Option<TimetableStop>,
) -> StationBoardStop {
    let mut messages: HashSet<Message> = HashSet::new();

    if let Some(realtime) = &realtime {
        if let Some(msgs) = &realtime.messages {
            for message in msgs {
                messages.insert(message.to_owned().into());
            }
            if let Some(departure) = &realtime.departure {
                for message in &departure.messages {
                    messages.insert(message.to_owned().into());
                }
            }
            if let Some(arrival) = &realtime.arrival {
                for message in &arrival.messages {
                    messages.insert(message.to_owned().into());
                }
            }
        }
    }

    let event_status = realtime
        .as_ref()
        .map(|stop| {
            let departure_arrival = stop.departure.as_ref().or_else(|| stop.arrival.as_ref());

            departure_arrival.map(|dep_arr| dep_arr.real_event_status.to_owned())
        })
        .flatten()
        .flatten()
        .or(stop
            .departure
            .as_ref()
            .map(|f| f.planned_event_status.to_owned())
            .flatten()
            .or(stop
                .arrival
                .as_ref()
                .map(|f| f.planned_event_status.to_owned())
                .flatten()));

    let hidden = realtime
        .as_ref()
        .map(|stop| {
            let departure_arrival = &stop
                .departure
                .as_ref()
                .or_else(|| stop.departure.as_ref().or_else(|| stop.arrival.as_ref()));

            departure_arrival.map(|dep_arr| dep_arr.hidden == Some(1))
        })
        .flatten()
        .or(stop
            .departure
            .as_ref()
            .map(|f| f.hidden == Some(1))
            .or(stop.arrival.as_ref().map(|f| f.hidden == Some(1))));

    let mut route = Vec::new();

    if let Some(stops) = stop
        .arrival
        .as_ref()
        .map(|arr| arr.planned_path.as_ref().unwrap())
    {
        let current_path: Option<Vec<&str>> = realtime
            .as_ref()
            .map(|realtime| {
                realtime
                    .arrival
                    .as_ref()
                    .map(|real_dep| real_dep.changed_path.as_ref())
            })
            .flatten()
            .flatten()
            .map(|path| path.split("|").collect());

        if let Some(current_path) = current_path {
            let old = stops.split("|").collect::<Vec<&str>>();

            for diff in wu_diff::diff(&old, &current_path) {
                match diff {
                    DiffResult::Common(same) => {
                        route.push(RouteStop {
                            name: current_path[same.new_index.unwrap()].to_string(),
                            cancelled: false,
                            added: false,
                        });
                    }
                    DiffResult::Added(add) => {
                        route.push(RouteStop {
                            name: current_path[add.new_index.unwrap()].to_string(),
                            cancelled: false,
                            added: true,
                        });
                    }
                    DiffResult::Removed(rem) => {
                        route.push(RouteStop {
                            name: old[rem.old_index.unwrap()].to_string(),
                            cancelled: true,
                            added: false,
                        });
                    }
                }
            }
        } else {
            for stop in stops.split("|") {
                route.push(RouteStop {
                    name: stop.to_string(),
                    cancelled: false,
                    added: false,
                });
            }
        }
    }

    let cancelled = event_status == Some(EventStatus::Cancelled);
    let added = event_status == Some(EventStatus::Added);

    route.push(RouteStop {
        name: String::from(station_name),
        cancelled: cancelled,
        added: added,
    });

    if let Some(stops) = stop
        .departure
        .as_ref()
        .map(|dep| dep.planned_path.as_ref().unwrap())
    {
        let current_path: Option<Vec<&str>> = realtime
            .as_ref()
            .map(|realtime| {
                realtime
                    .departure
                    .as_ref()
                    .map(|real_dep| real_dep.changed_path.as_ref())
            })
            .flatten()
            .flatten()
            .map(|path| path.split("|").collect());

        if let Some(current_path) = current_path {
            let old = stops.split("|").collect::<Vec<&str>>();

            for diff in wu_diff::diff(&old, &current_path) {
                match diff {
                    DiffResult::Common(same) => {
                        route.push(RouteStop {
                            name: current_path[same.new_index.unwrap()].to_string(),
                            cancelled: false,
                            added: false,
                        });
                    }
                    DiffResult::Added(add) => {
                        route.push(RouteStop {
                            name: current_path[add.new_index.unwrap()].to_string(),
                            cancelled: false,
                            added: true,
                        });
                    }
                    DiffResult::Removed(rem) => {
                        route.push(RouteStop {
                            name: old[rem.old_index.unwrap()].to_string(),
                            cancelled: true,
                            added: false,
                        });
                    }
                }
            }
        } else {
            for stop in stops.split("|") {
                route.push(RouteStop {
                    name: stop.to_string(),
                    cancelled: false,
                    added: false,
                });
            }
        }
    }

    StationBoardStop {
        id: stop.id,
        station_name: String::from(station_name),
        station_eva: String::from(station_eva),
        messages: messages.into_iter().collect(),
        cancelled: event_status == Some(EventStatus::Cancelled),
        added: event_status == Some(EventStatus::Added),
        hidden: hidden.unwrap_or(false),
        arrival: stop.arrival.as_ref().map(|arrival| {
            let plan_date = parse_iris_date(&arrival.planned_time.as_ref().unwrap()).unwrap();
            let plan_offset = plan_date.offset().fix();
            let real_date = realtime
                .as_ref()
                .map(|realtime| {
                    realtime
                        .arrival
                        .as_ref()
                        .map(|departure| departure.changed_time.as_ref())
                })
                .flatten()
                .flatten()
                .map(|time| parse_iris_date(&time).unwrap());
            let real_offset = real_date
                .as_ref()
                .map(|date| date.offset().fix())
                .unwrap_or_else(|| plan_offset);

            StationBoardStopArrival {
                planned_time: DateTime::<FixedOffset>::from_utc(plan_date.naive_utc(), plan_offset),
                real_time: real_date
                    .map(|date| DateTime::<FixedOffset>::from_utc(date.naive_utc(), real_offset)),
                wings: arrival
                    .wings
                    .as_ref()
                    .map(|wings| wings.split("|").map(|string| string.to_string()).collect())
                    .unwrap_or_default(),
                origin: route.first().unwrap().name.to_owned(),
            }
        }),
        departure: stop.departure.as_ref().map(|departure| {
            let plan_date = parse_iris_date(departure.planned_time.as_ref().unwrap()).unwrap();
            let plan_offset = plan_date.offset().fix();
            let real_date = realtime
                .as_ref()
                .map(|realtime| {
                    realtime
                        .departure
                        .as_ref()
                        .map(|departure| departure.changed_time.as_ref())
                })
                .flatten()
                .flatten()
                .map(|time| parse_iris_date(&time).unwrap());
            let real_offset = real_date
                .as_ref()
                .map(|date| date.offset().fix())
                .unwrap_or_else(|| plan_offset);

            StationBoardStopDeparture {
                planned_time: DateTime::<FixedOffset>::from_utc(plan_date.naive_utc(), plan_offset),
                real_time: real_date
                    .map(|date| DateTime::<FixedOffset>::from_utc(date.naive_utc(), real_offset)),
                wings: departure
                    .wings
                    .as_ref()
                    .map(|wings| wings.split("|").map(|string| string.to_string()).collect())
                    .unwrap_or_default(),
                direction: route.last().unwrap().name.to_owned(),
            }
        }),
        route: route,
        planned_platform: stop
            .departure
            .as_ref()
            .unwrap_or_else(|| stop.arrival.as_ref().unwrap())
            .planned_platform
            .to_owned(),
        real_platform: realtime
            .as_ref()
            .map(|realtime| {
                realtime
                    .departure
                    .as_ref()
                    .or_else(|| stop.arrival.as_ref())
                    .map(|dep_arr| dep_arr.planned_platform.to_owned())
            })
            .flatten()
            .flatten(),
        line_indicator: stop
            .departure
            .unwrap_or_else(|| stop.arrival.unwrap())
            .line_indicator
            .unwrap_or_else(|| stop.trip_label.as_ref().unwrap().train_number.to_owned()),
        train_type: stop
            .trip_label
            .as_ref()
            .map(|trip_label| trip_label.category.to_owned())
            .unwrap(),
        train_number: stop
            .trip_label
            .as_ref()
            .map(|trip_label| trip_label.train_number.to_owned())
            .unwrap()
            .to_owned(),
    }
}
