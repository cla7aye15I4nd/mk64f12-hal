#[doc = "Register `PID` reader"]
pub struct R(crate::R<PID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PID` writer"]
pub struct W(crate::W<PID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PID_SPEC>;
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
impl From<crate::W<PID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PID` reader - M0_PID And M1_PID For MPU"]
pub type PID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PID` writer - M0_PID And M1_PID For MPU"]
pub type PID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PID_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - M0_PID And M1_PID For MPU"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - M0_PID And M1_PID For MPU"]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PID_W<0> {
        PID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Process ID register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pid](index.html) module"]
pub struct PID_SPEC;
impl crate::RegisterSpec for PID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pid::R](R) reader structure"]
impl crate::Readable for PID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pid::W](W) writer structure"]
impl crate::Writable for PID_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PID to value 0"]
impl crate::Resettable for PID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
