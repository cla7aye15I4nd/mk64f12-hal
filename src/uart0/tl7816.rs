#[doc = "Register `TL7816` reader"]
pub struct R(crate::R<TL7816_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TL7816_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TL7816_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TL7816_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TL7816` writer"]
pub struct W(crate::W<TL7816_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TL7816_SPEC>;
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
impl From<crate::W<TL7816_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TL7816_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TLEN` reader - Transmit Length"]
pub type TLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TLEN` writer - Transmit Length"]
pub type TLEN_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TL7816_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Transmit Length"]
    #[inline(always)]
    pub fn tlen(&self) -> TLEN_R {
        TLEN_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Length"]
    #[inline(always)]
    #[must_use]
    pub fn tlen(&mut self) -> TLEN_W<0> {
        TLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART 7816 Transmit Length Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tl7816](index.html) module"]
pub struct TL7816_SPEC;
impl crate::RegisterSpec for TL7816_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tl7816::R](R) reader structure"]
impl crate::Readable for TL7816_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tl7816::W](W) writer structure"]
impl crate::Writable for TL7816_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TL7816 to value 0"]
impl crate::Resettable for TL7816_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
