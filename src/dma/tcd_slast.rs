#[doc = "Register `TCD%s_SLAST` reader"]
pub struct R(crate::R<TCD_SLAST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCD_SLAST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCD_SLAST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCD_SLAST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD%s_SLAST` writer"]
pub struct W(crate::W<TCD_SLAST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCD_SLAST_SPEC>;
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
impl From<crate::W<TCD_SLAST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCD_SLAST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLAST` reader - Last source Address Adjustment"]
pub type SLAST_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SLAST` writer - Last source Address Adjustment"]
pub type SLAST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCD_SLAST_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Last source Address Adjustment"]
    #[inline(always)]
    pub fn slast(&self) -> SLAST_R {
        SLAST_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Last source Address Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn slast(&mut self) -> SLAST_W<0> {
        SLAST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Last Source Address Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_slast](index.html) module"]
pub struct TCD_SLAST_SPEC;
impl crate::RegisterSpec for TCD_SLAST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcd_slast::R](R) reader structure"]
impl crate::Readable for TCD_SLAST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcd_slast::W](W) writer structure"]
impl crate::Writable for TCD_SLAST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCD%s_SLAST to value 0"]
impl crate::Resettable for TCD_SLAST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
