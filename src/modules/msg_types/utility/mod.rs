pub(super) type SenderClockTimeType = u16;

pub(super) enum UtilityMessage {
    NoOperation,
    JitterReductionTimestamps(SenderClockTimeType),
    DeltaClockstamp(SenderClockTimeType),
}

//---------------------------------------------------------------------
impl UtilityMessage {
    pub(super) fn new(sender_clock: SenderClockTimeType) {
        //  TODO
        Self::validate(sender_clock);
        unimplemented!()
    }
    fn validate(sender_clock: SenderClockTimeType) {
        //  TODO for delta clockstamp the sender_clock cannot be 0, every other value is allowed
        unimplemented!()
    }
}
