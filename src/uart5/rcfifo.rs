#[doc = "Register `RCFIFO` reader"]
pub struct R(crate::R<RCFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXCOUNT` reader - Receive Counter"]
pub type RXCOUNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Receive Counter"]
    #[inline(always)]
    pub fn rxcount(&self) -> RXCOUNT_R {
        RXCOUNT_R::new(self.bits)
    }
}
#[doc = "UART FIFO Receive Count\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcfifo](index.html) module"]
pub struct RCFIFO_SPEC;
impl crate::RegisterSpec for RCFIFO_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rcfifo::R](R) reader structure"]
impl crate::Readable for RCFIFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RCFIFO to value 0"]
impl crate::Resettable for RCFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
