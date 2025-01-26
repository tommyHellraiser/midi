pub(super) enum NotificationMessage {
    //  Triggered from Endpoint Discovery Messages
    EndpointInfo(EndpointInformationNotification),
    DeviceIdentity(DeviceIdentityNotification),
    EndpointName(EndpointNameNotification),
    ProductInstanceId(ProductInstanceIdNotification),
    StreamConfiguration(StreamConfigurationNotification),
    //  Triggered from Function Block Discovery Messages
    FunctionBlockInfo(FunctionBlockInformationNotification),
    FunctionBlockName(FunctionBlockNameNotification),
}

//-------------------------------------------------------------------
/// TODO Will need to read the configuration from a configuration instance.
///  Probably a static reference
struct EndpointInformationNotification {
    status: bool,
    number_of_function_blocks: u8,
    midi_2_compatibility: bool,
    midi_1_compatibility: bool,
    receives_jr_timestamp: bool,
    transmits_jr_timestamp: bool,
}

//-------------------------------------------------------------------
struct DeviceIdentityNotification {
    device_manufacturer: Vec<u8>,
    device_family: Vec<u8>,
    device_family_model_number: Vec<u8>,
    software_revision_level: Vec<u8>,
}


//-------------------------------------------------------------------
/// Contains the name of the MIDI endpoint, it must not be longer than 98 bytes
/// This name might trigger up to 7 consecutive `endpoint name notification` messages
struct EndpointNameNotification {
    name: String
}

//-------------------------------------------------------------------
/// Contains the Product Instance ID of the endpoint. It is recommended that the
/// Instance ID is the same as the Serial Number, and should be a unique number
/// per manufacturer, family and model
///
/// It needs to be an ASCII text contained in the ordinal range of 32-126 and
/// not exceed 42 bytes in size
///
/// This Serial number may trigger up to 3 UMP messages
struct ProductInstanceIdNotification {
    product_instance_id: String
}

//-------------------------------------------------------------------
/// Sends the configuration of the endpoint's stream. Since this crate is designed
/// to be used with MIDI 2.0 standard, the protocol field will always be 0x02
struct StreamConfigurationNotification {
    receive_jr_timestamp: bool,
    transmit_jr_timestamp: bool,
}

//-------------------------------------------------------------------
/// Sends information about the requested Function Block number in the Discovery Message
///
/// If the Discovery message contained 0xFF in the FB Number, then one message per
/// Function Block should be sent
struct FunctionBlockInformationNotification {
    active: bool,
    function_block_number: u8,
    ui_hint: UserInterfaceHintDirection,
    midi_1: Midi1Port,
    first_group: u8,
    span: u8,
    //  Always 0x01
    midi_ci_version_format: u8,
    //  Always 0 for now
    max_number_of_sysex_8_msgs: u8,
}

enum UserInterfaceHintDirection {
    Unknown,
    Receiver,
    Sender,
    Bidirectional,
}

enum Midi1Port {
    No,
    DontRestrictBandwidth,
    RestrictBandwidth,
}

//-------------------------------------------------------------------
/// Name should be encoded in UTF-8 and not be longer than 91 bytes in size.
/// This notification message may trigger up to 7 UMP messages
struct FunctionBlockNameNotification {
    name: String
}
