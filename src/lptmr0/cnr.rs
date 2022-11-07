#[doc = "Register `CNR` reader"]
pub struct R(crate::R<CNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNR` writer"]
pub struct W(crate::W<CNR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNR_SPEC>;
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
impl From<crate::W<CNR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNTER` reader - Counter Value"]
pub type COUNTER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNTER` writer - Counter Value"]
pub type COUNTER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Counter Value"]
    #[inline(always)]
    pub fn counter(&self) -> COUNTER_R {
        COUNTER_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn counter(&mut self) -> COUNTER_W<0> {
        COUNTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Power Timer Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnr](index.html) module"]
pub struct CNR_SPEC;
impl crate::RegisterSpec for CNR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnr::R](R) reader structure"]
impl crate::Readable for CNR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnr::W](W) writer structure"]
impl crate::Writable for CNR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNR to value 0"]
impl crate::Resettable for CNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
