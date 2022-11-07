#[doc = "Register `RMON_T_OVERSIZE` reader"]
pub struct R(crate::R<RMON_T_OVERSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMON_T_OVERSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMON_T_OVERSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMON_T_OVERSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TXPKTS` reader - Packet count"]
pub type TXPKTS_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Packet count"]
    #[inline(always)]
    pub fn txpkts(&self) -> TXPKTS_R {
        TXPKTS_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Tx Packets GT MAX_FL bytes and Good CRC Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_oversize](index.html) module"]
pub struct RMON_T_OVERSIZE_SPEC;
impl crate::RegisterSpec for RMON_T_OVERSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmon_t_oversize::R](R) reader structure"]
impl crate::Readable for RMON_T_OVERSIZE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RMON_T_OVERSIZE to value 0"]
impl crate::Resettable for RMON_T_OVERSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
