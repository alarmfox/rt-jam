//
// Generic Task that can be a WebSocketTask or WebTransportTask.
//
// Handles rollover of connection from WebTransport to WebSocket
//
use common::protos::packet_wrapper::PacketWrapper;
use log::debug;
use yew_webtransport::webtransport::WebTransportTask;

use super::webmedia::{ConnectOptions, WebMedia};

#[derive(Debug)]
pub(super) enum Task {
    WebTransport(WebTransportTask),
}

impl Task {
    pub fn connect(options: ConnectOptions) -> anyhow::Result<Self> {
        debug!("Task::connect trying WebTransport");
        match WebTransportTask::connect(options.clone()) {
            Ok(task) => return Ok(Task::WebTransport(task)),
            Err(e) => Err(e),
        }
    }

    pub fn send_packet(&self, packet: PacketWrapper) {
        match self {
            Task::WebTransport(wt) => wt.send_packet(packet),
        }
    }
}
