use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
/// A timetable is made of a set of TimetableStops and a potential Disruption
pub struct TimeTable {
    #[serde(rename = "station")]
    /// Station name
    pub station_name: String,
    /// EVA station number
    pub eva: Option<String>,
    #[serde(rename = "m", default)]
    pub disruptions: Vec<Message>,
    #[serde(rename = "s", default)]
    pub stops: Vec<TimetableStop>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
/// A stop is a part of a Timetable
pub struct TimetableStop {
    /// An id that uniquely identifies the stop. It consists of the following three elements separated by dashes
    /// * a 'daily trip id' that uniquely identifies a trip within one day. This id is typically reused on subsequent days. This could be negative.
    /// * a 6-digit date specifier (YYMMdd) that indicates the planned departure date of the trip from its start station.
    /// * an index that indicates the position of the stop within the trip (in rare cases, one trip may arrive multiple times at one station). Added trips get indices above 100.
    ///
    /// Example '-7874571842864554321-1403311221-11' would be used for a trip with daily trip id '-7874571842864554321' that starts on march the 31th 2014 and where the current station is the 11th stop.
    pub id: String,
    #[serde(rename = "eva")]
    /// The eva code of the station of this stop. Example '8000105' for Frankfurt(Main)Hbf
    pub eva_number: Option<String>,
    #[serde(rename = "dp")]
    pub departure: Option<ArrivalDeparture>,
    #[serde(rename = "ar")]
    pub arrival: Option<ArrivalDeparture>,
    #[serde(rename = "tl")]
    pub trip_label: Option<TripLabel>,
    #[serde(rename = "m")]
    pub messages: Option<Vec<Message>>,
    #[serde(rename = "conn", default)]
    pub connection: Vec<Connection>,
    #[serde(rename = "ref")]
    pub reference: Option<Reference>,
    #[serde(rename = "rtr")]
    /// Reference trip relation element
    pub reference_trip_relation: Option<ReferenceTripRelation>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
/// A reference trip relation holds how a reference trip is related to a stop, for instance the reference trip starts after the stop. Stop contains a collection of that type, only if reference trips are available
pub struct ReferenceTripRelation {
    #[serde(rename = "rt")]
    /// A reference trip is another real trip, but it doesn't have its own stops and events. It refers only to its ref-erenced regular trip. The reference trip collects mainly all different attributes of the referenced regular trip
    pub real_trip: RealTrip,
    #[serde(rename = "rts")]
    /// The reference trips relation to the stop, which contains it
    pub relation: ReplationType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ReplationType {
    #[serde(rename = "b")]
    /// The reference trip ends before that stop
    Before,
    #[serde(rename = "e")]
    /// The reference trips ends at that stop
    End,
    #[serde(rename = "c")]
    /// The stop is between reference trips start and end, in other words, the stop is contained within its travel path
    Between,
    #[serde(rename = "s")]
    /// The reference trip starts at that stop
    Start,
    #[serde(rename = "a")]
    /// The reference trip starts after that stop
    After,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct RealTrip {
    /// An id that uniquely identifies the reference trip. It consists of the following two elements separated by dashes:
    ///
    /// * A 'daily trip id' that uniquely identifies a reference trip within one day. This id is typically reused on subsequent days. This could be negative.
    /// * A 10-digit date specifier (YYMMddHHmm) that indicates the planned departure date of the referenced regular trip from its start station.
    ///
    /// Example: \
    /// '-7874571842864554321-1403311221' would be used for a trip with daily trip id '-7874571842864554321' that starts on march the 31th 2014.
    pub id: String,
    #[serde(rename = "c")]
    /// The cancellation flag. True means, the reference trip is cancelled
    pub cancelled: bool,
    #[serde(rename = "ea")]
    /// common data items that characterize a reference trip stop
    pub reference_trip_stop: ReferenceTripStopCharacterization,
    #[serde(rename = "rtl")]
    /// common data items that characterize a reference trip
    pub reference_trip: ReferenceTripCharacterization,
    pub sd: ReferenceTripStopCharacterization,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ReferenceTripStopCharacterization {
    /// The eva number of the correspondent stop of the regular trip
    pub eva: String,
    #[serde(rename = "i")]
    /// The index of the correspondent stop of the regular trip
    pub index: i32,
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "pt")]
    /// The planned time of the correspondent stop of the regular trip
    pub planned_time: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ReferenceTripCharacterization {
    #[serde(rename = "c")]
    /// Trip category, e.g. \"ICE\" or \"RE\"
    pub category: String,
    #[serde(rename = "n")]
    /// Trip/train number, e.g. \"4523\"
    pub number: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
/// A message that is associated with an event, a stop or a trip
pub struct Message {
    pub id: String,
    #[serde(rename = "c")]
    pub code: Option<i32>,
    #[serde(rename = "cat")]
    pub category: Option<String>,
    #[serde(rename = "del")]
    pub deleted: Option<i32>,
    #[serde(rename = "ec")]
    pub external_category: Option<String>,
    #[serde(rename = "elnk")]
    pub external_link: Option<String>,
    #[serde(rename = "ext")]
    pub external_text: Option<String>,
    #[serde(rename = "from")]
    /// The time, in ten digit 'YYMMddHHmm' format, e.g. '1404011437' for 14:37 on April the 1st of 2014.
    pub valid_from: Option<String>,
    #[serde(rename = "to")]
    /// The time, in ten digit 'YYMMddHHmm' format, e.g. '1404011437' for 14:37 on April the 1st of 2014.
    pub valid_to: Option<String>,
    #[serde(rename = "int")]
    pub internal_text: Option<String>,
    #[serde(rename = "o")]
    pub owner: Option<String>,
    #[serde(rename = "pr")]
    pub priority: Option<MessagePriority>,
    #[serde(rename = "t")]
    pub message_status: MessageStatus,
    pub trip_label: Option<Vec<TripLabel>>,
    #[serde(rename = "dm")]
    pub distributor_messages: Option<Vec<DistributorMessage>>,
    #[serde(rename = "ts")]
    /// The time, in ten digit 'YYMMddHHmm' format, e.g. '1404011437' for 14:37 on April the 1st of 2014.
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
/// An additional message to a given station-based disruption by a specific distributor.
pub struct DistributorMessage {
    #[serde(rename = "int")]
    pub internal_text: Option<String>,
    #[serde(rename = "n")]
    pub distributor_name: Option<String>,
    #[serde(rename = "t")]
    pub distributor_type: Option<DistributorType>,
    #[serde(rename = "ts")]
    /// The time, in ten digit 'YYMMddHHmm' format, e.g. '1404011437' for 14:37 on April the 1st of 2014.
    pub timestamp: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum MessagePriority {
    #[serde(rename = "1")]
    High,
    #[serde(rename = "2")]
    Medium,
    #[serde(rename = "3")]
    Low,
    #[serde(rename = "4")]
    Done,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum DistributorType {
    #[serde(rename = "s")]
    City,
    #[serde(rename = "r")]
    Region,
    #[serde(rename = "f")]
    LongDistance,
    #[serde(rename = "x")]
    Other,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum MessageStatus {
    #[serde(rename = "h")]
    /// A HIM message (generated through the Hafas Information Manager)
    HafasInformationManager,
    #[serde(rename = "q")]
    /// A message about a quality change
    QualityChange,
    #[serde(rename = "f")]
    ///  A free text message
    Free,
    #[serde(rename = "d")]
    /// A message about the cause of a delay
    CauseOfDelay,
    #[serde(rename = "i")]
    /// An IBIS message (generated from IRIS-AP)
    Ibis,
    #[serde(rename = "u")]
    /// An IBIS message (generated from IRIS-AP) not yet assigned to a train
    UnassignedIbis,
    #[serde(rename = "r")]
    /// A major disruption
    Disruption,
    #[serde(rename = "c")]
    /// A connection
    Connection,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
/// Contains common data items that characterize a Trip
pub struct TripLabel {
    #[serde(rename = "c")]
    /// Trip category, e.g. \"ICE\" or \"RE\"
    pub category: String,
    #[serde(rename = "f")]
    pub filter_flags: Option<String>,
    #[serde(rename = "n")]
    /// Trip/train number, e.g. \"4523\"
    pub train_number: String,
    #[serde(rename = "o")]
    /// Owner. A unique short-form and only intended to map a trip to specific evu.
    pub owner: String,
    #[serde(rename = "t")]
    pub trip_type: Option<TripType>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TripType {
    P,
    E,
    Z,
    S,
    H,
    N,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Reference {
    #[serde(rename = "rt", default)]
    /// The referred trips reference trip elements
    pub reffered_trips: Vec<TripLabel>,
    #[serde(rename = "tl")]
    pub trip_label: TripLabel,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
/// An event (arrival or departure) that is part of a stop
pub struct ArrivalDeparture {
    #[serde(rename = "cde")]
    pub changed_distant_endpoint: Option<String>,
    #[serde(rename = "clt")]
    /// Time when the cancellation of this stop was created. The time, in ten digit 'YYMMddHHmm' format, e.g. '1404011437' for 14:37 on April the 1st of 2014.
    pub cancellation_time: Option<String>,
    #[serde(rename = "cp")]
    pub changed_platform: Option<String>,
    #[serde(rename = "cpth")]
    pub changed_path: Option<String>,
    #[serde(rename = "cs")]
    pub real_event_status: Option<EventStatus>,
    #[serde(rename = "ct")]
    /// New estimated or actual departure or arrival time. The time, in ten digit 'YYMMddHHmm' format, e.g. '1404011437' for 14:37 on April the 1st of 2014
    pub changed_time: Option<String>,
    #[serde(rename = "dc")]
    pub distant_change: Option<i32>,
    #[serde(rename = "hi")]
    /// 1 if the event should not be shown on WBT because travellers are not supposed to enter or exit the train at this stop
    pub hidden: Option<u8>,
    #[serde(rename = "l")]
    /// The line indicator (e.g. \"3\" for an S-Bahn or \"45S\" for a bus)
    pub line_indicator: Option<String>,
    #[serde(default, rename = "m")]
    pub messages: Vec<Message>,
    #[serde(rename = "pde")]
    pub planned_distant_endpoint: Option<String>,
    #[serde(rename = "pp")]
    pub planned_platform: Option<String>,
    #[serde(rename = "ppth")]
    /// A sequence of station names separated by the pipe symbols ('|'). \
    /// E.g.: 'Mainz Hbf|Rüsselsheim|Frankfurt(M) Flughafen'. \
    /// For arrival, the path indicates the stations that come before the current station. The first element then is the trip's start station. \
    /// For departure, the path indicates the stations that come after the current station. The last element in the path then is the trip's destination station. \
    /// Note that the current station is never included in the path (neither for arrival nor for departure).\n
    pub planned_path: Option<String>,
    #[serde(rename = "ps")]
    pub planned_event_status: Option<EventStatus>,
    #[serde(rename = "pt")]
    /// Planned departure or arrival time. The time, in ten digit 'YYMMddHHmm' format, e.g. '1404011437' for 14:37 on April the 1st of 2014.
    pub planned_time: Option<String>,
    #[serde(rename = "tra")]
    /// Trip id of the next or previous train of a shared train. At the start stop this references the previous trip, at the last stop it references the next trip. E.g. '2016448009055686515-1403311438-1'
    pub transition: Option<String>,
    #[serde(rename = "wings")]
    /// A sequence of trip id separated by the pipe symbols ('|'). E.g. '-906407760000782942-1403311431'
    pub wings: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum EventStatus {
    #[serde(rename = "p")]
    /// The event was planned. This status is also used when the cancellation of an event has been revoked
    Planned,
    #[serde(rename = "c")]
    /// The event was cancelled (as changedstatus, can apply to planned and added stops)
    Cancelled,
    #[serde(rename = "a")]
    /// The event was added to the planned data (new stop)
    Added,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
/// Information about a connected train at a particular stop
pub struct Connection {
    #[serde(rename = "cs")]
    pub connection_status: ConnectionStatus,
    pub eva: Option<i64>,
    pub id: String,
    #[serde(rename = "ref")]
    pub reference: Option<Box<TimetableStop>>,
    pub s: Box<TimetableStop>,
    /// The time, in ten digit 'YYMMddHHmm' format, e.g. '1404011437' for 14:37 on April the 1st of 2014
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum ConnectionStatus {
    #[serde(rename = "w")]
    /// This (regular) connection is waiting
    Waiting,
    #[serde(rename = "n")]
    /// This (regular) connection CANNOT wait
    Transition,
    #[serde(rename = "a")]
    /// This is an alternative (unplanned) connection that has been introduced as a replacement for one regular connection that cannot wait.  \
    /// The connections \"tl\" (triplabel) attribute might in this case refer to the replaced connection (or more specifi-cally the trip from that connection). \
    /// Alternative connections are always waiting (they are re-moved otherwise)
    Alternative,
}
