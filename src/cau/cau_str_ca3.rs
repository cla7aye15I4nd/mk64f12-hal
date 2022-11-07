#[doc = "Register `CAU_STR_CA3` reader"]
pub struct R(crate::R<CAU_STR_CA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAU_STR_CA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAU_STR_CA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAU_STR_CA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CA3` reader - CA3"]
pub type CA3_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CA3"]
    #[inline(always)]
    pub fn ca3(&self) -> CA3_R {
        CA3_R::new(self.bits)
    }
}
#[doc = "General Purpose Register 3 - Store Register command\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cau_str_ca3](index.html) module"]
pub struct CAU_STR_CA3_SPEC;
impl crate::RegisterSpec for CAU_STR_CA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cau_str_ca3::R](R) reader structure"]
impl crate::Readable for CAU_STR_CA3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAU_STR_CA3 to value 0"]
impl crate::Resettable for CAU_STR_CA3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
