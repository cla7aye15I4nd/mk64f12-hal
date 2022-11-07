#[doc = "Register `CLMS` reader"]
pub struct R(crate::R<CLMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLMS` writer"]
pub struct W(crate::W<CLMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLMS_SPEC>;
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
impl From<crate::W<CLMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLMS` reader - Calibration Value"]
pub type CLMS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLMS` writer - Calibration Value"]
pub type CLMS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLMS_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    pub fn clms(&self) -> CLMS_R {
        CLMS_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn clms(&mut self) -> CLMS_W<0> {
        CLMS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Minus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clms](index.html) module"]
pub struct CLMS_SPEC;
impl crate::RegisterSpec for CLMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clms::R](R) reader structure"]
impl crate::Readable for CLMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clms::W](W) writer structure"]
impl crate::Writable for CLMS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLMS to value 0x20"]
impl crate::Resettable for CLMS_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
