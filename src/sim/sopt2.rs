#[doc = "Register `SOPT2` reader"]
pub struct R(crate::R<SOPT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOPT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOPT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOPT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOPT2` writer"]
pub struct W(crate::W<SOPT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOPT2_SPEC>;
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
impl From<crate::W<SOPT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOPT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCCLKOUTSEL` reader - RTC clock out select"]
pub type RTCCLKOUTSEL_R = crate::BitReader<RTCCLKOUTSEL_A>;
#[doc = "RTC clock out select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCCLKOUTSEL_A {
    #[doc = "0: RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    _0 = 0,
    #[doc = "1: RTC 32.768kHz clock is output on the RTC_CLKOUT pin."]
    _1 = 1,
}
impl From<RTCCLKOUTSEL_A> for bool {
    #[inline(always)]
    fn from(variant: RTCCLKOUTSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCCLKOUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCCLKOUTSEL_A {
        match self.bits {
            false => RTCCLKOUTSEL_A::_0,
            true => RTCCLKOUTSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTCCLKOUTSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTCCLKOUTSEL_A::_1
    }
}
#[doc = "Field `RTCCLKOUTSEL` writer - RTC clock out select"]
pub type RTCCLKOUTSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT2_SPEC, RTCCLKOUTSEL_A, O>;
impl<'a, const O: u8> RTCCLKOUTSEL_W<'a, O> {
    #[doc = "RTC 1 Hz clock is output on the RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RTCCLKOUTSEL_A::_0)
    }
    #[doc = "RTC 32.768kHz clock is output on the RTC_CLKOUT pin."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RTCCLKOUTSEL_A::_1)
    }
}
#[doc = "Field `CLKOUTSEL` reader - CLKOUT select"]
pub type CLKOUTSEL_R = crate::FieldReader<u8, CLKOUTSEL_A>;
#[doc = "CLKOUT select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUTSEL_A {
    #[doc = "0: FlexBus CLKOUT"]
    _000 = 0,
    #[doc = "2: Flash clock"]
    _010 = 2,
    #[doc = "3: LPO clock (1 kHz)"]
    _011 = 3,
    #[doc = "4: MCGIRCLK"]
    _100 = 4,
    #[doc = "5: RTC 32.768kHz clock"]
    _101 = 5,
    #[doc = "6: OSCERCLK0"]
    _110 = 6,
    #[doc = "7: IRC 48 MHz clock"]
    _111 = 7,
}
impl From<CLKOUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL_A) -> Self {
        variant as _
    }
}
impl CLKOUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKOUTSEL_A> {
        match self.bits {
            0 => Some(CLKOUTSEL_A::_000),
            2 => Some(CLKOUTSEL_A::_010),
            3 => Some(CLKOUTSEL_A::_011),
            4 => Some(CLKOUTSEL_A::_100),
            5 => Some(CLKOUTSEL_A::_101),
            6 => Some(CLKOUTSEL_A::_110),
            7 => Some(CLKOUTSEL_A::_111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CLKOUTSEL_A::_000
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CLKOUTSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CLKOUTSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CLKOUTSEL_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == CLKOUTSEL_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CLKOUTSEL_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == CLKOUTSEL_A::_111
    }
}
#[doc = "Field `CLKOUTSEL` writer - CLKOUT select"]
pub type CLKOUTSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOPT2_SPEC, u8, CLKOUTSEL_A, 3, O>;
impl<'a, const O: u8> CLKOUTSEL_W<'a, O> {
    #[doc = "FlexBus CLKOUT"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_000)
    }
    #[doc = "Flash clock"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_010)
    }
    #[doc = "LPO clock (1 kHz)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_011)
    }
    #[doc = "MCGIRCLK"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_100)
    }
    #[doc = "RTC 32.768kHz clock"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_101)
    }
    #[doc = "OSCERCLK0"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_110)
    }
    #[doc = "IRC 48 MHz clock"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_111)
    }
}
#[doc = "Field `FBSL` reader - FlexBus security level"]
pub type FBSL_R = crate::FieldReader<u8, FBSL_A>;
#[doc = "FlexBus security level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FBSL_A {
    #[doc = "0: All off-chip accesses (instruction and data) via the FlexBus are disallowed."]
    _00 = 0,
    #[doc = "1: All off-chip accesses (instruction and data) via the FlexBus are disallowed."]
    _01 = 1,
    #[doc = "2: Off-chip instruction accesses are disallowed. Data accesses are allowed."]
    _10 = 2,
    #[doc = "3: Off-chip instruction accesses and data accesses are allowed."]
    _11 = 3,
}
impl From<FBSL_A> for u8 {
    #[inline(always)]
    fn from(variant: FBSL_A) -> Self {
        variant as _
    }
}
impl FBSL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FBSL_A {
        match self.bits {
            0 => FBSL_A::_00,
            1 => FBSL_A::_01,
            2 => FBSL_A::_10,
            3 => FBSL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FBSL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FBSL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FBSL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FBSL_A::_11
    }
}
#[doc = "Field `FBSL` writer - FlexBus security level"]
pub type FBSL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SOPT2_SPEC, u8, FBSL_A, 2, O>;
impl<'a, const O: u8> FBSL_W<'a, O> {
    #[doc = "All off-chip accesses (instruction and data) via the FlexBus are disallowed."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FBSL_A::_00)
    }
    #[doc = "All off-chip accesses (instruction and data) via the FlexBus are disallowed."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FBSL_A::_01)
    }
    #[doc = "Off-chip instruction accesses are disallowed. Data accesses are allowed."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FBSL_A::_10)
    }
    #[doc = "Off-chip instruction accesses and data accesses are allowed."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FBSL_A::_11)
    }
}
#[doc = "Field `PTD7PAD` reader - PTD7 pad drive strength"]
pub type PTD7PAD_R = crate::BitReader<PTD7PAD_A>;
#[doc = "PTD7 pad drive strength\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTD7PAD_A {
    #[doc = "0: Single-pad drive strength for PTD7."]
    _0 = 0,
    #[doc = "1: Double pad drive strength for PTD7."]
    _1 = 1,
}
impl From<PTD7PAD_A> for bool {
    #[inline(always)]
    fn from(variant: PTD7PAD_A) -> Self {
        variant as u8 != 0
    }
}
impl PTD7PAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PTD7PAD_A {
        match self.bits {
            false => PTD7PAD_A::_0,
            true => PTD7PAD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PTD7PAD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PTD7PAD_A::_1
    }
}
#[doc = "Field `PTD7PAD` writer - PTD7 pad drive strength"]
pub type PTD7PAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT2_SPEC, PTD7PAD_A, O>;
impl<'a, const O: u8> PTD7PAD_W<'a, O> {
    #[doc = "Single-pad drive strength for PTD7."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTD7PAD_A::_0)
    }
    #[doc = "Double pad drive strength for PTD7."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTD7PAD_A::_1)
    }
}
#[doc = "Field `TRACECLKSEL` reader - Debug trace clock select"]
pub type TRACECLKSEL_R = crate::BitReader<TRACECLKSEL_A>;
#[doc = "Debug trace clock select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRACECLKSEL_A {
    #[doc = "0: MCGOUTCLK"]
    _0 = 0,
    #[doc = "1: Core/system clock"]
    _1 = 1,
}
impl From<TRACECLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TRACECLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl TRACECLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRACECLKSEL_A {
        match self.bits {
            false => TRACECLKSEL_A::_0,
            true => TRACECLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRACECLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRACECLKSEL_A::_1
    }
}
#[doc = "Field `TRACECLKSEL` writer - Debug trace clock select"]
pub type TRACECLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT2_SPEC, TRACECLKSEL_A, O>;
impl<'a, const O: u8> TRACECLKSEL_W<'a, O> {
    #[doc = "MCGOUTCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRACECLKSEL_A::_0)
    }
    #[doc = "Core/system clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRACECLKSEL_A::_1)
    }
}
#[doc = "Field `PLLFLLSEL` reader - PLL/FLL clock select"]
pub type PLLFLLSEL_R = crate::FieldReader<u8, PLLFLLSEL_A>;
#[doc = "PLL/FLL clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLFLLSEL_A {
    #[doc = "0: MCGFLLCLK clock"]
    _00 = 0,
    #[doc = "1: MCGPLLCLK clock"]
    _01 = 1,
    #[doc = "3: IRC48 MHz clock"]
    _11 = 3,
}
impl From<PLLFLLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLFLLSEL_A) -> Self {
        variant as _
    }
}
impl PLLFLLSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLFLLSEL_A> {
        match self.bits {
            0 => Some(PLLFLLSEL_A::_00),
            1 => Some(PLLFLLSEL_A::_01),
            3 => Some(PLLFLLSEL_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PLLFLLSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PLLFLLSEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PLLFLLSEL_A::_11
    }
}
#[doc = "Field `PLLFLLSEL` writer - PLL/FLL clock select"]
pub type PLLFLLSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOPT2_SPEC, u8, PLLFLLSEL_A, 2, O>;
impl<'a, const O: u8> PLLFLLSEL_W<'a, O> {
    #[doc = "MCGFLLCLK clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PLLFLLSEL_A::_00)
    }
    #[doc = "MCGPLLCLK clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PLLFLLSEL_A::_01)
    }
    #[doc = "IRC48 MHz clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PLLFLLSEL_A::_11)
    }
}
#[doc = "Field `USBSRC` reader - USB clock source select"]
pub type USBSRC_R = crate::BitReader<USBSRC_A>;
#[doc = "USB clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBSRC_A {
    #[doc = "0: External bypass clock (USB_CLKIN)."]
    _0 = 0,
    #[doc = "1: MCGFLLCLK , or MCGPLLCLK , or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the USB fractional divider as configured by SIM_CLKDIV2\\[USBFRAC, USBDIV\\]."]
    _1 = 1,
}
impl From<USBSRC_A> for bool {
    #[inline(always)]
    fn from(variant: USBSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl USBSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBSRC_A {
        match self.bits {
            false => USBSRC_A::_0,
            true => USBSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBSRC_A::_1
    }
}
#[doc = "Field `USBSRC` writer - USB clock source select"]
pub type USBSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT2_SPEC, USBSRC_A, O>;
impl<'a, const O: u8> USBSRC_W<'a, O> {
    #[doc = "External bypass clock (USB_CLKIN)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBSRC_A::_0)
    }
    #[doc = "MCGFLLCLK , or MCGPLLCLK , or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\], and then divided by the USB fractional divider as configured by SIM_CLKDIV2\\[USBFRAC, USBDIV\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBSRC_A::_1)
    }
}
#[doc = "Field `RMIISRC` reader - RMII clock source select"]
pub type RMIISRC_R = crate::BitReader<RMIISRC_A>;
#[doc = "RMII clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMIISRC_A {
    #[doc = "0: EXTAL clock"]
    _0 = 0,
    #[doc = "1: External bypass clock (ENET_1588_CLKIN)."]
    _1 = 1,
}
impl From<RMIISRC_A> for bool {
    #[inline(always)]
    fn from(variant: RMIISRC_A) -> Self {
        variant as u8 != 0
    }
}
impl RMIISRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMIISRC_A {
        match self.bits {
            false => RMIISRC_A::_0,
            true => RMIISRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RMIISRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RMIISRC_A::_1
    }
}
#[doc = "Field `RMIISRC` writer - RMII clock source select"]
pub type RMIISRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT2_SPEC, RMIISRC_A, O>;
impl<'a, const O: u8> RMIISRC_W<'a, O> {
    #[doc = "EXTAL clock"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RMIISRC_A::_0)
    }
    #[doc = "External bypass clock (ENET_1588_CLKIN)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RMIISRC_A::_1)
    }
}
#[doc = "Field `TIMESRC` reader - IEEE 1588 timestamp clock source select"]
pub type TIMESRC_R = crate::FieldReader<u8, TIMESRC_A>;
#[doc = "IEEE 1588 timestamp clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMESRC_A {
    #[doc = "0: Core/system clock."]
    _00 = 0,
    #[doc = "1: MCGFLLCLK , or MCGPLLCLK , or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    _01 = 1,
    #[doc = "2: OSCERCLK clock"]
    _10 = 2,
    #[doc = "3: External bypass clock (ENET_1588_CLKIN)."]
    _11 = 3,
}
impl From<TIMESRC_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMESRC_A) -> Self {
        variant as _
    }
}
impl TIMESRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMESRC_A {
        match self.bits {
            0 => TIMESRC_A::_00,
            1 => TIMESRC_A::_01,
            2 => TIMESRC_A::_10,
            3 => TIMESRC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TIMESRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TIMESRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TIMESRC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TIMESRC_A::_11
    }
}
#[doc = "Field `TIMESRC` writer - IEEE 1588 timestamp clock source select"]
pub type TIMESRC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SOPT2_SPEC, u8, TIMESRC_A, 2, O>;
impl<'a, const O: u8> TIMESRC_W<'a, O> {
    #[doc = "Core/system clock."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TIMESRC_A::_00)
    }
    #[doc = "MCGFLLCLK , or MCGPLLCLK , or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TIMESRC_A::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TIMESRC_A::_10)
    }
    #[doc = "External bypass clock (ENET_1588_CLKIN)."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TIMESRC_A::_11)
    }
}
#[doc = "Field `SDHCSRC` reader - SDHC clock source select"]
pub type SDHCSRC_R = crate::FieldReader<u8, SDHCSRC_A>;
#[doc = "SDHC clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SDHCSRC_A {
    #[doc = "0: Core/system clock."]
    _00 = 0,
    #[doc = "1: MCGFLLCLK, or MCGPLLCLK , or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    _01 = 1,
    #[doc = "2: OSCERCLK clock"]
    _10 = 2,
    #[doc = "3: External bypass clock (SDHC0_CLKIN)"]
    _11 = 3,
}
impl From<SDHCSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SDHCSRC_A) -> Self {
        variant as _
    }
}
impl SDHCSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDHCSRC_A {
        match self.bits {
            0 => SDHCSRC_A::_00,
            1 => SDHCSRC_A::_01,
            2 => SDHCSRC_A::_10,
            3 => SDHCSRC_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == SDHCSRC_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == SDHCSRC_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SDHCSRC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SDHCSRC_A::_11
    }
}
#[doc = "Field `SDHCSRC` writer - SDHC clock source select"]
pub type SDHCSRC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SOPT2_SPEC, u8, SDHCSRC_A, 2, O>;
impl<'a, const O: u8> SDHCSRC_W<'a, O> {
    #[doc = "Core/system clock."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(SDHCSRC_A::_00)
    }
    #[doc = "MCGFLLCLK, or MCGPLLCLK , or IRC48M clock as selected by SOPT2\\[PLLFLLSEL\\]."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(SDHCSRC_A::_01)
    }
    #[doc = "OSCERCLK clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SDHCSRC_A::_10)
    }
    #[doc = "External bypass clock (SDHC0_CLKIN)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SDHCSRC_A::_11)
    }
}
impl R {
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline(always)]
    pub fn rtcclkoutsel(&self) -> RTCCLKOUTSEL_R {
        RTCCLKOUTSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline(always)]
    pub fn clkoutsel(&self) -> CLKOUTSEL_R {
        CLKOUTSEL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:9 - FlexBus security level"]
    #[inline(always)]
    pub fn fbsl(&self) -> FBSL_R {
        FBSL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - PTD7 pad drive strength"]
    #[inline(always)]
    pub fn ptd7pad(&self) -> PTD7PAD_R {
        PTD7PAD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Debug trace clock select"]
    #[inline(always)]
    pub fn traceclksel(&self) -> TRACECLKSEL_R {
        TRACECLKSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - PLL/FLL clock select"]
    #[inline(always)]
    pub fn pllfllsel(&self) -> PLLFLLSEL_R {
        PLLFLLSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline(always)]
    pub fn usbsrc(&self) -> USBSRC_R {
        USBSRC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - RMII clock source select"]
    #[inline(always)]
    pub fn rmiisrc(&self) -> RMIISRC_R {
        RMIISRC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - IEEE 1588 timestamp clock source select"]
    #[inline(always)]
    pub fn timesrc(&self) -> TIMESRC_R {
        TIMESRC_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - SDHC clock source select"]
    #[inline(always)]
    pub fn sdhcsrc(&self) -> SDHCSRC_R {
        SDHCSRC_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - RTC clock out select"]
    #[inline(always)]
    #[must_use]
    pub fn rtcclkoutsel(&mut self) -> RTCCLKOUTSEL_W<4> {
        RTCCLKOUTSEL_W::new(self)
    }
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline(always)]
    #[must_use]
    pub fn clkoutsel(&mut self) -> CLKOUTSEL_W<5> {
        CLKOUTSEL_W::new(self)
    }
    #[doc = "Bits 8:9 - FlexBus security level"]
    #[inline(always)]
    #[must_use]
    pub fn fbsl(&mut self) -> FBSL_W<8> {
        FBSL_W::new(self)
    }
    #[doc = "Bit 11 - PTD7 pad drive strength"]
    #[inline(always)]
    #[must_use]
    pub fn ptd7pad(&mut self) -> PTD7PAD_W<11> {
        PTD7PAD_W::new(self)
    }
    #[doc = "Bit 12 - Debug trace clock select"]
    #[inline(always)]
    #[must_use]
    pub fn traceclksel(&mut self) -> TRACECLKSEL_W<12> {
        TRACECLKSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - PLL/FLL clock select"]
    #[inline(always)]
    #[must_use]
    pub fn pllfllsel(&mut self) -> PLLFLLSEL_W<16> {
        PLLFLLSEL_W::new(self)
    }
    #[doc = "Bit 18 - USB clock source select"]
    #[inline(always)]
    #[must_use]
    pub fn usbsrc(&mut self) -> USBSRC_W<18> {
        USBSRC_W::new(self)
    }
    #[doc = "Bit 19 - RMII clock source select"]
    #[inline(always)]
    #[must_use]
    pub fn rmiisrc(&mut self) -> RMIISRC_W<19> {
        RMIISRC_W::new(self)
    }
    #[doc = "Bits 20:21 - IEEE 1588 timestamp clock source select"]
    #[inline(always)]
    #[must_use]
    pub fn timesrc(&mut self) -> TIMESRC_W<20> {
        TIMESRC_W::new(self)
    }
    #[doc = "Bits 28:29 - SDHC clock source select"]
    #[inline(always)]
    #[must_use]
    pub fn sdhcsrc(&mut self) -> SDHCSRC_W<28> {
        SDHCSRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Options Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopt2](index.html) module"]
pub struct SOPT2_SPEC;
impl crate::RegisterSpec for SOPT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sopt2::R](R) reader structure"]
impl crate::Readable for SOPT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sopt2::W](W) writer structure"]
impl crate::Writable for SOPT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOPT2 to value 0x1000"]
impl crate::Resettable for SOPT2_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
