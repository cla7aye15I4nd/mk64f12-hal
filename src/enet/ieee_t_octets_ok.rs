#[doc = "Register `IEEE_T_OCTETS_OK` reader"]
pub struct R(crate::R<IEEE_T_OCTETS_OK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEEE_T_OCTETS_OK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEEE_T_OCTETS_OK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEEE_T_OCTETS_OK_SPEC>) -> Self {
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
#[doc = "Octet Count for Frames Transmitted w/o Error Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_t_octets_ok](index.html) module"]
pub struct IEEE_T_OCTETS_OK_SPEC;
impl crate::RegisterSpec for IEEE_T_OCTETS_OK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ieee_t_octets_ok::R](R) reader structure"]
impl crate::Readable for IEEE_T_OCTETS_OK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IEEE_T_OCTETS_OK to value 0"]
impl crate::Resettable for IEEE_T_OCTETS_OK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
