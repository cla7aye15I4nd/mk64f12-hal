#[doc = "Register `SYSCTL` reader"]
pub struct R(crate::R<SYSCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCTL` writer"]
pub struct W(crate::W<SYSCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SYSCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPGEN` reader - IPG Clock Enable"]
pub type IPGEN_R = crate::BitReader<IPGEN_A>;
#[doc = "IPG Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPGEN_A {
    #[doc = "0: Bus clock will be internally gated off."]
    _0 = 0,
    #[doc = "1: Bus clock will not be automatically gated off."]
    _1 = 1,
}
impl From<IPGEN_A> for bool {
    #[inline(always)]
    fn from(variant: IPGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IPGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPGEN_A {
        match self.bits {
            false => IPGEN_A::_0,
            true => IPGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPGEN_A::_1
    }
}
#[doc = "Field `IPGEN` writer - IPG Clock Enable"]
pub type IPGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTL_SPEC, IPGEN_A, O>;
impl<'a, const O: u8> IPGEN_W<'a, O> {
    #[doc = "Bus clock will be internally gated off."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPGEN_A::_0)
    }
    #[doc = "Bus clock will not be automatically gated off."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPGEN_A::_1)
    }
}
#[doc = "Field `HCKEN` reader - System Clock Enable"]
pub type HCKEN_R = crate::BitReader<HCKEN_A>;
#[doc = "System Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HCKEN_A {
    #[doc = "0: System clock will be internally gated off."]
    _0 = 0,
    #[doc = "1: System clock will not be automatically gated off."]
    _1 = 1,
}
impl From<HCKEN_A> for bool {
    #[inline(always)]
    fn from(variant: HCKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl HCKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HCKEN_A {
        match self.bits {
            false => HCKEN_A::_0,
            true => HCKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HCKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HCKEN_A::_1
    }
}
#[doc = "Field `HCKEN` writer - System Clock Enable"]
pub type HCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTL_SPEC, HCKEN_A, O>;
impl<'a, const O: u8> HCKEN_W<'a, O> {
    #[doc = "System clock will be internally gated off."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HCKEN_A::_0)
    }
    #[doc = "System clock will not be automatically gated off."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HCKEN_A::_1)
    }
}
#[doc = "Field `PEREN` reader - Peripheral Clock Enable"]
pub type PEREN_R = crate::BitReader<PEREN_A>;
#[doc = "Peripheral Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEREN_A {
    #[doc = "0: SDHC clock will be internally gated off."]
    _0 = 0,
    #[doc = "1: SDHC clock will not be automatically gated off."]
    _1 = 1,
}
impl From<PEREN_A> for bool {
    #[inline(always)]
    fn from(variant: PEREN_A) -> Self {
        variant as u8 != 0
    }
}
impl PEREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEREN_A {
        match self.bits {
            false => PEREN_A::_0,
            true => PEREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEREN_A::_1
    }
}
#[doc = "Field `PEREN` writer - Peripheral Clock Enable"]
pub type PEREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTL_SPEC, PEREN_A, O>;
impl<'a, const O: u8> PEREN_W<'a, O> {
    #[doc = "SDHC clock will be internally gated off."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PEREN_A::_0)
    }
    #[doc = "SDHC clock will not be automatically gated off."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PEREN_A::_1)
    }
}
#[doc = "Field `SDCLKEN` reader - SD Clock Enable"]
pub type SDCLKEN_R = crate::BitReader<bool>;
#[doc = "Field `SDCLKEN` writer - SD Clock Enable"]
pub type SDCLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTL_SPEC, bool, O>;
#[doc = "Field `DVS` reader - Divisor"]
pub type DVS_R = crate::FieldReader<u8, DVS_A>;
#[doc = "Divisor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DVS_A {
    #[doc = "0: Divisor by 1."]
    _0 = 0,
    #[doc = "1: Divisor by 2."]
    _1 = 1,
    #[doc = "14: Divisor by 15."]
    _1110 = 14,
    #[doc = "15: Divisor by 16."]
    _1111 = 15,
}
impl From<DVS_A> for u8 {
    #[inline(always)]
    fn from(variant: DVS_A) -> Self {
        variant as _
    }
}
impl DVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DVS_A> {
        match self.bits {
            0 => Some(DVS_A::_0),
            1 => Some(DVS_A::_1),
            14 => Some(DVS_A::_1110),
            15 => Some(DVS_A::_1111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DVS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DVS_A::_1
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == DVS_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == DVS_A::_1111
    }
}
#[doc = "Field `DVS` writer - Divisor"]
pub type DVS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSCTL_SPEC, u8, DVS_A, 4, O>;
impl<'a, const O: u8> DVS_W<'a, O> {
    #[doc = "Divisor by 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DVS_A::_0)
    }
    #[doc = "Divisor by 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DVS_A::_1)
    }
    #[doc = "Divisor by 15."]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(DVS_A::_1110)
    }
    #[doc = "Divisor by 16."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(DVS_A::_1111)
    }
}
#[doc = "Field `SDCLKFS` reader - SDCLK Frequency Select"]
pub type SDCLKFS_R = crate::FieldReader<u8, SDCLKFS_A>;
#[doc = "SDCLK Frequency Select\n\nValue on reset: 128"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDCLKFS_A {
    #[doc = "1: Base clock divided by 2."]
    _1 = 1,
    #[doc = "2: Base clock divided by 4."]
    _10 = 2,
    #[doc = "4: Base clock divided by 8."]
    _100 = 4,
    #[doc = "8: Base clock divided by 16."]
    _1000 = 8,
    #[doc = "16: Base clock divided by 32."]
    _10000 = 16,
    #[doc = "32: Base clock divided by 64."]
    _100000 = 32,
    #[doc = "64: Base clock divided by 128."]
    _1000000 = 64,
    #[doc = "128: Base clock divided by 256."]
    _10000000 = 128,
}
impl From<SDCLKFS_A> for u8 {
    #[inline(always)]
    fn from(variant: SDCLKFS_A) -> Self {
        variant as _
    }
}
impl SDCLKFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SDCLKFS_A> {
        match self.bits {
            1 => Some(SDCLKFS_A::_1),
            2 => Some(SDCLKFS_A::_10),
            4 => Some(SDCLKFS_A::_100),
            8 => Some(SDCLKFS_A::_1000),
            16 => Some(SDCLKFS_A::_10000),
            32 => Some(SDCLKFS_A::_100000),
            64 => Some(SDCLKFS_A::_1000000),
            128 => Some(SDCLKFS_A::_10000000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDCLKFS_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SDCLKFS_A::_10
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SDCLKFS_A::_100
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == SDCLKFS_A::_1000
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == SDCLKFS_A::_10000
    }
    #[doc = "Checks if the value of the field is `_100000`"]
    #[inline(always)]
    pub fn is_100000(&self) -> bool {
        *self == SDCLKFS_A::_100000
    }
    #[doc = "Checks if the value of the field is `_1000000`"]
    #[inline(always)]
    pub fn is_1000000(&self) -> bool {
        *self == SDCLKFS_A::_1000000
    }
    #[doc = "Checks if the value of the field is `_10000000`"]
    #[inline(always)]
    pub fn is_10000000(&self) -> bool {
        *self == SDCLKFS_A::_10000000
    }
}
#[doc = "Field `SDCLKFS` writer - SDCLK Frequency Select"]
pub type SDCLKFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSCTL_SPEC, u8, SDCLKFS_A, 8, O>;
impl<'a, const O: u8> SDCLKFS_W<'a, O> {
    #[doc = "Base clock divided by 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDCLKFS_A::_1)
    }
    #[doc = "Base clock divided by 4."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SDCLKFS_A::_10)
    }
    #[doc = "Base clock divided by 8."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SDCLKFS_A::_100)
    }
    #[doc = "Base clock divided by 16."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(SDCLKFS_A::_1000)
    }
    #[doc = "Base clock divided by 32."]
    #[inline(always)]
    pub fn _10000(self) -> &'a mut W {
        self.variant(SDCLKFS_A::_10000)
    }
    #[doc = "Base clock divided by 64."]
    #[inline(always)]
    pub fn _100000(self) -> &'a mut W {
        self.variant(SDCLKFS_A::_100000)
    }
    #[doc = "Base clock divided by 128."]
    #[inline(always)]
    pub fn _1000000(self) -> &'a mut W {
        self.variant(SDCLKFS_A::_1000000)
    }
    #[doc = "Base clock divided by 256."]
    #[inline(always)]
    pub fn _10000000(self) -> &'a mut W {
        self.variant(SDCLKFS_A::_10000000)
    }
}
#[doc = "Field `DTOCV` reader - Data Timeout Counter Value"]
pub type DTOCV_R = crate::FieldReader<u8, DTOCV_A>;
#[doc = "Data Timeout Counter Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTOCV_A {
    #[doc = "0: SDCLK x 2 13"]
    _0000 = 0,
    #[doc = "1: SDCLK x 2 14"]
    _0001 = 1,
    #[doc = "14: SDCLK x 2 27"]
    _1110 = 14,
}
impl From<DTOCV_A> for u8 {
    #[inline(always)]
    fn from(variant: DTOCV_A) -> Self {
        variant as _
    }
}
impl DTOCV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTOCV_A> {
        match self.bits {
            0 => Some(DTOCV_A::_0000),
            1 => Some(DTOCV_A::_0001),
            14 => Some(DTOCV_A::_1110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DTOCV_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == DTOCV_A::_0001
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == DTOCV_A::_1110
    }
}
#[doc = "Field `DTOCV` writer - Data Timeout Counter Value"]
pub type DTOCV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SYSCTL_SPEC, u8, DTOCV_A, 4, O>;
impl<'a, const O: u8> DTOCV_W<'a, O> {
    #[doc = "SDCLK x 2 13"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(DTOCV_A::_0000)
    }
    #[doc = "SDCLK x 2 14"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(DTOCV_A::_0001)
    }
    #[doc = "SDCLK x 2 27"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(DTOCV_A::_1110)
    }
}
#[doc = "Software Reset For ALL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTA_AW {
    #[doc = "0: No reset."]
    _0 = 0,
    #[doc = "1: Reset."]
    _1 = 1,
}
impl From<RSTA_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTA_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTA` writer - Software Reset For ALL"]
pub type RSTA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTL_SPEC, RSTA_AW, O>;
impl<'a, const O: u8> RSTA_W<'a, O> {
    #[doc = "No reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTA_AW::_0)
    }
    #[doc = "Reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTA_AW::_1)
    }
}
#[doc = "Software Reset For CMD Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTC_AW {
    #[doc = "0: No reset."]
    _0 = 0,
    #[doc = "1: Reset."]
    _1 = 1,
}
impl From<RSTC_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTC` writer - Software Reset For CMD Line"]
pub type RSTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTL_SPEC, RSTC_AW, O>;
impl<'a, const O: u8> RSTC_W<'a, O> {
    #[doc = "No reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTC_AW::_0)
    }
    #[doc = "Reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTC_AW::_1)
    }
}
#[doc = "Software Reset For DAT Line\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTD_AW {
    #[doc = "0: No reset."]
    _0 = 0,
    #[doc = "1: Reset."]
    _1 = 1,
}
impl From<RSTD_AW> for bool {
    #[inline(always)]
    fn from(variant: RSTD_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTD` writer - Software Reset For DAT Line"]
pub type RSTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTL_SPEC, RSTD_AW, O>;
impl<'a, const O: u8> RSTD_W<'a, O> {
    #[doc = "No reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSTD_AW::_0)
    }
    #[doc = "Reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSTD_AW::_1)
    }
}
#[doc = "Field `INITA` reader - Initialization Active"]
pub type INITA_R = crate::BitReader<bool>;
#[doc = "Field `INITA` writer - Initialization Active"]
pub type INITA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYSCTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - IPG Clock Enable"]
    #[inline(always)]
    pub fn ipgen(&self) -> IPGEN_R {
        IPGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System Clock Enable"]
    #[inline(always)]
    pub fn hcken(&self) -> HCKEN_R {
        HCKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Peripheral Clock Enable"]
    #[inline(always)]
    pub fn peren(&self) -> PEREN_R {
        PEREN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SD Clock Enable"]
    #[inline(always)]
    pub fn sdclken(&self) -> SDCLKEN_R {
        SDCLKEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Divisor"]
    #[inline(always)]
    pub fn dvs(&self) -> DVS_R {
        DVS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    pub fn sdclkfs(&self) -> SDCLKFS_R {
        SDCLKFS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline(always)]
    pub fn dtocv(&self) -> DTOCV_R {
        DTOCV_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - Initialization Active"]
    #[inline(always)]
    pub fn inita(&self) -> INITA_R {
        INITA_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IPG Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ipgen(&mut self) -> IPGEN_W<0> {
        IPGEN_W::new(self)
    }
    #[doc = "Bit 1 - System Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hcken(&mut self) -> HCKEN_W<1> {
        HCKEN_W::new(self)
    }
    #[doc = "Bit 2 - Peripheral Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn peren(&mut self) -> PEREN_W<2> {
        PEREN_W::new(self)
    }
    #[doc = "Bit 3 - SD Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdclken(&mut self) -> SDCLKEN_W<3> {
        SDCLKEN_W::new(self)
    }
    #[doc = "Bits 4:7 - Divisor"]
    #[inline(always)]
    #[must_use]
    pub fn dvs(&mut self) -> DVS_W<4> {
        DVS_W::new(self)
    }
    #[doc = "Bits 8:15 - SDCLK Frequency Select"]
    #[inline(always)]
    #[must_use]
    pub fn sdclkfs(&mut self) -> SDCLKFS_W<8> {
        SDCLKFS_W::new(self)
    }
    #[doc = "Bits 16:19 - Data Timeout Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn dtocv(&mut self) -> DTOCV_W<16> {
        DTOCV_W::new(self)
    }
    #[doc = "Bit 24 - Software Reset For ALL"]
    #[inline(always)]
    #[must_use]
    pub fn rsta(&mut self) -> RSTA_W<24> {
        RSTA_W::new(self)
    }
    #[doc = "Bit 25 - Software Reset For CMD Line"]
    #[inline(always)]
    #[must_use]
    pub fn rstc(&mut self) -> RSTC_W<25> {
        RSTC_W::new(self)
    }
    #[doc = "Bit 26 - Software Reset For DAT Line"]
    #[inline(always)]
    #[must_use]
    pub fn rstd(&mut self) -> RSTD_W<26> {
        RSTD_W::new(self)
    }
    #[doc = "Bit 27 - Initialization Active"]
    #[inline(always)]
    #[must_use]
    pub fn inita(&mut self) -> INITA_W<27> {
        INITA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sysctl](index.html) module"]
pub struct SYSCTL_SPEC;
impl crate::RegisterSpec for SYSCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sysctl::R](R) reader structure"]
impl crate::Readable for SYSCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sysctl::W](W) writer structure"]
impl crate::Writable for SYSCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCTL to value 0x8008"]
impl crate::Resettable for SYSCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8008;
}
