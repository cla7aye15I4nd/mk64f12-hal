#[doc = "Register `STCTRLH` reader"]
pub struct R(crate::R<STCTRLH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCTRLH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCTRLH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCTRLH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STCTRLH` writer"]
pub struct W(crate::W<STCTRLH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STCTRLH_SPEC>;
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
impl From<crate::W<STCTRLH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STCTRLH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDOGEN` reader - Enables or disables the WDOG's operation"]
pub type WDOGEN_R = crate::BitReader<WDOGEN_A>;
#[doc = "Enables or disables the WDOG's operation\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDOGEN_A {
    #[doc = "0: WDOG is disabled."]
    _0 = 0,
    #[doc = "1: WDOG is enabled."]
    _1 = 1,
}
impl From<WDOGEN_A> for bool {
    #[inline(always)]
    fn from(variant: WDOGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WDOGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOGEN_A {
        match self.bits {
            false => WDOGEN_A::_0,
            true => WDOGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WDOGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WDOGEN_A::_1
    }
}
#[doc = "Field `WDOGEN` writer - Enables or disables the WDOG's operation"]
pub type WDOGEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, STCTRLH_SPEC, WDOGEN_A, O>;
impl<'a, const O: u8> WDOGEN_W<'a, O> {
    #[doc = "WDOG is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WDOGEN_A::_0)
    }
    #[doc = "WDOG is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WDOGEN_A::_1)
    }
}
#[doc = "Field `CLKSRC` reader - Selects clock source for the WDOG timer and other internal timing operations."]
pub type CLKSRC_R = crate::BitReader<CLKSRC_A>;
#[doc = "Selects clock source for the WDOG timer and other internal timing operations.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLKSRC_A {
    #[doc = "0: WDOG clock sourced from LPO ."]
    _0 = 0,
    #[doc = "1: WDOG clock sourced from alternate clock source."]
    _1 = 1,
}
impl From<CLKSRC_A> for bool {
    #[inline(always)]
    fn from(variant: CLKSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl CLKSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKSRC_A {
        match self.bits {
            false => CLKSRC_A::_0,
            true => CLKSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CLKSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CLKSRC_A::_1
    }
}
#[doc = "Field `CLKSRC` writer - Selects clock source for the WDOG timer and other internal timing operations."]
pub type CLKSRC_W<'a, const O: u8> = crate::BitWriter<'a, u16, STCTRLH_SPEC, CLKSRC_A, O>;
impl<'a, const O: u8> CLKSRC_W<'a, O> {
    #[doc = "WDOG clock sourced from LPO ."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLKSRC_A::_0)
    }
    #[doc = "WDOG clock sourced from alternate clock source."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLKSRC_A::_1)
    }
}
#[doc = "Field `IRQRSTEN` reader - Used to enable the debug breadcrumbs feature"]
pub type IRQRSTEN_R = crate::BitReader<IRQRSTEN_A>;
#[doc = "Used to enable the debug breadcrumbs feature\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQRSTEN_A {
    #[doc = "0: WDOG time-out generates reset only."]
    _0 = 0,
    #[doc = "1: WDOG time-out initially generates an interrupt. After WCT, it generates a reset."]
    _1 = 1,
}
impl From<IRQRSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: IRQRSTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQRSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQRSTEN_A {
        match self.bits {
            false => IRQRSTEN_A::_0,
            true => IRQRSTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQRSTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQRSTEN_A::_1
    }
}
#[doc = "Field `IRQRSTEN` writer - Used to enable the debug breadcrumbs feature"]
pub type IRQRSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, STCTRLH_SPEC, IRQRSTEN_A, O>;
impl<'a, const O: u8> IRQRSTEN_W<'a, O> {
    #[doc = "WDOG time-out generates reset only."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQRSTEN_A::_0)
    }
    #[doc = "WDOG time-out initially generates an interrupt. After WCT, it generates a reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQRSTEN_A::_1)
    }
}
#[doc = "Field `WINEN` reader - Enables Windowing mode."]
pub type WINEN_R = crate::BitReader<WINEN_A>;
#[doc = "Enables Windowing mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WINEN_A {
    #[doc = "0: Windowing mode is disabled."]
    _0 = 0,
    #[doc = "1: Windowing mode is enabled."]
    _1 = 1,
}
impl From<WINEN_A> for bool {
    #[inline(always)]
    fn from(variant: WINEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WINEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WINEN_A {
        match self.bits {
            false => WINEN_A::_0,
            true => WINEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WINEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WINEN_A::_1
    }
}
#[doc = "Field `WINEN` writer - Enables Windowing mode."]
pub type WINEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, STCTRLH_SPEC, WINEN_A, O>;
impl<'a, const O: u8> WINEN_W<'a, O> {
    #[doc = "Windowing mode is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WINEN_A::_0)
    }
    #[doc = "Windowing mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WINEN_A::_1)
    }
}
#[doc = "Field `ALLOWUPDATE` reader - Enables updates to watchdog write-once registers, after the reset-triggered initial configuration window (WCT) closes, through unlock sequence"]
pub type ALLOWUPDATE_R = crate::BitReader<ALLOWUPDATE_A>;
#[doc = "Enables updates to watchdog write-once registers, after the reset-triggered initial configuration window (WCT) closes, through unlock sequence\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALLOWUPDATE_A {
    #[doc = "0: No further updates allowed to WDOG write-once registers."]
    _0 = 0,
    #[doc = "1: WDOG write-once registers can be unlocked for updating."]
    _1 = 1,
}
impl From<ALLOWUPDATE_A> for bool {
    #[inline(always)]
    fn from(variant: ALLOWUPDATE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALLOWUPDATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALLOWUPDATE_A {
        match self.bits {
            false => ALLOWUPDATE_A::_0,
            true => ALLOWUPDATE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALLOWUPDATE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALLOWUPDATE_A::_1
    }
}
#[doc = "Field `ALLOWUPDATE` writer - Enables updates to watchdog write-once registers, after the reset-triggered initial configuration window (WCT) closes, through unlock sequence"]
pub type ALLOWUPDATE_W<'a, const O: u8> = crate::BitWriter<'a, u16, STCTRLH_SPEC, ALLOWUPDATE_A, O>;
impl<'a, const O: u8> ALLOWUPDATE_W<'a, O> {
    #[doc = "No further updates allowed to WDOG write-once registers."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALLOWUPDATE_A::_0)
    }
    #[doc = "WDOG write-once registers can be unlocked for updating."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALLOWUPDATE_A::_1)
    }
}
#[doc = "Field `DBGEN` reader - Enables or disables WDOG in Debug mode."]
pub type DBGEN_R = crate::BitReader<DBGEN_A>;
#[doc = "Enables or disables WDOG in Debug mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGEN_A {
    #[doc = "0: WDOG is disabled in CPU Debug mode."]
    _0 = 0,
    #[doc = "1: WDOG is enabled in CPU Debug mode."]
    _1 = 1,
}
impl From<DBGEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGEN_A {
        match self.bits {
            false => DBGEN_A::_0,
            true => DBGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGEN_A::_1
    }
}
#[doc = "Field `DBGEN` writer - Enables or disables WDOG in Debug mode."]
pub type DBGEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, STCTRLH_SPEC, DBGEN_A, O>;
impl<'a, const O: u8> DBGEN_W<'a, O> {
    #[doc = "WDOG is disabled in CPU Debug mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGEN_A::_0)
    }
    #[doc = "WDOG is enabled in CPU Debug mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGEN_A::_1)
    }
}
#[doc = "Field `STOPEN` reader - Enables or disables WDOG in Stop mode."]
pub type STOPEN_R = crate::BitReader<STOPEN_A>;
#[doc = "Enables or disables WDOG in Stop mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPEN_A {
    #[doc = "0: WDOG is disabled in CPU Stop mode."]
    _0 = 0,
    #[doc = "1: WDOG is enabled in CPU Stop mode."]
    _1 = 1,
}
impl From<STOPEN_A> for bool {
    #[inline(always)]
    fn from(variant: STOPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPEN_A {
        match self.bits {
            false => STOPEN_A::_0,
            true => STOPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STOPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STOPEN_A::_1
    }
}
#[doc = "Field `STOPEN` writer - Enables or disables WDOG in Stop mode."]
pub type STOPEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, STCTRLH_SPEC, STOPEN_A, O>;
impl<'a, const O: u8> STOPEN_W<'a, O> {
    #[doc = "WDOG is disabled in CPU Stop mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STOPEN_A::_0)
    }
    #[doc = "WDOG is enabled in CPU Stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOPEN_A::_1)
    }
}
#[doc = "Field `WAITEN` reader - Enables or disables WDOG in Wait mode."]
pub type WAITEN_R = crate::BitReader<WAITEN_A>;
#[doc = "Enables or disables WDOG in Wait mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAITEN_A {
    #[doc = "0: WDOG is disabled in CPU Wait mode."]
    _0 = 0,
    #[doc = "1: WDOG is enabled in CPU Wait mode."]
    _1 = 1,
}
impl From<WAITEN_A> for bool {
    #[inline(always)]
    fn from(variant: WAITEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WAITEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAITEN_A {
        match self.bits {
            false => WAITEN_A::_0,
            true => WAITEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WAITEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAITEN_A::_1
    }
}
#[doc = "Field `WAITEN` writer - Enables or disables WDOG in Wait mode."]
pub type WAITEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, STCTRLH_SPEC, WAITEN_A, O>;
impl<'a, const O: u8> WAITEN_W<'a, O> {
    #[doc = "WDOG is disabled in CPU Wait mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAITEN_A::_0)
    }
    #[doc = "WDOG is enabled in CPU Wait mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAITEN_A::_1)
    }
}
#[doc = "Field `TESTWDOG` reader - Puts the watchdog in the functional test mode"]
pub type TESTWDOG_R = crate::BitReader<bool>;
#[doc = "Field `TESTWDOG` writer - Puts the watchdog in the functional test mode"]
pub type TESTWDOG_W<'a, const O: u8> = crate::BitWriter<'a, u16, STCTRLH_SPEC, bool, O>;
#[doc = "Field `TESTSEL` reader - Effective only if TESTWDOG is set. Selects the test to be run on the watchdog timer."]
pub type TESTSEL_R = crate::BitReader<TESTSEL_A>;
#[doc = "Effective only if TESTWDOG is set. Selects the test to be run on the watchdog timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TESTSEL_A {
    #[doc = "0: Quick test. The timer runs in normal operation. You can load a small time-out value to do a quick test."]
    _0 = 0,
    #[doc = "1: Byte test. Puts the timer in the byte test mode where individual bytes of the timer are enabled for operation and are compared for time-out against the corresponding byte of the programmed time-out value. Select the byte through BYTESEL\\[1:0\\]
for testing."]
    _1 = 1,
}
impl From<TESTSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TESTSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl TESTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TESTSEL_A {
        match self.bits {
            false => TESTSEL_A::_0,
            true => TESTSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TESTSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TESTSEL_A::_1
    }
}
#[doc = "Field `TESTSEL` writer - Effective only if TESTWDOG is set. Selects the test to be run on the watchdog timer."]
pub type TESTSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, STCTRLH_SPEC, TESTSEL_A, O>;
impl<'a, const O: u8> TESTSEL_W<'a, O> {
    #[doc = "Quick test. The timer runs in normal operation. You can load a small time-out value to do a quick test."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TESTSEL_A::_0)
    }
    #[doc = "Byte test. Puts the timer in the byte test mode where individual bytes of the timer are enabled for operation and are compared for time-out against the corresponding byte of the programmed time-out value. Select the byte through BYTESEL\\[1:0\\]
for testing."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TESTSEL_A::_1)
    }
}
#[doc = "Field `BYTESEL` reader - This 2-bit field selects the byte to be tested when the watchdog is in the byte test mode."]
pub type BYTESEL_R = crate::FieldReader<u8, BYTESEL_A>;
#[doc = "This 2-bit field selects the byte to be tested when the watchdog is in the byte test mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BYTESEL_A {
    #[doc = "0: Byte 0 selected"]
    _00 = 0,
    #[doc = "1: Byte 1 selected"]
    _01 = 1,
    #[doc = "2: Byte 2 selected"]
    _10 = 2,
    #[doc = "3: Byte 3 selected"]
    _11 = 3,
}
impl From<BYTESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: BYTESEL_A) -> Self {
        variant as _
    }
}
impl BYTESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYTESEL_A {
        match self.bits {
            0 => BYTESEL_A::_00,
            1 => BYTESEL_A::_01,
            2 => BYTESEL_A::_10,
            3 => BYTESEL_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == BYTESEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == BYTESEL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BYTESEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == BYTESEL_A::_11
    }
}
#[doc = "Field `BYTESEL` writer - This 2-bit field selects the byte to be tested when the watchdog is in the byte test mode."]
pub type BYTESEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, STCTRLH_SPEC, u8, BYTESEL_A, 2, O>;
impl<'a, const O: u8> BYTESEL_W<'a, O> {
    #[doc = "Byte 0 selected"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(BYTESEL_A::_00)
    }
    #[doc = "Byte 1 selected"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(BYTESEL_A::_01)
    }
    #[doc = "Byte 2 selected"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(BYTESEL_A::_10)
    }
    #[doc = "Byte 3 selected"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(BYTESEL_A::_11)
    }
}
#[doc = "Field `DISTESTWDOG` reader - Allows the WDOG's functional test mode to be disabled permanently"]
pub type DISTESTWDOG_R = crate::BitReader<DISTESTWDOG_A>;
#[doc = "Allows the WDOG's functional test mode to be disabled permanently\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISTESTWDOG_A {
    #[doc = "0: WDOG functional test mode is not disabled."]
    _0 = 0,
    #[doc = "1: WDOG functional test mode is disabled permanently until reset."]
    _1 = 1,
}
impl From<DISTESTWDOG_A> for bool {
    #[inline(always)]
    fn from(variant: DISTESTWDOG_A) -> Self {
        variant as u8 != 0
    }
}
impl DISTESTWDOG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISTESTWDOG_A {
        match self.bits {
            false => DISTESTWDOG_A::_0,
            true => DISTESTWDOG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DISTESTWDOG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DISTESTWDOG_A::_1
    }
}
#[doc = "Field `DISTESTWDOG` writer - Allows the WDOG's functional test mode to be disabled permanently"]
pub type DISTESTWDOG_W<'a, const O: u8> = crate::BitWriter<'a, u16, STCTRLH_SPEC, DISTESTWDOG_A, O>;
impl<'a, const O: u8> DISTESTWDOG_W<'a, O> {
    #[doc = "WDOG functional test mode is not disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DISTESTWDOG_A::_0)
    }
    #[doc = "WDOG functional test mode is disabled permanently until reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DISTESTWDOG_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Enables or disables the WDOG's operation"]
    #[inline(always)]
    pub fn wdogen(&self) -> WDOGEN_R {
        WDOGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects clock source for the WDOG timer and other internal timing operations."]
    #[inline(always)]
    pub fn clksrc(&self) -> CLKSRC_R {
        CLKSRC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Used to enable the debug breadcrumbs feature"]
    #[inline(always)]
    pub fn irqrsten(&self) -> IRQRSTEN_R {
        IRQRSTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables Windowing mode."]
    #[inline(always)]
    pub fn winen(&self) -> WINEN_R {
        WINEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables updates to watchdog write-once registers, after the reset-triggered initial configuration window (WCT) closes, through unlock sequence"]
    #[inline(always)]
    pub fn allowupdate(&self) -> ALLOWUPDATE_R {
        ALLOWUPDATE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enables or disables WDOG in Debug mode."]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables or disables WDOG in Stop mode."]
    #[inline(always)]
    pub fn stopen(&self) -> STOPEN_R {
        STOPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables or disables WDOG in Wait mode."]
    #[inline(always)]
    pub fn waiten(&self) -> WAITEN_R {
        WAITEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Puts the watchdog in the functional test mode"]
    #[inline(always)]
    pub fn testwdog(&self) -> TESTWDOG_R {
        TESTWDOG_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Effective only if TESTWDOG is set. Selects the test to be run on the watchdog timer."]
    #[inline(always)]
    pub fn testsel(&self) -> TESTSEL_R {
        TESTSEL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - This 2-bit field selects the byte to be tested when the watchdog is in the byte test mode."]
    #[inline(always)]
    pub fn bytesel(&self) -> BYTESEL_R {
        BYTESEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Allows the WDOG's functional test mode to be disabled permanently"]
    #[inline(always)]
    pub fn distestwdog(&self) -> DISTESTWDOG_R {
        DISTESTWDOG_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables or disables the WDOG's operation"]
    #[inline(always)]
    #[must_use]
    pub fn wdogen(&mut self) -> WDOGEN_W<0> {
        WDOGEN_W::new(self)
    }
    #[doc = "Bit 1 - Selects clock source for the WDOG timer and other internal timing operations."]
    #[inline(always)]
    #[must_use]
    pub fn clksrc(&mut self) -> CLKSRC_W<1> {
        CLKSRC_W::new(self)
    }
    #[doc = "Bit 2 - Used to enable the debug breadcrumbs feature"]
    #[inline(always)]
    #[must_use]
    pub fn irqrsten(&mut self) -> IRQRSTEN_W<2> {
        IRQRSTEN_W::new(self)
    }
    #[doc = "Bit 3 - Enables Windowing mode."]
    #[inline(always)]
    #[must_use]
    pub fn winen(&mut self) -> WINEN_W<3> {
        WINEN_W::new(self)
    }
    #[doc = "Bit 4 - Enables updates to watchdog write-once registers, after the reset-triggered initial configuration window (WCT) closes, through unlock sequence"]
    #[inline(always)]
    #[must_use]
    pub fn allowupdate(&mut self) -> ALLOWUPDATE_W<4> {
        ALLOWUPDATE_W::new(self)
    }
    #[doc = "Bit 5 - Enables or disables WDOG in Debug mode."]
    #[inline(always)]
    #[must_use]
    pub fn dbgen(&mut self) -> DBGEN_W<5> {
        DBGEN_W::new(self)
    }
    #[doc = "Bit 6 - Enables or disables WDOG in Stop mode."]
    #[inline(always)]
    #[must_use]
    pub fn stopen(&mut self) -> STOPEN_W<6> {
        STOPEN_W::new(self)
    }
    #[doc = "Bit 7 - Enables or disables WDOG in Wait mode."]
    #[inline(always)]
    #[must_use]
    pub fn waiten(&mut self) -> WAITEN_W<7> {
        WAITEN_W::new(self)
    }
    #[doc = "Bit 10 - Puts the watchdog in the functional test mode"]
    #[inline(always)]
    #[must_use]
    pub fn testwdog(&mut self) -> TESTWDOG_W<10> {
        TESTWDOG_W::new(self)
    }
    #[doc = "Bit 11 - Effective only if TESTWDOG is set. Selects the test to be run on the watchdog timer."]
    #[inline(always)]
    #[must_use]
    pub fn testsel(&mut self) -> TESTSEL_W<11> {
        TESTSEL_W::new(self)
    }
    #[doc = "Bits 12:13 - This 2-bit field selects the byte to be tested when the watchdog is in the byte test mode."]
    #[inline(always)]
    #[must_use]
    pub fn bytesel(&mut self) -> BYTESEL_W<12> {
        BYTESEL_W::new(self)
    }
    #[doc = "Bit 14 - Allows the WDOG's functional test mode to be disabled permanently"]
    #[inline(always)]
    #[must_use]
    pub fn distestwdog(&mut self) -> DISTESTWDOG_W<14> {
        DISTESTWDOG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Status and Control Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stctrlh](index.html) module"]
pub struct STCTRLH_SPEC;
impl crate::RegisterSpec for STCTRLH_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [stctrlh::R](R) reader structure"]
impl crate::Readable for STCTRLH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stctrlh::W](W) writer structure"]
impl crate::Writable for STCTRLH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STCTRLH to value 0x01d3"]
impl crate::Resettable for STCTRLH_SPEC {
    const RESET_VALUE: Self::Ux = 0x01d3;
}
