#[doc = "Register `CLM4` reader"]
pub struct R(crate::R<CLM4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLM4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLM4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLM4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLM4` writer"]
pub struct W(crate::W<CLM4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLM4_SPEC>;
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
impl From<crate::W<CLM4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLM4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLM4` reader - Calibration Value"]
pub type CLM4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLM4` writer - Calibration Value"]
pub type CLM4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLM4_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Calibration Value"]
    #[inline(always)]
    pub fn clm4(&self) -> CLM4_R {
        CLM4_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn clm4(&mut self) -> CLM4_W<0> {
        CLM4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Minus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clm4](index.html) module"]
pub struct CLM4_SPEC;
impl crate::RegisterSpec for CLM4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clm4::R](R) reader structure"]
impl crate::Readable for CLM4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clm4::W](W) writer structure"]
impl crate::Writable for CLM4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLM4 to value 0x0200"]
impl crate::Resettable for CLM4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
