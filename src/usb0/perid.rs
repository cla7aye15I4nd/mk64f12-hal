#[doc = "Register `PERID` reader"]
pub struct R(crate::R<PERID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ID` reader - Peripheral Identification"]
pub type ID_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Peripheral Identification"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits & 0x3f)
    }
}
#[doc = "Peripheral ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perid](index.html) module"]
pub struct PERID_SPEC;
impl crate::RegisterSpec for PERID_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [perid::R](R) reader structure"]
impl crate::Readable for PERID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PERID to value 0x04"]
impl crate::Resettable for PERID_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
