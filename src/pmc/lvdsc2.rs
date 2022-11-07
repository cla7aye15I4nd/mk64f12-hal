#[doc = "Register `LVDSC2` reader"]
pub struct R(crate::R<LVDSC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LVDSC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LVDSC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LVDSC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LVDSC2` writer"]
pub struct W(crate::W<LVDSC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVDSC2_SPEC>;
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
impl From<crate::W<LVDSC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVDSC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVWV` reader - Low-Voltage Warning Voltage Select"]
pub type LVWV_R = crate::FieldReader<u8, LVWV_A>;
#[doc = "Low-Voltage Warning Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVWV_A {
    #[doc = "0: Low trip point selected (VLVW = VLVW1)"]
    _00 = 0,
    #[doc = "1: Mid 1 trip point selected (VLVW = VLVW2)"]
    _01 = 1,
    #[doc = "2: Mid 2 trip point selected (VLVW = VLVW3)"]
    _10 = 2,
    #[doc = "3: High trip point selected (VLVW = VLVW4)"]
    _11 = 3,
}
impl From<LVWV_A> for u8 {
    #[inline(always)]
    fn from(variant: LVWV_A) -> Self {
        variant as _
    }
}
impl LVWV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVWV_A {
        match self.bits {
            0 => LVWV_A::_00,
            1 => LVWV_A::_01,
            2 => LVWV_A::_10,
            3 => LVWV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LVWV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LVWV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == LVWV_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == LVWV_A::_11
    }
}
#[doc = "Field `LVWV` writer - Low-Voltage Warning Voltage Select"]
pub type LVWV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, LVDSC2_SPEC, u8, LVWV_A, 2, O>;
impl<'a, const O: u8> LVWV_W<'a, O> {
    #[doc = "Low trip point selected (VLVW = VLVW1)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LVWV_A::_00)
    }
    #[doc = "Mid 1 trip point selected (VLVW = VLVW2)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LVWV_A::_01)
    }
    #[doc = "Mid 2 trip point selected (VLVW = VLVW3)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(LVWV_A::_10)
    }
    #[doc = "High trip point selected (VLVW = VLVW4)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(LVWV_A::_11)
    }
}
#[doc = "Field `LVWIE` reader - Low-Voltage Warning Interrupt Enable"]
pub type LVWIE_R = crate::BitReader<LVWIE_A>;
#[doc = "Low-Voltage Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVWIE_A {
    #[doc = "0: Hardware interrupt disabled (use polling)"]
    _0 = 0,
    #[doc = "1: Request a hardware interrupt when LVWF = 1"]
    _1 = 1,
}
impl From<LVWIE_A> for bool {
    #[inline(always)]
    fn from(variant: LVWIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LVWIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVWIE_A {
        match self.bits {
            false => LVWIE_A::_0,
            true => LVWIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVWIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVWIE_A::_1
    }
}
#[doc = "Field `LVWIE` writer - Low-Voltage Warning Interrupt Enable"]
pub type LVWIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, LVDSC2_SPEC, LVWIE_A, O>;
impl<'a, const O: u8> LVWIE_W<'a, O> {
    #[doc = "Hardware interrupt disabled (use polling)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVWIE_A::_0)
    }
    #[doc = "Request a hardware interrupt when LVWF = 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVWIE_A::_1)
    }
}
#[doc = "Field `LVWACK` writer - Low-Voltage Warning Acknowledge"]
pub type LVWACK_W<'a, const O: u8> = crate::BitWriter<'a, u8, LVDSC2_SPEC, bool, O>;
#[doc = "Field `LVWF` reader - Low-Voltage Warning Flag"]
pub type LVWF_R = crate::BitReader<LVWF_A>;
#[doc = "Low-Voltage Warning Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVWF_A {
    #[doc = "0: Low-voltage warning event not detected"]
    _0 = 0,
    #[doc = "1: Low-voltage warning event detected"]
    _1 = 1,
}
impl From<LVWF_A> for bool {
    #[inline(always)]
    fn from(variant: LVWF_A) -> Self {
        variant as u8 != 0
    }
}
impl LVWF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVWF_A {
        match self.bits {
            false => LVWF_A::_0,
            true => LVWF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVWF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVWF_A::_1
    }
}
impl R {
    #[doc = "Bits 0:1 - Low-Voltage Warning Voltage Select"]
    #[inline(always)]
    pub fn lvwv(&self) -> LVWV_R {
        LVWV_R::new(self.bits & 3)
    }
    #[doc = "Bit 5 - Low-Voltage Warning Interrupt Enable"]
    #[inline(always)]
    pub fn lvwie(&self) -> LVWIE_R {
        LVWIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Low-Voltage Warning Flag"]
    #[inline(always)]
    pub fn lvwf(&self) -> LVWF_R {
        LVWF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low-Voltage Warning Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn lvwv(&mut self) -> LVWV_W<0> {
        LVWV_W::new(self)
    }
    #[doc = "Bit 5 - Low-Voltage Warning Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvwie(&mut self) -> LVWIE_W<5> {
        LVWIE_W::new(self)
    }
    #[doc = "Bit 6 - Low-Voltage Warning Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn lvwack(&mut self) -> LVWACK_W<6> {
        LVWACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Voltage Detect Status And Control 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvdsc2](index.html) module"]
pub struct LVDSC2_SPEC;
impl crate::RegisterSpec for LVDSC2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lvdsc2::R](R) reader structure"]
impl crate::Readable for LVDSC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lvdsc2::W](W) writer structure"]
impl crate::Writable for LVDSC2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVDSC2 to value 0"]
impl crate::Resettable for LVDSC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
