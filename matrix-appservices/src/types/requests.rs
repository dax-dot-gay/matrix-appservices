use matrix_sdk::ruma::api::appservice as appservice;

#[allow(missing_docs)]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum EventRequest {
    Push(appservice::event::push_events::v1::Request),
}

#[allow(missing_docs)]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PingRequest {
    Ping(appservice::ping::send_ping::v1::Request),
}

#[allow(missing_docs)]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum QueryRequest {
    RoomAlias(appservice::query::query_room_alias::v1::Request),
    UserId(appservice::query::query_user_id::v1::Request),
}

#[allow(missing_docs)]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ThirdPartyRequest {
    GetLocationForProtocol(appservice::thirdparty::get_location_for_protocol::v1::Request),
    GetLocationForRoomAlias(appservice::thirdparty::get_location_for_room_alias::v1::Request),
    GetProtocol(appservice::thirdparty::get_protocol::v1::Request),
    GetUserForProtocol(appservice::thirdparty::get_user_for_protocol::v1::Request),
    GetUserForUserId(appservice::thirdparty::get_user_for_user_id::v1::Request),
}

#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub enum RequestMessage {
    Event(EventRequest),
    Ping(PingRequest),
    Query(QueryRequest),
    ThirdParty(ThirdPartyRequest),
}

#[allow(missing_docs)]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum EventReply {
    Push(appservice::event::push_events::v1::Response),
}

#[allow(missing_docs)]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PingReply {
    Ping(appservice::ping::send_ping::v1::Response),
}

#[allow(missing_docs)]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum QueryReply {
    RoomAlias(appservice::query::query_room_alias::v1::Response),
    UserId(appservice::query::query_user_id::v1::Response),
}

#[allow(missing_docs)]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ThirdPartyReply {
    GetLocationForProtocol(appservice::thirdparty::get_location_for_protocol::v1::Response),
    GetLocationForRoomAlias(appservice::thirdparty::get_location_for_room_alias::v1::Response),
    GetProtocol(appservice::thirdparty::get_protocol::v1::Response),
    GetUserForProtocol(appservice::thirdparty::get_user_for_protocol::v1::Response),
    GetUserForUserId(appservice::thirdparty::get_user_for_user_id::v1::Response),
}

#[allow(missing_docs)]
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum RequestReply {
    Event(EventReply),
    Ping(PingReply),
    Query(QueryReply),
    ThirdParty(ThirdPartyReply),
}

#[allow(missing_docs)]
#[derive(Clone, Debug)]
pub struct Request {
    pub message: RequestMessage,
    pub reply_to: async_channel::Sender<crate::Result<RequestReply>>,
}
