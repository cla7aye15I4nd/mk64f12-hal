#[doc = "Register `IDLY` reader"]
pub struct R(crate::R<IDLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IDLY` writer"]
pub struct W(crate::W<IDLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDLY_SPEC>;
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
impl From<crate::W<IDLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IDLY` reader - PDB Interrupt Delay"]
pub type IDLY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `IDLY` writer - PDB Interrupt Delay"]
pub type IDLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDLY_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - PDB Interrupt Delay"]
    #[inline(always)]
    pub fn idly(&self) -> IDLY_R {
        IDLY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PDB Interrupt Delay"]
    #[inline(always)]
    #[must_use]
    pub fn idly(&mut self) -> IDLY_W<0> {
        IDLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idly](index.html) module"]
pub struct IDLY_SPEC;
impl crate::RegisterSpec for IDLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idly::R](R) reader structure"]
impl crate::Readable for IDLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [idly::W](W) writer structure"]
impl crate::Writable for IDLY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IDLY to value 0xffff"]
impl crate::Resettable for IDLY_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
