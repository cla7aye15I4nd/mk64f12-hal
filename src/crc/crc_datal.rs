#[doc = "Register `DATAL` reader"]
pub struct R(crate::R<CRC_DATAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRC_DATAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRC_DATAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRC_DATAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATAL` writer"]
pub struct W(crate::W<CRC_DATAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRC_DATAL_SPEC>;
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
impl From<crate::W<CRC_DATAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRC_DATAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAL` reader - DATAL stores the lower 16 bits of the 16/32 bit CRC"]
pub type DATAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATAL` writer - DATAL stores the lower 16 bits of the 16/32 bit CRC"]
pub type DATAL_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CRC_DATAL_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - DATAL stores the lower 16 bits of the 16/32 bit CRC"]
    #[inline(always)]
    pub fn datal(&self) -> DATAL_R {
        DATAL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - DATAL stores the lower 16 bits of the 16/32 bit CRC"]
    #[inline(always)]
    #[must_use]
    pub fn datal(&mut self) -> DATAL_W<0> {
        DATAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC_DATAL register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_datal](index.html) module"]
pub struct CRC_DATAL_SPEC;
impl crate::RegisterSpec for CRC_DATAL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [crc_datal::R](R) reader structure"]
impl crate::Readable for CRC_DATAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crc_datal::W](W) writer structure"]
impl crate::Writable for CRC_DATAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATAL to value 0xffff"]
impl crate::Resettable for CRC_DATAL_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
