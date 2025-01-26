use crate::modules::msg_types::stream::UMPStreamMessage;

pub mod stream;
pub mod common;

#[derive(Default)]
pub enum MessageType {
    Utility,
    System,
    MIDI1VoiceChannel,
    DataWithSystemExclusive,
    MIDI2VoiceChannel,
    Data,
    FlexData,
    UMPStream(UMPStreamMessage),
    #[default]
    Unset,
}

impl MessageType {
    pub fn new() -> Self {
        Self::Unset
    }
    ///  Read the MIDI message and attempt to build the received message
    /// according to the received MT. Return None if the message is incomplete,
    /// ie, when the Format field is neither 0x0 nor 0x3.
    ///
    /// When the message is complete, return an instance of self
    pub fn build_from_midi_message(&mut self, midi_msg: &[u32]) {
        //  Will probably have to read from a static ref for multiple UMP msgs streams
        unimplemented!()
    }
}

pub trait ParseMidiMessage {
    fn from_ump_message(message: &[u32]);
    fn into_ump_message(self);
}
