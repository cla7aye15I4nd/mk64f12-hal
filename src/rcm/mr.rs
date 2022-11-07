#[doc = "Register `MR` reader"]
pub struct R(crate::R<MR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `EZP_MS` reader - EZP_MS_B pin state"]
pub type EZP_MS_R = crate::BitReader<EZP_MS_A>;
#[doc = "EZP_MS_B pin state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EZP_MS_A {
    #[doc = "0: Pin deasserted (logic 1)"]
    _0 = 0,
    #[doc = "1: Pin asserted (logic 0)"]
    _1 = 1,
}
impl From<EZP_MS_A> for bool {
    #[inline(always)]
    fn from(variant: EZP_MS_A) -> Self {
        variant as u8 != 0
    }
}
impl EZP_MS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EZP_MS_A {
        match self.bits {
            false => EZP_MS_A::_0,
            true => EZP_MS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EZP_MS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EZP_MS_A::_1
    }
}
impl R {
    #[doc = "Bit 1 - EZP_MS_B pin state"]
    #[inline(always)]
    pub fn ezp_ms(&self) -> EZP_MS_R {
        EZP_MS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mr](index.html) module"]
pub struct MR_SPEC;
impl crate::RegisterSpec for MR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [mr::R](R) reader structure"]
impl crate::Readable for MR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MR to value 0"]
impl crate::Resettable for MR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
