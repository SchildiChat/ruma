use ruma_common::Outgoing;
use ruma_identifiers::UserId;

#[allow(unused)]
pub struct Thing<'t, T> {
    some: &'t str,
    t: &'t T,
}

#[derive(Debug)]
pub struct IncomingThing<T> {
    some: String,
    t: T,
}

#[allow(unused)]
#[derive(Copy, Clone, Debug, Outgoing, serde::Serialize)]
pub struct OtherThing<'t> {
    some: &'t str,
    t: &'t [u8],
}

#[derive(Outgoing)]
#[incoming_derive(!Deserialize)]
pub struct FakeRequest<'a, T> {
    pub abc: &'a str,
    pub thing: Thing<'a, T>,
    pub device_id: &'a ::ruma_identifiers::DeviceId,
    pub user_id: &'a UserId,
    pub bytes: &'a [u8],
    pub recursive: &'a [Thing<'a, T>],
    pub option: Option<&'a [u8]>,
    pub depth: Option<&'a [(&'a str, &'a str)]>,
    pub arc_type: std::sync::Arc<&'a ::ruma_identifiers::ServerName>,
}

#[derive(Outgoing)]
#[incoming_derive(!Deserialize)]
pub enum EnumThing<'a, T> {
    Abc(&'a str),
    Stuff(Thing<'a, T>),
    Boxy(&'a ::ruma_identifiers::DeviceId),
    Other(Option<&'a str>),
    StructVar { stuff: &'a str, more: &'a ::ruma_identifiers::ServerName },
}
