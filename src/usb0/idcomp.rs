#[doc = "Register `IDCOMP` reader"]
pub struct R(crate::R<IDCOMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDCOMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDCOMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDCOMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NID` reader - Ones' complement of PERID\\[ID\\]. bits."]
pub type NID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Ones' complement of PERID\\[ID\\]. bits."]
    #[inline(always)]
    pub fn nid(&self) -> NID_R {
        NID_R::new(self.bits & 0x3f)
    }
}
#[doc = "Peripheral ID Complement register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idcomp](index.html) module"]
pub struct IDCOMP_SPEC;
impl crate::RegisterSpec for IDCOMP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [idcomp::R](R) reader structure"]
impl crate::Readable for IDCOMP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IDCOMP to value 0xfb"]
impl crate::Resettable for IDCOMP_SPEC {
    const RESET_VALUE: Self::Ux = 0xfb;
}
