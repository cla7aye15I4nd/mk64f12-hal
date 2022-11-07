#[doc = "Register `RMON_R_OCTETS` reader"]
pub struct R(crate::R<RMON_R_OCTETS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMON_R_OCTETS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMON_R_OCTETS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMON_R_OCTETS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT` reader - Octet count"]
pub type COUNT_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Octet count"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new(self.bits)
    }
}
#[doc = "Rx Octets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_r_octets](index.html) module"]
pub struct RMON_R_OCTETS_SPEC;
impl crate::RegisterSpec for RMON_R_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmon_r_octets::R](R) reader structure"]
impl crate::Readable for RMON_R_OCTETS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RMON_R_OCTETS to value 0"]
impl crate::Resettable for RMON_R_OCTETS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
