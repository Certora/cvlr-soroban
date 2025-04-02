pub struct Addr<'a>(pub &'a soroban_sdk::Address);

impl cvlr_log::CvlrLog for Addr<'_> {
    #[inline(always)]
    fn log(&self, tag: &str, logger: &mut cvlr_log::CvlrLogger) {
        // lower 8 bits is tag (77), we want the upper 56 bits which is a u64
        let v = self.0.to_val().get_payload() >> 8;
        logger.log_u64(tag, v);
    }
}
