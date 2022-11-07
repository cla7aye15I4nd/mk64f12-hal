#[doc = "Register `FDPROT` reader"]
pub struct R(crate::R<FDPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DPROT` reader - D-Flash Region Protect"]
pub type DPROT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - D-Flash Region Protect"]
    #[inline(always)]
    pub fn dprot(&self) -> DPROT_R {
        DPROT_R::new(self.bits)
    }
}
#[doc = "Non-volatile D-Flash Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdprot](index.html) module"]
pub struct FDPROT_SPEC;
impl crate::RegisterSpec for FDPROT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fdprot::R](R) reader structure"]
impl crate::Readable for FDPROT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDPROT to value 0xff"]
impl crate::Resettable for FDPROT_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
