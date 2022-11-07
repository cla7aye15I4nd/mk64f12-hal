#[doc = "Register `CLM1` reader"]
pub struct R(crate::R<CLM1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLM1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLM1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLM1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLM1` writer"]
pub struct W(crate::W<CLM1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLM1_SPEC>;
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
impl From<crate::W<CLM1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLM1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLM1` reader - Calibration Value"]
pub type CLM1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLM1` writer - Calibration Value"]
pub type CLM1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLM1_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 0:6 - Calibration Value"]
    #[inline(always)]
    pub fn clm1(&self) -> CLM1_R {
        CLM1_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn clm1(&mut self) -> CLM1_W<0> {
        CLM1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Minus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clm1](index.html) module"]
pub struct CLM1_SPEC;
impl crate::RegisterSpec for CLM1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clm1::R](R) reader structure"]
impl crate::Readable for CLM1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clm1::W](W) writer structure"]
impl crate::Writable for CLM1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLM1 to value 0x40"]
impl crate::Resettable for CLM1_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
