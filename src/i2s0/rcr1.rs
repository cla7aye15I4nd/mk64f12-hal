#[doc = "Register `RCR1` reader"]
pub struct R(crate::R<RCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCR1` writer"]
pub struct W(crate::W<RCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCR1_SPEC>;
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
impl From<crate::W<RCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFW` reader - Receive FIFO Watermark"]
pub type RFW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFW` writer - Receive FIFO Watermark"]
pub type RFW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCR1_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Receive FIFO Watermark"]
    #[inline(always)]
    pub fn rfw(&self) -> RFW_R {
        RFW_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Receive FIFO Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn rfw(&mut self) -> RFW_W<0> {
        RFW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI Receive Configuration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr1](index.html) module"]
pub struct RCR1_SPEC;
impl crate::RegisterSpec for RCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcr1::R](R) reader structure"]
impl crate::Readable for RCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcr1::W](W) writer structure"]
impl crate::Writable for RCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCR1 to value 0"]
impl crate::Resettable for RCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
