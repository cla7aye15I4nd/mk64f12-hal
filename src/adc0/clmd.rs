#[doc = "Register `CLMD` reader"]
pub struct R(crate::R<CLMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLMD` writer"]
pub struct W(crate::W<CLMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLMD_SPEC>;
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
impl From<crate::W<CLMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLMD` reader - Calibration Value"]
pub type CLMD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLMD` writer - Calibration Value"]
pub type CLMD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLMD_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    pub fn clmd(&self) -> CLMD_R {
        CLMD_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn clmd(&mut self) -> CLMD_W<0> {
        CLMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Minus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clmd](index.html) module"]
pub struct CLMD_SPEC;
impl crate::RegisterSpec for CLMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clmd::R](R) reader structure"]
impl crate::Readable for CLMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clmd::W](W) writer structure"]
impl crate::Writable for CLMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLMD to value 0x0a"]
impl crate::Resettable for CLMD_SPEC {
    const RESET_VALUE: Self::Ux = 0x0a;
}
