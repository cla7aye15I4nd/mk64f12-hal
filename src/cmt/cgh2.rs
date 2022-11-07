#[doc = "Register `CGH2` reader"]
pub struct R(crate::R<CGH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CGH2` writer"]
pub struct W(crate::W<CGH2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CGH2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGH2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SH` reader - Secondary Carrier High Time Data Value"]
pub type SH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SH` writer - Secondary Carrier High Time Data Value"]
pub type SH_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CGH2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Secondary Carrier High Time Data Value"]
    #[inline(always)]
    pub fn sh(&self) -> SH_R {
        SH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Secondary Carrier High Time Data Value"]
    #[inline(always)]
    #[must_use]
    pub fn sh(&mut self) -> SH_W<0> {
        SH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMT Carrier Generator High Data Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgh2](index.html) module"]
pub struct CGH2_SPEC;
impl crate::RegisterSpec for CGH2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cgh2::R](R) reader structure"]
impl crate::Readable for CGH2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgh2::W](W) writer structure"]
impl crate::Writable for CGH2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CGH2 to value 0"]
impl crate::Resettable for CGH2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
