#[doc = "Register `DATAW1S%sU` reader"]
pub struct R(crate::R<DATAW1SU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATAW1SU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATAW1SU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATAW1SU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATAW1S%sU` writer"]
pub struct W(crate::W<DATAW1SU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATAW1SU_SPEC>;
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
impl From<crate::W<DATAW1SU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATAW1SU_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `data` reader - Bits \\[63:32\\]
of data entry"]
pub type DATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `data` writer - Bits \\[63:32\\]
of data entry"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATAW1SU_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Bits \\[63:32\\]
of data entry"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits \\[63:32\\]
of data entry"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Data Storage (upper word)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dataw1su](index.html) module"]
pub struct DATAW1SU_SPEC;
impl crate::RegisterSpec for DATAW1SU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dataw1su::R](R) reader structure"]
impl crate::Readable for DATAW1SU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dataw1su::W](W) writer structure"]
impl crate::Writable for DATAW1SU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATAW1S%sU to value 0"]
impl crate::Resettable for DATAW1SU_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
