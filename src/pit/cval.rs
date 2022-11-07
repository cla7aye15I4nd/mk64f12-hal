#[doc = "Register `CVAL%s` reader"]
pub struct R(crate::R<CVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TVL` reader - Current Timer Value"]
pub type TVL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current Timer Value"]
    #[inline(always)]
    pub fn tvl(&self) -> TVL_R {
        TVL_R::new(self.bits)
    }
}
#[doc = "Current Timer Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cval](index.html) module"]
pub struct CVAL_SPEC;
impl crate::RegisterSpec for CVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cval::R](R) reader structure"]
impl crate::Readable for CVAL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CVAL%s to value 0"]
impl crate::Resettable for CVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
