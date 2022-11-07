#[doc = "Register `TWFIFO` reader"]
pub struct R(crate::R<TWFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TWFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TWFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TWFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TWFIFO` writer"]
pub struct W(crate::W<TWFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TWFIFO_SPEC>;
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
impl From<crate::W<TWFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TWFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXWATER` reader - Transmit Watermark"]
pub type TXWATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXWATER` writer - Transmit Watermark"]
pub type TXWATER_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TWFIFO_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Transmit Watermark"]
    #[inline(always)]
    pub fn txwater(&self) -> TXWATER_R {
        TXWATER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn txwater(&mut self) -> TXWATER_W<0> {
        TXWATER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART FIFO Transmit Watermark\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [twfifo](index.html) module"]
pub struct TWFIFO_SPEC;
impl crate::RegisterSpec for TWFIFO_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [twfifo::R](R) reader structure"]
impl crate::Readable for TWFIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [twfifo::W](W) writer structure"]
impl crate::Writable for TWFIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TWFIFO to value 0"]
impl crate::Resettable for TWFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
