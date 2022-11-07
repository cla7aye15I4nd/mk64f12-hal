#[doc = "Register `CGL1` reader"]
pub struct R(crate::R<CGL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CGL1` writer"]
pub struct W(crate::W<CGL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGL1_SPEC>;
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
impl From<crate::W<CGL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PL` reader - Primary Carrier Low Time Data Value"]
pub type PL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PL` writer - Primary Carrier Low Time Data Value"]
pub type PL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CGL1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Primary Carrier Low Time Data Value"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Primary Carrier Low Time Data Value"]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PL_W<0> {
        PL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMT Carrier Generator Low Data Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgl1](index.html) module"]
pub struct CGL1_SPEC;
impl crate::RegisterSpec for CGL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cgl1::R](R) reader structure"]
impl crate::Readable for CGL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgl1::W](W) writer structure"]
impl crate::Writable for CGL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CGL1 to value 0"]
impl crate::Resettable for CGL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
