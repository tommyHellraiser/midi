pub(super) enum DiscoveryMessage {
    Endpoint(EndpointDiscovery),
    FunctionBlock(FunctionBlockDiscovery),
}

//------------------------------------------------------------
struct EndpointDiscovery {
    filter_bitmap: EndpointNotificationFilter
}

/// Used by the MIDI Device to request info from the endpoint. Each subfield that
/// is requested should be sent in a separate UMP message as response to the Host
struct EndpointNotificationFilter {
    stream_configuration: bool, // s
    product_instance_id: bool,  // i
    endpoint_name: bool,        // n
    device_identity: bool,      // d
    endpoint_info: bool,        // e
}

impl EndpointNotificationFilter {
    fn from_bitmap(bitmap: u8) -> Self {
        //  First three fields are reserved and not used
        Self {
            stream_configuration: bitmap[4],
            product_instance_id: bitmap[3],
            endpoint_name: bitmap[2],
            device_identity: bitmap[1],
            endpoint_info: bitmap[0]
        }
    }
}

//------------------------------------------------------------

struct FunctionBlockDiscovery {
    block_number: u8,
    filter: FunctionBlockDiscoveryFilter
}

struct FunctionBlockDiscoveryFilter {
    name: bool,
    information: bool,
}

impl FunctionBlockDiscoveryFilter {
    fn from_bitmap(bitmap: u8) -> Self {
        //  First 6 fields are reserved and not used
        Self {
            name: bitmap[1],
            information: bitmap[0],
        }
    }
}
