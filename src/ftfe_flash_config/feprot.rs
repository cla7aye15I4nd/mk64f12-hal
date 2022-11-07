#[doc = "Register `FEPROT` reader"]
pub struct R(crate::R<FEPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EPROT` reader - no description available"]
pub type EPROT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - no description available"]
    #[inline(always)]
    pub fn eprot(&self) -> EPROT_R {
        EPROT_R::new(self.bits)
    }
}
#[doc = "Non-volatile EERAM Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [feprot](index.html) module"]
pub struct FEPROT_SPEC;
impl crate::RegisterSpec for FEPROT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [feprot::R](R) reader structure"]
impl crate::Readable for FEPROT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FEPROT to value 0xff"]
impl crate::Resettable for FEPROT_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
