#[doc = "Register `TOVALH` reader"]
pub struct R(crate::R<TOVALH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOVALH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOVALH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOVALH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOVALH` writer"]
pub struct W(crate::W<TOVALH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOVALH_SPEC>;
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
impl From<crate::W<TOVALH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOVALH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOVALHIGH` reader - Defines the upper 16 bits of the 32-bit time-out value for the watchdog timer"]
pub type TOVALHIGH_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TOVALHIGH` writer - Defines the upper 16 bits of the 32-bit time-out value for the watchdog timer"]
pub type TOVALHIGH_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TOVALH_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Defines the upper 16 bits of the 32-bit time-out value for the watchdog timer"]
    #[inline(always)]
    pub fn tovalhigh(&self) -> TOVALHIGH_R {
        TOVALHIGH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the upper 16 bits of the 32-bit time-out value for the watchdog timer"]
    #[inline(always)]
    #[must_use]
    pub fn tovalhigh(&mut self) -> TOVALHIGH_W<0> {
        TOVALHIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Time-out Value Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tovalh](index.html) module"]
pub struct TOVALH_SPEC;
impl crate::RegisterSpec for TOVALH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tovalh::R](R) reader structure"]
impl crate::Readable for TOVALH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tovalh::W](W) writer structure"]
impl crate::Writable for TOVALH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOVALH to value 0x4c"]
impl crate::Resettable for TOVALH_SPEC {
    const RESET_VALUE: Self::Ux = 0x4c;
}
