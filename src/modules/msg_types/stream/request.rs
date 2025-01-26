pub(super) struct StreamConfigurationRequest {
    protocol: MidiProtocol,
    receive_jr_timestamp: bool,
    transmit_jr_timestamp: bool,
}

enum MidiProtocol {
    V1,
    V2
}