//! Types for per-message profiles from MSC4144.

use serde::{Deserialize, Serialize};

use crate::room::EncryptedFile;

/// A profile to use for an individual message instead of the sender's room profile.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[cfg_attr(not(ruma_unstable_exhaustive_types), non_exhaustive)]
pub struct PerMessageProfile {
    /// An opaque identifier for this profile, scoped to the sending Matrix user.
    pub id: String,

    /// The display name to use for this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub displayname: Option<String>,

    /// The avatar URL to use for this message.
    ///
    /// An empty string explicitly clears the sender's room avatar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    /// The encrypted avatar to use for this message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_file: Option<Box<EncryptedFile>>,

    /// Whether the message body includes a fallback for clients that do not support profiles.
    #[serde(default, skip_serializing_if = "ruma_common::serde::is_default")]
    pub has_fallback: bool,
}

impl PerMessageProfile {
    /// Creates a per-message profile with the given opaque identifier.
    pub fn new(id: String) -> Self {
        Self {
            id,
            displayname: None,
            avatar_url: None,
            avatar_file: None,
            has_fallback: false,
        }
    }
}
