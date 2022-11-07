#[doc = "Register `DAT%sL` reader"]
pub struct R(crate::R<DATL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAT%sL` writer"]
pub struct W(crate::W<DATL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATL_SPEC>;
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
impl From<crate::W<DATL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0` reader - When the DAC buffer is not enabled, DATA\\[11:0\\]
controls the output voltage based on the following formula: V out = V in * (1 + DACDAT0\\[11:0\\])/4096 When the DAC buffer is enabled, DATA is mapped to the 16-word buffer"]
pub type DATA0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA0` writer - When the DAC buffer is not enabled, DATA\\[11:0\\]
controls the output voltage based on the following formula: V out = V in * (1 + DACDAT0\\[11:0\\])/4096 When the DAC buffer is enabled, DATA is mapped to the 16-word buffer"]
pub type DATA0_W<'a, const O: u8> = crate::FieldWriter<'a, u8, DATL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - When the DAC buffer is not enabled, DATA\\[11:0\\]
controls the output voltage based on the following formula: V out = V in * (1 + DACDAT0\\[11:0\\])/4096 When the DAC buffer is enabled, DATA is mapped to the 16-word buffer"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - When the DAC buffer is not enabled, DATA\\[11:0\\]
controls the output voltage based on the following formula: V out = V in * (1 + DACDAT0\\[11:0\\])/4096 When the DAC buffer is enabled, DATA is mapped to the 16-word buffer"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> DATA0_W<0> {
        DATA0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Data Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [datl](index.html) module"]
pub struct DATL_SPEC;
impl crate::RegisterSpec for DATL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [datl::R](R) reader structure"]
impl crate::Readable for DATL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [datl::W](W) writer structure"]
impl crate::Writable for DATL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAT%sL to value 0"]
impl crate::Resettable for DATL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
