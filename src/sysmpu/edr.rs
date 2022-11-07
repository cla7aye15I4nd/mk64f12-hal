#[doc = "Register `EDR%s` reader"]
pub struct R(crate::R<EDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ERW` reader - Error Read/Write"]
pub type ERW_R = crate::BitReader<ERW_A>;
#[doc = "Error Read/Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERW_A {
    #[doc = "0: Read"]
    _0 = 0,
    #[doc = "1: Write"]
    _1 = 1,
}
impl From<ERW_A> for bool {
    #[inline(always)]
    fn from(variant: ERW_A) -> Self {
        variant as u8 != 0
    }
}
impl ERW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERW_A {
        match self.bits {
            false => ERW_A::_0,
            true => ERW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERW_A::_1
    }
}
#[doc = "Field `EATTR` reader - Error Attributes"]
pub type EATTR_R = crate::FieldReader<u8, EATTR_A>;
#[doc = "Error Attributes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EATTR_A {
    #[doc = "0: User mode, instruction access"]
    _000 = 0,
    #[doc = "1: User mode, data access"]
    _001 = 1,
    #[doc = "2: Supervisor mode, instruction access"]
    _010 = 2,
    #[doc = "3: Supervisor mode, data access"]
    _011 = 3,
}
impl From<EATTR_A> for u8 {
    #[inline(always)]
    fn from(variant: EATTR_A) -> Self {
        variant as _
    }
}
impl EATTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EATTR_A> {
        match self.bits {
            0 => Some(EATTR_A::_000),
            1 => Some(EATTR_A::_001),
            2 => Some(EATTR_A::_010),
            3 => Some(EATTR_A::_011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == EATTR_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == EATTR_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == EATTR_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == EATTR_A::_011
    }
}
#[doc = "Field `EMN` reader - Error Master Number"]
pub type EMN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPID` reader - Error Process Identification"]
pub type EPID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EACD` reader - Error Access Control Detail"]
pub type EACD_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bit 0 - Error Read/Write"]
    #[inline(always)]
    pub fn erw(&self) -> ERW_R {
        ERW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Error Attributes"]
    #[inline(always)]
    pub fn eattr(&self) -> EATTR_R {
        EATTR_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:7 - Error Master Number"]
    #[inline(always)]
    pub fn emn(&self) -> EMN_R {
        EMN_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - Error Process Identification"]
    #[inline(always)]
    pub fn epid(&self) -> EPID_R {
        EPID_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Error Access Control Detail"]
    #[inline(always)]
    pub fn eacd(&self) -> EACD_R {
        EACD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Error Detail Register, slave port n\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [edr](index.html) module"]
pub struct EDR_SPEC;
impl crate::RegisterSpec for EDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [edr::R](R) reader structure"]
impl crate::Readable for EDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EDR%s to value 0"]
impl crate::Resettable for EDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
