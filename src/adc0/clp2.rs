#[doc = "Register `CLP2` reader"]
pub struct R(crate::R<CLP2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLP2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLP2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLP2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLP2` writer"]
pub struct W(crate::W<CLP2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLP2_SPEC>;
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
impl From<crate::W<CLP2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLP2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLP2` reader - Calibration Value"]
pub type CLP2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLP2` writer - Calibration Value"]
pub type CLP2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLP2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Calibration Value"]
    #[inline(always)]
    pub fn clp2(&self) -> CLP2_R {
        CLP2_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn clp2(&mut self) -> CLP2_W<0> {
        CLP2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Plus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp2](index.html) module"]
pub struct CLP2_SPEC;
impl crate::RegisterSpec for CLP2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clp2::R](R) reader structure"]
impl crate::Readable for CLP2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clp2::W](W) writer structure"]
impl crate::Writable for CLP2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLP2 to value 0x80"]
impl crate::Resettable for CLP2_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
