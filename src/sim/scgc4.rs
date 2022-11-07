#[doc = "Register `SCGC4` reader"]
pub struct R(crate::R<SCGC4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGC4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGC4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGC4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGC4` writer"]
pub struct W(crate::W<SCGC4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGC4_SPEC>;
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
impl From<crate::W<SCGC4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGC4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EWM` reader - EWM Clock Gate Control"]
pub type EWM_R = crate::BitReader<EWM_A>;
#[doc = "EWM Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWM_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<EWM_A> for bool {
    #[inline(always)]
    fn from(variant: EWM_A) -> Self {
        variant as u8 != 0
    }
}
impl EWM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EWM_A {
        match self.bits {
            false => EWM_A::_0,
            true => EWM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EWM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EWM_A::_1
    }
}
#[doc = "Field `EWM` writer - EWM Clock Gate Control"]
pub type EWM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC4_SPEC, EWM_A, O>;
impl<'a, const O: u8> EWM_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EWM_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EWM_A::_1)
    }
}
#[doc = "Field `CMT` reader - CMT Clock Gate Control"]
pub type CMT_R = crate::BitReader<CMT_A>;
#[doc = "CMT Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMT_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<CMT_A> for bool {
    #[inline(always)]
    fn from(variant: CMT_A) -> Self {
        variant as u8 != 0
    }
}
impl CMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMT_A {
        match self.bits {
            false => CMT_A::_0,
            true => CMT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMT_A::_1
    }
}
#[doc = "Field `CMT` writer - CMT Clock Gate Control"]
pub type CMT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC4_SPEC, CMT_A, O>;
impl<'a, const O: u8> CMT_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMT_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMT_A::_1)
    }
}
#[doc = "Field `I2C0` reader - I2C0 Clock Gate Control"]
pub type I2C0_R = crate::BitReader<I2C0_A>;
#[doc = "I2C0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<I2C0_A> for bool {
    #[inline(always)]
    fn from(variant: I2C0_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C0_A {
        match self.bits {
            false => I2C0_A::_0,
            true => I2C0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == I2C0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == I2C0_A::_1
    }
}
#[doc = "Field `I2C0` writer - I2C0 Clock Gate Control"]
pub type I2C0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC4_SPEC, I2C0_A, O>;
impl<'a, const O: u8> I2C0_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2C0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2C0_A::_1)
    }
}
#[doc = "Field `I2C1` reader - I2C1 Clock Gate Control"]
pub type I2C1_R = crate::BitReader<I2C1_A>;
#[doc = "I2C1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2C1_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<I2C1_A> for bool {
    #[inline(always)]
    fn from(variant: I2C1_A) -> Self {
        variant as u8 != 0
    }
}
impl I2C1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1_A {
        match self.bits {
            false => I2C1_A::_0,
            true => I2C1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == I2C1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == I2C1_A::_1
    }
}
#[doc = "Field `I2C1` writer - I2C1 Clock Gate Control"]
pub type I2C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC4_SPEC, I2C1_A, O>;
impl<'a, const O: u8> I2C1_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(I2C1_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(I2C1_A::_1)
    }
}
#[doc = "Field `UART0` reader - UART0 Clock Gate Control"]
pub type UART0_R = crate::BitReader<UART0_A>;
#[doc = "UART0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<UART0_A> for bool {
    #[inline(always)]
    fn from(variant: UART0_A) -> Self {
        variant as u8 != 0
    }
}
impl UART0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0_A {
        match self.bits {
            false => UART0_A::_0,
            true => UART0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART0_A::_1
    }
}
#[doc = "Field `UART0` writer - UART0 Clock Gate Control"]
pub type UART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC4_SPEC, UART0_A, O>;
impl<'a, const O: u8> UART0_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART0_A::_1)
    }
}
#[doc = "Field `UART1` reader - UART1 Clock Gate Control"]
pub type UART1_R = crate::BitReader<UART1_A>;
#[doc = "UART1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART1_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<UART1_A> for bool {
    #[inline(always)]
    fn from(variant: UART1_A) -> Self {
        variant as u8 != 0
    }
}
impl UART1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART1_A {
        match self.bits {
            false => UART1_A::_0,
            true => UART1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART1_A::_1
    }
}
#[doc = "Field `UART1` writer - UART1 Clock Gate Control"]
pub type UART1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC4_SPEC, UART1_A, O>;
impl<'a, const O: u8> UART1_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART1_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART1_A::_1)
    }
}
#[doc = "Field `UART2` reader - UART2 Clock Gate Control"]
pub type UART2_R = crate::BitReader<UART2_A>;
#[doc = "UART2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART2_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<UART2_A> for bool {
    #[inline(always)]
    fn from(variant: UART2_A) -> Self {
        variant as u8 != 0
    }
}
impl UART2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART2_A {
        match self.bits {
            false => UART2_A::_0,
            true => UART2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART2_A::_1
    }
}
#[doc = "Field `UART2` writer - UART2 Clock Gate Control"]
pub type UART2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC4_SPEC, UART2_A, O>;
impl<'a, const O: u8> UART2_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART2_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART2_A::_1)
    }
}
#[doc = "Field `UART3` reader - UART3 Clock Gate Control"]
pub type UART3_R = crate::BitReader<UART3_A>;
#[doc = "UART3 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART3_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<UART3_A> for bool {
    #[inline(always)]
    fn from(variant: UART3_A) -> Self {
        variant as u8 != 0
    }
}
impl UART3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART3_A {
        match self.bits {
            false => UART3_A::_0,
            true => UART3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART3_A::_1
    }
}
#[doc = "Field `UART3` writer - UART3 Clock Gate Control"]
pub type UART3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC4_SPEC, UART3_A, O>;
impl<'a, const O: u8> UART3_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART3_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART3_A::_1)
    }
}
#[doc = "Field `USBOTG` reader - USB Clock Gate Control"]
pub type USBOTG_R = crate::BitReader<USBOTG_A>;
#[doc = "USB Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBOTG_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<USBOTG_A> for bool {
    #[inline(always)]
    fn from(variant: USBOTG_A) -> Self {
        variant as u8 != 0
    }
}
impl USBOTG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBOTG_A {
        match self.bits {
            false => USBOTG_A::_0,
            true => USBOTG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBOTG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBOTG_A::_1
    }
}
#[doc = "Field `USBOTG` writer - USB Clock Gate Control"]
pub type USBOTG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC4_SPEC, USBOTG_A, O>;
impl<'a, const O: u8> USBOTG_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBOTG_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBOTG_A::_1)
    }
}
#[doc = "Field `CMP` reader - Comparator Clock Gate Control"]
pub type CMP_R = crate::BitReader<CMP_A>;
#[doc = "Comparator Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<CMP_A> for bool {
    #[inline(always)]
    fn from(variant: CMP_A) -> Self {
        variant as u8 != 0
    }
}
impl CMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMP_A {
        match self.bits {
            false => CMP_A::_0,
            true => CMP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMP_A::_1
    }
}
#[doc = "Field `CMP` writer - Comparator Clock Gate Control"]
pub type CMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC4_SPEC, CMP_A, O>;
impl<'a, const O: u8> CMP_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMP_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMP_A::_1)
    }
}
#[doc = "Field `VREF` reader - VREF Clock Gate Control"]
pub type VREF_R = crate::BitReader<VREF_A>;
#[doc = "VREF Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREF_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<VREF_A> for bool {
    #[inline(always)]
    fn from(variant: VREF_A) -> Self {
        variant as u8 != 0
    }
}
impl VREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREF_A {
        match self.bits {
            false => VREF_A::_0,
            true => VREF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VREF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VREF_A::_1
    }
}
#[doc = "Field `VREF` writer - VREF Clock Gate Control"]
pub type VREF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC4_SPEC, VREF_A, O>;
impl<'a, const O: u8> VREF_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VREF_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VREF_A::_1)
    }
}
impl R {
    #[doc = "Bit 1 - EWM Clock Gate Control"]
    #[inline(always)]
    pub fn ewm(&self) -> EWM_R {
        EWM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CMT Clock Gate Control"]
    #[inline(always)]
    pub fn cmt(&self) -> CMT_R {
        CMT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C0 Clock Gate Control"]
    #[inline(always)]
    pub fn i2c0(&self) -> I2C0_R {
        I2C0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C1 Clock Gate Control"]
    #[inline(always)]
    pub fn i2c1(&self) -> I2C1_R {
        I2C1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - UART0 Clock Gate Control"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - UART1 Clock Gate Control"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - UART2 Clock Gate Control"]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - UART3 Clock Gate Control"]
    #[inline(always)]
    pub fn uart3(&self) -> UART3_R {
        UART3_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 18 - USB Clock Gate Control"]
    #[inline(always)]
    pub fn usbotg(&self) -> USBOTG_R {
        USBOTG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Comparator Clock Gate Control"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - VREF Clock Gate Control"]
    #[inline(always)]
    pub fn vref(&self) -> VREF_R {
        VREF_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - EWM Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn ewm(&mut self) -> EWM_W<1> {
        EWM_W::new(self)
    }
    #[doc = "Bit 2 - CMT Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn cmt(&mut self) -> CMT_W<2> {
        CMT_W::new(self)
    }
    #[doc = "Bit 6 - I2C0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0(&mut self) -> I2C0_W<6> {
        I2C0_W::new(self)
    }
    #[doc = "Bit 7 - I2C1 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1(&mut self) -> I2C1_W<7> {
        I2C1_W::new(self)
    }
    #[doc = "Bit 10 - UART0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn uart0(&mut self) -> UART0_W<10> {
        UART0_W::new(self)
    }
    #[doc = "Bit 11 - UART1 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn uart1(&mut self) -> UART1_W<11> {
        UART1_W::new(self)
    }
    #[doc = "Bit 12 - UART2 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn uart2(&mut self) -> UART2_W<12> {
        UART2_W::new(self)
    }
    #[doc = "Bit 13 - UART3 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn uart3(&mut self) -> UART3_W<13> {
        UART3_W::new(self)
    }
    #[doc = "Bit 18 - USB Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn usbotg(&mut self) -> USBOTG_W<18> {
        USBOTG_W::new(self)
    }
    #[doc = "Bit 19 - Comparator Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn cmp(&mut self) -> CMP_W<19> {
        CMP_W::new(self)
    }
    #[doc = "Bit 20 - VREF Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn vref(&mut self) -> VREF_W<20> {
        VREF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Gating Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgc4](index.html) module"]
pub struct SCGC4_SPEC;
impl crate::RegisterSpec for SCGC4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgc4::R](R) reader structure"]
impl crate::Readable for SCGC4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgc4::W](W) writer structure"]
impl crate::Writable for SCGC4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGC4 to value 0xf010_0030"]
impl crate::Resettable for SCGC4_SPEC {
    const RESET_VALUE: Self::Ux = 0xf010_0030;
}
