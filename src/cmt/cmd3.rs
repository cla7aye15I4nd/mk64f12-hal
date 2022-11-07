#[doc = "Register `CMD3` reader"]
pub struct R(crate::R<CMD3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD3` writer"]
pub struct W(crate::W<CMD3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD3_SPEC>;
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
impl From<crate::W<CMD3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SB` reader - Controls the upper space periods of the modulator for all modes."]
pub type SB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SB` writer - Controls the upper space periods of the modulator for all modes."]
pub type SB_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CMD3_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Controls the upper space periods of the modulator for all modes."]
    #[inline(always)]
    pub fn sb(&self) -> SB_R {
        SB_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Controls the upper space periods of the modulator for all modes."]
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
#[doc = "CMT Modulator Data Register Space High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd3](index.html) module"]
pub struct CMD3_SPEC;
impl crate::RegisterSpec for CMD3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cmd3::R](R) reader structure"]
impl crate::Readable for CMD3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd3::W](W) writer structure"]
impl crate::Writable for CMD3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMD3 to value 0"]
impl crate::Resettable for CMD3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
