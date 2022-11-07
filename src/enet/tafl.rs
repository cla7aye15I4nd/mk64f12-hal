#[doc = "Register `TAFL` reader"]
pub struct R(crate::R<TAFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAFL` writer"]
pub struct W(crate::W<TAFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAFL_SPEC>;
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
impl From<crate::W<TAFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAFL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_ALMOST_FULL` reader - Value Of The Transmit FIFO Almost Full Threshold"]
pub type TX_ALMOST_FULL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX_ALMOST_FULL` writer - Value Of The Transmit FIFO Almost Full Threshold"]
pub type TX_ALMOST_FULL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAFL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Value Of The Transmit FIFO Almost Full Threshold"]
    #[inline(always)]
    pub fn tx_almost_full(&self) -> TX_ALMOST_FULL_R {
        TX_ALMOST_FULL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Value Of The Transmit FIFO Almost Full Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn tx_almost_full(&mut self) -> TX_ALMOST_FULL_W<0> {
        TX_ALMOST_FULL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit FIFO Almost Full Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tafl](index.html) module"]
pub struct TAFL_SPEC;
impl crate::RegisterSpec for TAFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tafl::R](R) reader structure"]
impl crate::Readable for TAFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tafl::W](W) writer structure"]
impl crate::Writable for TAFL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAFL to value 0x08"]
impl crate::Resettable for TAFL_SPEC {
    const RESET_VALUE: Self::Ux = 0x08;
}
