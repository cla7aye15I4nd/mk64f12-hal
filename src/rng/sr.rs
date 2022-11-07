#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SECV` reader - Security Violation"]
pub type SECV_R = crate::BitReader<SECV_A>;
#[doc = "Security Violation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SECV_A {
    #[doc = "0: No security violation"]
    _0 = 0,
    #[doc = "1: Security violation"]
    _1 = 1,
}
impl From<SECV_A> for bool {
    #[inline(always)]
    fn from(variant: SECV_A) -> Self {
        variant as u8 != 0
    }
}
impl SECV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SECV_A {
        match self.bits {
            false => SECV_A::_0,
            true => SECV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SECV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SECV_A::_1
    }
}
#[doc = "Field `LRS` reader - Last Read Status"]
pub type LRS_R = crate::BitReader<LRS_A>;
#[doc = "Last Read Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LRS_A {
    #[doc = "0: No underflow"]
    _0 = 0,
    #[doc = "1: Underflow"]
    _1 = 1,
}
impl From<LRS_A> for bool {
    #[inline(always)]
    fn from(variant: LRS_A) -> Self {
        variant as u8 != 0
    }
}
impl LRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRS_A {
        match self.bits {
            false => LRS_A::_0,
            true => LRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LRS_A::_1
    }
}
#[doc = "Field `ORU` reader - Output Register Underflow"]
pub type ORU_R = crate::BitReader<ORU_A>;
#[doc = "Output Register Underflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORU_A {
    #[doc = "0: No underflow"]
    _0 = 0,
    #[doc = "1: Underflow"]
    _1 = 1,
}
impl From<ORU_A> for bool {
    #[inline(always)]
    fn from(variant: ORU_A) -> Self {
        variant as u8 != 0
    }
}
impl ORU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ORU_A {
        match self.bits {
            false => ORU_A::_0,
            true => ORU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ORU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ORU_A::_1
    }
}
#[doc = "Field `ERRI` reader - Error Interrupt"]
pub type ERRI_R = crate::BitReader<ERRI_A>;
#[doc = "Error Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRI_A {
    #[doc = "0: No underflow"]
    _0 = 0,
    #[doc = "1: Underflow"]
    _1 = 1,
}
impl From<ERRI_A> for bool {
    #[inline(always)]
    fn from(variant: ERRI_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRI_A {
        match self.bits {
            false => ERRI_A::_0,
            true => ERRI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERRI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERRI_A::_1
    }
}
#[doc = "Field `SLP` reader - Sleep"]
pub type SLP_R = crate::BitReader<SLP_A>;
#[doc = "Sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLP_A {
    #[doc = "0: Normal mode"]
    _0 = 0,
    #[doc = "1: Sleep (low-power) mode"]
    _1 = 1,
}
impl From<SLP_A> for bool {
    #[inline(always)]
    fn from(variant: SLP_A) -> Self {
        variant as u8 != 0
    }
}
impl SLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLP_A {
        match self.bits {
            false => SLP_A::_0,
            true => SLP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLP_A::_1
    }
}
#[doc = "Field `OREG_LVL` reader - Output Register Level"]
pub type OREG_LVL_R = crate::FieldReader<u8, OREG_LVL_A>;
#[doc = "Output Register Level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OREG_LVL_A {
    #[doc = "0: No words (empty)"]
    _0 = 0,
    #[doc = "1: One word (valid)"]
    _1 = 1,
}
impl From<OREG_LVL_A> for u8 {
    #[inline(always)]
    fn from(variant: OREG_LVL_A) -> Self {
        variant as _
    }
}
impl OREG_LVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OREG_LVL_A> {
        match self.bits {
            0 => Some(OREG_LVL_A::_0),
            1 => Some(OREG_LVL_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OREG_LVL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OREG_LVL_A::_1
    }
}
#[doc = "Field `OREG_SIZE` reader - Output Register Size"]
pub type OREG_SIZE_R = crate::FieldReader<u8, OREG_SIZE_A>;
#[doc = "Output Register Size\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OREG_SIZE_A {
    #[doc = "1: One word (this value is fixed)"]
    _1 = 1,
}
impl From<OREG_SIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: OREG_SIZE_A) -> Self {
        variant as _
    }
}
impl OREG_SIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OREG_SIZE_A> {
        match self.bits {
            1 => Some(OREG_SIZE_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OREG_SIZE_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Security Violation"]
    #[inline(always)]
    pub fn secv(&self) -> SECV_R {
        SECV_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Last Read Status"]
    #[inline(always)]
    pub fn lrs(&self) -> LRS_R {
        LRS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output Register Underflow"]
    #[inline(always)]
    pub fn oru(&self) -> ORU_R {
        ORU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error Interrupt"]
    #[inline(always)]
    pub fn erri(&self) -> ERRI_R {
        ERRI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sleep"]
    #[inline(always)]
    pub fn slp(&self) -> SLP_R {
        SLP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Output Register Level"]
    #[inline(always)]
    pub fn oreg_lvl(&self) -> OREG_LVL_R {
        OREG_LVL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Output Register Size"]
    #[inline(always)]
    pub fn oreg_size(&self) -> OREG_SIZE_R {
        OREG_SIZE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "RNGA Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0x0001_0000"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
