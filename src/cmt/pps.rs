#[doc = "Register `PPS` reader"]
pub struct R(crate::R<PPS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPS` writer"]
pub struct W(crate::W<PPS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPS_SPEC>;
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
impl From<crate::W<PPS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PPSDIV` reader - Primary Prescaler Divider"]
pub type PPSDIV_R = crate::FieldReader<u8, PPSDIV_A>;
#[doc = "Primary Prescaler Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPSDIV_A {
    #[doc = "0: Bus clock * 1"]
    _0000 = 0,
    #[doc = "1: Bus clock * 2"]
    _0001 = 1,
    #[doc = "2: Bus clock * 3"]
    _0010 = 2,
    #[doc = "3: Bus clock * 4"]
    _0011 = 3,
    #[doc = "4: Bus clock * 5"]
    _0100 = 4,
    #[doc = "5: Bus clock * 6"]
    _0101 = 5,
    #[doc = "6: Bus clock * 7"]
    _0110 = 6,
    #[doc = "7: Bus clock * 8"]
    _0111 = 7,
    #[doc = "8: Bus clock * 9"]
    _1000 = 8,
    #[doc = "9: Bus clock * 10"]
    _1001 = 9,
    #[doc = "10: Bus clock * 11"]
    _1010 = 10,
    #[doc = "11: Bus clock * 12"]
    _1011 = 11,
    #[doc = "12: Bus clock * 13"]
    _1100 = 12,
    #[doc = "13: Bus clock * 14"]
    _1101 = 13,
    #[doc = "14: Bus clock * 15"]
    _1110 = 14,
    #[doc = "15: Bus clock * 16"]
    _1111 = 15,
}
impl From<PPSDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: PPSDIV_A) -> Self {
        variant as _
    }
}
impl PPSDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PPSDIV_A {
        match self.bits {
            0 => PPSDIV_A::_0000,
            1 => PPSDIV_A::_0001,
            2 => PPSDIV_A::_0010,
            3 => PPSDIV_A::_0011,
            4 => PPSDIV_A::_0100,
            5 => PPSDIV_A::_0101,
            6 => PPSDIV_A::_0110,
            7 => PPSDIV_A::_0111,
            8 => PPSDIV_A::_1000,
            9 => PPSDIV_A::_1001,
            10 => PPSDIV_A::_1010,
            11 => PPSDIV_A::_1011,
            12 => PPSDIV_A::_1100,
            13 => PPSDIV_A::_1101,
            14 => PPSDIV_A::_1110,
            15 => PPSDIV_A::_1111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == PPSDIV_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == PPSDIV_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == PPSDIV_A::_0010
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == PPSDIV_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == PPSDIV_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == PPSDIV_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == PPSDIV_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == PPSDIV_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == PPSDIV_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == PPSDIV_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == PPSDIV_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == PPSDIV_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == PPSDIV_A::_1100
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == PPSDIV_A::_1101
    }
    #[doc = "Checks if the value of the field is `_1110`"]
    #[inline(always)]
    pub fn is_1110(&self) -> bool {
        *self == PPSDIV_A::_1110
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == PPSDIV_A::_1111
    }
}
#[doc = "Field `PPSDIV` writer - Primary Prescaler Divider"]
pub type PPSDIV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, PPS_SPEC, u8, PPSDIV_A, 4, O>;
impl<'a, const O: u8> PPSDIV_W<'a, O> {
    #[doc = "Bus clock * 1"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(PPSDIV_A::_0000)
    }
    #[doc = "Bus clock * 2"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(PPSDIV_A::_0001)
    }
    #[doc = "Bus clock * 3"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(PPSDIV_A::_0010)
    }
    #[doc = "Bus clock * 4"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut W {
        self.variant(PPSDIV_A::_0011)
    }
    #[doc = "Bus clock * 5"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut W {
        self.variant(PPSDIV_A::_0100)
    }
    #[doc = "Bus clock * 6"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut W {
        self.variant(PPSDIV_A::_0101)
    }
    #[doc = "Bus clock * 7"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut W {
        self.variant(PPSDIV_A::_0110)
    }
    #[doc = "Bus clock * 8"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut W {
        self.variant(PPSDIV_A::_0111)
    }
    #[doc = "Bus clock * 9"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(PPSDIV_A::_1000)
    }
    #[doc = "Bus clock * 10"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(PPSDIV_A::_1001)
    }
    #[doc = "Bus clock * 11"]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(PPSDIV_A::_1010)
    }
    #[doc = "Bus clock * 12"]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(PPSDIV_A::_1011)
    }
    #[doc = "Bus clock * 13"]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(PPSDIV_A::_1100)
    }
    #[doc = "Bus clock * 14"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(PPSDIV_A::_1101)
    }
    #[doc = "Bus clock * 15"]
    #[inline(always)]
    pub fn _1110(self) -> &'a mut W {
        self.variant(PPSDIV_A::_1110)
    }
    #[doc = "Bus clock * 16"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(PPSDIV_A::_1111)
    }
}
impl R {
    #[doc = "Bits 0:3 - Primary Prescaler Divider"]
    #[inline(always)]
    pub fn ppsdiv(&self) -> PPSDIV_R {
        PPSDIV_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Primary Prescaler Divider"]
    #[inline(always)]
    #[must_use]
    pub fn ppsdiv(&mut self) -> PPSDIV_W<0> {
        PPSDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMT Primary Prescaler Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pps](index.html) module"]
pub struct PPS_SPEC;
impl crate::RegisterSpec for PPS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pps::R](R) reader structure"]
impl crate::Readable for PPS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pps::W](W) writer structure"]
impl crate::Writable for PPS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PPS to value 0"]
impl crate::Resettable for PPS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
