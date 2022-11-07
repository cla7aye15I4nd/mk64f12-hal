#[doc = "Register `CLPS` reader"]
pub struct R(crate::R<CLPS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLPS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLPS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLPS` writer"]
pub struct W(crate::W<CLPS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLPS_SPEC>;
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
impl From<crate::W<CLPS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLPS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLPS` reader - Calibration Value"]
pub type CLPS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLPS` writer - Calibration Value"]
pub type CLPS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLPS_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    pub fn clps(&self) -> CLPS_R {
        CLPS_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn clps(&mut self) -> CLPS_W<0> {
        CLPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Plus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clps](index.html) module"]
pub struct CLPS_SPEC;
impl crate::RegisterSpec for CLPS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clps::R](R) reader structure"]
impl crate::Readable for CLPS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clps::W](W) writer structure"]
impl crate::Writable for CLPS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLPS to value 0x20"]
impl crate::Resettable for CLPS_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
