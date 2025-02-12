use super::TrackPublicationInner;
use crate::id::TrackSid;
use crate::track::{RemoteTrack, Track, TrackDimension, TrackKind, TrackSource};
use livekit_protocol as proto;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct RemoteTrackPublication {
    inner: Arc<TrackPublicationInner>,
}

impl RemoteTrackPublication {
    pub(crate) fn new(info: proto::TrackInfo, track: Option<RemoteTrack>) -> Self {
        Self {
            inner: Arc::new(TrackPublicationInner::new(info, track.map(Into::into))),
        }
    }

    #[inline]
    pub fn sid(&self) -> TrackSid {
        self.inner.sid()
    }

    #[inline]
    pub fn name(&self) -> String {
        self.inner.name()
    }

    #[inline]
    pub fn kind(&self) -> TrackKind {
        self.inner.kind()
    }

    #[inline]
    pub fn source(&self) -> TrackSource {
        self.inner.source()
    }

    #[inline]
    pub fn simulcasted(&self) -> bool {
        self.inner.simulcasted()
    }

    #[inline]
    pub fn dimension(&self) -> TrackDimension {
        self.inner.dimension()
    }

    #[inline]
    pub fn track(&self) -> Option<RemoteTrack> {
        self.inner.track().map(|track| track.try_into().unwrap())
    }

    #[inline]
    pub fn mime_type(&self) -> String {
        self.inner.mime_type()
    }

    #[inline]
    pub fn is_muted(&self) -> bool {
        self.inner.is_muted()
    }

    #[inline]
    pub fn is_remote(&self) -> bool {
        true
    }

    #[inline]
    pub(crate) fn update_track(&self, track: Option<Track>) {
        self.inner.update_track(track);
    }

    #[inline]
    pub(crate) fn update_info(&self, info: proto::TrackInfo) {
        self.inner.update_info(info);
    }
}
