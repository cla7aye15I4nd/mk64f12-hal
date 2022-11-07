#[doc = "Register `CLP0` reader"]
pub struct R(crate::R<CLP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLP0` writer"]
pub struct W(crate::W<CLP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLP0_SPEC>;
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
impl From<crate::W<CLP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLP0` reader - Calibration Value"]
pub type CLP0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLP0` writer - Calibration Value"]
pub type CLP0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLP0_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    pub fn clp0(&self) -> CLP0_R {
        CLP0_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn clp0(&mut self) -> CLP0_W<0> {
        CLP0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Plus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp0](index.html) module"]
pub struct CLP0_SPEC;
impl crate::RegisterSpec for CLP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clp0::R](R) reader structure"]
impl crate::Readable for CLP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clp0::W](W) writer structure"]
impl crate::Writable for CLP0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLP0 to value 0x20"]
impl crate::Resettable for CLP0_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
