#[doc = "Register `SCGC6` reader"]
pub struct R(crate::R<SCGC6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGC6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGC6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGC6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGC6` writer"]
pub struct W(crate::W<SCGC6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGC6_SPEC>;
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
impl From<crate::W<SCGC6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGC6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTF` reader - Flash Memory Clock Gate Control"]
pub type FTF_R = crate::BitReader<FTF_A>;
#[doc = "Flash Memory Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTF_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<FTF_A> for bool {
    #[inline(always)]
    fn from(variant: FTF_A) -> Self {
        variant as u8 != 0
    }
}
impl FTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTF_A {
        match self.bits {
            false => FTF_A::_0,
            true => FTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTF_A::_1
    }
}
#[doc = "Field `FTF` writer - Flash Memory Clock Gate Control"]
pub type FTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, FTF_A, O>;
impl<'a, const O: u8> FTF_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTF_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTF_A::_1)
    }
}
#[doc = "Field `DMAMUX` reader - DMA Mux Clock Gate Control"]
pub type DMAMUX_R = crate::BitReader<DMAMUX_A>;
#[doc = "DMA Mux Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAMUX_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<DMAMUX_A> for bool {
    #[inline(always)]
    fn from(variant: DMAMUX_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAMUX_A {
        match self.bits {
            false => DMAMUX_A::_0,
            true => DMAMUX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAMUX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAMUX_A::_1
    }
}
#[doc = "Field `DMAMUX` writer - DMA Mux Clock Gate Control"]
pub type DMAMUX_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, DMAMUX_A, O>;
impl<'a, const O: u8> DMAMUX_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAMUX_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAMUX_A::_1)
    }
}
#[doc = "Field `FLEXCAN0` reader - FlexCAN0 Clock Gate Control"]
pub type FLEXCAN0_R = crate::BitReader<FLEXCAN0_A>;
#[doc = "FlexCAN0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXCAN0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<FLEXCAN0_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXCAN0_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXCAN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXCAN0_A {
        match self.bits {
            false => FLEXCAN0_A::_0,
            true => FLEXCAN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLEXCAN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLEXCAN0_A::_1
    }
}
#[doc = "Field `FLEXCAN0` writer - FlexCAN0 Clock Gate Control"]
pub type FLEXCAN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, FLEXCAN0_A, O>;
impl<'a, const O: u8> FLEXCAN0_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLEXCAN0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLEXCAN0_A::_1)
    }
}
#[doc = "Field `RNGA` reader - RNGA Clock Gate Control"]
pub type RNGA_R = crate::BitReader<bool>;
#[doc = "Field `RNGA` writer - RNGA Clock Gate Control"]
pub type RNGA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, bool, O>;
#[doc = "Field `SPI0` reader - SPI0 Clock Gate Control"]
pub type SPI0_R = crate::BitReader<SPI0_A>;
#[doc = "SPI0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<SPI0_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_A {
        match self.bits {
            false => SPI0_A::_0,
            true => SPI0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPI0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPI0_A::_1
    }
}
#[doc = "Field `SPI0` writer - SPI0 Clock Gate Control"]
pub type SPI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, SPI0_A, O>;
impl<'a, const O: u8> SPI0_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI0_A::_1)
    }
}
#[doc = "Field `SPI1` reader - SPI1 Clock Gate Control"]
pub type SPI1_R = crate::BitReader<SPI1_A>;
#[doc = "SPI1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI1_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<SPI1_A> for bool {
    #[inline(always)]
    fn from(variant: SPI1_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI1_A {
        match self.bits {
            false => SPI1_A::_0,
            true => SPI1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPI1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPI1_A::_1
    }
}
#[doc = "Field `SPI1` writer - SPI1 Clock Gate Control"]
pub type SPI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, SPI1_A, O>;
impl<'a, const O: u8> SPI1_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI1_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI1_A::_1)
    }
}
#[doc = "Field `I2S` reader - I2S Clock Gate Control"]
pub type I2S_R = crate::BitReader<I2S_A>;
#[doc = "I2S Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2S_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<I2S_A> for bool {
    #[inline(always)]
    fn from(variant: I2S_A) -> Self {
        variant as u8 != 0
    }
}
impl I2S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S_A {
        match self.bits {
            false => I2S_A::_0,
            true => I2S_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == I2S_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == I2S_A::_1
    }
}
#[doc = "Field `I2S` writer - I2S Clock Gate Control"]
pub type I2S_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, I2S_A, O>;
impl<'a, const O: u8> I2S_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2S_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2S_A::_1)
    }
}
#[doc = "Field `CRC` reader - CRC Clock Gate Control"]
pub type CRC_R = crate::BitReader<CRC_A>;
#[doc = "CRC Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRC_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<CRC_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_A) -> Self {
        variant as u8 != 0
    }
}
impl CRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_A {
        match self.bits {
            false => CRC_A::_0,
            true => CRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRC_A::_1
    }
}
#[doc = "Field `CRC` writer - CRC Clock Gate Control"]
pub type CRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, CRC_A, O>;
impl<'a, const O: u8> CRC_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC_A::_1)
    }
}
#[doc = "Field `USBDCD` reader - USB DCD Clock Gate Control"]
pub type USBDCD_R = crate::BitReader<USBDCD_A>;
#[doc = "USB DCD Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBDCD_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<USBDCD_A> for bool {
    #[inline(always)]
    fn from(variant: USBDCD_A) -> Self {
        variant as u8 != 0
    }
}
impl USBDCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBDCD_A {
        match self.bits {
            false => USBDCD_A::_0,
            true => USBDCD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBDCD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBDCD_A::_1
    }
}
#[doc = "Field `USBDCD` writer - USB DCD Clock Gate Control"]
pub type USBDCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, USBDCD_A, O>;
impl<'a, const O: u8> USBDCD_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBDCD_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBDCD_A::_1)
    }
}
#[doc = "Field `PDB` reader - PDB Clock Gate Control"]
pub type PDB_R = crate::BitReader<PDB_A>;
#[doc = "PDB Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDB_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PDB_A> for bool {
    #[inline(always)]
    fn from(variant: PDB_A) -> Self {
        variant as u8 != 0
    }
}
impl PDB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDB_A {
        match self.bits {
            false => PDB_A::_0,
            true => PDB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDB_A::_1
    }
}
#[doc = "Field `PDB` writer - PDB Clock Gate Control"]
pub type PDB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, PDB_A, O>;
impl<'a, const O: u8> PDB_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDB_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDB_A::_1)
    }
}
#[doc = "Field `PIT` reader - PIT Clock Gate Control"]
pub type PIT_R = crate::BitReader<PIT_A>;
#[doc = "PIT Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIT_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PIT_A> for bool {
    #[inline(always)]
    fn from(variant: PIT_A) -> Self {
        variant as u8 != 0
    }
}
impl PIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIT_A {
        match self.bits {
            false => PIT_A::_0,
            true => PIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIT_A::_1
    }
}
#[doc = "Field `PIT` writer - PIT Clock Gate Control"]
pub type PIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, PIT_A, O>;
impl<'a, const O: u8> PIT_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIT_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIT_A::_1)
    }
}
#[doc = "Field `FTM0` reader - FTM0 Clock Gate Control"]
pub type FTM0_R = crate::BitReader<FTM0_A>;
#[doc = "FTM0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<FTM0_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0_A {
        match self.bits {
            false => FTM0_A::_0,
            true => FTM0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0_A::_1
    }
}
#[doc = "Field `FTM0` writer - FTM0 Clock Gate Control"]
pub type FTM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, FTM0_A, O>;
impl<'a, const O: u8> FTM0_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0_A::_1)
    }
}
#[doc = "Field `FTM1` reader - FTM1 Clock Gate Control"]
pub type FTM1_R = crate::BitReader<FTM1_A>;
#[doc = "FTM1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM1_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<FTM1_A> for bool {
    #[inline(always)]
    fn from(variant: FTM1_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1_A {
        match self.bits {
            false => FTM1_A::_0,
            true => FTM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM1_A::_1
    }
}
#[doc = "Field `FTM1` writer - FTM1 Clock Gate Control"]
pub type FTM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, FTM1_A, O>;
impl<'a, const O: u8> FTM1_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1_A::_1)
    }
}
#[doc = "Field `FTM2` reader - FTM2 Clock Gate Control"]
pub type FTM2_R = crate::BitReader<FTM2_A>;
#[doc = "FTM2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM2_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<FTM2_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2_A {
        match self.bits {
            false => FTM2_A::_0,
            true => FTM2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM2_A::_1
    }
}
#[doc = "Field `FTM2` writer - FTM2 Clock Gate Control"]
pub type FTM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, FTM2_A, O>;
impl<'a, const O: u8> FTM2_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2_A::_1)
    }
}
#[doc = "Field `ADC0` reader - ADC0 Clock Gate Control"]
pub type ADC0_R = crate::BitReader<ADC0_A>;
#[doc = "ADC0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<ADC0_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0_A {
        match self.bits {
            false => ADC0_A::_0,
            true => ADC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADC0_A::_1
    }
}
#[doc = "Field `ADC0` writer - ADC0 Clock Gate Control"]
pub type ADC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, ADC0_A, O>;
impl<'a, const O: u8> ADC0_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC0_A::_1)
    }
}
#[doc = "Field `RTC` reader - RTC Access Control"]
pub type RTC_R = crate::BitReader<RTC_A>;
#[doc = "RTC Access Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTC_A {
    #[doc = "0: Access and interrupts disabled"]
    _0 = 0,
    #[doc = "1: Access and interrupts enabled"]
    _1 = 1,
}
impl From<RTC_A> for bool {
    #[inline(always)]
    fn from(variant: RTC_A) -> Self {
        variant as u8 != 0
    }
}
impl RTC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTC_A {
        match self.bits {
            false => RTC_A::_0,
            true => RTC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTC_A::_1
    }
}
#[doc = "Field `RTC` writer - RTC Access Control"]
pub type RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, RTC_A, O>;
impl<'a, const O: u8> RTC_W<'a, O> {
    #[doc = "Access and interrupts disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTC_A::_0)
    }
    #[doc = "Access and interrupts enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTC_A::_1)
    }
}
#[doc = "Field `DAC0` reader - DAC0 Clock Gate Control"]
pub type DAC0_R = crate::BitReader<DAC0_A>;
#[doc = "DAC0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<DAC0_A> for bool {
    #[inline(always)]
    fn from(variant: DAC0_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC0_A {
        match self.bits {
            false => DAC0_A::_0,
            true => DAC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAC0_A::_1
    }
}
#[doc = "Field `DAC0` writer - DAC0 Clock Gate Control"]
pub type DAC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, DAC0_A, O>;
impl<'a, const O: u8> DAC0_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAC0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAC0_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Flash Memory Clock Gate Control"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Mux Clock Gate Control"]
    #[inline(always)]
    pub fn dmamux(&self) -> DMAMUX_R {
        DMAMUX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - FlexCAN0 Clock Gate Control"]
    #[inline(always)]
    pub fn flexcan0(&self) -> FLEXCAN0_R {
        FLEXCAN0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 9 - RNGA Clock Gate Control"]
    #[inline(always)]
    pub fn rnga(&self) -> RNGA_R {
        RNGA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI0 Clock Gate Control"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI1 Clock Gate Control"]
    #[inline(always)]
    pub fn spi1(&self) -> SPI1_R {
        SPI1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - I2S Clock Gate Control"]
    #[inline(always)]
    pub fn i2s(&self) -> I2S_R {
        I2S_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - CRC Clock Gate Control"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - USB DCD Clock Gate Control"]
    #[inline(always)]
    pub fn usbdcd(&self) -> USBDCD_R {
        USBDCD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PDB Clock Gate Control"]
    #[inline(always)]
    pub fn pdb(&self) -> PDB_R {
        PDB_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PIT Clock Gate Control"]
    #[inline(always)]
    pub fn pit(&self) -> PIT_R {
        PIT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - FTM0 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm0(&self) -> FTM0_R {
        FTM0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - FTM1 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm1(&self) -> FTM1_R {
        FTM1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FTM2 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm2(&self) -> FTM2_R {
        FTM2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - ADC0 Clock Gate Control"]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - RTC Access Control"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - DAC0 Clock Gate Control"]
    #[inline(always)]
    pub fn dac0(&self) -> DAC0_R {
        DAC0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Memory Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn ftf(&mut self) -> FTF_W<0> {
        FTF_W::new(self)
    }
    #[doc = "Bit 1 - DMA Mux Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn dmamux(&mut self) -> DMAMUX_W<1> {
        DMAMUX_W::new(self)
    }
    #[doc = "Bit 4 - FlexCAN0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn flexcan0(&mut self) -> FLEXCAN0_W<4> {
        FLEXCAN0_W::new(self)
    }
    #[doc = "Bit 9 - RNGA Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn rnga(&mut self) -> RNGA_W<9> {
        RNGA_W::new(self)
    }
    #[doc = "Bit 12 - SPI0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<12> {
        SPI0_W::new(self)
    }
    #[doc = "Bit 13 - SPI1 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn spi1(&mut self) -> SPI1_W<13> {
        SPI1_W::new(self)
    }
    #[doc = "Bit 15 - I2S Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn i2s(&mut self) -> I2S_W<15> {
        I2S_W::new(self)
    }
    #[doc = "Bit 18 - CRC Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<18> {
        CRC_W::new(self)
    }
    #[doc = "Bit 21 - USB DCD Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn usbdcd(&mut self) -> USBDCD_W<21> {
        USBDCD_W::new(self)
    }
    #[doc = "Bit 22 - PDB Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn pdb(&mut self) -> PDB_W<22> {
        PDB_W::new(self)
    }
    #[doc = "Bit 23 - PIT Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn pit(&mut self) -> PIT_W<23> {
        PIT_W::new(self)
    }
    #[doc = "Bit 24 - FTM0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn ftm0(&mut self) -> FTM0_W<24> {
        FTM0_W::new(self)
    }
    #[doc = "Bit 25 - FTM1 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn ftm1(&mut self) -> FTM1_W<25> {
        FTM1_W::new(self)
    }
    #[doc = "Bit 26 - FTM2 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn ftm2(&mut self) -> FTM2_W<26> {
        FTM2_W::new(self)
    }
    #[doc = "Bit 27 - ADC0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn adc0(&mut self) -> ADC0_W<27> {
        ADC0_W::new(self)
    }
    #[doc = "Bit 29 - RTC Access Control"]
    #[inline(always)]
    #[must_use]
    pub fn rtc(&mut self) -> RTC_W<29> {
        RTC_W::new(self)
    }
    #[doc = "Bit 31 - DAC0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn dac0(&mut self) -> DAC0_W<31> {
        DAC0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Gating Control Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgc6](index.html) module"]
pub struct SCGC6_SPEC;
impl crate::RegisterSpec for SCGC6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgc6::R](R) reader structure"]
impl crate::Readable for SCGC6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgc6::W](W) writer structure"]
impl crate::Writable for SCGC6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGC6 to value 0x4000_0001"]
impl crate::Resettable for SCGC6_SPEC {
    const RESET_VALUE: Self::Ux = 0x4000_0001;
}
