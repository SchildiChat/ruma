///! Types for the [`m.bridge`] event.

use ruma_common::{OwnedUserId, OwnedMxcUri};
use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

/// The content of an `m.bridge` event.
#[derive(Clone, Debug, Default, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
#[ruma_event(type = "m.bridge", kind = State, state_key_type = String)]
pub struct BridgeEventContent {
    /// The user ID of the bridge bot in this room.
    #[serde(rename = "bridgebot", skip_serializing_if = "Option::is_none")]
    pub bridge_bot: Option<OwnedUserId>,

    /// Remote network protocol information for the bridge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<ProtocolInfo>,
}

impl BridgeEventContent {
    /// Create an empty `BridgeEventContent`.
    pub fn new() -> Self {
        Self::default()
    }
}

/// Protocol info for [`m.bridge`] events.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct ProtocolInfo {
    /// Protocol avatar url.
    pub avatar_url: Option<OwnedMxcUri>,
    /// Protocol name.
    pub displayname: Option<String>,
    /// Protocol ID.
    pub id: Option<String>,
}

impl ProtocolInfo {
    /// Create an empty `ProtocolInfo`.
    pub fn new() -> Self {
        Self::default()
    }
}
