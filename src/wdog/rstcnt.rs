#[doc = "Register `RSTCNT` reader"]
pub struct R(crate::R<RSTCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSTCNT` writer"]
pub struct W(crate::W<RSTCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSTCNT_SPEC>;
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
impl From<crate::W<RSTCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSTCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSTCNT` reader - Counts the number of times the watchdog resets the system"]
pub type RSTCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RSTCNT` writer - Counts the number of times the watchdog resets the system"]
pub type RSTCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, RSTCNT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Counts the number of times the watchdog resets the system"]
    #[inline(always)]
    pub fn rstcnt(&self) -> RSTCNT_R {
        RSTCNT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counts the number of times the watchdog resets the system"]
    #[inline(always)]
    #[must_use]
    pub fn rstcnt(&mut self) -> RSTCNT_W<0> {
        RSTCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Reset Count register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcnt](index.html) module"]
pub struct RSTCNT_SPEC;
impl crate::RegisterSpec for RSTCNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rstcnt::R](R) reader structure"]
impl crate::Readable for RSTCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rstcnt::W](W) writer structure"]
impl crate::Writable for RSTCNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSTCNT to value 0"]
impl crate::Resettable for RSTCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
