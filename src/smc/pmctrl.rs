#[doc = "Register `PMCTRL` reader"]
pub struct R(crate::R<PMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMCTRL` writer"]
pub struct W(crate::W<PMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMCTRL_SPEC>;
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
impl From<crate::W<PMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOPM` reader - Stop Mode Control"]
pub type STOPM_R = crate::FieldReader<u8, STOPM_A>;
#[doc = "Stop Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STOPM_A {
    #[doc = "0: Normal Stop (STOP)"]
    _000 = 0,
    #[doc = "2: Very-Low-Power Stop (VLPS)"]
    _010 = 2,
    #[doc = "3: Low-Leakage Stop (LLS)"]
    _011 = 3,
    #[doc = "4: Very-Low-Leakage Stop (VLLSx)"]
    _100 = 4,
    #[doc = "6: Reseved"]
    _110 = 6,
}
impl From<STOPM_A> for u8 {
    #[inline(always)]
    fn from(variant: STOPM_A) -> Self {
        variant as _
    }
}
impl STOPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STOPM_A> {
        match self.bits {
            0 => Some(STOPM_A::_000),
            2 => Some(STOPM_A::_010),
            3 => Some(STOPM_A::_011),
            4 => Some(STOPM_A::_100),
            6 => Some(STOPM_A::_110),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == STOPM_A::_000
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == STOPM_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == STOPM_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == STOPM_A::_100
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == STOPM_A::_110
    }
}
#[doc = "Field `STOPM` writer - Stop Mode Control"]
pub type STOPM_W<'a, const O: u8> = crate::FieldWriter<'a, u8, PMCTRL_SPEC, u8, STOPM_A, 3, O>;
impl<'a, const O: u8> STOPM_W<'a, O> {
    #[doc = "Normal Stop (STOP)"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(STOPM_A::_000)
    }
    #[doc = "Very-Low-Power Stop (VLPS)"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(STOPM_A::_010)
    }
    #[doc = "Low-Leakage Stop (LLS)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(STOPM_A::_011)
    }
    #[doc = "Very-Low-Leakage Stop (VLLSx)"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(STOPM_A::_100)
    }
    #[doc = "Reseved"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(STOPM_A::_110)
    }
}
#[doc = "Field `STOPA` reader - Stop Aborted"]
pub type STOPA_R = crate::BitReader<STOPA_A>;
#[doc = "Stop Aborted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPA_A {
    #[doc = "0: The previous stop mode entry was successsful."]
    _0 = 0,
    #[doc = "1: The previous stop mode entry was aborted."]
    _1 = 1,
}
impl From<STOPA_A> for bool {
    #[inline(always)]
    fn from(variant: STOPA_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPA_A {
        match self.bits {
            false => STOPA_A::_0,
            true => STOPA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STOPA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STOPA_A::_1
    }
}
#[doc = "Field `RUNM` reader - Run Mode Control"]
pub type RUNM_R = crate::FieldReader<u8, RUNM_A>;
#[doc = "Run Mode Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RUNM_A {
    #[doc = "0: Normal Run mode (RUN)"]
    _00 = 0,
    #[doc = "2: Very-Low-Power Run mode (VLPR)"]
    _10 = 2,
}
impl From<RUNM_A> for u8 {
    #[inline(always)]
    fn from(variant: RUNM_A) -> Self {
        variant as _
    }
}
impl RUNM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RUNM_A> {
        match self.bits {
            0 => Some(RUNM_A::_00),
            2 => Some(RUNM_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RUNM_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RUNM_A::_10
    }
}
#[doc = "Field `RUNM` writer - Run Mode Control"]
pub type RUNM_W<'a, const O: u8> = crate::FieldWriter<'a, u8, PMCTRL_SPEC, u8, RUNM_A, 2, O>;
impl<'a, const O: u8> RUNM_W<'a, O> {
    #[doc = "Normal Run mode (RUN)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RUNM_A::_00)
    }
    #[doc = "Very-Low-Power Run mode (VLPR)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RUNM_A::_10)
    }
}
#[doc = "Field `LPWUI` reader - Low-Power Wake Up On Interrupt"]
pub type LPWUI_R = crate::BitReader<LPWUI_A>;
#[doc = "Low-Power Wake Up On Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPWUI_A {
    #[doc = "0: The system remains in a VLP mode on an interrupt"]
    _0 = 0,
    #[doc = "1: The system exits to Normal RUN mode on an interrupt"]
    _1 = 1,
}
impl From<LPWUI_A> for bool {
    #[inline(always)]
    fn from(variant: LPWUI_A) -> Self {
        variant as u8 != 0
    }
}
impl LPWUI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPWUI_A {
        match self.bits {
            false => LPWUI_A::_0,
            true => LPWUI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPWUI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPWUI_A::_1
    }
}
#[doc = "Field `LPWUI` writer - Low-Power Wake Up On Interrupt"]
pub type LPWUI_W<'a, const O: u8> = crate::BitWriter<'a, u8, PMCTRL_SPEC, LPWUI_A, O>;
impl<'a, const O: u8> LPWUI_W<'a, O> {
    #[doc = "The system remains in a VLP mode on an interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPWUI_A::_0)
    }
    #[doc = "The system exits to Normal RUN mode on an interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPWUI_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Stop Mode Control"]
    #[inline(always)]
    pub fn stopm(&self) -> STOPM_R {
        STOPM_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Stop Aborted"]
    #[inline(always)]
    pub fn stopa(&self) -> STOPA_R {
        STOPA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Run Mode Control"]
    #[inline(always)]
    pub fn runm(&self) -> RUNM_R {
        RUNM_R::new((self.bits >> 5) & 3)
    }
    #[doc = "Bit 7 - Low-Power Wake Up On Interrupt"]
    #[inline(always)]
    pub fn lpwui(&self) -> LPWUI_R {
        LPWUI_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Stop Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn stopm(&mut self) -> STOPM_W<0> {
        STOPM_W::new(self)
    }
    #[doc = "Bits 5:6 - Run Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn runm(&mut self) -> RUNM_W<5> {
        RUNM_W::new(self)
    }
    #[doc = "Bit 7 - Low-Power Wake Up On Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn lpwui(&mut self) -> LPWUI_W<7> {
        LPWUI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Mode Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmctrl](index.html) module"]
pub struct PMCTRL_SPEC;
impl crate::RegisterSpec for PMCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pmctrl::R](R) reader structure"]
impl crate::Readable for PMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmctrl::W](W) writer structure"]
impl crate::Writable for PMCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMCTRL to value 0"]
impl crate::Resettable for PMCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
