#[doc = "Register `FILT1` reader"]
pub struct R(crate::R<FILT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILT1` writer"]
pub struct W(crate::W<FILT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILT1_SPEC>;
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
impl From<crate::W<FILT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTSEL` reader - Filter Pin Select"]
pub type FILTSEL_R = crate::FieldReader<u8, FILTSEL_A>;
#[doc = "Filter Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FILTSEL_A {
    #[doc = "0: Select LLWU_P0 for filter"]
    _0000 = 0,
    #[doc = "15: Select LLWU_P15 for filter"]
    _1111 = 15,
}
impl From<FILTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTSEL_A) -> Self {
        variant as _
    }
}
impl FILTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FILTSEL_A> {
        match self.bits {
            0 => Some(FILTSEL_A::_0000),
            15 => Some(FILTSEL_A::_1111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == FILTSEL_A::_0000
    }
    #[doc = "Checks if the value of the field is `_1111`"]
    #[inline(always)]
    pub fn is_1111(&self) -> bool {
        *self == FILTSEL_A::_1111
    }
}
#[doc = "Field `FILTSEL` writer - Filter Pin Select"]
pub type FILTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u8, FILT1_SPEC, u8, FILTSEL_A, 4, O>;
impl<'a, const O: u8> FILTSEL_W<'a, O> {
    #[doc = "Select LLWU_P0 for filter"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(FILTSEL_A::_0000)
    }
    #[doc = "Select LLWU_P15 for filter"]
    #[inline(always)]
    pub fn _1111(self) -> &'a mut W {
        self.variant(FILTSEL_A::_1111)
    }
}
#[doc = "Field `FILTE` reader - Digital Filter On External Pin"]
pub type FILTE_R = crate::FieldReader<u8, FILTE_A>;
#[doc = "Digital Filter On External Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FILTE_A {
    #[doc = "0: Filter disabled"]
    _00 = 0,
    #[doc = "1: Filter posedge detect enabled"]
    _01 = 1,
    #[doc = "2: Filter negedge detect enabled"]
    _10 = 2,
    #[doc = "3: Filter any edge detect enabled"]
    _11 = 3,
}
impl From<FILTE_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTE_A) -> Self {
        variant as _
    }
}
impl FILTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTE_A {
        match self.bits {
            0 => FILTE_A::_00,
            1 => FILTE_A::_01,
            2 => FILTE_A::_10,
            3 => FILTE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FILTE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FILTE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FILTE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FILTE_A::_11
    }
}
#[doc = "Field `FILTE` writer - Digital Filter On External Pin"]
pub type FILTE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, FILT1_SPEC, u8, FILTE_A, 2, O>;
impl<'a, const O: u8> FILTE_W<'a, O> {
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FILTE_A::_00)
    }
    #[doc = "Filter posedge detect enabled"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FILTE_A::_01)
    }
    #[doc = "Filter negedge detect enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FILTE_A::_10)
    }
    #[doc = "Filter any edge detect enabled"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FILTE_A::_11)
    }
}
#[doc = "Field `FILTF` reader - Filter Detect Flag"]
pub type FILTF_R = crate::BitReader<FILTF_A>;
#[doc = "Filter Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FILTF_A {
    #[doc = "0: Pin Filter 1 was not a wakeup source"]
    _0 = 0,
    #[doc = "1: Pin Filter 1 was a wakeup source"]
    _1 = 1,
}
impl From<FILTF_A> for bool {
    #[inline(always)]
    fn from(variant: FILTF_A) -> Self {
        variant as u8 != 0
    }
}
impl FILTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTF_A {
        match self.bits {
            false => FILTF_A::_0,
            true => FILTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FILTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FILTF_A::_1
    }
}
#[doc = "Field `FILTF` writer - Filter Detect Flag"]
pub type FILTF_W<'a, const O: u8> = crate::BitWriter<'a, u8, FILT1_SPEC, FILTF_A, O>;
impl<'a, const O: u8> FILTF_W<'a, O> {
    #[doc = "Pin Filter 1 was not a wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FILTF_A::_0)
    }
    #[doc = "Pin Filter 1 was a wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FILTF_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Filter Pin Select"]
    #[inline(always)]
    pub fn filtsel(&self) -> FILTSEL_R {
        FILTSEL_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 5:6 - Digital Filter On External Pin"]
    #[inline(always)]
    pub fn filte(&self) -> FILTE_R {
        FILTE_R::new((self.bits >> 5) & 3)
    }
    #[doc = "Bit 7 - Filter Detect Flag"]
    #[inline(always)]
    pub fn filtf(&self) -> FILTF_R {
        FILTF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Filter Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn filtsel(&mut self) -> FILTSEL_W<0> {
        FILTSEL_W::new(self)
    }
    #[doc = "Bits 5:6 - Digital Filter On External Pin"]
    #[inline(always)]
    #[must_use]
    pub fn filte(&mut self) -> FILTE_W<5> {
        FILTE_W::new(self)
    }
    #[doc = "Bit 7 - Filter Detect Flag"]
    #[inline(always)]
    #[must_use]
    pub fn filtf(&mut self) -> FILTF_W<7> {
        FILTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LLWU Pin Filter 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filt1](index.html) module"]
pub struct FILT1_SPEC;
impl crate::RegisterSpec for FILT1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [filt1::R](R) reader structure"]
impl crate::Readable for FILT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filt1::W](W) writer structure"]
impl crate::Writable for FILT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FILT1 to value 0"]
impl crate::Resettable for FILT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
