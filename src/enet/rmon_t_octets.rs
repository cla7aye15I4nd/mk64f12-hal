#[doc = "Register `RMON_T_OCTETS` reader"]
pub struct R(crate::R<RMON_T_OCTETS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMON_T_OCTETS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMON_T_OCTETS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMON_T_OCTETS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXOCTS` reader - Octet count"]
pub type TXOCTS_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Octet count"]
    #[inline(always)]
    pub fn txocts(&self) -> TXOCTS_R {
        TXOCTS_R::new(self.bits)
    }
}
#[doc = "Tx Octets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_octets](index.html) module"]
pub struct RMON_T_OCTETS_SPEC;
impl crate::RegisterSpec for RMON_T_OCTETS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmon_t_octets::R](R) reader structure"]
impl crate::Readable for RMON_T_OCTETS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RMON_T_OCTETS to value 0"]
impl crate::Resettable for RMON_T_OCTETS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
