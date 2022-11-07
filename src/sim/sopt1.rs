#[doc = "Register `SOPT1` reader"]
pub struct R(crate::R<SOPT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOPT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOPT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOPT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOPT1` writer"]
pub struct W(crate::W<SOPT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOPT1_SPEC>;
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
impl From<crate::W<SOPT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOPT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMSIZE` reader - RAM size"]
pub type RAMSIZE_R = crate::FieldReader<u8, RAMSIZE_A>;
#[doc = "RAM size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAMSIZE_A {
    #[doc = "1: 8 KB"]
    _0001 = 1,
    #[doc = "3: 16 KB"]
    _0011 = 3,
    #[doc = "4: 24 KB"]
    _0100 = 4,
    #[doc = "5: 32 KB"]
    _0101 = 5,
    #[doc = "6: 48 KB"]
    _0110 = 6,
    #[doc = "7: 64 KB"]
    _0111 = 7,
    #[doc = "8: 96 KB"]
    _1000 = 8,
    #[doc = "9: 128 KB"]
    _1001 = 9,
    #[doc = "11: 256 KB"]
    _1011 = 11,
}
impl From<RAMSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMSIZE_A) -> Self {
        variant as _
    }
}
impl RAMSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAMSIZE_A> {
        match self.bits {
            1 => Some(RAMSIZE_A::_0001),
            3 => Some(RAMSIZE_A::_0011),
            4 => Some(RAMSIZE_A::_0100),
            5 => Some(RAMSIZE_A::_0101),
            6 => Some(RAMSIZE_A::_0110),
            7 => Some(RAMSIZE_A::_0111),
            8 => Some(RAMSIZE_A::_1000),
            9 => Some(RAMSIZE_A::_1001),
            11 => Some(RAMSIZE_A::_1011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == RAMSIZE_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == RAMSIZE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == RAMSIZE_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == RAMSIZE_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == RAMSIZE_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == RAMSIZE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == RAMSIZE_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == RAMSIZE_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == RAMSIZE_A::_1011
    }
}
#[doc = "Field `OSC32KSEL` reader - 32K oscillator clock select"]
pub type OSC32KSEL_R = crate::FieldReader<u8, OSC32KSEL_A>;
#[doc = "32K oscillator clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSC32KSEL_A {
    #[doc = "0: System oscillator (OSC32KCLK)"]
    _00 = 0,
    #[doc = "2: RTC 32.768kHz oscillator"]
    _10 = 2,
    #[doc = "3: LPO 1 kHz"]
    _11 = 3,
}
impl From<OSC32KSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OSC32KSEL_A) -> Self {
        variant as _
    }
}
impl OSC32KSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OSC32KSEL_A> {
        match self.bits {
            0 => Some(OSC32KSEL_A::_00),
            2 => Some(OSC32KSEL_A::_10),
            3 => Some(OSC32KSEL_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OSC32KSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OSC32KSEL_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OSC32KSEL_A::_11
    }
}
#[doc = "Field `OSC32KSEL` writer - 32K oscillator clock select"]
pub type OSC32KSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOPT1_SPEC, u8, OSC32KSEL_A, 2, O>;
impl<'a, const O: u8> OSC32KSEL_W<'a, O> {
    #[doc = "System oscillator (OSC32KCLK)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OSC32KSEL_A::_00)
    }
    #[doc = "RTC 32.768kHz oscillator"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OSC32KSEL_A::_10)
    }
    #[doc = "LPO 1 kHz"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OSC32KSEL_A::_11)
    }
}
#[doc = "Field `USBVSTBY` reader - USB voltage regulator in standby mode during VLPR and VLPW modes"]
pub type USBVSTBY_R = crate::BitReader<USBVSTBY_A>;
#[doc = "USB voltage regulator in standby mode during VLPR and VLPW modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBVSTBY_A {
    #[doc = "0: USB voltage regulator not in standby during VLPR and VLPW modes."]
    _0 = 0,
    #[doc = "1: USB voltage regulator in standby during VLPR and VLPW modes."]
    _1 = 1,
}
impl From<USBVSTBY_A> for bool {
    #[inline(always)]
    fn from(variant: USBVSTBY_A) -> Self {
        variant as u8 != 0
    }
}
impl USBVSTBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBVSTBY_A {
        match self.bits {
            false => USBVSTBY_A::_0,
            true => USBVSTBY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBVSTBY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBVSTBY_A::_1
    }
}
#[doc = "Field `USBVSTBY` writer - USB voltage regulator in standby mode during VLPR and VLPW modes"]
pub type USBVSTBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT1_SPEC, USBVSTBY_A, O>;
impl<'a, const O: u8> USBVSTBY_W<'a, O> {
    #[doc = "USB voltage regulator not in standby during VLPR and VLPW modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBVSTBY_A::_0)
    }
    #[doc = "USB voltage regulator in standby during VLPR and VLPW modes."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBVSTBY_A::_1)
    }
}
#[doc = "Field `USBSSTBY` reader - USB voltage regulator in standby mode during Stop, VLPS, LLS and VLLS modes."]
pub type USBSSTBY_R = crate::BitReader<USBSSTBY_A>;
#[doc = "USB voltage regulator in standby mode during Stop, VLPS, LLS and VLLS modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBSSTBY_A {
    #[doc = "0: USB voltage regulator not in standby during Stop, VLPS, LLS and VLLS modes."]
    _0 = 0,
    #[doc = "1: USB voltage regulator in standby during Stop, VLPS, LLS and VLLS modes."]
    _1 = 1,
}
impl From<USBSSTBY_A> for bool {
    #[inline(always)]
    fn from(variant: USBSSTBY_A) -> Self {
        variant as u8 != 0
    }
}
impl USBSSTBY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBSSTBY_A {
        match self.bits {
            false => USBSSTBY_A::_0,
            true => USBSSTBY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBSSTBY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBSSTBY_A::_1
    }
}
#[doc = "Field `USBSSTBY` writer - USB voltage regulator in standby mode during Stop, VLPS, LLS and VLLS modes."]
pub type USBSSTBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT1_SPEC, USBSSTBY_A, O>;
impl<'a, const O: u8> USBSSTBY_W<'a, O> {
    #[doc = "USB voltage regulator not in standby during Stop, VLPS, LLS and VLLS modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBSSTBY_A::_0)
    }
    #[doc = "USB voltage regulator in standby during Stop, VLPS, LLS and VLLS modes."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBSSTBY_A::_1)
    }
}
#[doc = "Field `USBREGEN` reader - USB voltage regulator enable"]
pub type USBREGEN_R = crate::BitReader<USBREGEN_A>;
#[doc = "USB voltage regulator enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBREGEN_A {
    #[doc = "0: USB voltage regulator is disabled."]
    _0 = 0,
    #[doc = "1: USB voltage regulator is enabled."]
    _1 = 1,
}
impl From<USBREGEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBREGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl USBREGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBREGEN_A {
        match self.bits {
            false => USBREGEN_A::_0,
            true => USBREGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBREGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBREGEN_A::_1
    }
}
#[doc = "Field `USBREGEN` writer - USB voltage regulator enable"]
pub type USBREGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT1_SPEC, USBREGEN_A, O>;
impl<'a, const O: u8> USBREGEN_W<'a, O> {
    #[doc = "USB voltage regulator is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBREGEN_A::_0)
    }
    #[doc = "USB voltage regulator is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBREGEN_A::_1)
    }
}
impl R {
    #[doc = "Bits 12:15 - RAM size"]
    #[inline(always)]
    pub fn ramsize(&self) -> RAMSIZE_R {
        RAMSIZE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:19 - 32K oscillator clock select"]
    #[inline(always)]
    pub fn osc32ksel(&self) -> OSC32KSEL_R {
        OSC32KSEL_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 29 - USB voltage regulator in standby mode during VLPR and VLPW modes"]
    #[inline(always)]
    pub fn usbvstby(&self) -> USBVSTBY_R {
        USBVSTBY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - USB voltage regulator in standby mode during Stop, VLPS, LLS and VLLS modes."]
    #[inline(always)]
    pub fn usbsstby(&self) -> USBSSTBY_R {
        USBSSTBY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - USB voltage regulator enable"]
    #[inline(always)]
    pub fn usbregen(&self) -> USBREGEN_R {
        USBREGEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 18:19 - 32K oscillator clock select"]
    #[inline(always)]
    #[must_use]
    pub fn osc32ksel(&mut self) -> OSC32KSEL_W<18> {
        OSC32KSEL_W::new(self)
    }
    #[doc = "Bit 29 - USB voltage regulator in standby mode during VLPR and VLPW modes"]
    #[inline(always)]
    #[must_use]
    pub fn usbvstby(&mut self) -> USBVSTBY_W<29> {
        USBVSTBY_W::new(self)
    }
    #[doc = "Bit 30 - USB voltage regulator in standby mode during Stop, VLPS, LLS and VLLS modes."]
    #[inline(always)]
    #[must_use]
    pub fn usbsstby(&mut self) -> USBSSTBY_W<30> {
        USBSSTBY_W::new(self)
    }
    #[doc = "Bit 31 - USB voltage regulator enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbregen(&mut self) -> USBREGEN_W<31> {
        USBREGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Options Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopt1](index.html) module"]
pub struct SOPT1_SPEC;
impl crate::RegisterSpec for SOPT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sopt1::R](R) reader structure"]
impl crate::Readable for SOPT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sopt1::W](W) writer structure"]
impl crate::Writable for SOPT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOPT1 to value 0x8000_0000"]
impl crate::Resettable for SOPT1_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
