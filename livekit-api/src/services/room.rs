use super::{ServiceBase, ServiceResult, LIVEKIT_PACKAGE};
use crate::services::twirp_client::TwirpClient;
use crate::{access_token::VideoGrants, get_env_keys};
use livekit_protocol as proto;

const SVC: &'static str = "RoomService";

#[derive(Debug, Clone, Default)]
pub struct CreateRoomOptions {
    pub empty_timeout: u32,
    pub max_participants: u32,
    pub node_id: String,
    pub metadata: String,
    pub egress: Option<proto::RoomEgress>, // TODO(theomonnom): Better API?
}

#[derive(Debug, Clone, Default)]
pub struct UpdateParticipantOptions {
    pub metadata: String,
    pub permission: Option<proto::ParticipantPermission>,
    pub name: String, // No effect if left empty
}

#[derive(Debug, Clone, Default)]
pub struct SendDataOptions {
    pub kind: proto::data_packet::Kind,
    pub destination_sids: Vec<String>,
    pub topic: Option<String>,
}

#[derive(Debug)]
pub struct RoomClient {
    base: ServiceBase,
    client: TwirpClient,
}

impl RoomClient {
    pub fn with_api_key(host: &str, api_key: &str, api_secret: &str) -> Self {
        Self {
            base: ServiceBase::with_api_key(api_key, api_secret),
            client: TwirpClient::new(host, LIVEKIT_PACKAGE, None),
        }
    }

    pub fn new(host: &str) -> ServiceResult<Self> {
        let (api_key, api_secret) = get_env_keys()?;
        Ok(Self::with_api_key(host, &api_key, &api_secret))
    }

    pub async fn create_room(
        &self,
        name: &str,
        options: CreateRoomOptions,
    ) -> ServiceResult<proto::Room> {
        self.client
            .request(
                SVC,
                "CreateRoom",
                proto::CreateRoomRequest {
                    name: name.to_owned(),
                    empty_timeout: options.empty_timeout,
                    max_participants: options.max_participants,
                    node_id: options.node_id,
                    metadata: options.metadata,
                    egress: options.egress,
                },
                self.base.auth_header(VideoGrants {
                    room_create: true,
                    ..Default::default()
                })?,
            )
            .await
            .map_err(Into::into)
    }

    pub async fn list_rooms(&self, names: Vec<String>) -> ServiceResult<Vec<proto::Room>> {
        let resp: proto::ListRoomsResponse = self
            .client
            .request(
                SVC,
                "ListRooms",
                proto::ListRoomsRequest { names },
                self.base.auth_header(VideoGrants {
                    room_list: true,
                    ..Default::default()
                })?,
            )
            .await?;

        Ok(resp.rooms)
    }

    pub async fn delete_room(&self, room: &str) -> ServiceResult<()> {
        self.client
            .request(
                SVC,
                "DeleteRoom",
                proto::DeleteRoomRequest {
                    room: room.to_owned(),
                },
                self.base.auth_header(VideoGrants {
                    room_create: true,
                    ..Default::default()
                })?,
            )
            .await
            .map_err(Into::into)
    }

    pub async fn update_room_metadata(
        &self,
        room: &str,
        metadata: &str,
    ) -> ServiceResult<proto::Room> {
        self.client
            .request(
                SVC,
                "UpdateRoomMetadata",
                proto::UpdateRoomMetadataRequest {
                    room: room.to_owned(),
                    metadata: metadata.to_owned(),
                },
                self.base.auth_header(VideoGrants {
                    room_admin: true,
                    room: room.to_owned(),
                    ..Default::default()
                })?,
            )
            .await
            .map_err(Into::into)
    }

    pub async fn list_participants(
        &self,
        room: &str,
    ) -> ServiceResult<Vec<proto::ParticipantInfo>> {
        let resp: proto::ListParticipantsResponse = self
            .client
            .request(
                SVC,
                "ListParticipants",
                proto::ListParticipantsRequest {
                    room: room.to_owned(),
                },
                self.base.auth_header(VideoGrants {
                    room_admin: true,
                    room: room.to_owned(),
                    ..Default::default()
                })?,
            )
            .await?;

        Ok(resp.participants)
    }

    pub async fn get_participant(
        &self,
        room: &str,
        identity: &str,
    ) -> ServiceResult<proto::ParticipantInfo> {
        self.client
            .request(
                SVC,
                "GetParticipant",
                proto::RoomParticipantIdentity {
                    room: room.to_owned(),
                    identity: identity.to_owned(),
                },
                self.base.auth_header(VideoGrants {
                    room_admin: true,
                    room: room.to_owned(),
                    ..Default::default()
                })?,
            )
            .await
            .map_err(Into::into)
    }

    pub async fn remove_participant(&self, room: &str, identity: &str) -> ServiceResult<()> {
        self.client
            .request(
                SVC,
                "RemoveParticipant",
                proto::RoomParticipantIdentity {
                    room: room.to_owned(),
                    identity: identity.to_owned(),
                },
                self.base.auth_header(VideoGrants {
                    room_admin: true,
                    room: room.to_owned(),
                    ..Default::default()
                })?,
            )
            .await
            .map_err(Into::into)
    }

    pub async fn mute_published_track(
        &self,
        room: &str,
        identity: &str,
        track_sid: &str,
        muted: bool,
    ) -> ServiceResult<proto::TrackInfo> {
        let resp: proto::MuteRoomTrackResponse = self
            .client
            .request(
                SVC,
                "MutePublishedTrack",
                proto::MuteRoomTrackRequest {
                    room: room.to_owned(),
                    identity: identity.to_owned(),
                    track_sid: track_sid.to_owned(),
                    muted,
                },
                self.base.auth_header(VideoGrants {
                    room_admin: true,
                    room: room.to_owned(),
                    ..Default::default()
                })?,
            )
            .await?;

        Ok(resp.track.unwrap())
    }

    pub async fn update_participant(
        &self,
        room: &str,
        identity: &str,
        options: UpdateParticipantOptions,
    ) -> ServiceResult<proto::ParticipantInfo> {
        self.client
            .request(
                SVC,
                "UpdateParticipant",
                proto::UpdateParticipantRequest {
                    room: room.to_owned(),
                    identity: identity.to_owned(),
                    permission: options.permission,
                    metadata: options.metadata,
                    name: options.name,
                },
                self.base.auth_header(VideoGrants {
                    room_admin: true,
                    room: room.to_owned(),
                    ..Default::default()
                })?,
            )
            .await
            .map_err(Into::into)
    }

    pub async fn update_subscriptions(
        &self,
        room: &str,
        identity: &str,
        track_sids: Vec<String>,
        subscribe: bool,
    ) -> ServiceResult<()> {
        self.client
            .request(
                SVC,
                "UpdateSubscriptions",
                proto::UpdateSubscriptionsRequest {
                    room: room.to_owned(),
                    identity: identity.to_owned(),
                    track_sids,
                    subscribe,
                    ..Default::default()
                },
                self.base.auth_header(VideoGrants {
                    room_admin: true,
                    room: room.to_owned(),
                    ..Default::default()
                })?,
            )
            .await
            .map_err(Into::into)
    }

    pub async fn send_data(
        &self,
        room: &str,
        data: Vec<u8>,
        options: SendDataOptions,
    ) -> ServiceResult<()> {
        self.client
            .request(
                SVC,
                "SendData",
                proto::SendDataRequest {
                    room: room.to_owned(),
                    data,
                    destination_sids: options.destination_sids,
                    topic: options.topic,
                    kind: options.kind as i32,
                },
                self.base.auth_header(VideoGrants {
                    room_admin: true,
                    room: room.to_owned(),
                    ..Default::default()
                })?,
            )
            .await
            .map_err(Into::into)
    }
}
