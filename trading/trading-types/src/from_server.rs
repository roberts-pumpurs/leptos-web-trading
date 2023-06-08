use serde::{Deserialize, Serialize};

use crate::common::{Order, RequestId, Size, Tick};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ServerMessage {
    TraderTimeAck,
    ConnectionInfo(Latency),
    TickSetWhole(Vec<TickData>),
    TickUpdate(TickData),
    OrderAccepted(RequestId),
    OrderRejected(RequestId, String),
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct TickData {
    pub back: Size,
    pub lay: Size,
    pub tick: Tick,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct Latency {
    pub ms: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct TraderInfo {
    pub balance: Size,
    pub exposure: Size,
    pub orders: Vec<Order>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct MatchInfo {
    pub total_matched: Size,
    pub total_available: Size,
    pub end_date: chrono::DateTime<chrono::Utc>,
}
