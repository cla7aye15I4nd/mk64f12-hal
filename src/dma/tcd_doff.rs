#[doc = "Register `TCD%s_DOFF` reader"]
pub struct R(crate::R<TCD_DOFF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCD_DOFF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCD_DOFF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCD_DOFF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD%s_DOFF` writer"]
pub struct W(crate::W<TCD_DOFF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCD_DOFF_SPEC>;
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
impl From<crate::W<TCD_DOFF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCD_DOFF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DOFF` reader - Destination Address Signed offset"]
pub type DOFF_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DOFF` writer - Destination Address Signed offset"]
pub type DOFF_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TCD_DOFF_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Destination Address Signed offset"]
    #[inline(always)]
    pub fn doff(&self) -> DOFF_R {
        DOFF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Destination Address Signed offset"]
    #[inline(always)]
    #[must_use]
    pub fn doff(&mut self) -> DOFF_W<0> {
        DOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Signed Destination Address Offset\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_doff](index.html) module"]
pub struct TCD_DOFF_SPEC;
impl crate::RegisterSpec for TCD_DOFF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tcd_doff::R](R) reader structure"]
impl crate::Readable for TCD_DOFF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcd_doff::W](W) writer structure"]
impl crate::Writable for TCD_DOFF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCD%s_DOFF to value 0"]
impl crate::Resettable for TCD_DOFF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
