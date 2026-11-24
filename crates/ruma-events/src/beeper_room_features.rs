//! Types for the `com.beeper.room_features` event.

use std::collections::BTreeMap;

use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The bridge capabilities advertised for a room.
#[derive(Clone, Debug, Default, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
#[ruma_event(type = "com.beeper.room_features", kind = State, state_key_type = String)]
pub struct BeeperRoomFeaturesEventContent {
    /// Capability fields as advertised by the bridge.
    #[serde(default, flatten)]
    pub features: BTreeMap<String, Value>,
}
