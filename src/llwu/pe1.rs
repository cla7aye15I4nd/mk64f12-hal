#[doc = "Register `PE1` reader"]
pub struct R(crate::R<PE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PE1` writer"]
pub struct W(crate::W<PE1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PE1_SPEC>;
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
impl From<crate::W<PE1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PE1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUPE0` reader - Wakeup Pin Enable For LLWU_P0"]
pub type WUPE0_R = crate::FieldReader<u8, WUPE0_A>;
#[doc = "Wakeup Pin Enable For LLWU_P0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUPE0_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE0_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE0_A) -> Self {
        variant as _
    }
}
impl WUPE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE0_A {
        match self.bits {
            0 => WUPE0_A::_00,
            1 => WUPE0_A::_01,
            2 => WUPE0_A::_10,
            3 => WUPE0_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE0_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE0_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE0_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE0_A::_11
    }
}
#[doc = "Field `WUPE0` writer - Wakeup Pin Enable For LLWU_P0"]
pub type WUPE0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, PE1_SPEC, u8, WUPE0_A, 2, O>;
impl<'a, const O: u8> WUPE0_W<'a, O> {
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE0_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE0_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE0_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE0_A::_11)
    }
}
#[doc = "Field `WUPE1` reader - Wakeup Pin Enable For LLWU_P1"]
pub type WUPE1_R = crate::FieldReader<u8, WUPE1_A>;
#[doc = "Wakeup Pin Enable For LLWU_P1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUPE1_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE1_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE1_A) -> Self {
        variant as _
    }
}
impl WUPE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE1_A {
        match self.bits {
            0 => WUPE1_A::_00,
            1 => WUPE1_A::_01,
            2 => WUPE1_A::_10,
            3 => WUPE1_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE1_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE1_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE1_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE1_A::_11
    }
}
#[doc = "Field `WUPE1` writer - Wakeup Pin Enable For LLWU_P1"]
pub type WUPE1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, PE1_SPEC, u8, WUPE1_A, 2, O>;
impl<'a, const O: u8> WUPE1_W<'a, O> {
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE1_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE1_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE1_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE1_A::_11)
    }
}
#[doc = "Field `WUPE2` reader - Wakeup Pin Enable For LLWU_P2"]
pub type WUPE2_R = crate::FieldReader<u8, WUPE2_A>;
#[doc = "Wakeup Pin Enable For LLWU_P2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUPE2_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE2_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE2_A) -> Self {
        variant as _
    }
}
impl WUPE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE2_A {
        match self.bits {
            0 => WUPE2_A::_00,
            1 => WUPE2_A::_01,
            2 => WUPE2_A::_10,
            3 => WUPE2_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE2_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE2_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE2_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE2_A::_11
    }
}
#[doc = "Field `WUPE2` writer - Wakeup Pin Enable For LLWU_P2"]
pub type WUPE2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, PE1_SPEC, u8, WUPE2_A, 2, O>;
impl<'a, const O: u8> WUPE2_W<'a, O> {
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE2_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE2_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE2_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE2_A::_11)
    }
}
#[doc = "Field `WUPE3` reader - Wakeup Pin Enable For LLWU_P3"]
pub type WUPE3_R = crate::FieldReader<u8, WUPE3_A>;
#[doc = "Wakeup Pin Enable For LLWU_P3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUPE3_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE3_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE3_A) -> Self {
        variant as _
    }
}
impl WUPE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE3_A {
        match self.bits {
            0 => WUPE3_A::_00,
            1 => WUPE3_A::_01,
            2 => WUPE3_A::_10,
            3 => WUPE3_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE3_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE3_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE3_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE3_A::_11
    }
}
#[doc = "Field `WUPE3` writer - Wakeup Pin Enable For LLWU_P3"]
pub type WUPE3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, PE1_SPEC, u8, WUPE3_A, 2, O>;
impl<'a, const O: u8> WUPE3_W<'a, O> {
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE3_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE3_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE3_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE3_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P0"]
    #[inline(always)]
    pub fn wupe0(&self) -> WUPE0_R {
        WUPE0_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P1"]
    #[inline(always)]
    pub fn wupe1(&self) -> WUPE1_R {
        WUPE1_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P2"]
    #[inline(always)]
    pub fn wupe2(&self) -> WUPE2_R {
        WUPE2_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P3"]
    #[inline(always)]
    pub fn wupe3(&self) -> WUPE3_R {
        WUPE3_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P0"]
    #[inline(always)]
    #[must_use]
    pub fn wupe0(&mut self) -> WUPE0_W<0> {
        WUPE0_W::new(self)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P1"]
    #[inline(always)]
    #[must_use]
    pub fn wupe1(&mut self) -> WUPE1_W<2> {
        WUPE1_W::new(self)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P2"]
    #[inline(always)]
    #[must_use]
    pub fn wupe2(&mut self) -> WUPE2_W<4> {
        WUPE2_W::new(self)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P3"]
    #[inline(always)]
    #[must_use]
    pub fn wupe3(&mut self) -> WUPE3_W<6> {
        WUPE3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LLWU Pin Enable 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe1](index.html) module"]
pub struct PE1_SPEC;
impl crate::RegisterSpec for PE1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pe1::R](R) reader structure"]
impl crate::Readable for PE1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pe1::W](W) writer structure"]
impl crate::Writable for PE1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PE1 to value 0"]
impl crate::Resettable for PE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
