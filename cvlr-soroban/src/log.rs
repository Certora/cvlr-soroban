pub struct Addr<'a>(pub &'a soroban_sdk::Address);
pub struct Sym<'a>(pub &'a soroban_sdk::Symbol);

pub struct BN<'a>(pub &'a soroban_sdk::BytesN<32>);
pub struct B<'a>(pub &'a soroban_sdk::Bytes);

pub trait AsCvlr<'a> {
    type Cvlr;
    fn as_cvlr(&'a self) -> Self::Cvlr;
}

impl<'a> AsCvlr<'a> for soroban_sdk::Address {
    type Cvlr = Addr<'a>;
    #[inline(always)]
    fn as_cvlr(&'a self) -> Addr<'a> {
        Addr(self)
    }
}

impl<'a> AsCvlr<'a> for soroban_sdk::Symbol {
    type Cvlr = Sym<'a>;
    #[inline(always)]
    fn as_cvlr(&'a self) -> Sym<'a> {
        Sym(self)
    }
}

impl<'a> AsCvlr<'a> for soroban_sdk::BytesN<32> {
    type Cvlr = BN<'a>;
    #[inline(always)]
    fn as_cvlr(&'a self) -> BN<'a> {
        BN(self)
    }
}

impl<'a> AsCvlr<'a> for soroban_sdk::Bytes {
    type Cvlr = B<'a>;
    #[inline(always)]
    fn as_cvlr(&'a self) -> B<'a> {
        B(self)
    }
}

impl<'a> From<Addr<'a>> for soroban_sdk::Address {
    #[inline(always)]
    fn from(v: Addr<'a>) -> Self {
        v.0.clone()
    }
}

impl<'a> From<Sym<'a>> for soroban_sdk::Symbol {
    #[inline(always)]
    fn from(v: Sym<'a>) -> Self {
        v.0.clone()
    }
}

impl<'a> From<BN<'a>> for soroban_sdk::BytesN<32> {
    #[inline(always)]
    fn from(v: BN<'a>) -> Self {
        v.0.clone()
    }
}

impl<'a> From<B<'a>> for soroban_sdk::Bytes {
    #[inline(always)]
    fn from(v: B<'a>) -> Self {
        v.0.clone()
    }
}

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

// Note: this only works if Bytes was created with nondet_bytes_n().
impl cvlr_log::CvlrLog for BN<'_> {
    #[inline(always)]
    fn log(&self, tag: &str, logger: &mut cvlr_log::CvlrLogger) {
        logger.log_u64(tag, self.0.to_val().get_payload() >> 8);
    }
}

// Note: this only works if Bytes was created with nondet_bytes().
impl cvlr_log::CvlrLog for B<'_> {
    #[inline(always)]
    fn log(&self, tag: &str, logger: &mut cvlr_log::CvlrLogger) {
        logger.log_u64(tag, self.0.to_val().get_payload() >> 8);
    }
}
