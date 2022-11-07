#[doc = "Register `MMCBOOT` reader"]
pub struct R(crate::R<MMCBOOT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMCBOOT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMCBOOT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMCBOOT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMCBOOT` writer"]
pub struct W(crate::W<MMCBOOT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMCBOOT_SPEC>;
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
impl From<crate::W<MMCBOOT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMCBOOT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTOCVACK` reader - Boot ACK Time Out Counter Value"]
pub type DTOCVACK_R = crate::FieldReader<u8, DTOCVACK_A>;
#[doc = "Boot ACK Time Out Counter Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DTOCVACK_A {
    #[doc = "0: SDCLK x 2^8"]
    _0000 = 0,
    #[doc = "1: SDCLK x 2^9"]
    _0001 = 1,
    #[doc = "2: SDCLK x 2^10"]
    _0010 = 2,
    #[doc = "3: SDCLK x 2^11"]
    _0011 = 3,
    #[doc = "4: SDCLK x 2^12"]
    _0100 = 4,
    #[doc = "5: SDCLK x 2^13"]
    _0101 = 5,
    #[doc = "6: SDCLK x 2^14"]
    _0110 = 6,
    #[doc = "7: SDCLK x 2^15"]
    _0111 = 7,
    #[doc = "14: SDCLK x 2^22"]
    _1110 = 14,
}
impl From<DTOCVACK_A> for u8 {
    #[inline(always)]
    fn from(variant: DTOCVACK_A) -> Self {
        variant as _
    }
}
impl DTOCVACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTOCVACK_A> {
        match self.bits {
            0 => Some(DTOCVACK_A::_0000),
            1 => Some(DTOCVACK_A::_0001),
            2 => Some(DTOCVACK_A::_0010),
            3 => Some(DTOCVACK_A::_0011),
            4 => Some(DTOCVACK_A::_0100),
            5 => Some(DTOCVACK_A::_0101),
            6 => Some(DTOCVACK_A::_0110),
            7 => Some(DTOCVACK_A::_0111),
            14 => Some(DTOCVACK_A::_1110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DTOCVACK_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == DTOCVACK_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == DTOCVACK_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == DTOCVACK_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == DTOCVACK_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == DTOCVACK_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == DTOCVACK_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == DTOCVACK_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == DTOCVACK_A::_1110
    }
}
#[doc = "Field `DTOCVACK` writer - Boot ACK Time Out Counter Value"]
pub type DTOCVACK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, MMCBOOT_SPEC, u8, DTOCVACK_A, 4, O>;
impl<'a, const O: u8> DTOCVACK_W<'a, O> {
    #[doc = "SDCLK x 2^8"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(DTOCVACK_A::_0000)
    }
    #[doc = "SDCLK x 2^9"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(DTOCVACK_A::_0001)
    }
    #[doc = "SDCLK x 2^10"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(DTOCVACK_A::_0010)
    }
    #[doc = "SDCLK x 2^11"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(DTOCVACK_A::_0011)
    }
    #[doc = "SDCLK x 2^12"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(DTOCVACK_A::_0100)
    }
    #[doc = "SDCLK x 2^13"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(DTOCVACK_A::_0101)
    }
    #[doc = "SDCLK x 2^14"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(DTOCVACK_A::_0110)
    }
    #[doc = "SDCLK x 2^15"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(DTOCVACK_A::_0111)
    }
    #[doc = "SDCLK x 2^22"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(DTOCVACK_A::_1110)
    }
}
#[doc = "Field `BOOTACK` reader - Boot Ack Mode Select"]
pub type BOOTACK_R = crate::BitReader<BOOTACK_A>;
#[doc = "Boot Ack Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOTACK_A {
    #[doc = "0: No ack."]
    _0 = 0,
    #[doc = "1: Ack."]
    _1 = 1,
}
impl From<BOOTACK_A> for bool {
    #[inline(always)]
    fn from(variant: BOOTACK_A) -> Self {
        variant as u8 != 0
    }
}
impl BOOTACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOTACK_A {
        match self.bits {
            false => BOOTACK_A::_0,
            true => BOOTACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOOTACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOOTACK_A::_1
    }
}
#[doc = "Field `BOOTACK` writer - Boot Ack Mode Select"]
pub type BOOTACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCBOOT_SPEC, BOOTACK_A, O>;
impl<'a, const O: u8> BOOTACK_W<'a, O> {
    #[doc = "No ack."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOOTACK_A::_0)
    }
    #[doc = "Ack."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOOTACK_A::_1)
    }
}
#[doc = "Field `BOOTMODE` reader - Boot Mode Select"]
pub type BOOTMODE_R = crate::BitReader<BOOTMODE_A>;
#[doc = "Boot Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOTMODE_A {
    #[doc = "0: Normal boot."]
    _0 = 0,
    #[doc = "1: Alternative boot."]
    _1 = 1,
}
impl From<BOOTMODE_A> for bool {
    #[inline(always)]
    fn from(variant: BOOTMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl BOOTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOTMODE_A {
        match self.bits {
            false => BOOTMODE_A::_0,
            true => BOOTMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOOTMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOOTMODE_A::_1
    }
}
#[doc = "Field `BOOTMODE` writer - Boot Mode Select"]
pub type BOOTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCBOOT_SPEC, BOOTMODE_A, O>;
impl<'a, const O: u8> BOOTMODE_W<'a, O> {
    #[doc = "Normal boot."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOOTMODE_A::_0)
    }
    #[doc = "Alternative boot."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOOTMODE_A::_1)
    }
}
#[doc = "Field `BOOTEN` reader - Boot Mode Enable"]
pub type BOOTEN_R = crate::BitReader<BOOTEN_A>;
#[doc = "Boot Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOOTEN_A {
    #[doc = "0: Fast boot disable."]
    _0 = 0,
    #[doc = "1: Fast boot enable."]
    _1 = 1,
}
impl From<BOOTEN_A> for bool {
    #[inline(always)]
    fn from(variant: BOOTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BOOTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOTEN_A {
        match self.bits {
            false => BOOTEN_A::_0,
            true => BOOTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BOOTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BOOTEN_A::_1
    }
}
#[doc = "Field `BOOTEN` writer - Boot Mode Enable"]
pub type BOOTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCBOOT_SPEC, BOOTEN_A, O>;
impl<'a, const O: u8> BOOTEN_W<'a, O> {
    #[doc = "Fast boot disable."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BOOTEN_A::_0)
    }
    #[doc = "Fast boot enable."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BOOTEN_A::_1)
    }
}
#[doc = "Field `AUTOSABGEN` reader - When boot, enable auto stop at block gap function"]
pub type AUTOSABGEN_R = crate::BitReader<bool>;
#[doc = "Field `AUTOSABGEN` writer - When boot, enable auto stop at block gap function"]
pub type AUTOSABGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MMCBOOT_SPEC, bool, O>;
#[doc = "Field `BOOTBLKCNT` reader - Defines the stop at block gap value of automatic mode"]
pub type BOOTBLKCNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BOOTBLKCNT` writer - Defines the stop at block gap value of automatic mode"]
pub type BOOTBLKCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMCBOOT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:3 - Boot ACK Time Out Counter Value"]
    #[inline(always)]
    pub fn dtocvack(&self) -> DTOCVACK_R {
        DTOCVACK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Boot Ack Mode Select"]
    #[inline(always)]
    pub fn bootack(&self) -> BOOTACK_R {
        BOOTACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Boot Mode Select"]
    #[inline(always)]
    pub fn bootmode(&self) -> BOOTMODE_R {
        BOOTMODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Boot Mode Enable"]
    #[inline(always)]
    pub fn booten(&self) -> BOOTEN_R {
        BOOTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When boot, enable auto stop at block gap function"]
    #[inline(always)]
    pub fn autosabgen(&self) -> AUTOSABGEN_R {
        AUTOSABGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Defines the stop at block gap value of automatic mode"]
    #[inline(always)]
    pub fn bootblkcnt(&self) -> BOOTBLKCNT_R {
        BOOTBLKCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Boot ACK Time Out Counter Value"]
    #[inline(always)]
    #[must_use]
    pub fn dtocvack(&mut self) -> DTOCVACK_W<0> {
        DTOCVACK_W::new(self)
    }
    #[doc = "Bit 4 - Boot Ack Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn bootack(&mut self) -> BOOTACK_W<4> {
        BOOTACK_W::new(self)
    }
    #[doc = "Bit 5 - Boot Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn bootmode(&mut self) -> BOOTMODE_W<5> {
        BOOTMODE_W::new(self)
    }
    #[doc = "Bit 6 - Boot Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn booten(&mut self) -> BOOTEN_W<6> {
        BOOTEN_W::new(self)
    }
    #[doc = "Bit 7 - When boot, enable auto stop at block gap function"]
    #[inline(always)]
    #[must_use]
    pub fn autosabgen(&mut self) -> AUTOSABGEN_W<7> {
        AUTOSABGEN_W::new(self)
    }
    #[doc = "Bits 16:31 - Defines the stop at block gap value of automatic mode"]
    #[inline(always)]
    #[must_use]
    pub fn bootblkcnt(&mut self) -> BOOTBLKCNT_W<16> {
        BOOTBLKCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MMC Boot register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmcboot](index.html) module"]
pub struct MMCBOOT_SPEC;
impl crate::RegisterSpec for MMCBOOT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmcboot::R](R) reader structure"]
impl crate::Readable for MMCBOOT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmcboot::W](W) writer structure"]
impl crate::Writable for MMCBOOT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMCBOOT to value 0"]
impl crate::Resettable for MMCBOOT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
