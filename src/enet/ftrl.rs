#[doc = "Register `FTRL` reader"]
pub struct R(crate::R<FTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FTRL` writer"]
pub struct W(crate::W<FTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FTRL_SPEC>;
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
impl From<crate::W<FTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRUNC_FL` reader - Frame Truncation Length"]
pub type TRUNC_FL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TRUNC_FL` writer - Frame Truncation Length"]
pub type TRUNC_FL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FTRL_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bits 0:13 - Frame Truncation Length"]
    #[inline(always)]
    pub fn trunc_fl(&self) -> TRUNC_FL_R {
        TRUNC_FL_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Frame Truncation Length"]
    #[inline(always)]
    #[must_use]
    pub fn trunc_fl(&mut self) -> TRUNC_FL_W<0> {
        TRUNC_FL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame Truncation Length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ftrl](index.html) module"]
pub struct FTRL_SPEC;
impl crate::RegisterSpec for FTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ftrl::R](R) reader structure"]
impl crate::Readable for FTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ftrl::W](W) writer structure"]
impl crate::Writable for FTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FTRL to value 0x07ff"]
impl crate::Resettable for FTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x07ff;
}
