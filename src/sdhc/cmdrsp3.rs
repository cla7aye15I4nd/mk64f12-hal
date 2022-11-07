#[doc = "Register `CMDRSP3` reader"]
pub struct R(crate::R<CMDRSP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMDRSP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMDRSP3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMDRSP3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMDRSP3` reader - Command Response 3"]
pub type CMDRSP3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Command Response 3"]
    #[inline(always)]
    pub fn cmdrsp3(&self) -> CMDRSP3_R {
        CMDRSP3_R::new(self.bits)
    }
}
#[doc = "Command Response 3\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmdrsp3](index.html) module"]
pub struct CMDRSP3_SPEC;
impl crate::RegisterSpec for CMDRSP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmdrsp3::R](R) reader structure"]
impl crate::Readable for CMDRSP3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMDRSP3 to value 0"]
impl crate::Resettable for CMDRSP3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
