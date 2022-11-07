#[doc = "Register `CV%s` reader"]
pub struct R(crate::R<CV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CV%s` writer"]
pub struct W(crate::W<CV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CV_SPEC>;
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
impl From<crate::W<CV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CV` reader - Compare Value."]
pub type CV_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CV` writer - Compare Value."]
pub type CV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CV_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Compare Value."]
    #[inline(always)]
    pub fn cv(&self) -> CV_R {
        CV_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Compare Value."]
    #[inline(always)]
    #[must_use]
    pub fn cv(&mut self) -> CV_W<0> {
        CV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Value Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cv](index.html) module"]
pub struct CV_SPEC;
impl crate::RegisterSpec for CV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cv::R](R) reader structure"]
impl crate::Readable for CV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cv::W](W) writer structure"]
impl crate::Writable for CV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CV%s to value 0"]
impl crate::Resettable for CV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
