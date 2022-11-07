#[doc = "Register `LVDSC1` reader"]
pub struct R(crate::R<LVDSC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LVDSC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LVDSC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LVDSC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LVDSC1` writer"]
pub struct W(crate::W<LVDSC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVDSC1_SPEC>;
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
impl From<crate::W<LVDSC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVDSC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVDV` reader - Low-Voltage Detect Voltage Select"]
pub type LVDV_R = crate::FieldReader<u8, LVDV_A>;
#[doc = "Low-Voltage Detect Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LVDV_A {
    #[doc = "0: Low trip point selected (V LVD = V LVDL )"]
    _00 = 0,
    #[doc = "1: High trip point selected (V LVD = V LVDH )"]
    _01 = 1,
}
impl From<LVDV_A> for u8 {
    #[inline(always)]
    fn from(variant: LVDV_A) -> Self {
        variant as _
    }
}
impl LVDV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LVDV_A> {
        match self.bits {
            0 => Some(LVDV_A::_00),
            1 => Some(LVDV_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == LVDV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == LVDV_A::_01
    }
}
#[doc = "Field `LVDV` writer - Low-Voltage Detect Voltage Select"]
pub type LVDV_W<'a, const O: u8> = crate::FieldWriter<'a, u8, LVDSC1_SPEC, u8, LVDV_A, 2, O>;
impl<'a, const O: u8> LVDV_W<'a, O> {
    #[doc = "Low trip point selected (V LVD = V LVDL )"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(LVDV_A::_00)
    }
    #[doc = "High trip point selected (V LVD = V LVDH )"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(LVDV_A::_01)
    }
}
#[doc = "Field `LVDRE` reader - Low-Voltage Detect Reset Enable"]
pub type LVDRE_R = crate::BitReader<LVDRE_A>;
#[doc = "Low-Voltage Detect Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVDRE_A {
    #[doc = "0: LVDF does not generate hardware resets"]
    _0 = 0,
    #[doc = "1: Force an MCU reset when LVDF = 1"]
    _1 = 1,
}
impl From<LVDRE_A> for bool {
    #[inline(always)]
    fn from(variant: LVDRE_A) -> Self {
        variant as u8 != 0
    }
}
impl LVDRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDRE_A {
        match self.bits {
            false => LVDRE_A::_0,
            true => LVDRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVDRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVDRE_A::_1
    }
}
#[doc = "Field `LVDRE` writer - Low-Voltage Detect Reset Enable"]
pub type LVDRE_W<'a, const O: u8> = crate::BitWriter<'a, u8, LVDSC1_SPEC, LVDRE_A, O>;
impl<'a, const O: u8> LVDRE_W<'a, O> {
    #[doc = "LVDF does not generate hardware resets"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVDRE_A::_0)
    }
    #[doc = "Force an MCU reset when LVDF = 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVDRE_A::_1)
    }
}
#[doc = "Field `LVDIE` reader - Low-Voltage Detect Interrupt Enable"]
pub type LVDIE_R = crate::BitReader<LVDIE_A>;
#[doc = "Low-Voltage Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVDIE_A {
    #[doc = "0: Hardware interrupt disabled (use polling)"]
    _0 = 0,
    #[doc = "1: Request a hardware interrupt when LVDF = 1"]
    _1 = 1,
}
impl From<LVDIE_A> for bool {
    #[inline(always)]
    fn from(variant: LVDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LVDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDIE_A {
        match self.bits {
            false => LVDIE_A::_0,
            true => LVDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVDIE_A::_1
    }
}
#[doc = "Field `LVDIE` writer - Low-Voltage Detect Interrupt Enable"]
pub type LVDIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, LVDSC1_SPEC, LVDIE_A, O>;
impl<'a, const O: u8> LVDIE_W<'a, O> {
    #[doc = "Hardware interrupt disabled (use polling)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LVDIE_A::_0)
    }
    #[doc = "Request a hardware interrupt when LVDF = 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LVDIE_A::_1)
    }
}
#[doc = "Field `LVDACK` writer - Low-Voltage Detect Acknowledge"]
pub type LVDACK_W<'a, const O: u8> = crate::BitWriter<'a, u8, LVDSC1_SPEC, bool, O>;
#[doc = "Field `LVDF` reader - Low-Voltage Detect Flag"]
pub type LVDF_R = crate::BitReader<LVDF_A>;
#[doc = "Low-Voltage Detect Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LVDF_A {
    #[doc = "0: Low-voltage event not detected"]
    _0 = 0,
    #[doc = "1: Low-voltage event detected"]
    _1 = 1,
}
impl From<LVDF_A> for bool {
    #[inline(always)]
    fn from(variant: LVDF_A) -> Self {
        variant as u8 != 0
    }
}
impl LVDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LVDF_A {
        match self.bits {
            false => LVDF_A::_0,
            true => LVDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LVDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LVDF_A::_1
    }
}
impl R {
    #[doc = "Bits 0:1 - Low-Voltage Detect Voltage Select"]
    #[inline(always)]
    pub fn lvdv(&self) -> LVDV_R {
        LVDV_R::new(self.bits & 3)
    }
    #[doc = "Bit 4 - Low-Voltage Detect Reset Enable"]
    #[inline(always)]
    pub fn lvdre(&self) -> LVDRE_R {
        LVDRE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Low-Voltage Detect Interrupt Enable"]
    #[inline(always)]
    pub fn lvdie(&self) -> LVDIE_R {
        LVDIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Low-Voltage Detect Flag"]
    #[inline(always)]
    pub fn lvdf(&self) -> LVDF_R {
        LVDF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Low-Voltage Detect Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn lvdv(&mut self) -> LVDV_W<0> {
        LVDV_W::new(self)
    }
    #[doc = "Bit 4 - Low-Voltage Detect Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvdre(&mut self) -> LVDRE_W<4> {
        LVDRE_W::new(self)
    }
    #[doc = "Bit 5 - Low-Voltage Detect Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lvdie(&mut self) -> LVDIE_W<5> {
        LVDIE_W::new(self)
    }
    #[doc = "Bit 6 - Low-Voltage Detect Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn lvdack(&mut self) -> LVDACK_W<6> {
        LVDACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Voltage Detect Status And Control 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvdsc1](index.html) module"]
pub struct LVDSC1_SPEC;
impl crate::RegisterSpec for LVDSC1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [lvdsc1::R](R) reader structure"]
impl crate::Readable for LVDSC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lvdsc1::W](W) writer structure"]
impl crate::Writable for LVDSC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVDSC1 to value 0x10"]
impl crate::Resettable for LVDSC1_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
