#[doc = "Register `IEEE_R_MACERR` reader"]
pub struct R(crate::R<IEEE_R_MACERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEEE_R_MACERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEEE_R_MACERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEEE_R_MACERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT` reader - Count"]
pub type COUNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Count"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Receive FIFO Overflow Count Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_r_macerr](index.html) module"]
pub struct IEEE_R_MACERR_SPEC;
impl crate::RegisterSpec for IEEE_R_MACERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ieee_r_macerr::R](R) reader structure"]
impl crate::Readable for IEEE_R_MACERR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IEEE_R_MACERR to value 0"]
impl crate::Resettable for IEEE_R_MACERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
