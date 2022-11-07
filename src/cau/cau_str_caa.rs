#[doc = "Register `CAU_STR_CAA` reader"]
pub struct R(crate::R<CAU_STR_CAA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAU_STR_CAA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAU_STR_CAA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAU_STR_CAA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACC` reader - ACC"]
pub type ACC_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ACC"]
    #[inline(always)]
    pub fn acc(&self) -> ACC_R {
        ACC_R::new(self.bits)
    }
}
#[doc = "Accumulator register - Store Register command\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cau_str_caa](index.html) module"]
pub struct CAU_STR_CAA_SPEC;
impl crate::RegisterSpec for CAU_STR_CAA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cau_str_caa::R](R) reader structure"]
impl crate::Readable for CAU_STR_CAA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAU_STR_CAA to value 0"]
impl crate::Resettable for CAU_STR_CAA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
