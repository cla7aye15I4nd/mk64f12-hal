#[doc = "Register `CLM3` reader"]
pub struct R(crate::R<CLM3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLM3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLM3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLM3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLM3` writer"]
pub struct W(crate::W<CLM3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLM3_SPEC>;
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
impl From<crate::W<CLM3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLM3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLM3` reader - Calibration Value"]
pub type CLM3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLM3` writer - Calibration Value"]
pub type CLM3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLM3_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - Calibration Value"]
    #[inline(always)]
    pub fn clm3(&self) -> CLM3_R {
        CLM3_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn clm3(&mut self) -> CLM3_W<0> {
        CLM3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Minus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clm3](index.html) module"]
pub struct CLM3_SPEC;
impl crate::RegisterSpec for CLM3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clm3::R](R) reader structure"]
impl crate::Readable for CLM3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clm3::W](W) writer structure"]
impl crate::Writable for CLM3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLM3 to value 0x0100"]
impl crate::Resettable for CLM3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
