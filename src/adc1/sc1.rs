#[doc = "Register `SC1%s` reader"]
pub struct R(crate::R<SC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC1%s` writer"]
pub struct W(crate::W<SC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC1_SPEC>;
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
impl From<crate::W<SC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCH` reader - Input channel select"]
pub type ADCH_R = crate::FieldReader<u8, ADCH_A>;
#[doc = "Input channel select\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCH_A {
    #[doc = "0: When DIFF=0, DADP0 is selected as input; when DIFF=1, DAD0 is selected as input."]
    _00000 = 0,
    #[doc = "1: When DIFF=0, DADP1 is selected as input; when DIFF=1, DAD1 is selected as input."]
    _00001 = 1,
    #[doc = "2: When DIFF=0, DADP2 is selected as input; when DIFF=1, DAD2 is selected as input."]
    _00010 = 2,
    #[doc = "3: When DIFF=0, DADP3 is selected as input; when DIFF=1, DAD3 is selected as input."]
    _00011 = 3,
    #[doc = "4: When DIFF=0, AD4 is selected as input; when DIFF=1, it is reserved."]
    _00100 = 4,
    #[doc = "5: When DIFF=0, AD5 is selected as input; when DIFF=1, it is reserved."]
    _00101 = 5,
    #[doc = "6: When DIFF=0, AD6 is selected as input; when DIFF=1, it is reserved."]
    _00110 = 6,
    #[doc = "7: When DIFF=0, AD7 is selected as input; when DIFF=1, it is reserved."]
    _00111 = 7,
    #[doc = "8: When DIFF=0, AD8 is selected as input; when DIFF=1, it is reserved."]
    _01000 = 8,
    #[doc = "9: When DIFF=0, AD9 is selected as input; when DIFF=1, it is reserved."]
    _01001 = 9,
    #[doc = "10: When DIFF=0, AD10 is selected as input; when DIFF=1, it is reserved."]
    _01010 = 10,
    #[doc = "11: When DIFF=0, AD11 is selected as input; when DIFF=1, it is reserved."]
    _01011 = 11,
    #[doc = "12: When DIFF=0, AD12 is selected as input; when DIFF=1, it is reserved."]
    _01100 = 12,
    #[doc = "13: When DIFF=0, AD13 is selected as input; when DIFF=1, it is reserved."]
    _01101 = 13,
    #[doc = "14: When DIFF=0, AD14 is selected as input; when DIFF=1, it is reserved."]
    _01110 = 14,
    #[doc = "15: When DIFF=0, AD15 is selected as input; when DIFF=1, it is reserved."]
    _01111 = 15,
    #[doc = "16: When DIFF=0, AD16 is selected as input; when DIFF=1, it is reserved."]
    _10000 = 16,
    #[doc = "17: When DIFF=0, AD17 is selected as input; when DIFF=1, it is reserved."]
    _10001 = 17,
    #[doc = "18: When DIFF=0, AD18 is selected as input; when DIFF=1, it is reserved."]
    _10010 = 18,
    #[doc = "19: When DIFF=0, AD19 is selected as input; when DIFF=1, it is reserved."]
    _10011 = 19,
    #[doc = "20: When DIFF=0, AD20 is selected as input; when DIFF=1, it is reserved."]
    _10100 = 20,
    #[doc = "21: When DIFF=0, AD21 is selected as input; when DIFF=1, it is reserved."]
    _10101 = 21,
    #[doc = "22: When DIFF=0, AD22 is selected as input; when DIFF=1, it is reserved."]
    _10110 = 22,
    #[doc = "23: When DIFF=0, AD23 is selected as input; when DIFF=1, it is reserved."]
    _10111 = 23,
    #[doc = "26: When DIFF=0, Temp Sensor (single-ended) is selected as input; when DIFF=1, Temp Sensor (differential) is selected as input."]
    _11010 = 26,
    #[doc = "27: When DIFF=0, Bandgap (single-ended) is selected as input; when DIFF=1, Bandgap (differential) is selected as input."]
    _11011 = 27,
    #[doc = "29: When DIFF=0,VREFSH is selected as input; when DIFF=1, -VREFSH (differential) is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    _11101 = 29,
    #[doc = "30: When DIFF=0,VREFSL is selected as input; when DIFF=1, it is reserved. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    _11110 = 30,
    #[doc = "31: Module is disabled."]
    _11111 = 31,
}
impl From<ADCH_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCH_A) -> Self {
        variant as _
    }
}
impl ADCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCH_A> {
        match self.bits {
            0 => Some(ADCH_A::_00000),
            1 => Some(ADCH_A::_00001),
            2 => Some(ADCH_A::_00010),
            3 => Some(ADCH_A::_00011),
            4 => Some(ADCH_A::_00100),
            5 => Some(ADCH_A::_00101),
            6 => Some(ADCH_A::_00110),
            7 => Some(ADCH_A::_00111),
            8 => Some(ADCH_A::_01000),
            9 => Some(ADCH_A::_01001),
            10 => Some(ADCH_A::_01010),
            11 => Some(ADCH_A::_01011),
            12 => Some(ADCH_A::_01100),
            13 => Some(ADCH_A::_01101),
            14 => Some(ADCH_A::_01110),
            15 => Some(ADCH_A::_01111),
            16 => Some(ADCH_A::_10000),
            17 => Some(ADCH_A::_10001),
            18 => Some(ADCH_A::_10010),
            19 => Some(ADCH_A::_10011),
            20 => Some(ADCH_A::_10100),
            21 => Some(ADCH_A::_10101),
            22 => Some(ADCH_A::_10110),
            23 => Some(ADCH_A::_10111),
            26 => Some(ADCH_A::_11010),
            27 => Some(ADCH_A::_11011),
            29 => Some(ADCH_A::_11101),
            30 => Some(ADCH_A::_11110),
            31 => Some(ADCH_A::_11111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00000`"]
    #[inline(always)]
    pub fn is_00000(&self) -> bool {
        *self == ADCH_A::_00000
    }
    #[doc = "Checks if the value of the field is `_00001`"]
    #[inline(always)]
    pub fn is_00001(&self) -> bool {
        *self == ADCH_A::_00001
    }
    #[doc = "Checks if the value of the field is `_00010`"]
    #[inline(always)]
    pub fn is_00010(&self) -> bool {
        *self == ADCH_A::_00010
    }
    #[doc = "Checks if the value of the field is `_00011`"]
    #[inline(always)]
    pub fn is_00011(&self) -> bool {
        *self == ADCH_A::_00011
    }
    #[doc = "Checks if the value of the field is `_00100`"]
    #[inline(always)]
    pub fn is_00100(&self) -> bool {
        *self == ADCH_A::_00100
    }
    #[doc = "Checks if the value of the field is `_00101`"]
    #[inline(always)]
    pub fn is_00101(&self) -> bool {
        *self == ADCH_A::_00101
    }
    #[doc = "Checks if the value of the field is `_00110`"]
    #[inline(always)]
    pub fn is_00110(&self) -> bool {
        *self == ADCH_A::_00110
    }
    #[doc = "Checks if the value of the field is `_00111`"]
    #[inline(always)]
    pub fn is_00111(&self) -> bool {
        *self == ADCH_A::_00111
    }
    #[doc = "Checks if the value of the field is `_01000`"]
    #[inline(always)]
    pub fn is_01000(&self) -> bool {
        *self == ADCH_A::_01000
    }
    #[doc = "Checks if the value of the field is `_01001`"]
    #[inline(always)]
    pub fn is_01001(&self) -> bool {
        *self == ADCH_A::_01001
    }
    #[doc = "Checks if the value of the field is `_01010`"]
    #[inline(always)]
    pub fn is_01010(&self) -> bool {
        *self == ADCH_A::_01010
    }
    #[doc = "Checks if the value of the field is `_01011`"]
    #[inline(always)]
    pub fn is_01011(&self) -> bool {
        *self == ADCH_A::_01011
    }
    #[doc = "Checks if the value of the field is `_01100`"]
    #[inline(always)]
    pub fn is_01100(&self) -> bool {
        *self == ADCH_A::_01100
    }
    #[doc = "Checks if the value of the field is `_01101`"]
    #[inline(always)]
    pub fn is_01101(&self) -> bool {
        *self == ADCH_A::_01101
    }
    #[doc = "Checks if the value of the field is `_01110`"]
    #[inline(always)]
    pub fn is_01110(&self) -> bool {
        *self == ADCH_A::_01110
    }
    #[doc = "Checks if the value of the field is `_01111`"]
    #[inline(always)]
    pub fn is_01111(&self) -> bool {
        *self == ADCH_A::_01111
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == ADCH_A::_10000
    }
    #[doc = "Checks if the value of the field is `_10001`"]
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == ADCH_A::_10001
    }
    #[doc = "Checks if the value of the field is `_10010`"]
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == ADCH_A::_10010
    }
    #[doc = "Checks if the value of the field is `_10011`"]
    #[inline(always)]
    pub fn is_10011(&self) -> bool {
        *self == ADCH_A::_10011
    }
    #[doc = "Checks if the value of the field is `_10100`"]
    #[inline(always)]
    pub fn is_10100(&self) -> bool {
        *self == ADCH_A::_10100
    }
    #[doc = "Checks if the value of the field is `_10101`"]
    #[inline(always)]
    pub fn is_10101(&self) -> bool {
        *self == ADCH_A::_10101
    }
    #[doc = "Checks if the value of the field is `_10110`"]
    #[inline(always)]
    pub fn is_10110(&self) -> bool {
        *self == ADCH_A::_10110
    }
    #[doc = "Checks if the value of the field is `_10111`"]
    #[inline(always)]
    pub fn is_10111(&self) -> bool {
        *self == ADCH_A::_10111
    }
    #[doc = "Checks if the value of the field is `_11010`"]
    #[inline(always)]
    pub fn is_11010(&self) -> bool {
        *self == ADCH_A::_11010
    }
    #[doc = "Checks if the value of the field is `_11011`"]
    #[inline(always)]
    pub fn is_11011(&self) -> bool {
        *self == ADCH_A::_11011
    }
    #[doc = "Checks if the value of the field is `_11101`"]
    #[inline(always)]
    pub fn is_11101(&self) -> bool {
        *self == ADCH_A::_11101
    }
    #[doc = "Checks if the value of the field is `_11110`"]
    #[inline(always)]
    pub fn is_11110(&self) -> bool {
        *self == ADCH_A::_11110
    }
    #[doc = "Checks if the value of the field is `_11111`"]
    #[inline(always)]
    pub fn is_11111(&self) -> bool {
        *self == ADCH_A::_11111
    }
}
#[doc = "Field `ADCH` writer - Input channel select"]
pub type ADCH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SC1_SPEC, u8, ADCH_A, 5, O>;
impl<'a, const O: u8> ADCH_W<'a, O> {
    #[doc = "When DIFF=0, DADP0 is selected as input; when DIFF=1, DAD0 is selected as input."]
    #[inline(always)]
    pub fn _00000(self) -> &'a mut W {
        self.variant(ADCH_A::_00000)
    }
    #[doc = "When DIFF=0, DADP1 is selected as input; when DIFF=1, DAD1 is selected as input."]
    #[inline(always)]
    pub fn _00001(self) -> &'a mut W {
        self.variant(ADCH_A::_00001)
    }
    #[doc = "When DIFF=0, DADP2 is selected as input; when DIFF=1, DAD2 is selected as input."]
    #[inline(always)]
    pub fn _00010(self) -> &'a mut W {
        self.variant(ADCH_A::_00010)
    }
    #[doc = "When DIFF=0, DADP3 is selected as input; when DIFF=1, DAD3 is selected as input."]
    #[inline(always)]
    pub fn _00011(self) -> &'a mut W {
        self.variant(ADCH_A::_00011)
    }
    #[doc = "When DIFF=0, AD4 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _00100(self) -> &'a mut W {
        self.variant(ADCH_A::_00100)
    }
    #[doc = "When DIFF=0, AD5 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _00101(self) -> &'a mut W {
        self.variant(ADCH_A::_00101)
    }
    #[doc = "When DIFF=0, AD6 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _00110(self) -> &'a mut W {
        self.variant(ADCH_A::_00110)
    }
    #[doc = "When DIFF=0, AD7 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _00111(self) -> &'a mut W {
        self.variant(ADCH_A::_00111)
    }
    #[doc = "When DIFF=0, AD8 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _01000(self) -> &'a mut W {
        self.variant(ADCH_A::_01000)
    }
    #[doc = "When DIFF=0, AD9 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _01001(self) -> &'a mut W {
        self.variant(ADCH_A::_01001)
    }
    #[doc = "When DIFF=0, AD10 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _01010(self) -> &'a mut W {
        self.variant(ADCH_A::_01010)
    }
    #[doc = "When DIFF=0, AD11 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _01011(self) -> &'a mut W {
        self.variant(ADCH_A::_01011)
    }
    #[doc = "When DIFF=0, AD12 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _01100(self) -> &'a mut W {
        self.variant(ADCH_A::_01100)
    }
    #[doc = "When DIFF=0, AD13 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _01101(self) -> &'a mut W {
        self.variant(ADCH_A::_01101)
    }
    #[doc = "When DIFF=0, AD14 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _01110(self) -> &'a mut W {
        self.variant(ADCH_A::_01110)
    }
    #[doc = "When DIFF=0, AD15 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _01111(self) -> &'a mut W {
        self.variant(ADCH_A::_01111)
    }
    #[doc = "When DIFF=0, AD16 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _10000(self) -> &'a mut W {
        self.variant(ADCH_A::_10000)
    }
    #[doc = "When DIFF=0, AD17 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _10001(self) -> &'a mut W {
        self.variant(ADCH_A::_10001)
    }
    #[doc = "When DIFF=0, AD18 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _10010(self) -> &'a mut W {
        self.variant(ADCH_A::_10010)
    }
    #[doc = "When DIFF=0, AD19 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _10011(self) -> &'a mut W {
        self.variant(ADCH_A::_10011)
    }
    #[doc = "When DIFF=0, AD20 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _10100(self) -> &'a mut W {
        self.variant(ADCH_A::_10100)
    }
    #[doc = "When DIFF=0, AD21 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _10101(self) -> &'a mut W {
        self.variant(ADCH_A::_10101)
    }
    #[doc = "When DIFF=0, AD22 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _10110(self) -> &'a mut W {
        self.variant(ADCH_A::_10110)
    }
    #[doc = "When DIFF=0, AD23 is selected as input; when DIFF=1, it is reserved."]
    #[inline(always)]
    pub fn _10111(self) -> &'a mut W {
        self.variant(ADCH_A::_10111)
    }
    #[doc = "When DIFF=0, Temp Sensor (single-ended) is selected as input; when DIFF=1, Temp Sensor (differential) is selected as input."]
    #[inline(always)]
    pub fn _11010(self) -> &'a mut W {
        self.variant(ADCH_A::_11010)
    }
    #[doc = "When DIFF=0, Bandgap (single-ended) is selected as input; when DIFF=1, Bandgap (differential) is selected as input."]
    #[inline(always)]
    pub fn _11011(self) -> &'a mut W {
        self.variant(ADCH_A::_11011)
    }
    #[doc = "When DIFF=0,VREFSH is selected as input; when DIFF=1, -VREFSH (differential) is selected as input. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    #[inline(always)]
    pub fn _11101(self) -> &'a mut W {
        self.variant(ADCH_A::_11101)
    }
    #[doc = "When DIFF=0,VREFSL is selected as input; when DIFF=1, it is reserved. Voltage reference selected is determined by SC2\\[REFSEL\\]."]
    #[inline(always)]
    pub fn _11110(self) -> &'a mut W {
        self.variant(ADCH_A::_11110)
    }
    #[doc = "Module is disabled."]
    #[inline(always)]
    pub fn _11111(self) -> &'a mut W {
        self.variant(ADCH_A::_11111)
    }
}
#[doc = "Field `DIFF` reader - Differential Mode Enable"]
pub type DIFF_R = crate::BitReader<DIFF_A>;
#[doc = "Differential Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIFF_A {
    #[doc = "0: Single-ended conversions and input channels are selected."]
    _0 = 0,
    #[doc = "1: Differential conversions and input channels are selected."]
    _1 = 1,
}
impl From<DIFF_A> for bool {
    #[inline(always)]
    fn from(variant: DIFF_A) -> Self {
        variant as u8 != 0
    }
}
impl DIFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIFF_A {
        match self.bits {
            false => DIFF_A::_0,
            true => DIFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIFF_A::_1
    }
}
#[doc = "Field `DIFF` writer - Differential Mode Enable"]
pub type DIFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC1_SPEC, DIFF_A, O>;
impl<'a, const O: u8> DIFF_W<'a, O> {
    #[doc = "Single-ended conversions and input channels are selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIFF_A::_0)
    }
    #[doc = "Differential conversions and input channels are selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIFF_A::_1)
    }
}
#[doc = "Field `AIEN` reader - Interrupt Enable"]
pub type AIEN_R = crate::BitReader<AIEN_A>;
#[doc = "Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AIEN_A {
    #[doc = "0: Conversion complete interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Conversion complete interrupt is enabled."]
    _1 = 1,
}
impl From<AIEN_A> for bool {
    #[inline(always)]
    fn from(variant: AIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AIEN_A {
        match self.bits {
            false => AIEN_A::_0,
            true => AIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AIEN_A::_1
    }
}
#[doc = "Field `AIEN` writer - Interrupt Enable"]
pub type AIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC1_SPEC, AIEN_A, O>;
impl<'a, const O: u8> AIEN_W<'a, O> {
    #[doc = "Conversion complete interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AIEN_A::_0)
    }
    #[doc = "Conversion complete interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AIEN_A::_1)
    }
}
#[doc = "Field `COCO` reader - Conversion Complete Flag"]
pub type COCO_R = crate::BitReader<COCO_A>;
#[doc = "Conversion Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COCO_A {
    #[doc = "0: Conversion is not completed."]
    _0 = 0,
    #[doc = "1: Conversion is completed."]
    _1 = 1,
}
impl From<COCO_A> for bool {
    #[inline(always)]
    fn from(variant: COCO_A) -> Self {
        variant as u8 != 0
    }
}
impl COCO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COCO_A {
        match self.bits {
            false => COCO_A::_0,
            true => COCO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COCO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COCO_A::_1
    }
}
impl R {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    pub fn adch(&self) -> ADCH_R {
        ADCH_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Differential Mode Enable"]
    #[inline(always)]
    pub fn diff(&self) -> DIFF_R {
        DIFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Enable"]
    #[inline(always)]
    pub fn aien(&self) -> AIEN_R {
        AIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Conversion Complete Flag"]
    #[inline(always)]
    pub fn coco(&self) -> COCO_R {
        COCO_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input channel select"]
    #[inline(always)]
    #[must_use]
    pub fn adch(&mut self) -> ADCH_W<0> {
        ADCH_W::new(self)
    }
    #[doc = "Bit 5 - Differential Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn diff(&mut self) -> DIFF_W<5> {
        DIFF_W::new(self)
    }
    #[doc = "Bit 6 - Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aien(&mut self) -> AIEN_W<6> {
        AIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Status and Control Registers 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc1](index.html) module"]
pub struct SC1_SPEC;
impl crate::RegisterSpec for SC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc1::R](R) reader structure"]
impl crate::Readable for SC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc1::W](W) writer structure"]
impl crate::Writable for SC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SC1%s to value 0x1f"]
impl crate::Resettable for SC1_SPEC {
    const RESET_VALUE: Self::Ux = 0x1f;
}
