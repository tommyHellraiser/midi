# Elements in UMP Message
## Message Type (MT)
4 bits long in every UMP Message Type. Each corresponding value is mentioned in the
`specs general` document. Defines the content of the message and the fields that'll
be present.

## Format
2 bits, and indicate the total length of the UMP message:
- 0x0: Start and complete in a single UMP msg
- 0x1: Start of a message that spans two or more UMP msgs
- 0x2: Continuing a message that spans two or more UMP msgs
- 0x3: End of message that spans two or more UMP msgs

## Status


## Data


## Group


# UMP Stream Messages
These messages are addressed to the UMP Endpoint without a group assigned to them.
All UMP messages are 128 bits long. There are several types of Stream Messages.

## Endpoint discovery
Used when the device requests information about the endpoint via UMP stream.
- MT: 0xF. 4 bits
- Format: always 0x0, 2 bits, Stream messages are 128 bits long and can be completed within
a single UMP msg
- Status: always 0 filled, 10 bits
- UMP Version major: 8 bits