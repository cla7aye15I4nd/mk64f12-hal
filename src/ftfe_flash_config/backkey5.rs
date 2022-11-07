#[doc = "Register `BACKKEY5` reader"]
pub struct R(crate::R<BACKKEY5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BACKKEY5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BACKKEY5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BACKKEY5_SPEC>) -> Self {
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
#[doc = "Backdoor Comparison Key 5.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [backkey5](index.html) module"]
pub struct BACKKEY5_SPEC;
impl crate::RegisterSpec for BACKKEY5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [backkey5::R](R) reader structure"]
impl crate::Readable for BACKKEY5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BACKKEY5 to value 0xff"]
impl crate::Resettable for BACKKEY5_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
