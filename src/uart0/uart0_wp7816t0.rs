#[doc = "Register `WP7816T0` reader"]
pub struct R(crate::R<UART0_WP7816T0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART0_WP7816T0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART0_WP7816T0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART0_WP7816T0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WP7816T0` writer"]
pub struct W(crate::W<UART0_WP7816T0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART0_WP7816T0_SPEC>;
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
impl From<crate::W<UART0_WP7816T0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART0_WP7816T0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WI` reader - Wait Time Integer (C7816\\[TTYPE\\]
= 0)"]
pub type WI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WI` writer - Wait Time Integer (C7816\\[TTYPE\\]
= 0)"]
pub type WI_W<'a, const O: u8> = crate::FieldWriter<'a, u8, UART0_WP7816T0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Wait Time Integer (C7816\\[TTYPE\\]
= 0)"]
    #[inline(always)]
    pub fn wi(&self) -> WI_R {
        WI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Wait Time Integer (C7816\\[TTYPE\\]
= 0)"]
    #[inline(always)]
    #[must_use]
    pub fn wi(&mut self) -> WI_W<0> {
        WI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART 7816 Wait Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0_wp7816t0](index.html) module"]
pub struct UART0_WP7816T0_SPEC;
impl crate::RegisterSpec for UART0_WP7816T0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [uart0_wp7816t0::R](R) reader structure"]
impl crate::Readable for UART0_WP7816T0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart0_wp7816t0::W](W) writer structure"]
impl crate::Writable for UART0_WP7816T0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WP7816T0 to value 0x0a"]
impl crate::Resettable for UART0_WP7816T0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}
