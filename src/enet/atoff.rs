#[doc = "Register `ATOFF` reader"]
pub struct R(crate::R<ATOFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATOFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATOFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATOFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATOFF` writer"]
pub struct W(crate::W<ATOFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATOFF_SPEC>;
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
impl From<crate::W<ATOFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATOFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFFSET` reader - Offset value for one-shot event generation"]
pub type OFFSET_R = crate::FieldReader<u32, u32>;
#[doc = "Field `OFFSET` writer - Offset value for one-shot event generation"]
pub type OFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATOFF_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Offset value for one-shot event generation"]
    #[inline(always)]
    pub fn offset(&self) -> OFFSET_R {
        OFFSET_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Offset value for one-shot event generation"]
    #[inline(always)]
    #[must_use]
    pub fn offset(&mut self) -> OFFSET_W<0> {
        OFFSET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Offset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atoff](index.html) module"]
pub struct ATOFF_SPEC;
impl crate::RegisterSpec for ATOFF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atoff::R](R) reader structure"]
impl crate::Readable for ATOFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atoff::W](W) writer structure"]
impl crate::Writable for ATOFF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATOFF to value 0"]
impl crate::Resettable for ATOFF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
