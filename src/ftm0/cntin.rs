#[doc = "Register `CNTIN` reader"]
pub struct R(crate::R<CNTIN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTIN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTIN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTIN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTIN` writer"]
pub struct W(crate::W<CNTIN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTIN_SPEC>;
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
impl From<crate::W<CNTIN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTIN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT` reader - Initial Value Of The FTM Counter"]
pub type INIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INIT` writer - Initial Value Of The FTM Counter"]
pub type INIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNTIN_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Initial Value Of The FTM Counter"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Initial Value Of The FTM Counter"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<0> {
        INIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Counter Initial Value\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntin](index.html) module"]
pub struct CNTIN_SPEC;
impl crate::RegisterSpec for CNTIN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cntin::R](R) reader structure"]
impl crate::Readable for CNTIN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cntin::W](W) writer structure"]
impl crate::Writable for CNTIN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CNTIN to value 0"]
impl crate::Resettable for CNTIN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
