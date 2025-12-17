pub struct Addr<'a>(pub &'a soroban_sdk::Address);
pub struct Sym<'a>(pub &'a soroban_sdk::Symbol);

impl cvlr_log::CvlrLog for Addr<'_> {
    #[inline(always)]
    fn log(&self, tag: &str, logger: &mut cvlr_log::CvlrLogger) {
        // lower 8 bits is tag (77), we want the upper 56 bits which is a u64
        let v = self.0.to_val().get_payload() >> 8;
        logger.log_u64(tag, v);
    }
}

// Note: this only works if Symbol was created with nondet_symbol().
impl cvlr_log::CvlrLog for Sym<'_> {
    #[inline(always)]
    fn log(&self, tag: &str, logger: &mut cvlr_log::CvlrLogger) {
        // lower 8 bits is tag (74), we want the upper 56 bits which is a u64
        logger.log_u64(tag, self.0.to_val().get_payload() >> 8);
    }
}
