#[doc = "Register `IEEE_T_DEF` reader"]
pub struct R(crate::R<IEEE_T_DEF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEEE_T_DEF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEEE_T_DEF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEEE_T_DEF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COUNT` reader - Frame count"]
pub type COUNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Frame count"]
    #[inline(always)]
    pub fn count(&self) -> COUNT_R {
        COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Frames Transmitted after Deferral Delay Statistic Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ieee_t_def](index.html) module"]
pub struct IEEE_T_DEF_SPEC;
impl crate::RegisterSpec for IEEE_T_DEF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ieee_t_def::R](R) reader structure"]
impl crate::Readable for IEEE_T_DEF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IEEE_T_DEF to value 0"]
impl crate::Resettable for IEEE_T_DEF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
