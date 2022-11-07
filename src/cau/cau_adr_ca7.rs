#[doc = "Register `CAU_ADR_CA7` writer"]
pub struct W(crate::W<CAU_ADR_CA7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAU_ADR_CA7_SPEC>;
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
impl From<crate::W<CAU_ADR_CA7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAU_ADR_CA7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CA7` writer - CA7"]
pub type CA7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAU_ADR_CA7_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - CA7"]
    #[inline(always)]
    #[must_use]
    pub fn ca7(&mut self) -> CA7_W<0> {
        CA7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose Register 7 - Add to register command\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cau_adr_ca7](index.html) module"]
pub struct CAU_ADR_CA7_SPEC;
impl crate::RegisterSpec for CAU_ADR_CA7_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cau_adr_ca7::W](W) writer structure"]
impl crate::Writable for CAU_ADR_CA7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAU_ADR_CA7 to value 0"]
impl crate::Resettable for CAU_ADR_CA7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
