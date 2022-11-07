#[doc = "Register `TCFIFO` reader"]
pub struct R(crate::R<TCFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXCOUNT` reader - Transmit Counter"]
pub type TXCOUNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Transmit Counter"]
    #[inline(always)]
    pub fn txcount(&self) -> TXCOUNT_R {
        TXCOUNT_R::new(self.bits)
    }
}
#[doc = "UART FIFO Transmit Count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcfifo](index.html) module"]
pub struct TCFIFO_SPEC;
impl crate::RegisterSpec for TCFIFO_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tcfifo::R](R) reader structure"]
impl crate::Readable for TCFIFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TCFIFO to value 0"]
impl crate::Resettable for TCFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
