use crate::common::{LobbyId, TraderId, Order, RequestId};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TraderMessage {
    InitConnection(RequestId, LobbyId, TraderId),
    PlaceOrder(RequestId, Order),
}
