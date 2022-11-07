#[doc = "Register `CAU_STR_CASR` reader"]
pub struct R(crate::R<CAU_STR_CASR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAU_STR_CASR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAU_STR_CASR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAU_STR_CASR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IC` reader - no description available"]
pub type IC_R = crate::BitReader<IC_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IC_A {
    #[doc = "0: No illegal commands issued"]
    _0 = 0,
    #[doc = "1: Illegal command issued"]
    _1 = 1,
}
impl From<IC_A> for bool {
    #[inline(always)]
    fn from(variant: IC_A) -> Self {
        variant as u8 != 0
    }
}
impl IC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IC_A {
        match self.bits {
            false => IC_A::_0,
            true => IC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IC_A::_1
    }
}
#[doc = "Field `DPE` reader - no description available"]
pub type DPE_R = crate::BitReader<DPE_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPE_A {
    #[doc = "0: No error detected"]
    _0 = 0,
    #[doc = "1: DES key parity error detected"]
    _1 = 1,
}
impl From<DPE_A> for bool {
    #[inline(always)]
    fn from(variant: DPE_A) -> Self {
        variant as u8 != 0
    }
}
impl DPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPE_A {
        match self.bits {
            false => DPE_A::_0,
            true => DPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPE_A::_1
    }
}
#[doc = "Field `VER` reader - CAU version"]
pub type VER_R = crate::FieldReader<u8, VER_A>;
#[doc = "CAU version\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VER_A {
    #[doc = "1: Initial CAU version"]
    _0001 = 1,
    #[doc = "2: Second version, added support for SHA-256 algorithm.(This is the value on this device)"]
    _0010 = 2,
}
impl From<VER_A> for u8 {
    #[inline(always)]
    fn from(variant: VER_A) -> Self {
        variant as _
    }
}
impl VER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VER_A> {
        match self.bits {
            1 => Some(VER_A::_0001),
            2 => Some(VER_A::_0010),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == VER_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0010`"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == VER_A::_0010
    }
}
impl R {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    pub fn ic(&self) -> IC_R {
        IC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    pub fn dpe(&self) -> DPE_R {
        DPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 28:31 - CAU version"]
    #[inline(always)]
    pub fn ver(&self) -> VER_R {
        VER_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "Status register - Store Register command\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cau_str_casr](index.html) module"]
pub struct CAU_STR_CASR_SPEC;
impl crate::RegisterSpec for CAU_STR_CASR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cau_str_casr::R](R) reader structure"]
impl crate::Readable for CAU_STR_CASR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CAU_STR_CASR to value 0x2000_0000"]
impl crate::Resettable for CAU_STR_CASR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_0000;
}
