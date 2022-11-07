#[doc = "Register `CLPD` reader"]
pub struct R(crate::R<CLPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLPD` writer"]
pub struct W(crate::W<CLPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLPD_SPEC>;
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
impl From<crate::W<CLPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLPD` reader - Calibration Value"]
pub type CLPD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLPD` writer - Calibration Value"]
pub type CLPD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLPD_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    pub fn clpd(&self) -> CLPD_R {
        CLPD_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn clpd(&mut self) -> CLPD_W<0> {
        CLPD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Plus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clpd](index.html) module"]
pub struct CLPD_SPEC;
impl crate::RegisterSpec for CLPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clpd::R](R) reader structure"]
impl crate::Readable for CLPD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clpd::W](W) writer structure"]
impl crate::Writable for CLPD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLPD to value 0x0a"]
impl crate::Resettable for CLPD_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}
