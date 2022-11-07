#[doc = "Register `CAU_AESC_CA0` writer"]
pub struct W(crate::W<CAU_AESC_CA0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAU_AESC_CA0_SPEC>;
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
impl From<crate::W<CAU_AESC_CA0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAU_AESC_CA0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CA0` writer - CA0"]
pub type CA0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAU_AESC_CA0_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - CA0"]
    #[inline(always)]
    #[must_use]
    pub fn ca0(&mut self) -> CA0_W<0> {
        CA0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose Register 0 - AES Column Operation command\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cau_aesc_ca0](index.html) module"]
pub struct CAU_AESC_CA0_SPEC;
impl crate::RegisterSpec for CAU_AESC_CA0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cau_aesc_ca0::W](W) writer structure"]
impl crate::Writable for CAU_AESC_CA0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAU_AESC_CA0 to value 0"]
impl crate::Resettable for CAU_AESC_CA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
