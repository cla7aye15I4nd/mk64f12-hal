#[doc = "Register `CESR` reader"]
pub struct R(crate::R<CESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CESR` writer"]
pub struct W(crate::W<CESR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CESR_SPEC>;
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
impl From<crate::W<CESR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CESR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLD` reader - Valid"]
pub type VLD_R = crate::BitReader<VLD_A>;
#[doc = "Valid\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VLD_A {
    #[doc = "0: MPU is disabled. All accesses from all bus masters are allowed."]
    _0 = 0,
    #[doc = "1: MPU is enabled"]
    _1 = 1,
}
impl From<VLD_A> for bool {
    #[inline(always)]
    fn from(variant: VLD_A) -> Self {
        variant as u8 != 0
    }
}
impl VLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLD_A {
        match self.bits {
            false => VLD_A::_0,
            true => VLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VLD_A::_1
    }
}
#[doc = "Field `VLD` writer - Valid"]
pub type VLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CESR_SPEC, VLD_A, O>;
impl<'a, const O: u8> VLD_W<'a, O> {
    #[doc = "MPU is disabled. All accesses from all bus masters are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VLD_A::_0)
    }
    #[doc = "MPU is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VLD_A::_1)
    }
}
#[doc = "Field `NRGD` reader - Number Of Region Descriptors"]
pub type NRGD_R = crate::FieldReader<u8, NRGD_A>;
#[doc = "Number Of Region Descriptors\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NRGD_A {
    #[doc = "0: 8 region descriptors"]
    _0000 = 0,
    #[doc = "1: 12 region descriptors"]
    _0001 = 1,
    #[doc = "2: 16 region descriptors"]
    _0010 = 2,
}
impl From<NRGD_A> for u8 {
    #[inline(always)]
    fn from(variant: NRGD_A) -> Self {
        variant as _
    }
}
impl NRGD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NRGD_A> {
        match self.bits {
            0 => Some(NRGD_A::_0000),
            1 => Some(NRGD_A::_0001),
            2 => Some(NRGD_A::_0010),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == NRGD_A::_0000
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == NRGD_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == NRGD_A::_0010
    }
}
#[doc = "Field `NSP` reader - Number Of Slave Ports"]
pub type NSP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HRL` reader - Hardware Revision Level"]
pub type HRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPERR` reader - Slave Port n Error"]
pub type SPERR_R = crate::FieldReader<u8, SPERR_A>;
#[doc = "Slave Port n Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPERR_A {
    #[doc = "0: No error has occurred for slave port n."]
    _0 = 0,
    #[doc = "1: An error has occurred for slave port n."]
    _1 = 1,
}
impl From<SPERR_A> for u8 {
    #[inline(always)]
    fn from(variant: SPERR_A) -> Self {
        variant as _
    }
}
impl SPERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPERR_A> {
        match self.bits {
            0 => Some(SPERR_A::_0),
            1 => Some(SPERR_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPERR_A::_1
    }
}
#[doc = "Field `SPERR` writer - Slave Port n Error"]
pub type SPERR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CESR_SPEC, u8, SPERR_A, 5, O>;
impl<'a, const O: u8> SPERR_W<'a, O> {
    #[doc = "No error has occurred for slave port n."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPERR_A::_0)
    }
    #[doc = "An error has occurred for slave port n."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPERR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    pub fn vld(&self) -> VLD_R {
        VLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number Of Region Descriptors"]
    #[inline(always)]
    pub fn nrgd(&self) -> NRGD_R {
        NRGD_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Number Of Slave Ports"]
    #[inline(always)]
    pub fn nsp(&self) -> NSP_R {
        NSP_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Hardware Revision Level"]
    #[inline(always)]
    pub fn hrl(&self) -> HRL_R {
        HRL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 27:31 - Slave Port n Error"]
    #[inline(always)]
    pub fn sperr(&self) -> SPERR_R {
        SPERR_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    #[must_use]
    pub fn vld(&mut self) -> VLD_W<0> {
        VLD_W::new(self)
    }
    #[doc = "Bits 27:31 - Slave Port n Error"]
    #[inline(always)]
    #[must_use]
    pub fn sperr(&mut self) -> SPERR_W<27> {
        SPERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control/Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cesr](index.html) module"]
pub struct CESR_SPEC;
impl crate::RegisterSpec for CESR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cesr::R](R) reader structure"]
impl crate::Readable for CESR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cesr::W](W) writer structure"]
impl crate::Writable for CESR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CESR to value 0x0081_5101"]
impl crate::Resettable for CESR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0081_5101;
}
