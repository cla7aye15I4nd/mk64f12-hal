#[doc = "Register `BDL` reader"]
pub struct R(crate::R<BDL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDL` writer"]
pub struct W(crate::W<BDL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDL_SPEC>;
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
impl From<crate::W<BDL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBR` reader - UART Baud Rate Bits"]
pub type SBR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SBR` writer - UART Baud Rate Bits"]
pub type SBR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, BDL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - UART Baud Rate Bits"]
    #[inline(always)]
    pub fn sbr(&self) -> SBR_R {
        SBR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - UART Baud Rate Bits"]
    #[inline(always)]
    #[must_use]
    pub fn sbr(&mut self) -> SBR_W<0> {
        SBR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Baud Rate Registers: Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdl](index.html) module"]
pub struct BDL_SPEC;
impl crate::RegisterSpec for BDL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bdl::R](R) reader structure"]
impl crate::Readable for BDL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdl::W](W) writer structure"]
impl crate::Writable for BDL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BDL to value 0x04"]
impl crate::Resettable for BDL_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
