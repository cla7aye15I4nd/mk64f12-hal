#[doc = "Register `D` reader"]
pub struct R(crate::R<D_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D` writer"]
pub struct W(crate::W<D_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D_SPEC>;
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
impl From<crate::W<D_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Data"]
pub type DATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA` writer - Data"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u8, D_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Data I/O register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d](index.html) module"]
pub struct D_SPEC;
impl crate::RegisterSpec for D_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [d::R](R) reader structure"]
impl crate::Readable for D_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d::W](W) writer structure"]
impl crate::Writable for D_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets D to value 0"]
impl crate::Resettable for D_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
