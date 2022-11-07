#[doc = "Register `DAT%sH` reader"]
pub struct R(crate::R<DATH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAT%sH` writer"]
pub struct W(crate::W<DATH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATH_SPEC>;
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
impl From<crate::W<DATH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA1` reader - When the DAC Buffer is not enabled, DATA\\[11:0\\]
controls the output voltage based on the following formula"]
pub type DATA1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA1` writer - When the DAC Buffer is not enabled, DATA\\[11:0\\]
controls the output voltage based on the following formula"]
pub type DATA1_W<'a, const O: u8> = crate::FieldWriter<'a, u8, DATH_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - When the DAC Buffer is not enabled, DATA\\[11:0\\]
controls the output voltage based on the following formula"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - When the DAC Buffer is not enabled, DATA\\[11:0\\]
controls the output voltage based on the following formula"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> DATA1_W<0> {
        DATA1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Data High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dath](index.html) module"]
pub struct DATH_SPEC;
impl crate::RegisterSpec for DATH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dath::R](R) reader structure"]
impl crate::Readable for DATH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dath::W](W) writer structure"]
impl crate::Writable for DATH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAT%sH to value 0"]
impl crate::Resettable for DATH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
