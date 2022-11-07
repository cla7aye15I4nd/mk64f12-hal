#[doc = "Register `LDVAL%s` reader"]
pub struct R(crate::R<LDVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LDVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LDVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LDVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LDVAL%s` writer"]
pub struct W(crate::W<LDVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LDVAL_SPEC>;
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
impl From<crate::W<LDVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LDVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSV` reader - Timer Start Value"]
pub type TSV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TSV` writer - Timer Start Value"]
pub type TSV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LDVAL_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Timer Start Value"]
    #[inline(always)]
    pub fn tsv(&self) -> TSV_R {
        TSV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer Start Value"]
    #[inline(always)]
    #[must_use]
    pub fn tsv(&mut self) -> TSV_W<0> {
        TSV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Load Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ldval](index.html) module"]
pub struct LDVAL_SPEC;
impl crate::RegisterSpec for LDVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ldval::R](R) reader structure"]
impl crate::Readable for LDVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ldval::W](W) writer structure"]
impl crate::Writable for LDVAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LDVAL%s to value 0"]
impl crate::Resettable for LDVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
