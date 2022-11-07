#[doc = "Register `FCFG1` reader"]
pub struct R(crate::R<FCFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FCFG1` writer"]
pub struct W(crate::W<FCFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FCFG1_SPEC>;
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
impl From<crate::W<FCFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FCFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLASHDIS` reader - Flash Disable"]
pub type FLASHDIS_R = crate::BitReader<FLASHDIS_A>;
#[doc = "Flash Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHDIS_A {
    #[doc = "0: Flash is enabled"]
    _0 = 0,
    #[doc = "1: Flash is disabled"]
    _1 = 1,
}
impl From<FLASHDIS_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASHDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHDIS_A {
        match self.bits {
            false => FLASHDIS_A::_0,
            true => FLASHDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLASHDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLASHDIS_A::_1
    }
}
#[doc = "Field `FLASHDIS` writer - Flash Disable"]
pub type FLASHDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCFG1_SPEC, FLASHDIS_A, O>;
impl<'a, const O: u8> FLASHDIS_W<'a, O> {
    #[doc = "Flash is enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLASHDIS_A::_0)
    }
    #[doc = "Flash is disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLASHDIS_A::_1)
    }
}
#[doc = "Field `FLASHDOZE` reader - Flash Doze"]
pub type FLASHDOZE_R = crate::BitReader<FLASHDOZE_A>;
#[doc = "Flash Doze\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLASHDOZE_A {
    #[doc = "0: Flash remains enabled during Wait mode"]
    _0 = 0,
    #[doc = "1: Flash is disabled for the duration of Wait mode"]
    _1 = 1,
}
impl From<FLASHDOZE_A> for bool {
    #[inline(always)]
    fn from(variant: FLASHDOZE_A) -> Self {
        variant as u8 != 0
    }
}
impl FLASHDOZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLASHDOZE_A {
        match self.bits {
            false => FLASHDOZE_A::_0,
            true => FLASHDOZE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLASHDOZE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLASHDOZE_A::_1
    }
}
#[doc = "Field `FLASHDOZE` writer - Flash Doze"]
pub type FLASHDOZE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FCFG1_SPEC, FLASHDOZE_A, O>;
impl<'a, const O: u8> FLASHDOZE_W<'a, O> {
    #[doc = "Flash remains enabled during Wait mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLASHDOZE_A::_0)
    }
    #[doc = "Flash is disabled for the duration of Wait mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLASHDOZE_A::_1)
    }
}
#[doc = "Field `DEPART` reader - FlexNVM partition"]
pub type DEPART_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EESIZE` reader - EEPROM size"]
pub type EESIZE_R = crate::FieldReader<u8, EESIZE_A>;
#[doc = "EEPROM size\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EESIZE_A {
    #[doc = "0: 16 KB"]
    _0000 = 0,
    #[doc = "1: 8 KB"]
    _0001 = 1,
    #[doc = "2: 4 KB"]
    _0010 = 2,
    #[doc = "3: 2 KB"]
    _0011 = 3,
    #[doc = "4: 1 KB"]
    _0100 = 4,
    #[doc = "5: 512 Bytes"]
    _0101 = 5,
    #[doc = "6: 256 Bytes"]
    _0110 = 6,
    #[doc = "7: 128 Bytes"]
    _0111 = 7,
    #[doc = "8: 64 Bytes"]
    _1000 = 8,
    #[doc = "9: 32 Bytes"]
    _1001 = 9,
    #[doc = "15: 0 Bytes"]
    _1111 = 15,
}
impl From<EESIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: EESIZE_A) -> Self {
        variant as _
    }
}
impl EESIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EESIZE_A> {
        match self.bits {
            0 => Some(EESIZE_A::_0000),
            1 => Some(EESIZE_A::_0001),
            2 => Some(EESIZE_A::_0010),
            3 => Some(EESIZE_A::_0011),
            4 => Some(EESIZE_A::_0100),
            5 => Some(EESIZE_A::_0101),
            6 => Some(EESIZE_A::_0110),
            7 => Some(EESIZE_A::_0111),
            8 => Some(EESIZE_A::_1000),
            9 => Some(EESIZE_A::_1001),
            15 => Some(EESIZE_A::_1111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == EESIZE_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == EESIZE_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == EESIZE_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == EESIZE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == EESIZE_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == EESIZE_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == EESIZE_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == EESIZE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == EESIZE_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == EESIZE_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == EESIZE_A::_1111
    }
}
#[doc = "Field `PFSIZE` reader - Program flash size"]
pub type PFSIZE_R = crate::FieldReader<u8, PFSIZE_A>;
#[doc = "Program flash size\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PFSIZE_A {
    #[doc = "3: 32 KB of program flash memory"]
    _0011 = 3,
    #[doc = "5: 64 KB of program flash memory"]
    _0101 = 5,
    #[doc = "7: 128 KB of program flash memory"]
    _0111 = 7,
    #[doc = "9: 256 KB of program flash memory"]
    _1001 = 9,
    #[doc = "11: 512 KB of program flash memory"]
    _1011 = 11,
    #[doc = "13: 1024 KB of program flash memory"]
    _1101 = 13,
    #[doc = "15: 1024 KB of program flash memory"]
    _1111 = 15,
}
impl From<PFSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PFSIZE_A) -> Self {
        variant as _
    }
}
impl PFSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PFSIZE_A> {
        match self.bits {
            3 => Some(PFSIZE_A::_0011),
            5 => Some(PFSIZE_A::_0101),
            7 => Some(PFSIZE_A::_0111),
            9 => Some(PFSIZE_A::_1001),
            11 => Some(PFSIZE_A::_1011),
            13 => Some(PFSIZE_A::_1101),
            15 => Some(PFSIZE_A::_1111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == PFSIZE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == PFSIZE_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == PFSIZE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == PFSIZE_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == PFSIZE_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == PFSIZE_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == PFSIZE_A::_1111
    }
}
#[doc = "Field `NVMSIZE` reader - FlexNVM size"]
pub type NVMSIZE_R = crate::FieldReader<u8, NVMSIZE_A>;
#[doc = "FlexNVM size\n\nValue on reset: 15"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NVMSIZE_A {
    #[doc = "0: 0 KB of FlexNVM"]
    _0000 = 0,
    #[doc = "3: 32 KB of FlexNVM"]
    _0011 = 3,
    #[doc = "5: 64 KB of FlexNVM"]
    _0101 = 5,
    #[doc = "7: 128 KB of FlexNVM"]
    _0111 = 7,
    #[doc = "9: 256 KB of FlexNVM"]
    _1001 = 9,
    #[doc = "11: 512 KB of FlexNVM"]
    _1011 = 11,
    #[doc = "15: 512 KB of FlexNVM"]
    _1111 = 15,
}
impl From<NVMSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: NVMSIZE_A) -> Self {
        variant as _
    }
}
impl NVMSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NVMSIZE_A> {
        match self.bits {
            0 => Some(NVMSIZE_A::_0000),
            3 => Some(NVMSIZE_A::_0011),
            5 => Some(NVMSIZE_A::_0101),
            7 => Some(NVMSIZE_A::_0111),
            9 => Some(NVMSIZE_A::_1001),
            11 => Some(NVMSIZE_A::_1011),
            15 => Some(NVMSIZE_A::_1111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == NVMSIZE_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == NVMSIZE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == NVMSIZE_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == NVMSIZE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == NVMSIZE_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == NVMSIZE_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == NVMSIZE_A::_1111
    }
}
impl R {
    #[doc = "Bit 0 - Flash Disable"]
    #[inline(always)]
    pub fn flashdis(&self) -> FLASHDIS_R {
        FLASHDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Doze"]
    #[inline(always)]
    pub fn flashdoze(&self) -> FLASHDOZE_R {
        FLASHDOZE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - FlexNVM partition"]
    #[inline(always)]
    pub fn depart(&self) -> DEPART_R {
        DEPART_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - EEPROM size"]
    #[inline(always)]
    pub fn eesize(&self) -> EESIZE_R {
        EESIZE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Program flash size"]
    #[inline(always)]
    pub fn pfsize(&self) -> PFSIZE_R {
        PFSIZE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - FlexNVM size"]
    #[inline(always)]
    pub fn nvmsize(&self) -> NVMSIZE_R {
        NVMSIZE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Disable"]
    #[inline(always)]
    #[must_use]
    pub fn flashdis(&mut self) -> FLASHDIS_W<0> {
        FLASHDIS_W::new(self)
    }
    #[doc = "Bit 1 - Flash Doze"]
    #[inline(always)]
    #[must_use]
    pub fn flashdoze(&mut self) -> FLASHDOZE_W<1> {
        FLASHDOZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg1](index.html) module"]
pub struct FCFG1_SPEC;
impl crate::RegisterSpec for FCFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcfg1::R](R) reader structure"]
impl crate::Readable for FCFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fcfg1::W](W) writer structure"]
impl crate::Writable for FCFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FCFG1 to value 0xff0f_0f00"]
impl crate::Resettable for FCFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0xff0f_0f00;
}
