#[doc = "Register `DACINT%s` reader"]
pub struct R(crate::R<DACINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACINT%s` writer"]
pub struct W(crate::W<DACINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACINT_SPEC>;
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
impl From<crate::W<DACINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT` reader - DAC Interval"]
pub type INT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INT` writer - DAC Interval"]
pub type INT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DACINT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - DAC Interval"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - DAC Interval"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<0> {
        INT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Interval n register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacint](index.html) module"]
pub struct DACINT_SPEC;
impl crate::RegisterSpec for DACINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dacint::R](R) reader structure"]
impl crate::Readable for DACINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dacint::W](W) writer structure"]
impl crate::Writable for DACINT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACINT%s to value 0"]
impl crate::Resettable for DACINT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
