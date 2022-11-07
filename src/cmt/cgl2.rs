#[doc = "Register `CGL2` reader"]
pub struct R(crate::R<CGL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CGL2` writer"]
pub struct W(crate::W<CGL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGL2_SPEC>;
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
impl From<crate::W<CGL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SL` reader - Secondary Carrier Low Time Data Value"]
pub type SL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SL` writer - Secondary Carrier Low Time Data Value"]
pub type SL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CGL2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Secondary Carrier Low Time Data Value"]
    #[inline(always)]
    pub fn sl(&self) -> SL_R {
        SL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Secondary Carrier Low Time Data Value"]
    #[inline(always)]
    #[must_use]
    pub fn sl(&mut self) -> SL_W<0> {
        SL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMT Carrier Generator Low Data Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgl2](index.html) module"]
pub struct CGL2_SPEC;
impl crate::RegisterSpec for CGL2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cgl2::R](R) reader structure"]
impl crate::Readable for CGL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgl2::W](W) writer structure"]
impl crate::Writable for CGL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CGL2 to value 0"]
impl crate::Resettable for CGL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
