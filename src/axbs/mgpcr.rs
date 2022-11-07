#[doc = "Register `MGPCR%s` reader"]
pub struct R(crate::R<MGPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MGPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MGPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MGPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MGPCR%s` writer"]
pub struct W(crate::W<MGPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MGPCR_SPEC>;
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
impl From<crate::W<MGPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MGPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AULB` reader - Arbitrates On Undefined Length Bursts"]
pub type AULB_R = crate::FieldReader<u8, AULB_A>;
#[doc = "Arbitrates On Undefined Length Bursts\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AULB_A {
    #[doc = "0: No arbitration is allowed during an undefined length burst"]
    _000 = 0,
    #[doc = "1: Arbitration is allowed at any time during an undefined length burst"]
    _001 = 1,
    #[doc = "2: Arbitration is allowed after four beats of an undefined length burst"]
    _010 = 2,
    #[doc = "3: Arbitration is allowed after eight beats of an undefined length burst"]
    _011 = 3,
    #[doc = "4: Arbitration is allowed after 16 beats of an undefined length burst"]
    _100 = 4,
}
impl From<AULB_A> for u8 {
    #[inline(always)]
    fn from(variant: AULB_A) -> Self {
        variant as _
    }
}
impl AULB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AULB_A> {
        match self.bits {
            0 => Some(AULB_A::_000),
            1 => Some(AULB_A::_001),
            2 => Some(AULB_A::_010),
            3 => Some(AULB_A::_011),
            4 => Some(AULB_A::_100),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == AULB_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == AULB_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == AULB_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == AULB_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == AULB_A::_100
    }
}
#[doc = "Field `AULB` writer - Arbitrates On Undefined Length Bursts"]
pub type AULB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MGPCR_SPEC, u8, AULB_A, 3, O>;
impl<'a, const O: u8> AULB_W<'a, O> {
    #[doc = "No arbitration is allowed during an undefined length burst"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(AULB_A::_000)
    }
    #[doc = "Arbitration is allowed at any time during an undefined length burst"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(AULB_A::_001)
    }
    #[doc = "Arbitration is allowed after four beats of an undefined length burst"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(AULB_A::_010)
    }
    #[doc = "Arbitration is allowed after eight beats of an undefined length burst"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(AULB_A::_011)
    }
    #[doc = "Arbitration is allowed after 16 beats of an undefined length burst"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(AULB_A::_100)
    }
}
impl R {
    #[doc = "Bits 0:2 - Arbitrates On Undefined Length Bursts"]
    #[inline(always)]
    pub fn aulb(&self) -> AULB_R {
        AULB_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Arbitrates On Undefined Length Bursts"]
    #[inline(always)]
    #[must_use]
    pub fn aulb(&mut self) -> AULB_W<0> {
        AULB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master General Purpose Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mgpcr](index.html) module"]
pub struct MGPCR_SPEC;
impl crate::RegisterSpec for MGPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mgpcr::R](R) reader structure"]
impl crate::Readable for MGPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mgpcr::W](W) writer structure"]
impl crate::Writable for MGPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MGPCR%s to value 0"]
impl crate::Resettable for MGPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
