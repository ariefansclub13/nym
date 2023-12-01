use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct NodeConfig {
    pub id: String,
    pub country: String,
}

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ConnectionState {
    Connected,
    #[default]
    Disconnected,
    Connecting,
    Disconnecting,
    Unknown,
}

#[derive(Default, Debug, Serialize, Deserialize, TS, Clone)]
#[ts(export)]
pub enum VpnMode {
    Mixnet,
    #[default]
    TwoHop,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TunnelConfig {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Default)]
pub struct AppState {
    pub state: ConnectionState,
    pub error: Option<String>,
    pub vpn_mode: VpnMode,
    pub entry_node: Option<NodeConfig>,
    pub exit_node: Option<NodeConfig>,
    pub tunnel: Option<TunnelConfig>,
    pub connection_start_time: Option<OffsetDateTime>,
}