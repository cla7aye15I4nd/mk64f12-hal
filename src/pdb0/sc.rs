#[doc = "Register `SC` reader"]
pub struct R(crate::R<SC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC` writer"]
pub struct W(crate::W<SC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_SPEC>;
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
impl From<crate::W<SC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LDOK` reader - Load OK"]
pub type LDOK_R = crate::BitReader<bool>;
#[doc = "Field `LDOK` writer - Load OK"]
pub type LDOK_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC_SPEC, bool, O>;
#[doc = "Field `CONT` reader - Continuous Mode Enable"]
pub type CONT_R = crate::BitReader<CONT_A>;
#[doc = "Continuous Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT_A {
    #[doc = "0: PDB operation in One-Shot mode"]
    _0 = 0,
    #[doc = "1: PDB operation in Continuous mode"]
    _1 = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
impl CONT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::_0,
            true => CONT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CONT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CONT_A::_1
    }
}
#[doc = "Field `CONT` writer - Continuous Mode Enable"]
pub type CONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC_SPEC, CONT_A, O>;
impl<'a, const O: u8> CONT_W<'a, O> {
    #[doc = "PDB operation in One-Shot mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CONT_A::_0)
    }
    #[doc = "PDB operation in Continuous mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CONT_A::_1)
    }
}
#[doc = "Field `MULT` reader - Multiplication Factor Select for Prescaler"]
pub type MULT_R = crate::FieldReader<u8, MULT_A>;
#[doc = "Multiplication Factor Select for Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MULT_A {
    #[doc = "0: Multiplication factor is 1."]
    _00 = 0,
    #[doc = "1: Multiplication factor is 10."]
    _01 = 1,
    #[doc = "2: Multiplication factor is 20."]
    _10 = 2,
    #[doc = "3: Multiplication factor is 40."]
    _11 = 3,
}
impl From<MULT_A> for u8 {
    #[inline(always)]
    fn from(variant: MULT_A) -> Self {
        variant as _
    }
}
impl MULT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MULT_A {
        match self.bits {
            0 => MULT_A::_00,
            1 => MULT_A::_01,
            2 => MULT_A::_10,
            3 => MULT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MULT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MULT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MULT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MULT_A::_11
    }
}
#[doc = "Field `MULT` writer - Multiplication Factor Select for Prescaler"]
pub type MULT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SC_SPEC, u8, MULT_A, 2, O>;
impl<'a, const O: u8> MULT_W<'a, O> {
    #[doc = "Multiplication factor is 1."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MULT_A::_00)
    }
    #[doc = "Multiplication factor is 10."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MULT_A::_01)
    }
    #[doc = "Multiplication factor is 20."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MULT_A::_10)
    }
    #[doc = "Multiplication factor is 40."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(MULT_A::_11)
    }
}
#[doc = "Field `PDBIE` reader - PDB Interrupt Enable"]
pub type PDBIE_R = crate::BitReader<PDBIE_A>;
#[doc = "PDB Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDBIE_A {
    #[doc = "0: PDB interrupt disabled."]
    _0 = 0,
    #[doc = "1: PDB interrupt enabled."]
    _1 = 1,
}
impl From<PDBIE_A> for bool {
    #[inline(always)]
    fn from(variant: PDBIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PDBIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDBIE_A {
        match self.bits {
            false => PDBIE_A::_0,
            true => PDBIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDBIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDBIE_A::_1
    }
}
#[doc = "Field `PDBIE` writer - PDB Interrupt Enable"]
pub type PDBIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC_SPEC, PDBIE_A, O>;
impl<'a, const O: u8> PDBIE_W<'a, O> {
    #[doc = "PDB interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDBIE_A::_0)
    }
    #[doc = "PDB interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDBIE_A::_1)
    }
}
#[doc = "Field `PDBIF` reader - PDB Interrupt Flag"]
pub type PDBIF_R = crate::BitReader<bool>;
#[doc = "Field `PDBIF` writer - PDB Interrupt Flag"]
pub type PDBIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC_SPEC, bool, O>;
#[doc = "Field `PDBEN` reader - PDB Enable"]
pub type PDBEN_R = crate::BitReader<PDBEN_A>;
#[doc = "PDB Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDBEN_A {
    #[doc = "0: PDB disabled. Counter is off."]
    _0 = 0,
    #[doc = "1: PDB enabled."]
    _1 = 1,
}
impl From<PDBEN_A> for bool {
    #[inline(always)]
    fn from(variant: PDBEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PDBEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDBEN_A {
        match self.bits {
            false => PDBEN_A::_0,
            true => PDBEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDBEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDBEN_A::_1
    }
}
#[doc = "Field `PDBEN` writer - PDB Enable"]
pub type PDBEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC_SPEC, PDBEN_A, O>;
impl<'a, const O: u8> PDBEN_W<'a, O> {
    #[doc = "PDB disabled. Counter is off."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDBEN_A::_0)
    }
    #[doc = "PDB enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDBEN_A::_1)
    }
}
#[doc = "Field `TRGSEL` reader - Trigger Input Source Select"]
pub type TRGSEL_R = crate::FieldReader<u8, TRGSEL_A>;
#[doc = "Trigger Input Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRGSEL_A {
    #[doc = "0: Trigger-In 0 is selected."]
    _0000 = 0,
    #[doc = "1: Trigger-In 1 is selected."]
    _0001 = 1,
    #[doc = "2: Trigger-In 2 is selected."]
    _0010 = 2,
    #[doc = "3: Trigger-In 3 is selected."]
    _0011 = 3,
    #[doc = "4: Trigger-In 4 is selected."]
    _0100 = 4,
    #[doc = "5: Trigger-In 5 is selected."]
    _0101 = 5,
    #[doc = "6: Trigger-In 6 is selected."]
    _0110 = 6,
    #[doc = "7: Trigger-In 7 is selected."]
    _0111 = 7,
    #[doc = "8: Trigger-In 8 is selected."]
    _1000 = 8,
    #[doc = "9: Trigger-In 9 is selected."]
    _1001 = 9,
    #[doc = "10: Trigger-In 10 is selected."]
    _1010 = 10,
    #[doc = "11: Trigger-In 11 is selected."]
    _1011 = 11,
    #[doc = "12: Trigger-In 12 is selected."]
    _1100 = 12,
    #[doc = "13: Trigger-In 13 is selected."]
    _1101 = 13,
    #[doc = "14: Trigger-In 14 is selected."]
    _1110 = 14,
    #[doc = "15: Software trigger is selected."]
    _1111 = 15,
}
impl From<TRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL_A) -> Self {
        variant as _
    }
}
impl TRGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGSEL_A {
        match self.bits {
            0 => TRGSEL_A::_0000,
            1 => TRGSEL_A::_0001,
            2 => TRGSEL_A::_0010,
            3 => TRGSEL_A::_0011,
            4 => TRGSEL_A::_0100,
            5 => TRGSEL_A::_0101,
            6 => TRGSEL_A::_0110,
            7 => TRGSEL_A::_0111,
            8 => TRGSEL_A::_1000,
            9 => TRGSEL_A::_1001,
            10 => TRGSEL_A::_1010,
            11 => TRGSEL_A::_1011,
            12 => TRGSEL_A::_1100,
            13 => TRGSEL_A::_1101,
            14 => TRGSEL_A::_1110,
            15 => TRGSEL_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == TRGSEL_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == TRGSEL_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == TRGSEL_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == TRGSEL_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == TRGSEL_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == TRGSEL_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == TRGSEL_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == TRGSEL_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == TRGSEL_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == TRGSEL_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == TRGSEL_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == TRGSEL_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == TRGSEL_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == TRGSEL_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == TRGSEL_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == TRGSEL_A::_1111
    }
}
#[doc = "Field `TRGSEL` writer - Trigger Input Source Select"]
pub type TRGSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SC_SPEC, u8, TRGSEL_A, 4, O>;
impl<'a, const O: u8> TRGSEL_W<'a, O> {
    #[doc = "Trigger-In 0 is selected."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(TRGSEL_A::_0000)
    }
    #[doc = "Trigger-In 1 is selected."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(TRGSEL_A::_0001)
    }
    #[doc = "Trigger-In 2 is selected."]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(TRGSEL_A::_0010)
    }
    #[doc = "Trigger-In 3 is selected."]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(TRGSEL_A::_0011)
    }
    #[doc = "Trigger-In 4 is selected."]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(TRGSEL_A::_0100)
    }
    #[doc = "Trigger-In 5 is selected."]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(TRGSEL_A::_0101)
    }
    #[doc = "Trigger-In 6 is selected."]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(TRGSEL_A::_0110)
    }
    #[doc = "Trigger-In 7 is selected."]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(TRGSEL_A::_0111)
    }
    #[doc = "Trigger-In 8 is selected."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(TRGSEL_A::_1000)
    }
    #[doc = "Trigger-In 9 is selected."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(TRGSEL_A::_1001)
    }
    #[doc = "Trigger-In 10 is selected."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(TRGSEL_A::_1010)
    }
    #[doc = "Trigger-In 11 is selected."]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(TRGSEL_A::_1011)
    }
    #[doc = "Trigger-In 12 is selected."]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(TRGSEL_A::_1100)
    }
    #[doc = "Trigger-In 13 is selected."]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(TRGSEL_A::_1101)
    }
    #[doc = "Trigger-In 14 is selected."]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(TRGSEL_A::_1110)
    }
    #[doc = "Software trigger is selected."]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(TRGSEL_A::_1111)
    }
}
#[doc = "Field `PRESCALER` reader - Prescaler Divider Select"]
pub type PRESCALER_R = crate::FieldReader<u8, PRESCALER_A>;
#[doc = "Prescaler Divider Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRESCALER_A {
    #[doc = "0: Counting uses the peripheral clock divided by multiplication factor selected by MULT."]
    _000 = 0,
    #[doc = "1: Counting uses the peripheral clock divided by twice of the multiplication factor selected by MULT."]
    _001 = 1,
    #[doc = "2: Counting uses the peripheral clock divided by four times of the multiplication factor selected by MULT."]
    _010 = 2,
    #[doc = "3: Counting uses the peripheral clock divided by eight times of the multiplication factor selected by MULT."]
    _011 = 3,
    #[doc = "4: Counting uses the peripheral clock divided by 16 times of the multiplication factor selected by MULT."]
    _100 = 4,
    #[doc = "5: Counting uses the peripheral clock divided by 32 times of the multiplication factor selected by MULT."]
    _101 = 5,
    #[doc = "6: Counting uses the peripheral clock divided by 64 times of the multiplication factor selected by MULT."]
    _110 = 6,
    #[doc = "7: Counting uses the peripheral clock divided by 128 times of the multiplication factor selected by MULT."]
    _111 = 7,
}
impl From<PRESCALER_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCALER_A) -> Self {
        variant as _
    }
}
impl PRESCALER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRESCALER_A {
        match self.bits {
            0 => PRESCALER_A::_000,
            1 => PRESCALER_A::_001,
            2 => PRESCALER_A::_010,
            3 => PRESCALER_A::_011,
            4 => PRESCALER_A::_100,
            5 => PRESCALER_A::_101,
            6 => PRESCALER_A::_110,
            7 => PRESCALER_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PRESCALER_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PRESCALER_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PRESCALER_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PRESCALER_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PRESCALER_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PRESCALER_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PRESCALER_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == PRESCALER_A::_111
    }
}
#[doc = "Field `PRESCALER` writer - Prescaler Divider Select"]
pub type PRESCALER_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SC_SPEC, u8, PRESCALER_A, 3, O>;
impl<'a, const O: u8> PRESCALER_W<'a, O> {
    #[doc = "Counting uses the peripheral clock divided by multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PRESCALER_A::_000)
    }
    #[doc = "Counting uses the peripheral clock divided by twice of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PRESCALER_A::_001)
    }
    #[doc = "Counting uses the peripheral clock divided by four times of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PRESCALER_A::_010)
    }
    #[doc = "Counting uses the peripheral clock divided by eight times of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PRESCALER_A::_011)
    }
    #[doc = "Counting uses the peripheral clock divided by 16 times of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PRESCALER_A::_100)
    }
    #[doc = "Counting uses the peripheral clock divided by 32 times of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PRESCALER_A::_101)
    }
    #[doc = "Counting uses the peripheral clock divided by 64 times of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PRESCALER_A::_110)
    }
    #[doc = "Counting uses the peripheral clock divided by 128 times of the multiplication factor selected by MULT."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PRESCALER_A::_111)
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: DMA disabled."]
    _0 = 0,
    #[doc = "1: DMA enabled."]
    _1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::_0,
            true => DMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAEN_A::_1
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    #[doc = "DMA disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEN_A::_0)
    }
    #[doc = "DMA enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAEN_A::_1)
    }
}
#[doc = "Field `SWTRIG` writer - Software Trigger"]
pub type SWTRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC_SPEC, bool, O>;
#[doc = "Field `PDBEIE` reader - PDB Sequence Error Interrupt Enable"]
pub type PDBEIE_R = crate::BitReader<PDBEIE_A>;
#[doc = "PDB Sequence Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDBEIE_A {
    #[doc = "0: PDB sequence error interrupt disabled."]
    _0 = 0,
    #[doc = "1: PDB sequence error interrupt enabled."]
    _1 = 1,
}
impl From<PDBEIE_A> for bool {
    #[inline(always)]
    fn from(variant: PDBEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PDBEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDBEIE_A {
        match self.bits {
            false => PDBEIE_A::_0,
            true => PDBEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDBEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDBEIE_A::_1
    }
}
#[doc = "Field `PDBEIE` writer - PDB Sequence Error Interrupt Enable"]
pub type PDBEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC_SPEC, PDBEIE_A, O>;
impl<'a, const O: u8> PDBEIE_W<'a, O> {
    #[doc = "PDB sequence error interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDBEIE_A::_0)
    }
    #[doc = "PDB sequence error interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDBEIE_A::_1)
    }
}
#[doc = "Field `LDMOD` reader - Load Mode Select"]
pub type LDMOD_R = crate::FieldReader<u8, LDMOD_A>;
#[doc = "Load Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LDMOD_A {
    #[doc = "0: The internal registers are loaded with the values from their buffers immediately after 1 is written to LDOK."]
    _00 = 0,
    #[doc = "1: The internal registers are loaded with the values from their buffers when the PDB counter reaches the MOD register value after 1 is written to LDOK."]
    _01 = 1,
    #[doc = "2: The internal registers are loaded with the values from their buffers when a trigger input event is detected after 1 is written to LDOK."]
    _10 = 2,
    #[doc = "3: The internal registers are loaded with the values from their buffers when either the PDB counter reaches the MOD register value or a trigger input event is detected, after 1 is written to LDOK."]
    _11 = 3,
}
impl From<LDMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: LDMOD_A) -> Self {
        variant as _
    }
}
impl LDMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDMOD_A {
        match self.bits {
            0 => LDMOD_A::_00,
            1 => LDMOD_A::_01,
            2 => LDMOD_A::_10,
            3 => LDMOD_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LDMOD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LDMOD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LDMOD_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == LDMOD_A::_11
    }
}
#[doc = "Field `LDMOD` writer - Load Mode Select"]
pub type LDMOD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SC_SPEC, u8, LDMOD_A, 2, O>;
impl<'a, const O: u8> LDMOD_W<'a, O> {
    #[doc = "The internal registers are loaded with the values from their buffers immediately after 1 is written to LDOK."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LDMOD_A::_00)
    }
    #[doc = "The internal registers are loaded with the values from their buffers when the PDB counter reaches the MOD register value after 1 is written to LDOK."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LDMOD_A::_01)
    }
    #[doc = "The internal registers are loaded with the values from their buffers when a trigger input event is detected after 1 is written to LDOK."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LDMOD_A::_10)
    }
    #[doc = "The internal registers are loaded with the values from their buffers when either the PDB counter reaches the MOD register value or a trigger input event is detected, after 1 is written to LDOK."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(LDMOD_A::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Load OK"]
    #[inline(always)]
    pub fn ldok(&self) -> LDOK_R {
        LDOK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Continuous Mode Enable"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Multiplication Factor Select for Prescaler"]
    #[inline(always)]
    pub fn mult(&self) -> MULT_R {
        MULT_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 5 - PDB Interrupt Enable"]
    #[inline(always)]
    pub fn pdbie(&self) -> PDBIE_R {
        PDBIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PDB Interrupt Flag"]
    #[inline(always)]
    pub fn pdbif(&self) -> PDBIF_R {
        PDBIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PDB Enable"]
    #[inline(always)]
    pub fn pdben(&self) -> PDBEN_R {
        PDBEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Trigger Input Source Select"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Prescaler Divider Select"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - PDB Sequence Error Interrupt Enable"]
    #[inline(always)]
    pub fn pdbeie(&self) -> PDBEIE_R {
        PDBEIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Load Mode Select"]
    #[inline(always)]
    pub fn ldmod(&self) -> LDMOD_R {
        LDMOD_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Load OK"]
    #[inline(always)]
    #[must_use]
    pub fn ldok(&mut self) -> LDOK_W<0> {
        LDOK_W::new(self)
    }
    #[doc = "Bit 1 - Continuous Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<1> {
        CONT_W::new(self)
    }
    #[doc = "Bits 2:3 - Multiplication Factor Select for Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn mult(&mut self) -> MULT_W<2> {
        MULT_W::new(self)
    }
    #[doc = "Bit 5 - PDB Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pdbie(&mut self) -> PDBIE_W<5> {
        PDBIE_W::new(self)
    }
    #[doc = "Bit 6 - PDB Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn pdbif(&mut self) -> PDBIF_W<6> {
        PDBIF_W::new(self)
    }
    #[doc = "Bit 7 - PDB Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pdben(&mut self) -> PDBEN_W<7> {
        PDBEN_W::new(self)
    }
    #[doc = "Bits 8:11 - Trigger Input Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn trgsel(&mut self) -> TRGSEL_W<8> {
        TRGSEL_W::new(self)
    }
    #[doc = "Bits 12:14 - Prescaler Divider Select"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<12> {
        PRESCALER_W::new(self)
    }
    #[doc = "Bit 15 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<15> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 16 - Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swtrig(&mut self) -> SWTRIG_W<16> {
        SWTRIG_W::new(self)
    }
    #[doc = "Bit 17 - PDB Sequence Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pdbeie(&mut self) -> PDBEIE_W<17> {
        PDBEIE_W::new(self)
    }
    #[doc = "Bits 18:19 - Load Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn ldmod(&mut self) -> LDMOD_W<18> {
        LDMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status and Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc](index.html) module"]
pub struct SC_SPEC;
impl crate::RegisterSpec for SC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc::R](R) reader structure"]
impl crate::Readable for SC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc::W](W) writer structure"]
impl crate::Writable for SC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for SC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
