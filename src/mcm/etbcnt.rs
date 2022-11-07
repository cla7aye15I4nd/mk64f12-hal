#[doc = "Register `ETBCNT` reader"]
pub struct R(crate::R<ETBCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETBCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETBCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETBCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNTER` reader - Byte Count Counter Value"]
pub type COUNTER_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:10 - Byte Count Counter Value"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new((self.bits & 0x07ff) as u16)
    }
}
#[doc = "ETB Counter Value register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etbcnt](index.html) module"]
pub struct ETBCNT_SPEC;
impl crate::RegisterSpec for ETBCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etbcnt::R](R) reader structure"]
impl crate::Readable for ETBCNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ETBCNT to value 0"]
impl crate::Resettable for ETBCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
