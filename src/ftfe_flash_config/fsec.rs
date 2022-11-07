#[doc = "Register `FSEC` reader"]
pub struct R(crate::R<FSEC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSEC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSEC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSEC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SEC` reader - Flash Security"]
pub type SEC_R = crate::FieldReader<u8, SEC_A>;
#[doc = "Flash Security\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SEC_A {
    #[doc = "2: MCU security status is unsecure"]
    _10 = 2,
    #[doc = "3: MCU security status is secure"]
    _11 = 3,
}
impl From<SEC_A> for u8 {
    #[inline(always)]
    fn from(variant: SEC_A) -> Self {
        variant as _
    }
}
impl SEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SEC_A> {
        match self.bits {
            2 => Some(SEC_A::_10),
            3 => Some(SEC_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SEC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SEC_A::_11
    }
}
#[doc = "Field `FSLACC` reader - Freescale Failure Analysis Access Code"]
pub type FSLACC_R = crate::FieldReader<u8, FSLACC_A>;
#[doc = "Freescale Failure Analysis Access Code\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FSLACC_A {
    #[doc = "2: Freescale factory access denied"]
    _10 = 2,
    #[doc = "3: Freescale factory access granted"]
    _11 = 3,
}
impl From<FSLACC_A> for u8 {
    #[inline(always)]
    fn from(variant: FSLACC_A) -> Self {
        variant as _
    }
}
impl FSLACC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FSLACC_A> {
        match self.bits {
            2 => Some(FSLACC_A::_10),
            3 => Some(FSLACC_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FSLACC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FSLACC_A::_11
    }
}
#[doc = "Field `MEEN` reader - no description available"]
pub type MEEN_R = crate::FieldReader<u8, MEEN_A>;
#[doc = "no description available\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MEEN_A {
    #[doc = "2: Mass erase is disabled"]
    _10 = 2,
    #[doc = "3: Mass erase is enabled"]
    _11 = 3,
}
impl From<MEEN_A> for u8 {
    #[inline(always)]
    fn from(variant: MEEN_A) -> Self {
        variant as _
    }
}
impl MEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MEEN_A> {
        match self.bits {
            2 => Some(MEEN_A::_10),
            3 => Some(MEEN_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MEEN_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MEEN_A::_11
    }
}
#[doc = "Field `KEYEN` reader - Backdoor Key Security Enable"]
pub type KEYEN_R = crate::FieldReader<u8, KEYEN_A>;
#[doc = "Backdoor Key Security Enable\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KEYEN_A {
    #[doc = "2: Backdoor key access enabled"]
    _10 = 2,
    #[doc = "3: Backdoor key access disabled"]
    _11 = 3,
}
impl From<KEYEN_A> for u8 {
    #[inline(always)]
    fn from(variant: KEYEN_A) -> Self {
        variant as _
    }
}
impl KEYEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KEYEN_A> {
        match self.bits {
            2 => Some(KEYEN_A::_10),
            3 => Some(KEYEN_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == KEYEN_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == KEYEN_A::_11
    }
}
impl R {
    #[doc = "Bits 0:1 - Flash Security"]
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - Freescale Failure Analysis Access Code"]
    #[inline(always)]
    pub fn fslacc(&self) -> FSLACC_R {
        FSLACC_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bits 4:5 - no description available"]
    #[inline(always)]
    pub fn meen(&self) -> MEEN_R {
        MEEN_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bits 6:7 - Backdoor Key Security Enable"]
    #[inline(always)]
    pub fn keyen(&self) -> KEYEN_R {
        KEYEN_R::new((self.bits >> 6) & 3)
    }
}
#[doc = "Non-volatile Flash Security Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fsec](index.html) module"]
pub struct FSEC_SPEC;
impl crate::RegisterSpec for FSEC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fsec::R](R) reader structure"]
impl crate::Readable for FSEC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FSEC to value 0xff"]
impl crate::Resettable for FSEC_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
