#[doc = "Register `DCHPRI%s` reader"]
pub struct R(crate::R<DCHPRI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCHPRI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCHPRI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCHPRI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCHPRI%s` writer"]
pub struct W(crate::W<DCHPRI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCHPRI_SPEC>;
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
impl From<crate::W<DCHPRI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCHPRI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHPRI` reader - Channel n Arbitration Priority"]
pub type CHPRI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHPRI` writer - Channel n Arbitration Priority"]
pub type CHPRI_W<'a, const O: u8> = crate::FieldWriter<'a, u8, DCHPRI_SPEC, u8, u8, 4, O>;
#[doc = "Field `DPA` reader - Disable Preempt Ability"]
pub type DPA_R = crate::BitReader<DPA_A>;
#[doc = "Disable Preempt Ability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPA_A {
    #[doc = "0: Channel n can suspend a lower priority channel"]
    _0 = 0,
    #[doc = "1: Channel n cannot suspend any channel, regardless of channel priority"]
    _1 = 1,
}
impl From<DPA_A> for bool {
    #[inline(always)]
    fn from(variant: DPA_A) -> Self {
        variant as u8 != 0
    }
}
impl DPA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPA_A {
        match self.bits {
            false => DPA_A::_0,
            true => DPA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPA_A::_1
    }
}
#[doc = "Field `DPA` writer - Disable Preempt Ability"]
pub type DPA_W<'a, const O: u8> = crate::BitWriter<'a, u8, DCHPRI_SPEC, DPA_A, O>;
impl<'a, const O: u8> DPA_W<'a, O> {
    #[doc = "Channel n can suspend a lower priority channel"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPA_A::_0)
    }
    #[doc = "Channel n cannot suspend any channel, regardless of channel priority"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPA_A::_1)
    }
}
#[doc = "Field `ECP` reader - Enable Channel Preemption"]
pub type ECP_R = crate::BitReader<ECP_A>;
#[doc = "Enable Channel Preemption\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECP_A {
    #[doc = "0: Channel n cannot be suspended by a higher priority channel's service request"]
    _0 = 0,
    #[doc = "1: Channel n can be temporarily suspended by the service request of a higher priority channel"]
    _1 = 1,
}
impl From<ECP_A> for bool {
    #[inline(always)]
    fn from(variant: ECP_A) -> Self {
        variant as u8 != 0
    }
}
impl ECP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECP_A {
        match self.bits {
            false => ECP_A::_0,
            true => ECP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECP_A::_1
    }
}
#[doc = "Field `ECP` writer - Enable Channel Preemption"]
pub type ECP_W<'a, const O: u8> = crate::BitWriter<'a, u8, DCHPRI_SPEC, ECP_A, O>;
impl<'a, const O: u8> ECP_W<'a, O> {
    #[doc = "Channel n cannot be suspended by a higher priority channel's service request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ECP_A::_0)
    }
    #[doc = "Channel n can be temporarily suspended by the service request of a higher priority channel"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ECP_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Channel n Arbitration Priority"]
    #[inline(always)]
    pub fn chpri(&self) -> CHPRI_R {
        CHPRI_R::new(self.bits & 0x0f)
    }
    #[doc = "Bit 6 - Disable Preempt Ability"]
    #[inline(always)]
    pub fn dpa(&self) -> DPA_R {
        DPA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Channel Preemption"]
    #[inline(always)]
    pub fn ecp(&self) -> ECP_R {
        ECP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel n Arbitration Priority"]
    #[inline(always)]
    #[must_use]
    pub fn chpri(&mut self) -> CHPRI_W<0> {
        CHPRI_W::new(self)
    }
    #[doc = "Bit 6 - Disable Preempt Ability"]
    #[inline(always)]
    #[must_use]
    pub fn dpa(&mut self) -> DPA_W<6> {
        DPA_W::new(self)
    }
    #[doc = "Bit 7 - Enable Channel Preemption"]
    #[inline(always)]
    #[must_use]
    pub fn ecp(&mut self) -> ECP_W<7> {
        ECP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Priority Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dchpri](index.html) module"]
pub struct DCHPRI_SPEC;
impl crate::RegisterSpec for DCHPRI_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dchpri::R](R) reader structure"]
impl crate::Readable for DCHPRI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dchpri::W](W) writer structure"]
impl crate::Writable for DCHPRI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCHPRI%s to value 0"]
impl crate::Resettable for DCHPRI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
