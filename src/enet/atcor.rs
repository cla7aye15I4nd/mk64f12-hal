#[doc = "Register `ATCOR` reader"]
pub struct R(crate::R<ATCOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATCOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATCOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATCOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATCOR` writer"]
pub struct W(crate::W<ATCOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATCOR_SPEC>;
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
impl From<crate::W<ATCOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATCOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COR` reader - Correction Counter Wrap-Around Value"]
pub type COR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `COR` writer - Correction Counter Wrap-Around Value"]
pub type COR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATCOR_SPEC, u32, u32, 31, O>;
impl R {
    #[doc = "Bits 0:30 - Correction Counter Wrap-Around Value"]
    #[inline(always)]
    pub fn cor(&self) -> COR_R {
        COR_R::new(self.bits & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:30 - Correction Counter Wrap-Around Value"]
    #[inline(always)]
    #[must_use]
    pub fn cor(&mut self) -> COR_W<0> {
        COR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Correction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atcor](index.html) module"]
pub struct ATCOR_SPEC;
impl crate::RegisterSpec for ATCOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atcor::R](R) reader structure"]
impl crate::Readable for ATCOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atcor::W](W) writer structure"]
impl crate::Writable for ATCOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATCOR to value 0"]
impl crate::Resettable for ATCOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
