#[doc = "Register `RMON_T_P128TO255` reader"]
pub struct R(crate::R<RMON_T_P128TO255_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RMON_T_P128TO255_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RMON_T_P128TO255_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RMON_T_P128TO255_SPEC>) -> Self {
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
#[doc = "Tx 128- to 255-byte Packets Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rmon_t_p128to255](index.html) module"]
pub struct RMON_T_P128TO255_SPEC;
impl crate::RegisterSpec for RMON_T_P128TO255_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rmon_t_p128to255::R](R) reader structure"]
impl crate::Readable for RMON_T_P128TO255_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RMON_T_P128TO255 to value 0"]
impl crate::Resettable for RMON_T_P128TO255_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
