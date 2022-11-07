#[doc = "Register `VLLSCTRL` reader"]
pub struct R(crate::R<VLLSCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VLLSCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VLLSCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VLLSCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VLLSCTRL` writer"]
pub struct W(crate::W<VLLSCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VLLSCTRL_SPEC>;
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
impl From<crate::W<VLLSCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VLLSCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLLSM` reader - VLLS Mode Control"]
pub type VLLSM_R = crate::FieldReader<u8, VLLSM_A>;
#[doc = "VLLS Mode Control\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VLLSM_A {
    #[doc = "0: VLLS0"]
    _000 = 0,
    #[doc = "1: VLLS1"]
    _001 = 1,
    #[doc = "2: VLLS2"]
    _010 = 2,
    #[doc = "3: VLLS3"]
    _011 = 3,
}
impl From<VLLSM_A> for u8 {
    #[inline(always)]
    fn from(variant: VLLSM_A) -> Self {
        variant as _
    }
}
impl VLLSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VLLSM_A> {
        match self.bits {
            0 => Some(VLLSM_A::_000),
            1 => Some(VLLSM_A::_001),
            2 => Some(VLLSM_A::_010),
            3 => Some(VLLSM_A::_011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == VLLSM_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == VLLSM_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == VLLSM_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == VLLSM_A::_011
    }
}
#[doc = "Field `VLLSM` writer - VLLS Mode Control"]
pub type VLLSM_W<'a, const O: u8> = crate::FieldWriter<'a, u8, VLLSCTRL_SPEC, u8, VLLSM_A, 3, O>;
impl<'a, const O: u8> VLLSM_W<'a, O> {
    #[doc = "VLLS0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(VLLSM_A::_000)
    }
    #[doc = "VLLS1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(VLLSM_A::_001)
    }
    #[doc = "VLLS2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(VLLSM_A::_010)
    }
    #[doc = "VLLS3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(VLLSM_A::_011)
    }
}
#[doc = "Field `PORPO` reader - POR Power Option"]
pub type PORPO_R = crate::BitReader<PORPO_A>;
#[doc = "POR Power Option\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORPO_A {
    #[doc = "0: POR detect circuit is enabled in VLLS0."]
    _0 = 0,
    #[doc = "1: POR detect circuit is disabled in VLLS0."]
    _1 = 1,
}
impl From<PORPO_A> for bool {
    #[inline(always)]
    fn from(variant: PORPO_A) -> Self {
        variant as u8 != 0
    }
}
impl PORPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORPO_A {
        match self.bits {
            false => PORPO_A::_0,
            true => PORPO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORPO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORPO_A::_1
    }
}
#[doc = "Field `PORPO` writer - POR Power Option"]
pub type PORPO_W<'a, const O: u8> = crate::BitWriter<'a, u8, VLLSCTRL_SPEC, PORPO_A, O>;
impl<'a, const O: u8> PORPO_W<'a, O> {
    #[doc = "POR detect circuit is enabled in VLLS0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORPO_A::_0)
    }
    #[doc = "POR detect circuit is disabled in VLLS0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORPO_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - VLLS Mode Control"]
    #[inline(always)]
    pub fn vllsm(&self) -> VLLSM_R {
        VLLSM_R::new(self.bits & 7)
    }
    #[doc = "Bit 5 - POR Power Option"]
    #[inline(always)]
    pub fn porpo(&self) -> PORPO_R {
        PORPO_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - VLLS Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn vllsm(&mut self) -> VLLSM_W<0> {
        VLLSM_W::new(self)
    }
    #[doc = "Bit 5 - POR Power Option"]
    #[inline(always)]
    #[must_use]
    pub fn porpo(&mut self) -> PORPO_W<5> {
        PORPO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VLLS Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vllsctrl](index.html) module"]
pub struct VLLSCTRL_SPEC;
impl crate::RegisterSpec for VLLSCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [vllsctrl::R](R) reader structure"]
impl crate::Readable for VLLSCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vllsctrl::W](W) writer structure"]
impl crate::Writable for VLLSCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VLLSCTRL to value 0x03"]
impl crate::Resettable for VLLSCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
