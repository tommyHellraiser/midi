use crate::modules::msg_types::midi_1_voice::Midi1VoiceMessage;
use crate::modules::msg_types::midi_2_voice::Midi2VoiceMessage;
use crate::modules::msg_types::stream::StreamMessage;
use crate::modules::msg_types::utility::UtilityMessage;

pub mod stream;
pub mod common;
mod utility;
mod midi_1_voice;
mod midi_2_voice;

#[derive(Default)]
pub enum MessageType {
    Utility(UtilityMessage),
    System,
    MIDI1VoiceChannel(Midi1VoiceMessage),
    DataWithSystemExclusive,
    MIDI2VoiceChannel(Midi2VoiceMessage),
    Data,
    FlexData,
    UMPStream(StreamMessage),
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
