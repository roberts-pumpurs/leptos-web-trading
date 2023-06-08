use serde::{Deserialize, Serialize};

use crate::common::{Order, RequestId};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TraderMessage {
    PlaceOrder(RequestId, Order),
    // Persist connectivity
    TraderTime(chrono::DateTime<chrono::Utc>),
    TraderTimeAck(chrono::DateTime<chrono::Utc>),
}
