#[doc = "Register `WINH` reader"]
pub struct R(crate::R<WINH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WINH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WINH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WINH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WINH` writer"]
pub struct W(crate::W<WINH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WINH_SPEC>;
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
impl From<crate::W<WINH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WINH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WINHIGH` reader - Defines the upper 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
pub type WINHIGH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WINHIGH` writer - Defines the upper 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
pub type WINHIGH_W<'a, const O: u8> = crate::FieldWriter<'a, u16, WINH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Defines the upper 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
    #[inline(always)]
    pub fn winhigh(&self) -> WINHIGH_R {
        WINHIGH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the upper 16 bits of the 32-bit window for the windowed mode of operation of the watchdog"]
    #[inline(always)]
    #[must_use]
    pub fn winhigh(&mut self) -> WINHIGH_W<0> {
        WINHIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Window Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [winh](index.html) module"]
pub struct WINH_SPEC;
impl crate::RegisterSpec for WINH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [winh::R](R) reader structure"]
impl crate::Readable for WINH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [winh::W](W) writer structure"]
impl crate::Writable for WINH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WINH to value 0"]
impl crate::Resettable for WINH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
