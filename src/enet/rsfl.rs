#[doc = "Register `RSFL` reader"]
pub struct R(crate::R<RSFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSFL` writer"]
pub struct W(crate::W<RSFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSFL_SPEC>;
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
impl From<crate::W<RSFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSFL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RX_SECTION_FULL` reader - Value Of Receive FIFO Section Full Threshold"]
pub type RX_SECTION_FULL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RX_SECTION_FULL` writer - Value Of Receive FIFO Section Full Threshold"]
pub type RX_SECTION_FULL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RSFL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Value Of Receive FIFO Section Full Threshold"]
    #[inline(always)]
    pub fn rx_section_full(&self) -> RX_SECTION_FULL_R {
        RX_SECTION_FULL_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Value Of Receive FIFO Section Full Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn rx_section_full(&mut self) -> RX_SECTION_FULL_W<0> {
        RX_SECTION_FULL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive FIFO Section Full Threshold\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsfl](index.html) module"]
pub struct RSFL_SPEC;
impl crate::RegisterSpec for RSFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsfl::R](R) reader structure"]
impl crate::Readable for RSFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsfl::W](W) writer structure"]
impl crate::Writable for RSFL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSFL to value 0"]
impl crate::Resettable for RSFL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
