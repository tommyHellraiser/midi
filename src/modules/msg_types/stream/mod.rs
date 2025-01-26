use crate::modules::msg_types::stream::discovery::DiscoveryMessage;
use crate::modules::msg_types::stream::notification::NotificationMessage;
use crate::modules::msg_types::stream::request::StreamConfigurationRequest;

mod discovery;
mod notification;
mod request;

pub enum UMPStreamMessage {
    DiscoveryMessages(DiscoveryMessage),
    NotificationMessages(NotificationMessage),
    StreamConfigurationRequest(StreamConfigurationRequest),
    StartOfClipMessage,
    EndOfClipMessage,
}

