#[doc = "Register `CLP4` reader"]
pub struct R(crate::R<CLP4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLP4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLP4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLP4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLP4` writer"]
pub struct W(crate::W<CLP4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLP4_SPEC>;
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
impl From<crate::W<CLP4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLP4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLP4` reader - Calibration Value"]
pub type CLP4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CLP4` writer - Calibration Value"]
pub type CLP4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLP4_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Calibration Value"]
    #[inline(always)]
    pub fn clp4(&self) -> CLP4_R {
        CLP4_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn clp4(&mut self) -> CLP4_W<0> {
        CLP4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Plus-Side General Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clp4](index.html) module"]
pub struct CLP4_SPEC;
impl crate::RegisterSpec for CLP4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clp4::R](R) reader structure"]
impl crate::Readable for CLP4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clp4::W](W) writer structure"]
impl crate::Writable for CLP4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLP4 to value 0x0200"]
impl crate::Resettable for CLP4_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
