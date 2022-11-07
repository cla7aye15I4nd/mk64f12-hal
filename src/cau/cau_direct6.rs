#[doc = "Register `CAU_DIRECT6` writer"]
pub struct W(crate::W<CAU_DIRECT6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAU_DIRECT6_SPEC>;
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
impl From<crate::W<CAU_DIRECT6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAU_DIRECT6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAU_DIRECT6` writer - Direct register 6"]
pub type CAU_DIRECT6_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAU_DIRECT6_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Direct register 6"]
    #[inline(always)]
    #[must_use]
    pub fn cau_direct6(&mut self) -> CAU_DIRECT6_W<0> {
        CAU_DIRECT6_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Direct access register 6\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cau_direct6](index.html) module"]
pub struct CAU_DIRECT6_SPEC;
impl crate::RegisterSpec for CAU_DIRECT6_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cau_direct6::W](W) writer structure"]
impl crate::Writable for CAU_DIRECT6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAU_DIRECT6 to value 0"]
impl crate::Resettable for CAU_DIRECT6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
