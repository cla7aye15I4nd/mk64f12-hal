#[doc = "Register `TDSR` reader"]
pub struct R(crate::R<TDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TDSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TDSR` writer"]
pub struct W(crate::W<TDSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TDSR_SPEC>;
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
impl From<crate::W<TDSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TDSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `X_DES_START` reader - Pointer to the beginning of the transmit buffer descriptor queue."]
pub type X_DES_START_R = crate::FieldReader<u32, u32>;
#[doc = "Field `X_DES_START` writer - Pointer to the beginning of the transmit buffer descriptor queue."]
pub type X_DES_START_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TDSR_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 3:31 - Pointer to the beginning of the transmit buffer descriptor queue."]
    #[inline(always)]
    pub fn x_des_start(&self) -> X_DES_START_R {
        X_DES_START_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - Pointer to the beginning of the transmit buffer descriptor queue."]
    #[inline(always)]
    #[must_use]
    pub fn x_des_start(&mut self) -> X_DES_START_W<3> {
        X_DES_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Buffer Descriptor Ring Start Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdsr](index.html) module"]
pub struct TDSR_SPEC;
impl crate::RegisterSpec for TDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tdsr::R](R) reader structure"]
impl crate::Readable for TDSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tdsr::W](W) writer structure"]
impl crate::Writable for TDSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TDSR to value 0"]
impl crate::Resettable for TDSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
