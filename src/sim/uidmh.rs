#[doc = "Register `UIDMH` reader"]
pub struct R(crate::R<UIDMH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UIDMH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UIDMH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UIDMH_SPEC>) -> Self {
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
#[doc = "Unique Identification Register Mid-High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uidmh](index.html) module"]
pub struct UIDMH_SPEC;
impl crate::RegisterSpec for UIDMH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uidmh::R](R) reader structure"]
impl crate::Readable for UIDMH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UIDMH to value 0"]
impl crate::Resettable for UIDMH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
