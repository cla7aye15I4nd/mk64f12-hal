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
#[doc = "Field `MODE_LV` reader - Buffer Mode selection"]
pub type MODE_LV_R = crate::FieldReader<u8, MODE_LV_A>;
#[doc = "Buffer Mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_LV_A {
    #[doc = "0: Bandgap on only, for stabilization and startup"]
    _00 = 0,
    #[doc = "1: High power buffer mode enabled"]
    _01 = 1,
    #[doc = "2: Low-power buffer mode enabled"]
    _10 = 2,
}
impl From<MODE_LV_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_LV_A) -> Self {
        variant as _
    }
}
impl MODE_LV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_LV_A> {
        match self.bits {
            0 => Some(MODE_LV_A::_00),
            1 => Some(MODE_LV_A::_01),
            2 => Some(MODE_LV_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MODE_LV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MODE_LV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MODE_LV_A::_10
    }
}
#[doc = "Field `MODE_LV` writer - Buffer Mode selection"]
pub type MODE_LV_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SC_SPEC, u8, MODE_LV_A, 2, O>;
impl<'a, const O: u8> MODE_LV_W<'a, O> {
    #[doc = "Bandgap on only, for stabilization and startup"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MODE_LV_A::_00)
    }
    #[doc = "High power buffer mode enabled"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MODE_LV_A::_01)
    }
    #[doc = "Low-power buffer mode enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MODE_LV_A::_10)
    }
}
#[doc = "Field `VREFST` reader - Internal Voltage Reference stable"]
pub type VREFST_R = crate::BitReader<VREFST_A>;
#[doc = "Internal Voltage Reference stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFST_A {
    #[doc = "0: The module is disabled or not stable."]
    _0 = 0,
    #[doc = "1: The module is stable."]
    _1 = 1,
}
impl From<VREFST_A> for bool {
    #[inline(always)]
    fn from(variant: VREFST_A) -> Self {
        variant as u8 != 0
    }
}
impl VREFST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFST_A {
        match self.bits {
            false => VREFST_A::_0,
            true => VREFST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VREFST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VREFST_A::_1
    }
}
#[doc = "Field `ICOMPEN` reader - Second order curvature compensation enable"]
pub type ICOMPEN_R = crate::BitReader<ICOMPEN_A>;
#[doc = "Second order curvature compensation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICOMPEN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<ICOMPEN_A> for bool {
    #[inline(always)]
    fn from(variant: ICOMPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ICOMPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICOMPEN_A {
        match self.bits {
            false => ICOMPEN_A::_0,
            true => ICOMPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ICOMPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ICOMPEN_A::_1
    }
}
#[doc = "Field `ICOMPEN` writer - Second order curvature compensation enable"]
pub type ICOMPEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SC_SPEC, ICOMPEN_A, O>;
impl<'a, const O: u8> ICOMPEN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ICOMPEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ICOMPEN_A::_1)
    }
}
#[doc = "Field `REGEN` reader - Regulator enable"]
pub type REGEN_R = crate::BitReader<REGEN_A>;
#[doc = "Regulator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REGEN_A {
    #[doc = "0: Internal 1.75 V regulator is disabled."]
    _0 = 0,
    #[doc = "1: Internal 1.75 V regulator is enabled."]
    _1 = 1,
}
impl From<REGEN_A> for bool {
    #[inline(always)]
    fn from(variant: REGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl REGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REGEN_A {
        match self.bits {
            false => REGEN_A::_0,
            true => REGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == REGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == REGEN_A::_1
    }
}
#[doc = "Field `REGEN` writer - Regulator enable"]
pub type REGEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SC_SPEC, REGEN_A, O>;
impl<'a, const O: u8> REGEN_W<'a, O> {
    #[doc = "Internal 1.75 V regulator is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REGEN_A::_0)
    }
    #[doc = "Internal 1.75 V regulator is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REGEN_A::_1)
    }
}
#[doc = "Field `VREFEN` reader - Internal Voltage Reference enable"]
pub type VREFEN_R = crate::BitReader<VREFEN_A>;
#[doc = "Internal Voltage Reference enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFEN_A {
    #[doc = "0: The module is disabled."]
    _0 = 0,
    #[doc = "1: The module is enabled."]
    _1 = 1,
}
impl From<VREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VREFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFEN_A {
        match self.bits {
            false => VREFEN_A::_0,
            true => VREFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VREFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VREFEN_A::_1
    }
}
#[doc = "Field `VREFEN` writer - Internal Voltage Reference enable"]
pub type VREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SC_SPEC, VREFEN_A, O>;
impl<'a, const O: u8> VREFEN_W<'a, O> {
    #[doc = "The module is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VREFEN_A::_0)
    }
    #[doc = "The module is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VREFEN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Buffer Mode selection"]
    #[inline(always)]
    pub fn mode_lv(&self) -> MODE_LV_R {
        MODE_LV_R::new(self.bits & 3)
    }
    #[doc = "Bit 2 - Internal Voltage Reference stable"]
    #[inline(always)]
    pub fn vrefst(&self) -> VREFST_R {
        VREFST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Second order curvature compensation enable"]
    #[inline(always)]
    pub fn icompen(&self) -> ICOMPEN_R {
        ICOMPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Regulator enable"]
    #[inline(always)]
    pub fn regen(&self) -> REGEN_R {
        REGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Internal Voltage Reference enable"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Buffer Mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn mode_lv(&mut self) -> MODE_LV_W<0> {
        MODE_LV_W::new(self)
    }
    #[doc = "Bit 5 - Second order curvature compensation enable"]
    #[inline(always)]
    #[must_use]
    pub fn icompen(&mut self) -> ICOMPEN_W<5> {
        ICOMPEN_W::new(self)
    }
    #[doc = "Bit 6 - Regulator enable"]
    #[inline(always)]
    #[must_use]
    pub fn regen(&mut self) -> REGEN_W<6> {
        REGEN_W::new(self)
    }
    #[doc = "Bit 7 - Internal Voltage Reference enable"]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<7> {
        VREFEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VREF Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc](index.html) module"]
pub struct SC_SPEC;
impl crate::RegisterSpec for SC_SPEC {
    type Ux = u8;
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
