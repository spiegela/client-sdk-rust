/*
 * Copyright 2023 LiveKit
 *
 * Licensed under the Apache License, Version 2.0 (the “License”);
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an “AS IS” BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#pragma once

#include <memory>

#include "api/peer_connection_interface.h"
#include "livekit/data_channel.h"
#include "livekit/helper.h"
#include "livekit/jsep.h"
#include "livekit/media_stream.h"
#include "livekit/rtc_error.h"
#include "livekit/rtp_receiver.h"
#include "livekit/rtp_sender.h"
#include "livekit/rtp_transceiver.h"
#include "livekit/webrtc.h"
#include "rust/cxx.h"
#include "webrtc-sys/src/data_channel.rs.h"

namespace livekit {
class PeerConnection;
class NativePeerConnectionObserver;
}  // namespace livekit
#include "webrtc-sys/src/peer_connection.rs.h"

namespace livekit {

class PeerConnectionFactory;

class PeerConnection {
 public:
  PeerConnection(
      std::shared_ptr<RtcRuntime> rtc_runtime,
      std::unique_ptr<NativePeerConnectionObserver> observer,
      rtc::scoped_refptr<webrtc::PeerConnectionInterface> peer_connection);

  ~PeerConnection();

  void create_offer(
      RtcOfferAnswerOptions options,
      rust::Box<AsyncContext> ctx,
      rust::Fn<void(rust::Box<AsyncContext>,
                    std::unique_ptr<SessionDescription>)> on_success,
      rust::Fn<void(rust::Box<AsyncContext>, RtcError)> on_error) const;

  void create_answer(
      RtcOfferAnswerOptions options,
      rust::Box<AsyncContext> ctx,
      rust::Fn<void(rust::Box<AsyncContext>,
                    std::unique_ptr<SessionDescription>)> on_success,
      rust::Fn<void(rust::Box<AsyncContext>, RtcError)> on_error) const;

  void set_local_description(
      std::unique_ptr<SessionDescription> desc,
      rust::Box<AsyncContext> ctx,
      rust::Fn<void(rust::Box<AsyncContext>, RtcError)> on_complete) const;

  void set_remote_description(
      std::unique_ptr<SessionDescription> desc,
      rust::Box<AsyncContext> ctx,
      rust::Fn<void(rust::Box<AsyncContext>, RtcError)> on_complete) const;

  std::shared_ptr<DataChannel> create_data_channel(rust::String label,
                                                   DataChannelInit init) const;

  void add_ice_candidate(
      std::shared_ptr<IceCandidate> candidate,
      rust::Box<AsyncContext> ctx,
      rust::Fn<void(rust::Box<AsyncContext>, RtcError)> on_complete) const;

  std::shared_ptr<RtpSender> add_track(
      std::shared_ptr<MediaStreamTrack> track,
      const rust::Vec<rust::String>& stream_ids) const;

  void remove_track(std::shared_ptr<RtpSender> sender) const;

  std::shared_ptr<RtpTransceiver> add_transceiver(
      std::shared_ptr<MediaStreamTrack> track,
      RtpTransceiverInit init) const;

  std::shared_ptr<RtpTransceiver> add_transceiver_for_media(
      MediaType media_type,
      RtpTransceiverInit init) const;

  rust::Vec<RtpSenderPtr> get_senders() const;

  rust::Vec<RtpReceiverPtr> get_receivers() const;

  rust::Vec<RtpTransceiverPtr> get_transceivers() const;

  std::unique_ptr<SessionDescription> current_local_description() const;

  std::unique_ptr<SessionDescription> current_remote_description() const;

  std::unique_ptr<SessionDescription> pending_local_description() const;

  std::unique_ptr<SessionDescription> pending_remote_description() const;

  std::unique_ptr<SessionDescription> local_description() const;

  std::unique_ptr<SessionDescription> remote_description() const;

  PeerConnectionState connection_state() const;

  SignalingState signaling_state() const;

  IceGatheringState ice_gathering_state() const;

  IceConnectionState ice_connection_state() const;

  void close() const;

 private:
  std::shared_ptr<RtcRuntime> rtc_runtime_;
  std::unique_ptr<NativePeerConnectionObserver> observer_;
  rtc::scoped_refptr<webrtc::PeerConnectionInterface> peer_connection_;
};

static std::shared_ptr<PeerConnection> _shared_peer_connection() {
  return nullptr;  // Ignore
}

class NativePeerConnectionObserver : public webrtc::PeerConnectionObserver {
 public:
  NativePeerConnectionObserver(
      rust::Box<PeerConnectionObserverWrapper> observer);

  ~NativePeerConnectionObserver();

  void OnSignalingChange(
      webrtc::PeerConnectionInterface::SignalingState new_state) override;

  void OnAddStream(
      rtc::scoped_refptr<webrtc::MediaStreamInterface> stream) override;

  void OnRemoveStream(
      rtc::scoped_refptr<webrtc::MediaStreamInterface> stream) override;

  void OnDataChannel(
      rtc::scoped_refptr<webrtc::DataChannelInterface> data_channel) override;

  void OnRenegotiationNeeded() override;

  void OnNegotiationNeededEvent(uint32_t event_id) override;

  void OnIceConnectionChange(
      webrtc::PeerConnectionInterface::IceConnectionState new_state) override;

  void OnStandardizedIceConnectionChange(
      webrtc::PeerConnectionInterface::IceConnectionState new_state) override;

  void OnConnectionChange(
      webrtc::PeerConnectionInterface::PeerConnectionState new_state) override;

  void OnIceGatheringChange(
      webrtc::PeerConnectionInterface::IceGatheringState new_state) override;

  void OnIceCandidate(const webrtc::IceCandidateInterface* candidate) override;

  void OnIceCandidateError(const std::string& address,
                           int port,
                           const std::string& url,
                           int error_code,
                           const std::string& error_text) override;

  void OnIceCandidatesRemoved(
      const std::vector<cricket::Candidate>& candidates) override;

  void OnIceConnectionReceivingChange(bool receiving) override;

  void OnIceSelectedCandidatePairChanged(
      const cricket::CandidatePairChangeEvent& event) override;

  void OnAddTrack(
      rtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver,
      const std::vector<rtc::scoped_refptr<webrtc::MediaStreamInterface>>&
          streams) override;

  void OnTrack(
      rtc::scoped_refptr<webrtc::RtpTransceiverInterface> transceiver) override;

  void OnRemoveTrack(
      rtc::scoped_refptr<webrtc::RtpReceiverInterface> receiver) override;

  void OnInterestingUsage(int usage_pattern) override;

 private:
  friend PeerConnectionFactory;
  // The RtcRuntime is set inside PeerConnectionFactory, we can simplify that
  // once create_native_connection_observer is removed
  std::shared_ptr<RtcRuntime> rtc_runtime_;
  rust::Box<PeerConnectionObserverWrapper> observer_;
};

std::unique_ptr<NativePeerConnectionObserver>
create_native_peer_connection_observer(
    rust::Box<PeerConnectionObserverWrapper> observer);

}  // namespace livekit
