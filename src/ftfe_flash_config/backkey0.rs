#[doc = "Register `BACKKEY0` reader"]
pub struct R(crate::R<BACKKEY0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKKEY0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKKEY0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKKEY0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEY` reader - Backdoor Comparison Key."]
pub type KEY_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Backdoor Comparison Key."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new(self.bits)
    }
}
#[doc = "Backdoor Comparison Key 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backkey0](index.html) module"]
pub struct BACKKEY0_SPEC;
impl crate::RegisterSpec for BACKKEY0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [backkey0::R](R) reader structure"]
impl crate::Readable for BACKKEY0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BACKKEY0 to value 0xff"]
impl crate::Resettable for BACKKEY0_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
