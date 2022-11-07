#[doc = "Register `CMD4` reader"]
pub struct R(crate::R<CMD4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD4` writer"]
pub struct W(crate::W<CMD4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD4_SPEC>;
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
impl From<crate::W<CMD4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SB` reader - Controls the lower space periods of the modulator for all modes."]
pub type SB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SB` writer - Controls the lower space periods of the modulator for all modes."]
pub type SB_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CMD4_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Controls the lower space periods of the modulator for all modes."]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls the lower space periods of the modulator for all modes."]
    #[inline(always)]
    #[must_use]
    pub fn sb(&mut self) -> SB_W<0> {
        SB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMT Modulator Data Register Space Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd4](index.html) module"]
pub struct CMD4_SPEC;
impl crate::RegisterSpec for CMD4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cmd4::R](R) reader structure"]
impl crate::Readable for CMD4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd4::W](W) writer structure"]
impl crate::Writable for CMD4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD4 to value 0"]
impl crate::Resettable for CMD4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
