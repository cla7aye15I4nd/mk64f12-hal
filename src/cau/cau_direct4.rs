#[doc = "Register `CAU_DIRECT4` writer"]
pub struct W(crate::W<CAU_DIRECT4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAU_DIRECT4_SPEC>;
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
impl From<crate::W<CAU_DIRECT4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAU_DIRECT4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAU_DIRECT4` writer - Direct register 4"]
pub type CAU_DIRECT4_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAU_DIRECT4_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Direct register 4"]
    #[inline(always)]
    #[must_use]
    pub fn cau_direct4(&mut self) -> CAU_DIRECT4_W<0> {
        CAU_DIRECT4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Direct access register 4\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cau_direct4](index.html) module"]
pub struct CAU_DIRECT4_SPEC;
impl crate::RegisterSpec for CAU_DIRECT4_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cau_direct4::W](W) writer structure"]
impl crate::Writable for CAU_DIRECT4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAU_DIRECT4 to value 0"]
impl crate::Resettable for CAU_DIRECT4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
