use crate::modules::msg_types::common::{MidiAttributeType, MidiBankType, MidiCentsType, MidiChannelType, MidiCoarseTuningType, MidiIndexType, MidiLongDataType, MidiNoteNumberType, MidiNumberOfChannels, MidiProgramType, MidiSemitonesType, MidiSourceNote, MidiTuningBankNumberType, MidiTuningProgramNumberType, MidiVelocityType};

pub(super) enum Midi2VoiceMessage {
    NoteOff(NoteMessage),
    NoteOn(NoteMessage),
    PolyPressure(PolyPressureMessage),
    PerNoteController(PerNoteControllerMessage),
    PerNoteManagement(PerNoteManagementMessage),
    ControlChange(ControlChangeMessageType),
    //  RPN messages
    RegisteredParameterNumbers(RegisteredParameterNumbersMessage),
    //  Relative RPN Messages
    RelativeRegisteredParameterNumbers(RelativeRegisteredParameterNumbersMessage),
    ProgramChange(ProgramChangeMessage),
    ChannelPressure(ChannelPressureMessage),
    PitchBend(PitchBendMessage),
    PerNotePitchBend(PerNotePitchBendMessage),
    SensitivityPerNotePitchBend(SensitivityPerNotePitchBendMessage),
}

//----------------------------------------------------------------
struct NoteMessage {
    channel: MidiChannelType,
    note_number: MidiNoteNumberType,
    velocity: MidiVelocityType,
    attribute: MidiAttributeType
}

//----------------------------------------------------------------
struct PolyPressureMessage {
    channel: MidiChannelType,
    note_number: MidiNoteNumberType,
    data: MidiLongDataType
}

//----------------------------------------------------------------
struct PerNoteManagementMessage {
    channel: MidiChannelType,
    note_number: MidiNoteNumberType,
    detach: bool,
    reset: bool,
}

//----------------------------------------------------------------
enum PerNoteControllerMessage {
    Registered,
    Assignable,
}

//----------------------------------------------------------------
enum ControlChangeMessageType {
    Portamento(PortamentoCCMessage),
    OmniOnOffMono(OmniOnOffMonoCCMessage),
    Other(ControlChangeMessage),
}

struct PortamentoCCMessage {
    channel: MidiChannelType,
    index: MidiIndexType,
    source_note: MidiSourceNote,
}

struct OmniOnOffMonoCCMessage {
    channel: MidiChannelType,
    index: MidiIndexType,
    number_of_channels: MidiNumberOfChannels,
}

struct ControlChangeMessage {
    channel: MidiChannelType,
    index: MidiIndexType,
    control_change_data: MidiLongDataType
}

//----------------------------------------------------------------

enum RegisteredParameterNumbersMessage {
    PitchBendRange(PitchBendRangeMessage),
    CoarseTuning(CoarseTuningMessage),
    TuningProgramChange(TuningProgramChangeMessage),
    TuningBankSelect(TuningBankSelectMessage),
    MpeMcm(MpeMcmMessage),
    Other(CommonRpnMessage),
}

struct PitchBendRangeMessage {
    channel: MidiChannelType,
    bank: MidiBankType,
    index: MidiIndexType,
    semitones: MidiSemitonesType,
    cents: MidiCentsType,
}

struct CoarseTuningMessage {
    channel: MidiChannelType,
    bank: MidiBankType,
    index: MidiIndexType,
    coarse_tuning: MidiCoarseTuningType,
}

struct TuningProgramChangeMessage {
    channel: MidiChannelType,
    bank: MidiBankType,
    index: MidiIndexType,
    tuning_program_number: MidiTuningProgramNumberType
}

struct TuningBankSelectMessage {
    channel: MidiChannelType,
    bank: MidiBankType,
    index: MidiIndexType,
    tuning_bank_number: MidiTuningBankNumberType,
}

struct MpeMcmMessage {
    channel: MidiChannelType,
    bank: MidiBankType,
    index: MidiIndexType,
    number_of_channels: MidiNumberOfChannels,
}

struct CommonRpnMessage {
    channel: MidiChannelType,
    bank: MidiBankType,
    index: MidiIndexType,
    data: MidiLongDataType,
}

//----------------------------------------------------------------
struct RelativeControllerMessage {
    channel: MidiChannelType,
    bank: MidiBankType,
    index: MidiIndexType,
    // 32 bits
    data: MidiLongDataType
}
//----------------------------------------------------------------

enum RelativeRegisteredParameterNumbersMessage {
    RelativeRegisteredController(RelativeControllerMessage),
    RelativeAssignableController(RelativeControllerMessage),
}
//----------------------------------------------------------------
struct ProgramChangeMessage {
    channel: MidiChannelType,
    valid_bank: bool,
    program: MidiProgramType,
    bank_msb: MidiBankType,
    bank_lsb: MidiBankType
}

//----------------------------------------------------------------
struct ChannelPressureMessage {
    channel: MidiChannelType,
    data: MidiLongDataType
}

//----------------------------------------------------------------
struct PitchBendMessage {
    channel: MidiChannelType,
    data: MidiLongDataType
}

//----------------------------------------------------------------
struct PerNotePitchBendMessage {
    channel: MidiChannelType,
    note_number: MidiNoteNumberType,
    data: MidiLongDataType
}

//----------------------------------------------------------------
struct SensitivityPerNotePitchBendMessage {
    channel: MidiChannelType,
    units_of_100_cents: u8,
    fraction_of_100_cents: u32
}
