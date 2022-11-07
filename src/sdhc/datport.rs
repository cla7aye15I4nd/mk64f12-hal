#[doc = "Register `DATPORT` reader"]
pub struct R(crate::R<DATPORT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATPORT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATPORT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATPORT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATPORT` writer"]
pub struct W(crate::W<DATPORT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATPORT_SPEC>;
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
impl From<crate::W<DATPORT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATPORT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATCONT` reader - Data Content"]
pub type DATCONT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATCONT` writer - Data Content"]
pub type DATCONT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATPORT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Data Content"]
    #[inline(always)]
    pub fn datcont(&self) -> DATCONT_R {
        DATCONT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data Content"]
    #[inline(always)]
    #[must_use]
    pub fn datcont(&mut self) -> DATCONT_W<0> {
        DATCONT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Buffer Data Port register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datport](index.html) module"]
pub struct DATPORT_SPEC;
impl crate::RegisterSpec for DATPORT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [datport::R](R) reader structure"]
impl crate::Readable for DATPORT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datport::W](W) writer structure"]
impl crate::Writable for DATPORT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATPORT to value 0"]
impl crate::Resettable for DATPORT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
