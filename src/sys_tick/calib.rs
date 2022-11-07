#[doc = "Register `CALIB` reader"]
pub struct R(crate::R<CALIB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALIB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALIB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALIB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TENMS` reader - Reload value to use for 10ms timing"]
pub type TENMS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SKEW` reader - no description available"]
pub type SKEW_R = crate::BitReader<SKEW_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SKEW_A {
    #[doc = "0: 10ms calibration value is exact"]
    _0 = 0,
    #[doc = "1: 10ms calibration value is inexact, because of the clock frequency"]
    _1 = 1,
}
impl From<SKEW_A> for bool {
    #[inline(always)]
    fn from(variant: SKEW_A) -> Self {
        variant as u8 != 0
    }
}
impl SKEW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SKEW_A {
        match self.bits {
            false => SKEW_A::_0,
            true => SKEW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SKEW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SKEW_A::_1
    }
}
#[doc = "Field `NOREF` reader - no description available"]
pub type NOREF_R = crate::BitReader<NOREF_A>;
#[doc = "no description available\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOREF_A {
    #[doc = "0: The reference clock is provided"]
    _0 = 0,
    #[doc = "1: The reference clock is not provided"]
    _1 = 1,
}
impl From<NOREF_A> for bool {
    #[inline(always)]
    fn from(variant: NOREF_A) -> Self {
        variant as u8 != 0
    }
}
impl NOREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOREF_A {
        match self.bits {
            false => NOREF_A::_0,
            true => NOREF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NOREF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NOREF_A::_1
    }
}
impl R {
    #[doc = "Bits 0:23 - Reload value to use for 10ms timing"]
    #[inline(always)]
    pub fn tenms(&self) -> TENMS_R {
        TENMS_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 30 - no description available"]
    #[inline(always)]
    pub fn skew(&self) -> SKEW_R {
        SKEW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - no description available"]
    #[inline(always)]
    pub fn noref(&self) -> NOREF_R {
        NOREF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "SysTick Calibration Value Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calib](index.html) module"]
pub struct CALIB_SPEC;
impl crate::RegisterSpec for CALIB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calib::R](R) reader structure"]
impl crate::Readable for CALIB_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CALIB to value 0x8000_0000"]
impl crate::Resettable for CALIB_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
