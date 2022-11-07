#[doc = "Register `CMDRSP0` reader"]
pub struct R(crate::R<CMDRSP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDRSP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDRSP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDRSP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMDRSP0` reader - Command Response 0"]
pub type CMDRSP0_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 0"]
    #[inline(always)]
    pub fn cmdrsp0(&self) -> CMDRSP0_R {
        CMDRSP0_R::new(self.bits)
    }
}
#[doc = "Command Response 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdrsp0](index.html) module"]
pub struct CMDRSP0_SPEC;
impl crate::RegisterSpec for CMDRSP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdrsp0::R](R) reader structure"]
impl crate::Readable for CMDRSP0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMDRSP0 to value 0"]
impl crate::Resettable for CMDRSP0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
