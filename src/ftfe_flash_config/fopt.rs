#[doc = "Register `FOPT` reader"]
pub struct R(crate::R<FOPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FOPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FOPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FOPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LPBOOT` reader - no description available"]
pub type LPBOOT_R = crate::BitReader<LPBOOT_A>;
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPBOOT_A {
    #[doc = "0: Low-power boot"]
    _00 = 0,
    #[doc = "1: Normal boot"]
    _01 = 1,
}
impl From<LPBOOT_A> for bool {
    #[inline(always)]
    fn from(variant: LPBOOT_A) -> Self {
        variant as u8 != 0
    }
}
impl LPBOOT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPBOOT_A {
        match self.bits {
            false => LPBOOT_A::_00,
            true => LPBOOT_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LPBOOT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LPBOOT_A::_01
    }
}
#[doc = "Field `EZPORT_DIS` reader - no description available"]
pub type EZPORT_DIS_R = crate::BitReader<EZPORT_DIS_A>;
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EZPORT_DIS_A {
    #[doc = "0: EzPort operation is disabled"]
    _00 = 0,
    #[doc = "1: EzPort operation is enabled"]
    _01 = 1,
}
impl From<EZPORT_DIS_A> for bool {
    #[inline(always)]
    fn from(variant: EZPORT_DIS_A) -> Self {
        variant as u8 != 0
    }
}
impl EZPORT_DIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EZPORT_DIS_A {
        match self.bits {
            false => EZPORT_DIS_A::_00,
            true => EZPORT_DIS_A::_01,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == EZPORT_DIS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == EZPORT_DIS_A::_01
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn lpboot(&self) -> LPBOOT_R {
        LPBOOT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn ezport_dis(&self) -> EZPORT_DIS_R {
        EZPORT_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Non-volatile Flash Option Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fopt](index.html) module"]
pub struct FOPT_SPEC;
impl crate::RegisterSpec for FOPT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fopt::R](R) reader structure"]
impl crate::Readable for FOPT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FOPT to value 0xff"]
impl crate::Resettable for FOPT_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
