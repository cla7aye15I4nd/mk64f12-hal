#[doc = "Register `ATVR` reader"]
pub struct R(crate::R<ATVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATVR` writer"]
pub struct W(crate::W<ATVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATVR_SPEC>;
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
impl From<crate::W<ATVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATIME` reader - A write sets the timer"]
pub type ATIME_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ATIME` writer - A write sets the timer"]
pub type ATIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ATVR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - A write sets the timer"]
    #[inline(always)]
    pub fn atime(&self) -> ATIME_R {
        ATIME_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - A write sets the timer"]
    #[inline(always)]
    #[must_use]
    pub fn atime(&mut self) -> ATIME_W<0> {
        ATIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atvr](index.html) module"]
pub struct ATVR_SPEC;
impl crate::RegisterSpec for ATVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atvr::R](R) reader structure"]
impl crate::Readable for ATVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atvr::W](W) writer structure"]
impl crate::Writable for ATVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATVR to value 0"]
impl crate::Resettable for ATVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
