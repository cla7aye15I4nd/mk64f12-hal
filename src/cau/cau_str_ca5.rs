#[doc = "Register `CAU_STR_CA5` reader"]
pub struct R(crate::R<CAU_STR_CA5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAU_STR_CA5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAU_STR_CA5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAU_STR_CA5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CA5` reader - CA5"]
pub type CA5_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CA5"]
    #[inline(always)]
    pub fn ca5(&self) -> CA5_R {
        CA5_R::new(self.bits)
    }
}
#[doc = "General Purpose Register 5 - Store Register command\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cau_str_ca5](index.html) module"]
pub struct CAU_STR_CA5_SPEC;
impl crate::RegisterSpec for CAU_STR_CA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cau_str_ca5::R](R) reader structure"]
impl crate::Readable for CAU_STR_CA5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAU_STR_CA5 to value 0"]
impl crate::Resettable for CAU_STR_CA5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
