#[doc = "Register `SOPT7` reader"]
pub struct R(crate::R<SOPT7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOPT7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOPT7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOPT7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOPT7` writer"]
pub struct W(crate::W<SOPT7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOPT7_SPEC>;
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
impl From<crate::W<SOPT7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOPT7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC0TRGSEL` reader - ADC0 trigger select"]
pub type ADC0TRGSEL_R = crate::FieldReader<u8, ADC0TRGSEL_A>;
#[doc = "ADC0 trigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC0TRGSEL_A {
    #[doc = "0: PDB external trigger pin input (PDB0_EXTRG)"]
    _0000 = 0,
    #[doc = "1: High speed comparator 0 output"]
    _0001 = 1,
    #[doc = "2: High speed comparator 1 output"]
    _0010 = 2,
    #[doc = "3: High speed comparator 2 output"]
    _0011 = 3,
    #[doc = "4: PIT trigger 0"]
    _0100 = 4,
    #[doc = "5: PIT trigger 1"]
    _0101 = 5,
    #[doc = "6: PIT trigger 2"]
    _0110 = 6,
    #[doc = "7: PIT trigger 3"]
    _0111 = 7,
    #[doc = "8: FTM0 trigger"]
    _1000 = 8,
    #[doc = "9: FTM1 trigger"]
    _1001 = 9,
    #[doc = "10: FTM2 trigger"]
    _1010 = 10,
    #[doc = "11: FTM3 trigger"]
    _1011 = 11,
    #[doc = "12: RTC alarm"]
    _1100 = 12,
    #[doc = "13: RTC seconds"]
    _1101 = 13,
    #[doc = "14: Low-power timer (LPTMR) trigger"]
    _1110 = 14,
}
impl From<ADC0TRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC0TRGSEL_A) -> Self {
        variant as _
    }
}
impl ADC0TRGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC0TRGSEL_A> {
        match self.bits {
            0 => Some(ADC0TRGSEL_A::_0000),
            1 => Some(ADC0TRGSEL_A::_0001),
            2 => Some(ADC0TRGSEL_A::_0010),
            3 => Some(ADC0TRGSEL_A::_0011),
            4 => Some(ADC0TRGSEL_A::_0100),
            5 => Some(ADC0TRGSEL_A::_0101),
            6 => Some(ADC0TRGSEL_A::_0110),
            7 => Some(ADC0TRGSEL_A::_0111),
            8 => Some(ADC0TRGSEL_A::_1000),
            9 => Some(ADC0TRGSEL_A::_1001),
            10 => Some(ADC0TRGSEL_A::_1010),
            11 => Some(ADC0TRGSEL_A::_1011),
            12 => Some(ADC0TRGSEL_A::_1100),
            13 => Some(ADC0TRGSEL_A::_1101),
            14 => Some(ADC0TRGSEL_A::_1110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == ADC0TRGSEL_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == ADC0TRGSEL_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == ADC0TRGSEL_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == ADC0TRGSEL_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == ADC0TRGSEL_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == ADC0TRGSEL_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == ADC0TRGSEL_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == ADC0TRGSEL_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == ADC0TRGSEL_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == ADC0TRGSEL_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == ADC0TRGSEL_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == ADC0TRGSEL_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == ADC0TRGSEL_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == ADC0TRGSEL_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == ADC0TRGSEL_A::_1110
    }
}
#[doc = "Field `ADC0TRGSEL` writer - ADC0 trigger select"]
pub type ADC0TRGSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOPT7_SPEC, u8, ADC0TRGSEL_A, 4, O>;
impl<'a, const O: u8> ADC0TRGSEL_W<'a, O> {
    #[doc = "PDB external trigger pin input (PDB0_EXTRG)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0000)
    }
    #[doc = "High speed comparator 0 output"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0001)
    }
    #[doc = "High speed comparator 1 output"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0010)
    }
    #[doc = "High speed comparator 2 output"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0011)
    }
    #[doc = "PIT trigger 0"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0100)
    }
    #[doc = "PIT trigger 1"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0101)
    }
    #[doc = "PIT trigger 2"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0110)
    }
    #[doc = "PIT trigger 3"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_0111)
    }
    #[doc = "FTM0 trigger"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1000)
    }
    #[doc = "FTM1 trigger"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1001)
    }
    #[doc = "FTM2 trigger"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1010)
    }
    #[doc = "FTM3 trigger"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1011)
    }
    #[doc = "RTC alarm"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1100)
    }
    #[doc = "RTC seconds"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1101)
    }
    #[doc = "Low-power timer (LPTMR) trigger"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(ADC0TRGSEL_A::_1110)
    }
}
#[doc = "Field `ADC0PRETRGSEL` reader - ADC0 pretrigger select"]
pub type ADC0PRETRGSEL_R = crate::BitReader<ADC0PRETRGSEL_A>;
#[doc = "ADC0 pretrigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0PRETRGSEL_A {
    #[doc = "0: Pre-trigger A"]
    _0 = 0,
    #[doc = "1: Pre-trigger B"]
    _1 = 1,
}
impl From<ADC0PRETRGSEL_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0PRETRGSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0PRETRGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0PRETRGSEL_A {
        match self.bits {
            false => ADC0PRETRGSEL_A::_0,
            true => ADC0PRETRGSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADC0PRETRGSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADC0PRETRGSEL_A::_1
    }
}
#[doc = "Field `ADC0PRETRGSEL` writer - ADC0 pretrigger select"]
pub type ADC0PRETRGSEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SOPT7_SPEC, ADC0PRETRGSEL_A, O>;
impl<'a, const O: u8> ADC0PRETRGSEL_W<'a, O> {
    #[doc = "Pre-trigger A"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC0PRETRGSEL_A::_0)
    }
    #[doc = "Pre-trigger B"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC0PRETRGSEL_A::_1)
    }
}
#[doc = "Field `ADC0ALTTRGEN` reader - ADC0 alternate trigger enable"]
pub type ADC0ALTTRGEN_R = crate::BitReader<ADC0ALTTRGEN_A>;
#[doc = "ADC0 alternate trigger enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0ALTTRGEN_A {
    #[doc = "0: PDB trigger selected for ADC0."]
    _0 = 0,
    #[doc = "1: Alternate trigger selected for ADC0."]
    _1 = 1,
}
impl From<ADC0ALTTRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0ALTTRGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0ALTTRGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0ALTTRGEN_A {
        match self.bits {
            false => ADC0ALTTRGEN_A::_0,
            true => ADC0ALTTRGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADC0ALTTRGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADC0ALTTRGEN_A::_1
    }
}
#[doc = "Field `ADC0ALTTRGEN` writer - ADC0 alternate trigger enable"]
pub type ADC0ALTTRGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT7_SPEC, ADC0ALTTRGEN_A, O>;
impl<'a, const O: u8> ADC0ALTTRGEN_W<'a, O> {
    #[doc = "PDB trigger selected for ADC0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC0ALTTRGEN_A::_0)
    }
    #[doc = "Alternate trigger selected for ADC0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC0ALTTRGEN_A::_1)
    }
}
#[doc = "Field `ADC1TRGSEL` reader - ADC1 trigger select"]
pub type ADC1TRGSEL_R = crate::FieldReader<u8, ADC1TRGSEL_A>;
#[doc = "ADC1 trigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC1TRGSEL_A {
    #[doc = "0: PDB external trigger pin input (PDB0_EXTRG)"]
    _0000 = 0,
    #[doc = "1: High speed comparator 0 output"]
    _0001 = 1,
    #[doc = "2: High speed comparator 1 output"]
    _0010 = 2,
    #[doc = "3: High speed comparator 2 output"]
    _0011 = 3,
    #[doc = "4: PIT trigger 0"]
    _0100 = 4,
    #[doc = "5: PIT trigger 1"]
    _0101 = 5,
    #[doc = "6: PIT trigger 2"]
    _0110 = 6,
    #[doc = "7: PIT trigger 3"]
    _0111 = 7,
    #[doc = "8: FTM0 trigger"]
    _1000 = 8,
    #[doc = "9: FTM1 trigger"]
    _1001 = 9,
    #[doc = "10: FTM2 trigger"]
    _1010 = 10,
    #[doc = "11: FTM3 trigger"]
    _1011 = 11,
    #[doc = "12: RTC alarm"]
    _1100 = 12,
    #[doc = "13: RTC seconds"]
    _1101 = 13,
    #[doc = "14: Low-power timer (LPTMR) trigger"]
    _1110 = 14,
}
impl From<ADC1TRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC1TRGSEL_A) -> Self {
        variant as _
    }
}
impl ADC1TRGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC1TRGSEL_A> {
        match self.bits {
            0 => Some(ADC1TRGSEL_A::_0000),
            1 => Some(ADC1TRGSEL_A::_0001),
            2 => Some(ADC1TRGSEL_A::_0010),
            3 => Some(ADC1TRGSEL_A::_0011),
            4 => Some(ADC1TRGSEL_A::_0100),
            5 => Some(ADC1TRGSEL_A::_0101),
            6 => Some(ADC1TRGSEL_A::_0110),
            7 => Some(ADC1TRGSEL_A::_0111),
            8 => Some(ADC1TRGSEL_A::_1000),
            9 => Some(ADC1TRGSEL_A::_1001),
            10 => Some(ADC1TRGSEL_A::_1010),
            11 => Some(ADC1TRGSEL_A::_1011),
            12 => Some(ADC1TRGSEL_A::_1100),
            13 => Some(ADC1TRGSEL_A::_1101),
            14 => Some(ADC1TRGSEL_A::_1110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == ADC1TRGSEL_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == ADC1TRGSEL_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == ADC1TRGSEL_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == ADC1TRGSEL_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == ADC1TRGSEL_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == ADC1TRGSEL_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == ADC1TRGSEL_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == ADC1TRGSEL_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == ADC1TRGSEL_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == ADC1TRGSEL_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == ADC1TRGSEL_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == ADC1TRGSEL_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == ADC1TRGSEL_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == ADC1TRGSEL_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == ADC1TRGSEL_A::_1110
    }
}
#[doc = "Field `ADC1TRGSEL` writer - ADC1 trigger select"]
pub type ADC1TRGSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOPT7_SPEC, u8, ADC1TRGSEL_A, 4, O>;
impl<'a, const O: u8> ADC1TRGSEL_W<'a, O> {
    #[doc = "PDB external trigger pin input (PDB0_EXTRG)"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_0000)
    }
    #[doc = "High speed comparator 0 output"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_0001)
    }
    #[doc = "High speed comparator 1 output"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_0010)
    }
    #[doc = "High speed comparator 2 output"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_0011)
    }
    #[doc = "PIT trigger 0"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_0100)
    }
    #[doc = "PIT trigger 1"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_0101)
    }
    #[doc = "PIT trigger 2"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_0110)
    }
    #[doc = "PIT trigger 3"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_0111)
    }
    #[doc = "FTM0 trigger"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_1000)
    }
    #[doc = "FTM1 trigger"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_1001)
    }
    #[doc = "FTM2 trigger"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_1010)
    }
    #[doc = "FTM3 trigger"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_1011)
    }
    #[doc = "RTC alarm"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_1100)
    }
    #[doc = "RTC seconds"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_1101)
    }
    #[doc = "Low-power timer (LPTMR) trigger"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(ADC1TRGSEL_A::_1110)
    }
}
#[doc = "Field `ADC1PRETRGSEL` reader - ADC1 pre-trigger select"]
pub type ADC1PRETRGSEL_R = crate::BitReader<ADC1PRETRGSEL_A>;
#[doc = "ADC1 pre-trigger select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC1PRETRGSEL_A {
    #[doc = "0: Pre-trigger A selected for ADC1."]
    _0 = 0,
    #[doc = "1: Pre-trigger B selected for ADC1."]
    _1 = 1,
}
impl From<ADC1PRETRGSEL_A> for bool {
    #[inline(always)]
    fn from(variant: ADC1PRETRGSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC1PRETRGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC1PRETRGSEL_A {
        match self.bits {
            false => ADC1PRETRGSEL_A::_0,
            true => ADC1PRETRGSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADC1PRETRGSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADC1PRETRGSEL_A::_1
    }
}
#[doc = "Field `ADC1PRETRGSEL` writer - ADC1 pre-trigger select"]
pub type ADC1PRETRGSEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SOPT7_SPEC, ADC1PRETRGSEL_A, O>;
impl<'a, const O: u8> ADC1PRETRGSEL_W<'a, O> {
    #[doc = "Pre-trigger A selected for ADC1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC1PRETRGSEL_A::_0)
    }
    #[doc = "Pre-trigger B selected for ADC1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC1PRETRGSEL_A::_1)
    }
}
#[doc = "Field `ADC1ALTTRGEN` reader - ADC1 alternate trigger enable"]
pub type ADC1ALTTRGEN_R = crate::BitReader<ADC1ALTTRGEN_A>;
#[doc = "ADC1 alternate trigger enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC1ALTTRGEN_A {
    #[doc = "0: PDB trigger selected for ADC1"]
    _0 = 0,
    #[doc = "1: Alternate trigger selected for ADC1 as defined by ADC1TRGSEL."]
    _1 = 1,
}
impl From<ADC1ALTTRGEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADC1ALTTRGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC1ALTTRGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC1ALTTRGEN_A {
        match self.bits {
            false => ADC1ALTTRGEN_A::_0,
            true => ADC1ALTTRGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADC1ALTTRGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADC1ALTTRGEN_A::_1
    }
}
#[doc = "Field `ADC1ALTTRGEN` writer - ADC1 alternate trigger enable"]
pub type ADC1ALTTRGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT7_SPEC, ADC1ALTTRGEN_A, O>;
impl<'a, const O: u8> ADC1ALTTRGEN_W<'a, O> {
    #[doc = "PDB trigger selected for ADC1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC1ALTTRGEN_A::_0)
    }
    #[doc = "Alternate trigger selected for ADC1 as defined by ADC1TRGSEL."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC1ALTTRGEN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - ADC0 trigger select"]
    #[inline(always)]
    pub fn adc0trgsel(&self) -> ADC0TRGSEL_R {
        ADC0TRGSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - ADC0 pretrigger select"]
    #[inline(always)]
    pub fn adc0pretrgsel(&self) -> ADC0PRETRGSEL_R {
        ADC0PRETRGSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC0 alternate trigger enable"]
    #[inline(always)]
    pub fn adc0alttrgen(&self) -> ADC0ALTTRGEN_R {
        ADC0ALTTRGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - ADC1 trigger select"]
    #[inline(always)]
    pub fn adc1trgsel(&self) -> ADC1TRGSEL_R {
        ADC1TRGSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - ADC1 pre-trigger select"]
    #[inline(always)]
    pub fn adc1pretrgsel(&self) -> ADC1PRETRGSEL_R {
        ADC1PRETRGSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - ADC1 alternate trigger enable"]
    #[inline(always)]
    pub fn adc1alttrgen(&self) -> ADC1ALTTRGEN_R {
        ADC1ALTTRGEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - ADC0 trigger select"]
    #[inline(always)]
    #[must_use]
    pub fn adc0trgsel(&mut self) -> ADC0TRGSEL_W<0> {
        ADC0TRGSEL_W::new(self)
    }
    #[doc = "Bit 4 - ADC0 pretrigger select"]
    #[inline(always)]
    #[must_use]
    pub fn adc0pretrgsel(&mut self) -> ADC0PRETRGSEL_W<4> {
        ADC0PRETRGSEL_W::new(self)
    }
    #[doc = "Bit 7 - ADC0 alternate trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc0alttrgen(&mut self) -> ADC0ALTTRGEN_W<7> {
        ADC0ALTTRGEN_W::new(self)
    }
    #[doc = "Bits 8:11 - ADC1 trigger select"]
    #[inline(always)]
    #[must_use]
    pub fn adc1trgsel(&mut self) -> ADC1TRGSEL_W<8> {
        ADC1TRGSEL_W::new(self)
    }
    #[doc = "Bit 12 - ADC1 pre-trigger select"]
    #[inline(always)]
    #[must_use]
    pub fn adc1pretrgsel(&mut self) -> ADC1PRETRGSEL_W<12> {
        ADC1PRETRGSEL_W::new(self)
    }
    #[doc = "Bit 15 - ADC1 alternate trigger enable"]
    #[inline(always)]
    #[must_use]
    pub fn adc1alttrgen(&mut self) -> ADC1ALTTRGEN_W<15> {
        ADC1ALTTRGEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Options Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopt7](index.html) module"]
pub struct SOPT7_SPEC;
impl crate::RegisterSpec for SOPT7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sopt7::R](R) reader structure"]
impl crate::Readable for SOPT7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sopt7::W](W) writer structure"]
impl crate::Writable for SOPT7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOPT7 to value 0"]
impl crate::Resettable for SOPT7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
