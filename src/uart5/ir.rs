#[doc = "Register `IR` reader"]
pub struct R(crate::R<IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IR` writer"]
pub struct W(crate::W<IR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IR_SPEC>;
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
impl From<crate::W<IR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TNP` reader - Transmitter narrow pulse"]
pub type TNP_R = crate::FieldReader<u8, TNP_A>;
#[doc = "Transmitter narrow pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TNP_A {
    #[doc = "0: 3/16."]
    _00 = 0,
    #[doc = "1: 1/16."]
    _01 = 1,
    #[doc = "2: 1/32."]
    _10 = 2,
    #[doc = "3: 1/4."]
    _11 = 3,
}
impl From<TNP_A> for u8 {
    #[inline(always)]
    fn from(variant: TNP_A) -> Self {
        variant as _
    }
}
impl TNP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TNP_A {
        match self.bits {
            0 => TNP_A::_00,
            1 => TNP_A::_01,
            2 => TNP_A::_10,
            3 => TNP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TNP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TNP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TNP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TNP_A::_11
    }
}
#[doc = "Field `TNP` writer - Transmitter narrow pulse"]
pub type TNP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, IR_SPEC, u8, TNP_A, 2, O>;
impl<'a, const O: u8> TNP_W<'a, O> {
    #[doc = "3/16."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TNP_A::_00)
    }
    #[doc = "1/16."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TNP_A::_01)
    }
    #[doc = "1/32."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TNP_A::_10)
    }
    #[doc = "1/4."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TNP_A::_11)
    }
}
#[doc = "Field `IREN` reader - Infrared enable"]
pub type IREN_R = crate::BitReader<IREN_A>;
#[doc = "Infrared enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREN_A {
    #[doc = "0: IR disabled."]
    _0 = 0,
    #[doc = "1: IR enabled."]
    _1 = 1,
}
impl From<IREN_A> for bool {
    #[inline(always)]
    fn from(variant: IREN_A) -> Self {
        variant as u8 != 0
    }
}
impl IREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREN_A {
        match self.bits {
            false => IREN_A::_0,
            true => IREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IREN_A::_1
    }
}
#[doc = "Field `IREN` writer - Infrared enable"]
pub type IREN_W<'a, const O: u8> = crate::BitWriter<'a, u8, IR_SPEC, IREN_A, O>;
impl<'a, const O: u8> IREN_W<'a, O> {
    #[doc = "IR disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IREN_A::_0)
    }
    #[doc = "IR enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IREN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Transmitter narrow pulse"]
    #[inline(always)]
    pub fn tnp(&self) -> TNP_R {
        TNP_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Infrared enable"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transmitter narrow pulse"]
    #[inline(always)]
    #[must_use]
    pub fn tnp(&mut self) -> TNP_W<0> {
        TNP_W::new(self)
    }
    #[doc = "Bit 2 - Infrared enable"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<2> {
        IREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Infrared Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir](index.html) module"]
pub struct IR_SPEC;
impl crate::RegisterSpec for IR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ir::R](R) reader structure"]
impl crate::Readable for IR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ir::W](W) writer structure"]
impl crate::Writable for IR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
