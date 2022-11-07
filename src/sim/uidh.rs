#[doc = "Register `UIDH` reader"]
pub struct R(crate::R<UIDH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UIDH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UIDH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UIDH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UID` reader - Unique Identification"]
pub type UID_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Unique Identification"]
    #[inline(always)]
    pub fn uid(&self) -> UID_R {
        UID_R::new(self.bits)
    }
}
#[doc = "Unique Identification Register High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uidh](index.html) module"]
pub struct UIDH_SPEC;
impl crate::RegisterSpec for UIDH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uidh::R](R) reader structure"]
impl crate::Readable for UIDH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UIDH to value 0"]
impl crate::Resettable for UIDH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
