///! Types for the [`de.spiritcroc.space.catch_all`] event.

use ruma_macros::EventContent;
use serde::{Deserialize, Serialize};

/// The content of an `de.spiritcroc.space.catch_all` event.
#[derive(Clone, Debug, Default, Deserialize, Serialize, EventContent)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
#[ruma_event(type = "de.spiritcroc.space.catch_all", kind = State, state_key_type = String)]
pub struct SpaceCatchAllEventContent {
    /// Wheter to include all rooms that are not explicitly part of any space
    /// as part of this space. Required to be true for this state event to have
    /// any effect on space children.
    pub include_orphans: bool,

    /// Wheter to filter for DMs or group chats or allow both (when omitted).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_is_dm: Option<bool>,
}
