#[doc = "Register `DATALU` reader"]
pub struct R(crate::R<DATALU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATALU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATALU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATALU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATALU` writer"]
pub struct W(crate::W<DATALU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATALU_SPEC>;
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
impl From<crate::W<DATALU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATALU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATALU` reader - DATALL stores the second 8 bits of the 32 bit CRC"]
pub type DATALU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATALU` writer - DATALL stores the second 8 bits of the 32 bit CRC"]
pub type DATALU_W<'a, const O: u8> = crate::FieldWriter<'a, u8, DATALU_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - DATALL stores the second 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn datalu(&self) -> DATALU_R {
        DATALU_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - DATALL stores the second 8 bits of the 32 bit CRC"]
    #[inline(always)]
    #[must_use]
    pub fn datalu(&mut self) -> DATALU_W<0> {
        DATALU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC_DATALU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datalu](index.html) module"]
pub struct DATALU_SPEC;
impl crate::RegisterSpec for DATALU_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [datalu::R](R) reader structure"]
impl crate::Readable for DATALU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datalu::W](W) writer structure"]
impl crate::Writable for DATALU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATALU to value 0xff"]
impl crate::Resettable for DATALU_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
