#[doc = "Register `CMDRSP2` reader"]
pub struct R(crate::R<CMDRSP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDRSP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDRSP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDRSP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMDRSP2` reader - Command Response 2"]
pub type CMDRSP2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 2"]
    #[inline(always)]
    pub fn cmdrsp2(&self) -> CMDRSP2_R {
        CMDRSP2_R::new(self.bits)
    }
}
#[doc = "Command Response 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdrsp2](index.html) module"]
pub struct CMDRSP2_SPEC;
impl crate::RegisterSpec for CMDRSP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdrsp2::R](R) reader structure"]
impl crate::Readable for CMDRSP2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMDRSP2 to value 0"]
impl crate::Resettable for CMDRSP2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
