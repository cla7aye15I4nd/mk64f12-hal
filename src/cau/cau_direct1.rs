#[doc = "Register `CAU_DIRECT1` writer"]
pub struct W(crate::W<CAU_DIRECT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAU_DIRECT1_SPEC>;
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
impl From<crate::W<CAU_DIRECT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAU_DIRECT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAU_DIRECT1` writer - Direct register 1"]
pub type CAU_DIRECT1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAU_DIRECT1_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Direct register 1"]
    #[inline(always)]
    #[must_use]
    pub fn cau_direct1(&mut self) -> CAU_DIRECT1_W<0> {
        CAU_DIRECT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Direct access register 1\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cau_direct1](index.html) module"]
pub struct CAU_DIRECT1_SPEC;
impl crate::RegisterSpec for CAU_DIRECT1_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cau_direct1::W](W) writer structure"]
impl crate::Writable for CAU_DIRECT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAU_DIRECT1 to value 0"]
impl crate::Resettable for CAU_DIRECT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
