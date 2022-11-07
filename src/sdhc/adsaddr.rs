#[doc = "Register `ADSADDR` reader"]
pub struct R(crate::R<ADSADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADSADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADSADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADSADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADSADDR` writer"]
pub struct W(crate::W<ADSADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADSADDR_SPEC>;
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
impl From<crate::W<ADSADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADSADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADSADDR` reader - ADMA System Address"]
pub type ADSADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ADSADDR` writer - ADMA System Address"]
pub type ADSADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ADSADDR_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - ADMA System Address"]
    #[inline(always)]
    pub fn adsaddr(&self) -> ADSADDR_R {
        ADSADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - ADMA System Address"]
    #[inline(always)]
    #[must_use]
    pub fn adsaddr(&mut self) -> ADSADDR_W<2> {
        ADSADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADMA System Addressregister\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adsaddr](index.html) module"]
pub struct ADSADDR_SPEC;
impl crate::RegisterSpec for ADSADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adsaddr::R](R) reader structure"]
impl crate::Readable for ADSADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adsaddr::W](W) writer structure"]
impl crate::Writable for ADSADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADSADDR to value 0"]
impl crate::Resettable for ADSADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
