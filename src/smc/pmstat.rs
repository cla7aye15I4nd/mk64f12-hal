#[doc = "Register `PMSTAT` reader"]
pub struct R(crate::R<PMSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PMSTAT` reader - When debug is enabled, the PMSTAT will not update to STOP or VLPS"]
pub type PMSTAT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - When debug is enabled, the PMSTAT will not update to STOP or VLPS"]
    #[inline(always)]
    pub fn pmstat(&self) -> PMSTAT_R {
        PMSTAT_R::new(self.bits & 0x7f)
    }
}
#[doc = "Power Mode Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmstat](index.html) module"]
pub struct PMSTAT_SPEC;
impl crate::RegisterSpec for PMSTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pmstat::R](R) reader structure"]
impl crate::Readable for PMSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PMSTAT to value 0x01"]
impl crate::Resettable for PMSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
