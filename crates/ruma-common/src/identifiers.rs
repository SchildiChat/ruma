//! Types for [Matrix](https://matrix.org/) identifiers for devices, events, keys, rooms, servers,
//! users and URIs.

// FIXME: Remove once lint doesn't trigger on std::convert::TryFrom in identifiers/macros.rs anymore
#![allow(unused_qualifications)]

#[doc(inline)]
pub use ruma_identifiers_validation::{
    error::{
        Error as IdParseError, MatrixIdError, MatrixToError, MatrixUriError, MxcUriError,
        VoipVersionIdError,
    },
    KeyName, ID_MAX_BYTES,
};
use serde::de::{self, Deserializer, Unexpected};

#[doc(inline)]
pub use self::{
    base64_public_key::{Base64PublicKey, OwnedBase64PublicKey},
    base64_public_key_or_device_id::{Base64PublicKeyOrDeviceId, OwnedBase64PublicKeyOrDeviceId},
    client_secret::{ClientSecret, OwnedClientSecret},
    crypto_algorithms::{
        DeviceKeyAlgorithm, EventEncryptionAlgorithm, KeyDerivationAlgorithm, OneTimeKeyAlgorithm,
        SigningKeyAlgorithm,
    },
    device_id::{DeviceId, OwnedDeviceId},
    event_id::{EventId, OwnedEventId},
    key_id::{
        AnyKeyName, CrossSigningKeyId, CrossSigningOrDeviceSigningKeyId, DeviceKeyId,
        DeviceSigningKeyId, KeyAlgorithm, KeyId, OneTimeKeyId, OwnedCrossSigningKeyId,
        OwnedCrossSigningOrDeviceSigningKeyId, OwnedDeviceKeyId, OwnedDeviceSigningKeyId,
        OwnedKeyId, OwnedOneTimeKeyId, OwnedServerSigningKeyId, OwnedSigningKeyId,
        ServerSigningKeyId, SigningKeyId,
    },
    matrix_uri::{MatrixToUri, MatrixUri},
    mxc_uri::{MxcUri, OwnedMxcUri},
    one_time_key_name::{OneTimeKeyName, OwnedOneTimeKeyName},
    room_alias_id::{OwnedRoomAliasId, RoomAliasId},
    room_id::{OwnedRoomId, RoomId},
    room_or_alias_id::{OwnedRoomOrAliasId, RoomOrAliasId},
    room_version_id::RoomVersionId,
    server_name::{OwnedServerName, ServerName},
    server_signing_key_version::{OwnedServerSigningKeyVersion, ServerSigningKeyVersion},
    session_id::{OwnedSessionId, SessionId},
    signatures::{
        CrossSigningOrDeviceSignatures, DeviceSignatures, EntitySignatures, ServerSignatures,
        Signatures,
    },
    space_child_order::{OwnedSpaceChildOrder, SpaceChildOrder},
    transaction_id::{OwnedTransactionId, TransactionId},
    user_id::{OwnedUserId, UserId},
    voip_id::{OwnedVoipId, VoipId},
    voip_version_id::VoipVersionId,
};

pub mod matrix_uri;
pub mod user_id;

mod base64_public_key;
mod base64_public_key_or_device_id;
mod client_secret;
mod crypto_algorithms;
mod device_id;
mod event_id;
mod key_id;
mod mxc_uri;
mod one_time_key_name;
mod room_alias_id;
mod room_id;
mod room_or_alias_id;
mod room_version_id;
mod server_name;
mod server_signing_key_version;
mod session_id;
mod signatures;
mod space_child_order;
mod transaction_id;
mod voip_id;
mod voip_version_id;

/// Generates a random identifier localpart.
#[cfg(feature = "rand")]
fn generate_localpart(length: usize) -> Box<str> {
    use rand::Rng as _;
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .map(char::from)
        .take(length)
        .collect::<String>()
        .into_boxed_str()
}

/// Deserializes any type of id using the provided `TryFrom` implementation.
///
/// This is a helper function to reduce the boilerplate of the `Deserialize` implementations.
fn deserialize_id<'de, D, T>(deserializer: D, expected_str: &str) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: for<'a> TryFrom<&'a str>,
{
    crate::serde::deserialize_cow_str(deserializer).and_then(|v| {
        T::try_from(&v).map_err(|_| de::Error::invalid_value(Unexpected::Str(&v), &expected_str))
    })
}

/// Shorthand for `<&DeviceId>::from`.
#[macro_export]
macro_rules! device_id {
    ($s:expr) => {
        <&$crate::DeviceId as ::std::convert::From<_>>::from($s)
    };
}

/// Shorthand for `OwnedDeviceId::from`.
#[macro_export]
macro_rules! owned_device_id {
    ($s:expr) => {
        <$crate::OwnedDeviceId as ::std::convert::From<_>>::from($s)
    };
}

#[doc(hidden)]
pub mod __private_macros {
    pub use ruma_macros::{
        base64_public_key, event_id, mxc_uri, room_alias_id, room_id, room_version_id, server_name,
        server_signing_key_version, user_id,
    };
}

/// Compile-time checked [`EventId`] construction.
#[macro_export]
macro_rules! event_id {
    ($s:literal) => {
        $crate::__private_macros::event_id!($crate, $s)
    };
}

/// Compile-time checked [`OwnedEventId`] construction.
#[macro_export]
macro_rules! owned_event_id {
    ($s:literal) => {
        $crate::event_id!($s).to_owned()
    };
}

/// Compile-time checked [`RoomAliasId`] construction.
#[macro_export]
macro_rules! room_alias_id {
    ($s:literal) => {
        $crate::__private_macros::room_alias_id!($crate, $s)
    };
}

/// Compile-time checked [`OwnedRoomAliasId`] construction.
#[macro_export]
macro_rules! owned_room_alias_id {
    ($s:literal) => {
        $crate::room_alias_id!($s).to_owned()
    };
}

/// Compile-time checked [`RoomId`] construction.
#[macro_export]
macro_rules! room_id {
    ($s:literal) => {
        $crate::__private_macros::room_id!($crate, $s)
    };
}

/// Compile-time checked [`OwnedRoomId`] construction.
#[macro_export]
macro_rules! owned_room_id {
    ($s:literal) => {
        $crate::room_id!($s).to_owned()
    };
}

/// Compile-time checked [`RoomVersionId`] construction.
#[macro_export]
macro_rules! room_version_id {
    ($s:literal) => {
        $crate::__private_macros::room_version_id!($crate, $s)
    };
}

/// Compile-time checked [`ServerSigningKeyVersion`] construction.
#[macro_export]
macro_rules! server_signing_key_version {
    ($s:literal) => {
        $crate::__private_macros::server_signing_key_version!($crate, $s)
    };
}

/// Compile-time checked [`OwnedServerSigningKeyVersion`] construction.
#[macro_export]
macro_rules! owned_server_signing_key_version {
    ($s:literal) => {
        $crate::server_signing_key_version!($s).to_owned()
    };
}

/// Compile-time checked [`ServerName`] construction.
#[macro_export]
macro_rules! server_name {
    ($s:literal) => {
        $crate::__private_macros::server_name!($crate, $s)
    };
}

/// Compile-time checked [`OwnedServerName`] construction.
#[macro_export]
macro_rules! owned_server_name {
    ($s:literal) => {
        $crate::server_name!($s).to_owned()
    };
}

/// Compile-time checked [`SessionId`] construction.
#[macro_export]
macro_rules! session_id {
    ($s:literal) => {{
        const SESSION_ID: &$crate::SessionId = match $crate::SessionId::_priv_const_new($s) {
            Ok(id) => id,
            Err(e) => panic!("{}", e),
        };

        SESSION_ID
    }};
}

/// Compile-time checked [`OwnedSessionId`] construction.
#[macro_export]
macro_rules! owned_session_id {
    ($s:literal) => {
        $crate::session_id!($s).to_owned()
    };
}

/// Compile-time checked [`MxcUri`] construction.
#[macro_export]
macro_rules! mxc_uri {
    ($s:literal) => {
        $crate::__private_macros::mxc_uri!($crate, $s)
    };
}

/// Compile-time checked [`OwnedMxcUri`] construction.
#[macro_export]
macro_rules! owned_mxc_uri {
    ($s:literal) => {
        $crate::mxc_uri!($s).to_owned()
    };
}

/// Compile-time checked [`UserId`] construction.
#[macro_export]
macro_rules! user_id {
    ($s:literal) => {
        $crate::__private_macros::user_id!($crate, $s)
    };
}

/// Compile-time checked [`OwnedUserId`] construction.
#[macro_export]
macro_rules! owned_user_id {
    ($s:literal) => {
        $crate::user_id!($s).to_owned()
    };
}

/// Compile-time checked [`Base64PublicKey`] construction.
#[macro_export]
macro_rules! base64_public_key {
    ($s:literal) => {
        $crate::__private_macros::base64_public_key!($crate, $s)
    };
}

/// Compile-time checked [`OwnedBase64PublicKey`] construction.
#[macro_export]
macro_rules! owned_base64_public_key {
    ($s:literal) => {
        $crate::base64_public_key!($s).to_owned()
    };
}
