#[doc = "Register `PE4` reader"]
pub struct R(crate::R<PE4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PE4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PE4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PE4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PE4` writer"]
pub struct W(crate::W<PE4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PE4_SPEC>;
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
impl From<crate::W<PE4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PE4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUPE12` reader - Wakeup Pin Enable For LLWU_P12"]
pub type WUPE12_R = crate::FieldReader<u8, WUPE12_A>;
#[doc = "Wakeup Pin Enable For LLWU_P12\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUPE12_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE12_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE12_A) -> Self {
        variant as _
    }
}
impl WUPE12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE12_A {
        match self.bits {
            0 => WUPE12_A::_00,
            1 => WUPE12_A::_01,
            2 => WUPE12_A::_10,
            3 => WUPE12_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE12_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE12_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE12_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE12_A::_11
    }
}
#[doc = "Field `WUPE12` writer - Wakeup Pin Enable For LLWU_P12"]
pub type WUPE12_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, PE4_SPEC, u8, WUPE12_A, 2, O>;
impl<'a, const O: u8> WUPE12_W<'a, O> {
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE12_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE12_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE12_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE12_A::_11)
    }
}
#[doc = "Field `WUPE13` reader - Wakeup Pin Enable For LLWU_P13"]
pub type WUPE13_R = crate::FieldReader<u8, WUPE13_A>;
#[doc = "Wakeup Pin Enable For LLWU_P13\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUPE13_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE13_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE13_A) -> Self {
        variant as _
    }
}
impl WUPE13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE13_A {
        match self.bits {
            0 => WUPE13_A::_00,
            1 => WUPE13_A::_01,
            2 => WUPE13_A::_10,
            3 => WUPE13_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE13_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE13_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE13_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE13_A::_11
    }
}
#[doc = "Field `WUPE13` writer - Wakeup Pin Enable For LLWU_P13"]
pub type WUPE13_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, PE4_SPEC, u8, WUPE13_A, 2, O>;
impl<'a, const O: u8> WUPE13_W<'a, O> {
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE13_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE13_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE13_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE13_A::_11)
    }
}
#[doc = "Field `WUPE14` reader - Wakeup Pin Enable For LLWU_P14"]
pub type WUPE14_R = crate::FieldReader<u8, WUPE14_A>;
#[doc = "Wakeup Pin Enable For LLWU_P14\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUPE14_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE14_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE14_A) -> Self {
        variant as _
    }
}
impl WUPE14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE14_A {
        match self.bits {
            0 => WUPE14_A::_00,
            1 => WUPE14_A::_01,
            2 => WUPE14_A::_10,
            3 => WUPE14_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE14_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE14_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE14_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE14_A::_11
    }
}
#[doc = "Field `WUPE14` writer - Wakeup Pin Enable For LLWU_P14"]
pub type WUPE14_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, PE4_SPEC, u8, WUPE14_A, 2, O>;
impl<'a, const O: u8> WUPE14_W<'a, O> {
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE14_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE14_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE14_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE14_A::_11)
    }
}
#[doc = "Field `WUPE15` reader - Wakeup Pin Enable For LLWU_P15"]
pub type WUPE15_R = crate::FieldReader<u8, WUPE15_A>;
#[doc = "Wakeup Pin Enable For LLWU_P15\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUPE15_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE15_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE15_A) -> Self {
        variant as _
    }
}
impl WUPE15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE15_A {
        match self.bits {
            0 => WUPE15_A::_00,
            1 => WUPE15_A::_01,
            2 => WUPE15_A::_10,
            3 => WUPE15_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE15_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE15_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE15_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE15_A::_11
    }
}
#[doc = "Field `WUPE15` writer - Wakeup Pin Enable For LLWU_P15"]
pub type WUPE15_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, PE4_SPEC, u8, WUPE15_A, 2, O>;
impl<'a, const O: u8> WUPE15_W<'a, O> {
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE15_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE15_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE15_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE15_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P12"]
    #[inline(always)]
    pub fn wupe12(&self) -> WUPE12_R {
        WUPE12_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P13"]
    #[inline(always)]
    pub fn wupe13(&self) -> WUPE13_R {
        WUPE13_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P14"]
    #[inline(always)]
    pub fn wupe14(&self) -> WUPE14_R {
        WUPE14_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P15"]
    #[inline(always)]
    pub fn wupe15(&self) -> WUPE15_R {
        WUPE15_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P12"]
    #[inline(always)]
    #[must_use]
    pub fn wupe12(&mut self) -> WUPE12_W<0> {
        WUPE12_W::new(self)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P13"]
    #[inline(always)]
    #[must_use]
    pub fn wupe13(&mut self) -> WUPE13_W<2> {
        WUPE13_W::new(self)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P14"]
    #[inline(always)]
    #[must_use]
    pub fn wupe14(&mut self) -> WUPE14_W<4> {
        WUPE14_W::new(self)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P15"]
    #[inline(always)]
    #[must_use]
    pub fn wupe15(&mut self) -> WUPE15_W<6> {
        WUPE15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LLWU Pin Enable 4 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe4](index.html) module"]
pub struct PE4_SPEC;
impl crate::RegisterSpec for PE4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pe4::R](R) reader structure"]
impl crate::Readable for PE4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pe4::W](W) writer structure"]
impl crate::Writable for PE4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PE4 to value 0"]
impl crate::Resettable for PE4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
