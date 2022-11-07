#[doc = "Register `C%sV` reader"]
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
#[doc = "Register `C%sV` writer"]
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
#[doc = "Field `VAL` reader - Channel Value"]
pub type VAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VAL` writer - Channel Value"]
pub type VAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CV_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Channel Value"]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Channel Value"]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> VAL_W<0> {
        VAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel (n) Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cv](index.html) module"]
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
#[doc = "`reset()` method sets C%sV to value 0"]
impl crate::Resettable for CV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
