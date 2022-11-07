#[doc = "Register `SLTH` reader"]
pub struct R(crate::R<SLTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLTH` writer"]
pub struct W(crate::W<SLTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLTH_SPEC>;
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
impl From<crate::W<SLTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSLT` reader - Most significant byte of SCL low timeout value that determines the timeout period of SCL low."]
pub type SSLT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSLT` writer - Most significant byte of SCL low timeout value that determines the timeout period of SCL low."]
pub type SSLT_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SLTH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Most significant byte of SCL low timeout value that determines the timeout period of SCL low."]
    #[inline(always)]
    pub fn sslt(&self) -> SSLT_R {
        SSLT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Most significant byte of SCL low timeout value that determines the timeout period of SCL low."]
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
#[doc = "I2C SCL Low Timeout Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slth](index.html) module"]
pub struct SLTH_SPEC;
impl crate::RegisterSpec for SLTH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [slth::R](R) reader structure"]
impl crate::Readable for SLTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slth::W](W) writer structure"]
impl crate::Writable for SLTH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLTH to value 0"]
impl crate::Resettable for SLTH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
