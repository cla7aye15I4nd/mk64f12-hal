#[doc = "Register `RAFL` reader"]
pub struct R(crate::R<RAFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAFL` writer"]
pub struct W(crate::W<RAFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAFL_SPEC>;
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
impl From<crate::W<RAFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAFL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_ALMOST_FULL` reader - Value Of The Receive FIFO Almost Full Threshold"]
pub type RX_ALMOST_FULL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_ALMOST_FULL` writer - Value Of The Receive FIFO Almost Full Threshold"]
pub type RX_ALMOST_FULL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RAFL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Value Of The Receive FIFO Almost Full Threshold"]
    #[inline(always)]
    pub fn rx_almost_full(&self) -> RX_ALMOST_FULL_R {
        RX_ALMOST_FULL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Value Of The Receive FIFO Almost Full Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn rx_almost_full(&mut self) -> RX_ALMOST_FULL_W<0> {
        RX_ALMOST_FULL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive FIFO Almost Full Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rafl](index.html) module"]
pub struct RAFL_SPEC;
impl crate::RegisterSpec for RAFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rafl::R](R) reader structure"]
impl crate::Readable for RAFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rafl::W](W) writer structure"]
impl crate::Writable for RAFL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAFL to value 0x04"]
impl crate::Resettable for RAFL_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
