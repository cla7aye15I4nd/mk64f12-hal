#[doc = "Register `CMPL` reader"]
pub struct R(crate::R<CMPL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPL` writer"]
pub struct W(crate::W<CMPL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPL_SPEC>;
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
impl From<crate::W<CMPL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPAREL` reader - To prevent runaway code from changing this field, software should write to this field after a CPU reset even if the (default) minimum service time is required"]
pub type COMPAREL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPAREL` writer - To prevent runaway code from changing this field, software should write to this field after a CPU reset even if the (default) minimum service time is required"]
pub type COMPAREL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CMPL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - To prevent runaway code from changing this field, software should write to this field after a CPU reset even if the (default) minimum service time is required"]
    #[inline(always)]
    pub fn comparel(&self) -> COMPAREL_R {
        COMPAREL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - To prevent runaway code from changing this field, software should write to this field after a CPU reset even if the (default) minimum service time is required"]
    #[inline(always)]
    #[must_use]
    pub fn comparel(&mut self) -> COMPAREL_W<0> {
        COMPAREL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpl](index.html) module"]
pub struct CMPL_SPEC;
impl crate::RegisterSpec for CMPL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cmpl::R](R) reader structure"]
impl crate::Readable for CMPL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpl::W](W) writer structure"]
impl crate::Writable for CMPL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPL to value 0"]
impl crate::Resettable for CMPL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
