#[doc = "Register `TMROUTH` reader"]
pub struct R(crate::R<TMROUTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMROUTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMROUTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMROUTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TMROUTH` writer"]
pub struct W(crate::W<TMROUTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMROUTH_SPEC>;
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
impl From<crate::W<TMROUTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMROUTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMEROUTHIGH` reader - Shows the value of the upper 16 bits of the watchdog timer."]
pub type TIMEROUTHIGH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMEROUTHIGH` writer - Shows the value of the upper 16 bits of the watchdog timer."]
pub type TIMEROUTHIGH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, TMROUTH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Shows the value of the upper 16 bits of the watchdog timer."]
    #[inline(always)]
    pub fn timerouthigh(&self) -> TIMEROUTHIGH_R {
        TIMEROUTHIGH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Shows the value of the upper 16 bits of the watchdog timer."]
    #[inline(always)]
    #[must_use]
    pub fn timerouthigh(&mut self) -> TIMEROUTHIGH_W<0> {
        TIMEROUTHIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Timer Output Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmrouth](index.html) module"]
pub struct TMROUTH_SPEC;
impl crate::RegisterSpec for TMROUTH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tmrouth::R](R) reader structure"]
impl crate::Readable for TMROUTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmrouth::W](W) writer structure"]
impl crate::Writable for TMROUTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TMROUTH to value 0"]
impl crate::Resettable for TMROUTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
