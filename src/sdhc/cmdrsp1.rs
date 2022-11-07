#[doc = "Register `CMDRSP1` reader"]
pub struct R(crate::R<CMDRSP1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDRSP1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDRSP1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDRSP1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMDRSP1` reader - Command Response 1"]
pub type CMDRSP1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 1"]
    #[inline(always)]
    pub fn cmdrsp1(&self) -> CMDRSP1_R {
        CMDRSP1_R::new(self.bits)
    }
}
#[doc = "Command Response 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdrsp1](index.html) module"]
pub struct CMDRSP1_SPEC;
impl crate::RegisterSpec for CMDRSP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdrsp1::R](R) reader structure"]
impl crate::Readable for CMDRSP1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMDRSP1 to value 0"]
impl crate::Resettable for CMDRSP1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
