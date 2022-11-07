#[doc = "Register `TOVALL` reader"]
pub struct R(crate::R<TOVALL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOVALL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOVALL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOVALL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOVALL` writer"]
pub struct W(crate::W<TOVALL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOVALL_SPEC>;
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
impl From<crate::W<TOVALL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOVALL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOVALLOW` reader - Defines the lower 16 bits of the 32-bit time-out value for the watchdog timer"]
pub type TOVALLOW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOVALLOW` writer - Defines the lower 16 bits of the 32-bit time-out value for the watchdog timer"]
pub type TOVALLOW_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TOVALL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Defines the lower 16 bits of the 32-bit time-out value for the watchdog timer"]
    #[inline(always)]
    pub fn tovallow(&self) -> TOVALLOW_R {
        TOVALLOW_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the lower 16 bits of the 32-bit time-out value for the watchdog timer"]
    #[inline(always)]
    #[must_use]
    pub fn tovallow(&mut self) -> TOVALLOW_W<0> {
        TOVALLOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Time-out Value Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tovall](index.html) module"]
pub struct TOVALL_SPEC;
impl crate::RegisterSpec for TOVALL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tovall::R](R) reader structure"]
impl crate::Readable for TOVALL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tovall::W](W) writer structure"]
impl crate::Writable for TOVALL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOVALL to value 0x4b4c"]
impl crate::Resettable for TOVALL_SPEC {
    const RESET_VALUE: Self::Ux = 0x4b4c;
}
