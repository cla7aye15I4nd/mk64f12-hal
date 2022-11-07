#[doc = "Register `SLTL` reader"]
pub struct R(crate::R<SLTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLTL` writer"]
pub struct W(crate::W<SLTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLTL_SPEC>;
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
impl From<crate::W<SLTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSLT` reader - Least significant byte of SCL low timeout value that determines the timeout period of SCL low."]
pub type SSLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSLT` writer - Least significant byte of SCL low timeout value that determines the timeout period of SCL low."]
pub type SSLT_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SLTL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Least significant byte of SCL low timeout value that determines the timeout period of SCL low."]
    #[inline(always)]
    pub fn sslt(&self) -> SSLT_R {
        SSLT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Least significant byte of SCL low timeout value that determines the timeout period of SCL low."]
    #[inline(always)]
    #[must_use]
    pub fn sslt(&mut self) -> SSLT_W<0> {
        SSLT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C SCL Low Timeout Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sltl](index.html) module"]
pub struct SLTL_SPEC;
impl crate::RegisterSpec for SLTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sltl::R](R) reader structure"]
impl crate::Readable for SLTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sltl::W](W) writer structure"]
impl crate::Writable for SLTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLTL to value 0"]
impl crate::Resettable for SLTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
