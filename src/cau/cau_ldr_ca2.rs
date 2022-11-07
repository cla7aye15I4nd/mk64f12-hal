#[doc = "Register `CAU_LDR_CA2` writer"]
pub struct W(crate::W<CAU_LDR_CA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAU_LDR_CA2_SPEC>;
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
impl From<crate::W<CAU_LDR_CA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAU_LDR_CA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CA2` writer - CA2"]
pub type CA2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAU_LDR_CA2_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - CA2"]
    #[inline(always)]
    #[must_use]
    pub fn ca2(&mut self) -> CA2_W<0> {
        CA2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose Register 2 - Load Register command\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cau_ldr_ca2](index.html) module"]
pub struct CAU_LDR_CA2_SPEC;
impl crate::RegisterSpec for CAU_LDR_CA2_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cau_ldr_ca2::W](W) writer structure"]
impl crate::Writable for CAU_LDR_CA2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAU_LDR_CA2 to value 0"]
impl crate::Resettable for CAU_LDR_CA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
