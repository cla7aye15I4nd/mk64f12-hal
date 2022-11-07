#[doc = "Register `RA` reader"]
pub struct R(crate::R<RA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RA` writer"]
pub struct W(crate::W<RA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RA_SPEC>;
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
impl From<crate::W<RA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAD` reader - Range Slave Address"]
pub type RAD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RAD` writer - Range Slave Address"]
pub type RAD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, RA_SPEC, u8, u8, 7, O>;
impl R {
    #[doc = "Bits 1:7 - Range Slave Address"]
    #[inline(always)]
    pub fn rad(&self) -> RAD_R {
        RAD_R::new((self.bits >> 1) & 0x7f)
    }
}
impl W {
    #[doc = "Bits 1:7 - Range Slave Address"]
    #[inline(always)]
    #[must_use]
    pub fn rad(&mut self) -> RAD_W<1> {
        RAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Range Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ra](index.html) module"]
pub struct RA_SPEC;
impl crate::RegisterSpec for RA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ra::R](R) reader structure"]
impl crate::Readable for RA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ra::W](W) writer structure"]
impl crate::Writable for RA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RA to value 0"]
impl crate::Resettable for RA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
