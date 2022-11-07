#[doc = "Register `CAU_AESIC_CA5` writer"]
pub struct W(crate::W<CAU_AESIC_CA5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAU_AESIC_CA5_SPEC>;
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
impl From<crate::W<CAU_AESIC_CA5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAU_AESIC_CA5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CA5` writer - CA5"]
pub type CA5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAU_AESIC_CA5_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - CA5"]
    #[inline(always)]
    #[must_use]
    pub fn ca5(&mut self) -> CA5_W<0> {
        CA5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose Register 5 - AES Inverse Column Operation command\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cau_aesic_ca5](index.html) module"]
pub struct CAU_AESIC_CA5_SPEC;
impl crate::RegisterSpec for CAU_AESIC_CA5_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cau_aesic_ca5::W](W) writer structure"]
impl crate::Writable for CAU_AESIC_CA5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAU_AESIC_CA5 to value 0"]
impl crate::Resettable for CAU_AESIC_CA5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
