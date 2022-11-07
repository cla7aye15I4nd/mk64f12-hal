#[doc = "Register `R%s` reader"]
pub struct R(crate::R<R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `D` reader - Data result"]
pub type D_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Data result"]
    #[inline(always)]
    pub fn d(&self) -> D_R {
        D_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "ADC Data Result Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r](index.html) module"]
pub struct R_SPEC;
impl crate::RegisterSpec for R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [r::R](R) reader structure"]
impl crate::Readable for R_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets R%s to value 0"]
impl crate::Resettable for R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
