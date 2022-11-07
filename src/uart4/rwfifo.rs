#[doc = "Register `RWFIFO` reader"]
pub struct R(crate::R<RWFIFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RWFIFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RWFIFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RWFIFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RWFIFO` writer"]
pub struct W(crate::W<RWFIFO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RWFIFO_SPEC>;
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
impl From<crate::W<RWFIFO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RWFIFO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXWATER` reader - Receive Watermark"]
pub type RXWATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXWATER` writer - Receive Watermark"]
pub type RXWATER_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RWFIFO_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Receive Watermark"]
    #[inline(always)]
    pub fn rxwater(&self) -> RXWATER_R {
        RXWATER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn rxwater(&mut self) -> RXWATER_W<0> {
        RXWATER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART FIFO Receive Watermark\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rwfifo](index.html) module"]
pub struct RWFIFO_SPEC;
impl crate::RegisterSpec for RWFIFO_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [rwfifo::R](R) reader structure"]
impl crate::Readable for RWFIFO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rwfifo::W](W) writer structure"]
impl crate::Writable for RWFIFO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RWFIFO to value 0x01"]
impl crate::Resettable for RWFIFO_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
