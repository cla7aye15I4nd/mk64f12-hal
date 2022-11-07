#[doc = "Register `SDID` reader"]
pub struct R(crate::R<SDID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SDID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SDID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SDID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PINID` reader - Pincount identification"]
pub type PINID_R = crate::FieldReader<u8, PINID_A>;
#[doc = "Pincount identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PINID_A {
    #[doc = "2: 32-pin"]
    _0010 = 2,
    #[doc = "4: 48-pin"]
    _0100 = 4,
    #[doc = "5: 64-pin"]
    _0101 = 5,
    #[doc = "6: 80-pin"]
    _0110 = 6,
    #[doc = "7: 81-pin or 121-pin"]
    _0111 = 7,
    #[doc = "8: 100-pin"]
    _1000 = 8,
    #[doc = "9: 121-pin"]
    _1001 = 9,
    #[doc = "10: 144-pin"]
    _1010 = 10,
    #[doc = "11: Custom pinout (WLCSP)"]
    _1011 = 11,
    #[doc = "12: 169-pin"]
    _1100 = 12,
    #[doc = "14: 256-pin"]
    _1110 = 14,
}
impl From<PINID_A> for u8 {
    #[inline(always)]
    fn from(variant: PINID_A) -> Self {
        variant as _
    }
}
impl PINID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PINID_A> {
        match self.bits {
            2 => Some(PINID_A::_0010),
            4 => Some(PINID_A::_0100),
            5 => Some(PINID_A::_0101),
            6 => Some(PINID_A::_0110),
            7 => Some(PINID_A::_0111),
            8 => Some(PINID_A::_1000),
            9 => Some(PINID_A::_1001),
            10 => Some(PINID_A::_1010),
            11 => Some(PINID_A::_1011),
            12 => Some(PINID_A::_1100),
            14 => Some(PINID_A::_1110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == PINID_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == PINID_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == PINID_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == PINID_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == PINID_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == PINID_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == PINID_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == PINID_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == PINID_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == PINID_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == PINID_A::_1110
    }
}
#[doc = "Field `FAMID` reader - Kinetis family identification"]
pub type FAMID_R = crate::FieldReader<u8, FAMID_A>;
#[doc = "Kinetis family identification\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FAMID_A {
    #[doc = "0: K1x Family (without tamper)"]
    _000 = 0,
    #[doc = "1: K2x Family (without tamper)"]
    _001 = 1,
    #[doc = "2: K3x Family or K1x/K6x Family (with tamper)"]
    _010 = 2,
    #[doc = "3: K4x Family or K2x Family (with tamper)"]
    _011 = 3,
    #[doc = "4: K6x Family (without tamper)"]
    _100 = 4,
    #[doc = "5: K7x Family"]
    _101 = 5,
}
impl From<FAMID_A> for u8 {
    #[inline(always)]
    fn from(variant: FAMID_A) -> Self {
        variant as _
    }
}
impl FAMID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FAMID_A> {
        match self.bits {
            0 => Some(FAMID_A::_000),
            1 => Some(FAMID_A::_001),
            2 => Some(FAMID_A::_010),
            3 => Some(FAMID_A::_011),
            4 => Some(FAMID_A::_100),
            5 => Some(FAMID_A::_101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FAMID_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FAMID_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == FAMID_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == FAMID_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == FAMID_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == FAMID_A::_101
    }
}
#[doc = "Field `DIEID` reader - Device Die ID"]
pub type DIEID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REVID` reader - Device revision number"]
pub type REVID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SERIESID` reader - Kinetis Series ID"]
pub type SERIESID_R = crate::FieldReader<u8, SERIESID_A>;
#[doc = "Kinetis Series ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SERIESID_A {
    #[doc = "0: Kinetis K series"]
    _0000 = 0,
    #[doc = "1: Kinetis L series"]
    _0001 = 1,
    #[doc = "5: Kinetis W series"]
    _0101 = 5,
    #[doc = "6: Kinetis V series"]
    _0110 = 6,
}
impl From<SERIESID_A> for u8 {
    #[inline(always)]
    fn from(variant: SERIESID_A) -> Self {
        variant as _
    }
}
impl SERIESID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SERIESID_A> {
        match self.bits {
            0 => Some(SERIESID_A::_0000),
            1 => Some(SERIESID_A::_0001),
            5 => Some(SERIESID_A::_0101),
            6 => Some(SERIESID_A::_0110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == SERIESID_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == SERIESID_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == SERIESID_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == SERIESID_A::_0110
    }
}
#[doc = "Field `SUBFAMID` reader - Kinetis Sub-Family ID"]
pub type SUBFAMID_R = crate::FieldReader<u8, SUBFAMID_A>;
#[doc = "Kinetis Sub-Family ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SUBFAMID_A {
    #[doc = "0: Kx0 Subfamily"]
    _0000 = 0,
    #[doc = "1: Kx1 Subfamily (tamper detect)"]
    _0001 = 1,
    #[doc = "2: Kx2 Subfamily"]
    _0010 = 2,
    #[doc = "3: Kx3 Subfamily (tamper detect)"]
    _0011 = 3,
    #[doc = "4: Kx4 Subfamily"]
    _0100 = 4,
    #[doc = "5: Kx5 Subfamily (tamper detect)"]
    _0101 = 5,
    #[doc = "6: Kx6 Subfamily"]
    _0110 = 6,
}
impl From<SUBFAMID_A> for u8 {
    #[inline(always)]
    fn from(variant: SUBFAMID_A) -> Self {
        variant as _
    }
}
impl SUBFAMID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SUBFAMID_A> {
        match self.bits {
            0 => Some(SUBFAMID_A::_0000),
            1 => Some(SUBFAMID_A::_0001),
            2 => Some(SUBFAMID_A::_0010),
            3 => Some(SUBFAMID_A::_0011),
            4 => Some(SUBFAMID_A::_0100),
            5 => Some(SUBFAMID_A::_0101),
            6 => Some(SUBFAMID_A::_0110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == SUBFAMID_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == SUBFAMID_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == SUBFAMID_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == SUBFAMID_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == SUBFAMID_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == SUBFAMID_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == SUBFAMID_A::_0110
    }
}
#[doc = "Field `FAMILYID` reader - Kinetis Family ID"]
pub type FAMILYID_R = crate::FieldReader<u8, FAMILYID_A>;
#[doc = "Kinetis Family ID\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FAMILYID_A {
    #[doc = "1: K1x Family"]
    _0001 = 1,
    #[doc = "2: K2x Family"]
    _0010 = 2,
    #[doc = "3: K3x Family"]
    _0011 = 3,
    #[doc = "4: K4x Family"]
    _0100 = 4,
    #[doc = "6: K6x Family"]
    _0110 = 6,
    #[doc = "7: K7x Family"]
    _0111 = 7,
}
impl From<FAMILYID_A> for u8 {
    #[inline(always)]
    fn from(variant: FAMILYID_A) -> Self {
        variant as _
    }
}
impl FAMILYID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FAMILYID_A> {
        match self.bits {
            1 => Some(FAMILYID_A::_0001),
            2 => Some(FAMILYID_A::_0010),
            3 => Some(FAMILYID_A::_0011),
            4 => Some(FAMILYID_A::_0100),
            6 => Some(FAMILYID_A::_0110),
            7 => Some(FAMILYID_A::_0111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == FAMILYID_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == FAMILYID_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == FAMILYID_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == FAMILYID_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == FAMILYID_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == FAMILYID_A::_0111
    }
}
impl R {
    #[doc = "Bits 0:3 - Pincount identification"]
    #[inline(always)]
    pub fn pinid(&self) -> PINID_R {
        PINID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Kinetis family identification"]
    #[inline(always)]
    pub fn famid(&self) -> FAMID_R {
        FAMID_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:11 - Device Die ID"]
    #[inline(always)]
    pub fn dieid(&self) -> DIEID_R {
        DIEID_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:15 - Device revision number"]
    #[inline(always)]
    pub fn revid(&self) -> REVID_R {
        REVID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Kinetis Series ID"]
    #[inline(always)]
    pub fn seriesid(&self) -> SERIESID_R {
        SERIESID_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Kinetis Sub-Family ID"]
    #[inline(always)]
    pub fn subfamid(&self) -> SUBFAMID_R {
        SUBFAMID_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Kinetis Family ID"]
    #[inline(always)]
    pub fn familyid(&self) -> FAMILYID_R {
        FAMILYID_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "System Device Identification Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdid](index.html) module"]
pub struct SDID_SPEC;
impl crate::RegisterSpec for SDID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sdid::R](R) reader structure"]
impl crate::Readable for SDID_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SDID to value 0x0380"]
impl crate::Resettable for SDID_SPEC {
    const RESET_VALUE: Self::Ux = 0x0380;
}
