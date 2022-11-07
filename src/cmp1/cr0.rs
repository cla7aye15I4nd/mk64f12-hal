#[doc = "Register `CR0` reader"]
pub struct R(crate::R<CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR0` writer"]
pub struct W(crate::W<CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR0_SPEC>;
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
impl From<crate::W<CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HYSTCTR` reader - Comparator hard block hysteresis control"]
pub type HYSTCTR_R = crate::FieldReader<u8, HYSTCTR_A>;
#[doc = "Comparator hard block hysteresis control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HYSTCTR_A {
    #[doc = "0: Level 0"]
    _00 = 0,
    #[doc = "1: Level 1"]
    _01 = 1,
    #[doc = "2: Level 2"]
    _10 = 2,
    #[doc = "3: Level 3"]
    _11 = 3,
}
impl From<HYSTCTR_A> for u8 {
    #[inline(always)]
    fn from(variant: HYSTCTR_A) -> Self {
        variant as _
    }
}
impl HYSTCTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYSTCTR_A {
        match self.bits {
            0 => HYSTCTR_A::_00,
            1 => HYSTCTR_A::_01,
            2 => HYSTCTR_A::_10,
            3 => HYSTCTR_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == HYSTCTR_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == HYSTCTR_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == HYSTCTR_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == HYSTCTR_A::_11
    }
}
#[doc = "Field `HYSTCTR` writer - Comparator hard block hysteresis control"]
pub type HYSTCTR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CR0_SPEC, u8, HYSTCTR_A, 2, O>;
impl<'a, const O: u8> HYSTCTR_W<'a, O> {
    #[doc = "Level 0"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(HYSTCTR_A::_00)
    }
    #[doc = "Level 1"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(HYSTCTR_A::_01)
    }
    #[doc = "Level 2"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(HYSTCTR_A::_10)
    }
    #[doc = "Level 3"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(HYSTCTR_A::_11)
    }
}
#[doc = "Field `FILTER_CNT` reader - Filter Sample Count"]
pub type FILTER_CNT_R = crate::FieldReader<u8, FILTER_CNT_A>;
#[doc = "Filter Sample Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FILTER_CNT_A {
    #[doc = "0: Filter is disabled. If SE = 1, then COUT is a logic 0. This is not a legal state, and is not recommended. If SE = 0, COUT = COUTA."]
    _000 = 0,
    #[doc = "1: One sample must agree. The comparator output is simply sampled."]
    _001 = 1,
    #[doc = "2: 2 consecutive samples must agree."]
    _010 = 2,
    #[doc = "3: 3 consecutive samples must agree."]
    _011 = 3,
    #[doc = "4: 4 consecutive samples must agree."]
    _100 = 4,
    #[doc = "5: 5 consecutive samples must agree."]
    _101 = 5,
    #[doc = "6: 6 consecutive samples must agree."]
    _110 = 6,
    #[doc = "7: 7 consecutive samples must agree."]
    _111 = 7,
}
impl From<FILTER_CNT_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTER_CNT_A) -> Self {
        variant as _
    }
}
impl FILTER_CNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTER_CNT_A {
        match self.bits {
            0 => FILTER_CNT_A::_000,
            1 => FILTER_CNT_A::_001,
            2 => FILTER_CNT_A::_010,
            3 => FILTER_CNT_A::_011,
            4 => FILTER_CNT_A::_100,
            5 => FILTER_CNT_A::_101,
            6 => FILTER_CNT_A::_110,
            7 => FILTER_CNT_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FILTER_CNT_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FILTER_CNT_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == FILTER_CNT_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == FILTER_CNT_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == FILTER_CNT_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == FILTER_CNT_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == FILTER_CNT_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == FILTER_CNT_A::_111
    }
}
#[doc = "Field `FILTER_CNT` writer - Filter Sample Count"]
pub type FILTER_CNT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CR0_SPEC, u8, FILTER_CNT_A, 3, O>;
impl<'a, const O: u8> FILTER_CNT_W<'a, O> {
    #[doc = "Filter is disabled. If SE = 1, then COUT is a logic 0. This is not a legal state, and is not recommended. If SE = 0, COUT = COUTA."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::_000)
    }
    #[doc = "One sample must agree. The comparator output is simply sampled."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::_001)
    }
    #[doc = "2 consecutive samples must agree."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::_010)
    }
    #[doc = "3 consecutive samples must agree."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::_011)
    }
    #[doc = "4 consecutive samples must agree."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::_100)
    }
    #[doc = "5 consecutive samples must agree."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::_101)
    }
    #[doc = "6 consecutive samples must agree."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::_110)
    }
    #[doc = "7 consecutive samples must agree."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::_111)
    }
}
impl R {
    #[doc = "Bits 0:1 - Comparator hard block hysteresis control"]
    #[inline(always)]
    pub fn hystctr(&self) -> HYSTCTR_R {
        HYSTCTR_R::new(self.bits & 3)
    }
    #[doc = "Bits 4:6 - Filter Sample Count"]
    #[inline(always)]
    pub fn filter_cnt(&self) -> FILTER_CNT_R {
        FILTER_CNT_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparator hard block hysteresis control"]
    #[inline(always)]
    #[must_use]
    pub fn hystctr(&mut self) -> HYSTCTR_W<0> {
        HYSTCTR_W::new(self)
    }
    #[doc = "Bits 4:6 - Filter Sample Count"]
    #[inline(always)]
    #[must_use]
    pub fn filter_cnt(&mut self) -> FILTER_CNT_W<4> {
        FILTER_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMP Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](index.html) module"]
pub struct CR0_SPEC;
impl crate::RegisterSpec for CR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cr0::R](R) reader structure"]
impl crate::Readable for CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr0::W](W) writer structure"]
impl crate::Writable for CR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
