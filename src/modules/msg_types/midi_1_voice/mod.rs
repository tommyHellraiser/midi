use crate::modules::msg_types::common::{MidiChannelType, MidiDataType, MidiIndexType, MidiNoteNumberType, MidiProgramType, MidiVelocityType};

pub(super) enum Midi1VoiceMessage {
    NoteOff((MidiChannelType, MidiNoteNumberType, MidiVelocityType)),
    NoteOn((MidiChannelType, MidiNoteNumberType, MidiVelocityType)),
    PolyPressure((MidiChannelType, MidiNoteNumberType, MidiDataType)),
    ControlChange((MidiChannelType, MidiIndexType, MidiDataType)),
    ProgramChange((MidiChannelType, MidiProgramType)),
    ChannelPressure((MidiChannelType, MidiDataType)),
    //  (MIDI Channel, MIDI Data LSB, MIDI Data MSB)
    PitchBend((MidiChannelType, MidiDataType, MidiDataType)),
}