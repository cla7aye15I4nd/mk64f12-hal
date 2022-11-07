#[doc = "Register `CAU_STR_CA2` reader"]
pub struct R(crate::R<CAU_STR_CA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAU_STR_CA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAU_STR_CA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAU_STR_CA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CA2` reader - CA2"]
pub type CA2_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - CA2"]
    #[inline(always)]
    pub fn ca2(&self) -> CA2_R {
        CA2_R::new(self.bits)
    }
}
#[doc = "General Purpose Register 2 - Store Register command\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cau_str_ca2](index.html) module"]
pub struct CAU_STR_CA2_SPEC;
impl crate::RegisterSpec for CAU_STR_CA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cau_str_ca2::R](R) reader structure"]
impl crate::Readable for CAU_STR_CA2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAU_STR_CA2 to value 0"]
impl crate::Resettable for CAU_STR_CA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
