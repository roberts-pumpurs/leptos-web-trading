use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::common::{Order, RequestId, Size, Tick};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ServerMessage {
    TraderTimeAck,
    ConnectionInfo(Latency),
    TickSetWhole(Vec<TickData>),
    TickUpdate(TickData),
    NewLatestMatch(TickData),
    OrderStateUpdate(TraderOrders),
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TraderOrders {
    pub unmatched_orders: HashMap<Tick, Order>,
    pub matched_orders: HashMap<Tick, Order>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub struct TickData {
    pub total_matched: Size,
    pub available_backs: Size,
    pub available_lays: Size,
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
