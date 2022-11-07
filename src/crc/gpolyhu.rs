#[doc = "Register `GPOLYHU` reader"]
pub struct R(crate::R<GPOLYHU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPOLYHU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPOLYHU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPOLYHU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPOLYHU` writer"]
pub struct W(crate::W<GPOLYHU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPOLYHU_SPEC>;
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
impl From<crate::W<GPOLYHU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPOLYHU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPOLYHU` reader - POLYHU stores the fourth 8 bits of the 32 bit CRC"]
pub type GPOLYHU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GPOLYHU` writer - POLYHU stores the fourth 8 bits of the 32 bit CRC"]
pub type GPOLYHU_W<'a, const O: u8> = crate::FieldWriter<'a, u8, GPOLYHU_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - POLYHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline(always)]
    pub fn gpolyhu(&self) -> GPOLYHU_R {
        GPOLYHU_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - POLYHU stores the fourth 8 bits of the 32 bit CRC"]
    #[inline(always)]
    #[must_use]
    pub fn gpolyhu(&mut self) -> GPOLYHU_W<0> {
        GPOLYHU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC_GPOLYHU register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpolyhu](index.html) module"]
pub struct GPOLYHU_SPEC;
impl crate::RegisterSpec for GPOLYHU_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [gpolyhu::R](R) reader structure"]
impl crate::Readable for GPOLYHU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpolyhu::W](W) writer structure"]
impl crate::Writable for GPOLYHU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPOLYHU to value 0xff"]
impl crate::Resettable for GPOLYHU_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
