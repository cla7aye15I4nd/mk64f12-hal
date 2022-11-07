#[doc = "Register `GPOLYLL` reader"]
pub struct R(crate::R<CRC_GPOLYLL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_GPOLYLL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_GPOLYLL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_GPOLYLL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPOLYLL` writer"]
pub struct W(crate::W<CRC_GPOLYLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_GPOLYLL_SPEC>;
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
impl From<crate::W<CRC_GPOLYLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_GPOLYLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPOLYLL` reader - POLYLL stores the first 8 bits of the 32 bit CRC"]
pub type GPOLYLL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPOLYLL` writer - POLYLL stores the first 8 bits of the 32 bit CRC"]
pub type GPOLYLL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CRC_GPOLYLL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - POLYLL stores the first 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn gpolyll(&self) -> GPOLYLL_R {
        GPOLYLL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - POLYLL stores the first 8 bits of the 32 bit CRC"]
    #[inline(always)]
    #[must_use]
    pub fn gpolyll(&mut self) -> GPOLYLL_W<0> {
        GPOLYLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC_GPOLYLL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_gpolyll](index.html) module"]
pub struct CRC_GPOLYLL_SPEC;
impl crate::RegisterSpec for CRC_GPOLYLL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [crc_gpolyll::R](R) reader structure"]
impl crate::Readable for CRC_GPOLYLL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_gpolyll::W](W) writer structure"]
impl crate::Writable for CRC_GPOLYLL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPOLYLL to value 0xff"]
impl crate::Resettable for CRC_GPOLYLL_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
