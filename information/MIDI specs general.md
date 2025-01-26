# Introduction
MIDI packets, called Universal Midi Packets (UMPs) can contain up to 4 words.
Each word consists of 32 bits. Left most bits are MSBs and right most bits 
are the LSBs.

# Important items
## Message Type
Consist on the first 4 bits of any word, and indicate the type of message 
included in said word. For example Utility, MIDI Channel Voice Messages 
and the UMP size, and size of the Status Field.

## Group
It's a 4 bit value to address a UMP MIDI Message to one of 16 available groups.
The groups are nominated 1-16 in the spec, but the values that need to be sent to
specify each group are actually from 0-15.

Each group can contain 16 MIDI channels, and each group with its 16 channels 
are independent of other groups. This gives a total of 256 MIDI channels.

Messages that do not support channels, like System Messages and Data Messages, 
have to affect all MIDI groups and channels.

## Status
Multiple messages are defined for each message type, and each message in the 
Message Type has its own Status Value. The size of the Status Field depends on
the Message Type.

## Reserved
Messages marked as reserved should not be used at the moment. Field marked as
reserved should contain all zeroes and not be used. Reserved bits should 
contain zero. Reserved option flags should be set to zero. 

# Message Types
Message type define the type of content each word will have. It's defined in 
the first 4 bits of every message. The available MTs at the moment are:
- (MT - [UMP size]: Description)
- 0x0 - [32 bits]: Utility Messages
- 0x1 - [32 bits]: System Real Time and System Common Messages, except System Exclusives
- 0x2 - [32 bits]: MIDI 1.0 Channel Voice Messages
- 0x3 - [64 bits]: Data Messages (Including System Exclusive)
- 0x4 - [64 bits]: MIDI 2.0 Channel Voice Messages
- 0x5 - [128 bits]: Data Messages
- 0xD - [128 bits]: Flex Data Messages
- 0xF - [128 bits]: UMP Stream Messages

Every other message type is reserved and shall not be used.

# Jitter reduction
TODO Check this feature later, only available for MIDI 2.0, it might be useful to 
avoid timing errors in the stream, but it won't be a part of the MVP for this crate.

It's configurable for MIDI2, so it might be off for now by default.

# Function Blocks
These can be used to define the behaviour of a certain group, for example groups
5 and 6, or span a defined amount of groups, from 5 to 12. Function blocks may be 
bidirectional. They can also change their starting group and amount of groups
spanned. Function Blocks can also be declared as static, so they groups defined
at startup cannot be changed, and neither the properties nor Function Block names.

If a Port is unlikely to be used as IN/OUT, it's recommended to represent them as 
a single Function Block, for better compatibility with MIDI1.


