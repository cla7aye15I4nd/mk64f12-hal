#[doc = "Register `CLP3` reader"]
pub struct R(crate::R<CLP3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLP3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLP3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLP3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLP3` writer"]
pub struct W(crate::W<CLP3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLP3_SPEC>;
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
impl From<crate::W<CLP3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLP3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLP3` reader - Calibration Value"]
pub type CLP3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLP3` writer - Calibration Value"]
pub type CLP3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLP3_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:8 - Calibration Value"]
    #[inline(always)]
    pub fn clp3(&self) -> CLP3_R {
        CLP3_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn clp3(&mut self) -> CLP3_W<0> {
        CLP3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Plus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp3](index.html) module"]
pub struct CLP3_SPEC;
impl crate::RegisterSpec for CLP3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clp3::R](R) reader structure"]
impl crate::Readable for CLP3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clp3::W](W) writer structure"]
impl crate::Writable for CLP3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLP3 to value 0x0100"]
impl crate::Resettable for CLP3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
