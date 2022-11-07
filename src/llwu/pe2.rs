#[doc = "Register `PE2` reader"]
pub struct R(crate::R<PE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PE2` writer"]
pub struct W(crate::W<PE2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PE2_SPEC>;
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
impl From<crate::W<PE2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PE2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUPE4` reader - Wakeup Pin Enable For LLWU_P4"]
pub type WUPE4_R = crate::FieldReader<u8, WUPE4_A>;
#[doc = "Wakeup Pin Enable For LLWU_P4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUPE4_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE4_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE4_A) -> Self {
        variant as _
    }
}
impl WUPE4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE4_A {
        match self.bits {
            0 => WUPE4_A::_00,
            1 => WUPE4_A::_01,
            2 => WUPE4_A::_10,
            3 => WUPE4_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE4_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE4_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE4_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE4_A::_11
    }
}
#[doc = "Field `WUPE4` writer - Wakeup Pin Enable For LLWU_P4"]
pub type WUPE4_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, PE2_SPEC, u8, WUPE4_A, 2, O>;
impl<'a, const O: u8> WUPE4_W<'a, O> {
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE4_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE4_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE4_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE4_A::_11)
    }
}
#[doc = "Field `WUPE5` reader - Wakeup Pin Enable For LLWU_P5"]
pub type WUPE5_R = crate::FieldReader<u8, WUPE5_A>;
#[doc = "Wakeup Pin Enable For LLWU_P5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUPE5_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE5_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE5_A) -> Self {
        variant as _
    }
}
impl WUPE5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE5_A {
        match self.bits {
            0 => WUPE5_A::_00,
            1 => WUPE5_A::_01,
            2 => WUPE5_A::_10,
            3 => WUPE5_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE5_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE5_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE5_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE5_A::_11
    }
}
#[doc = "Field `WUPE5` writer - Wakeup Pin Enable For LLWU_P5"]
pub type WUPE5_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, PE2_SPEC, u8, WUPE5_A, 2, O>;
impl<'a, const O: u8> WUPE5_W<'a, O> {
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE5_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE5_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE5_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE5_A::_11)
    }
}
#[doc = "Field `WUPE6` reader - Wakeup Pin Enable For LLWU_P6"]
pub type WUPE6_R = crate::FieldReader<u8, WUPE6_A>;
#[doc = "Wakeup Pin Enable For LLWU_P6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUPE6_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE6_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE6_A) -> Self {
        variant as _
    }
}
impl WUPE6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE6_A {
        match self.bits {
            0 => WUPE6_A::_00,
            1 => WUPE6_A::_01,
            2 => WUPE6_A::_10,
            3 => WUPE6_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE6_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE6_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE6_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE6_A::_11
    }
}
#[doc = "Field `WUPE6` writer - Wakeup Pin Enable For LLWU_P6"]
pub type WUPE6_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, PE2_SPEC, u8, WUPE6_A, 2, O>;
impl<'a, const O: u8> WUPE6_W<'a, O> {
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE6_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE6_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE6_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE6_A::_11)
    }
}
#[doc = "Field `WUPE7` reader - Wakeup Pin Enable For LLWU_P7"]
pub type WUPE7_R = crate::FieldReader<u8, WUPE7_A>;
#[doc = "Wakeup Pin Enable For LLWU_P7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUPE7_A {
    #[doc = "0: External input pin disabled as wakeup input"]
    _00 = 0,
    #[doc = "1: External input pin enabled with rising edge detection"]
    _01 = 1,
    #[doc = "2: External input pin enabled with falling edge detection"]
    _10 = 2,
    #[doc = "3: External input pin enabled with any change detection"]
    _11 = 3,
}
impl From<WUPE7_A> for u8 {
    #[inline(always)]
    fn from(variant: WUPE7_A) -> Self {
        variant as _
    }
}
impl WUPE7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPE7_A {
        match self.bits {
            0 => WUPE7_A::_00,
            1 => WUPE7_A::_01,
            2 => WUPE7_A::_10,
            3 => WUPE7_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WUPE7_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WUPE7_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WUPE7_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WUPE7_A::_11
    }
}
#[doc = "Field `WUPE7` writer - Wakeup Pin Enable For LLWU_P7"]
pub type WUPE7_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, PE2_SPEC, u8, WUPE7_A, 2, O>;
impl<'a, const O: u8> WUPE7_W<'a, O> {
    #[doc = "External input pin disabled as wakeup input"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WUPE7_A::_00)
    }
    #[doc = "External input pin enabled with rising edge detection"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WUPE7_A::_01)
    }
    #[doc = "External input pin enabled with falling edge detection"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WUPE7_A::_10)
    }
    #[doc = "External input pin enabled with any change detection"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WUPE7_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P4"]
    #[inline(always)]
    pub fn wupe4(&self) -> WUPE4_R {
        WUPE4_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P5"]
    #[inline(always)]
    pub fn wupe5(&self) -> WUPE5_R {
        WUPE5_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P6"]
    #[inline(always)]
    pub fn wupe6(&self) -> WUPE6_R {
        WUPE6_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P7"]
    #[inline(always)]
    pub fn wupe7(&self) -> WUPE7_R {
        WUPE7_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Wakeup Pin Enable For LLWU_P4"]
    #[inline(always)]
    #[must_use]
    pub fn wupe4(&mut self) -> WUPE4_W<0> {
        WUPE4_W::new(self)
    }
    #[doc = "Bits 2:3 - Wakeup Pin Enable For LLWU_P5"]
    #[inline(always)]
    #[must_use]
    pub fn wupe5(&mut self) -> WUPE5_W<2> {
        WUPE5_W::new(self)
    }
    #[doc = "Bits 4:5 - Wakeup Pin Enable For LLWU_P6"]
    #[inline(always)]
    #[must_use]
    pub fn wupe6(&mut self) -> WUPE6_W<4> {
        WUPE6_W::new(self)
    }
    #[doc = "Bits 6:7 - Wakeup Pin Enable For LLWU_P7"]
    #[inline(always)]
    #[must_use]
    pub fn wupe7(&mut self) -> WUPE7_W<6> {
        WUPE7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LLWU Pin Enable 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pe2](index.html) module"]
pub struct PE2_SPEC;
impl crate::RegisterSpec for PE2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pe2::R](R) reader structure"]
impl crate::Readable for PE2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pe2::W](W) writer structure"]
impl crate::Writable for PE2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PE2 to value 0"]
impl crate::Resettable for PE2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
