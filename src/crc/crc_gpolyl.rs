#[doc = "Register `GPOLYL` reader"]
pub struct R(crate::R<CRC_GPOLYL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_GPOLYL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_GPOLYL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_GPOLYL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPOLYL` writer"]
pub struct W(crate::W<CRC_GPOLYL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_GPOLYL_SPEC>;
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
impl From<crate::W<CRC_GPOLYL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_GPOLYL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPOLYL` reader - POLYL stores the lower 16 bits of the 16/32 bit CRC polynomial value"]
pub type GPOLYL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `GPOLYL` writer - POLYL stores the lower 16 bits of the 16/32 bit CRC polynomial value"]
pub type GPOLYL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CRC_GPOLYL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - POLYL stores the lower 16 bits of the 16/32 bit CRC polynomial value"]
    #[inline(always)]
    pub fn gpolyl(&self) -> GPOLYL_R {
        GPOLYL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - POLYL stores the lower 16 bits of the 16/32 bit CRC polynomial value"]
    #[inline(always)]
    #[must_use]
    pub fn gpolyl(&mut self) -> GPOLYL_W<0> {
        GPOLYL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC_GPOLYL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_gpolyl](index.html) module"]
pub struct CRC_GPOLYL_SPEC;
impl crate::RegisterSpec for CRC_GPOLYL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crc_gpolyl::R](R) reader structure"]
impl crate::Readable for CRC_GPOLYL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_gpolyl::W](W) writer structure"]
impl crate::Writable for CRC_GPOLYL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPOLYL to value 0xffff"]
impl crate::Resettable for CRC_GPOLYL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
