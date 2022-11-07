#[doc = "Register `RDSR` reader"]
pub struct R(crate::R<RDSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RDSR` writer"]
pub struct W(crate::W<RDSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RDSR_SPEC>;
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
impl From<crate::W<RDSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RDSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R_DES_START` reader - Pointer to the beginning of the receive buffer descriptor queue."]
pub type R_DES_START_R = crate::FieldReader<u32, u32>;
#[doc = "Field `R_DES_START` writer - Pointer to the beginning of the receive buffer descriptor queue."]
pub type R_DES_START_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RDSR_SPEC, u32, u32, 29, O>;
impl R {
    #[doc = "Bits 3:31 - Pointer to the beginning of the receive buffer descriptor queue."]
    #[inline(always)]
    pub fn r_des_start(&self) -> R_DES_START_R {
        R_DES_START_R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - Pointer to the beginning of the receive buffer descriptor queue."]
    #[inline(always)]
    #[must_use]
    pub fn r_des_start(&mut self) -> R_DES_START_W<3> {
        R_DES_START_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Descriptor Ring Start Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdsr](index.html) module"]
pub struct RDSR_SPEC;
impl crate::RegisterSpec for RDSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdsr::R](R) reader structure"]
impl crate::Readable for RDSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rdsr::W](W) writer structure"]
impl crate::Writable for RDSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RDSR to value 0"]
impl crate::Resettable for RDSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
