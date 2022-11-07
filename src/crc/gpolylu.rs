#[doc = "Register `GPOLYLU` reader"]
pub struct R(crate::R<GPOLYLU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPOLYLU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPOLYLU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPOLYLU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPOLYLU` writer"]
pub struct W(crate::W<GPOLYLU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPOLYLU_SPEC>;
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
impl From<crate::W<GPOLYLU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPOLYLU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPOLYLU` reader - POLYLL stores the second 8 bits of the 32 bit CRC"]
pub type GPOLYLU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPOLYLU` writer - POLYLL stores the second 8 bits of the 32 bit CRC"]
pub type GPOLYLU_W<'a, const O: u8> = crate::FieldWriter<'a, u8, GPOLYLU_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - POLYLL stores the second 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn gpolylu(&self) -> GPOLYLU_R {
        GPOLYLU_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - POLYLL stores the second 8 bits of the 32 bit CRC"]
    #[inline(always)]
    #[must_use]
    pub fn gpolylu(&mut self) -> GPOLYLU_W<0> {
        GPOLYLU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC_GPOLYLU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpolylu](index.html) module"]
pub struct GPOLYLU_SPEC;
impl crate::RegisterSpec for GPOLYLU_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [gpolylu::R](R) reader structure"]
impl crate::Readable for GPOLYLU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpolylu::W](W) writer structure"]
impl crate::Writable for GPOLYLU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPOLYLU to value 0xff"]
impl crate::Resettable for GPOLYLU_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
