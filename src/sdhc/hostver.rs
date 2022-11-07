#[doc = "Register `HOSTVER` reader"]
pub struct R(crate::R<HOSTVER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HOSTVER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HOSTVER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HOSTVER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SVN` reader - Specification Version Number"]
pub type SVN_R = crate::FieldReader<u8, SVN_A>;
#[doc = "Specification Version Number\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SVN_A {
    #[doc = "1: SD host specification version 2.0, supports test event register and ADMA."]
    _1 = 1,
}
impl From<SVN_A> for u8 {
    #[inline(always)]
    fn from(variant: SVN_A) -> Self {
        variant as _
    }
}
impl SVN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SVN_A> {
        match self.bits {
            1 => Some(SVN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SVN_A::_1
    }
}
#[doc = "Field `VVN` reader - Vendor Version Number"]
pub type VVN_R = crate::FieldReader<u8, VVN_A>;
#[doc = "Vendor Version Number\n\nValue on reset: 18"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VVN_A {
    #[doc = "0: Freescale SDHC version 1.0"]
    _0 = 0,
    #[doc = "16: Freescale SDHC version 2.0"]
    _10000 = 16,
    #[doc = "17: Freescale SDHC version 2.1"]
    _10001 = 17,
    #[doc = "18: Freescale SDHC version 2.2"]
    _10010 = 18,
}
impl From<VVN_A> for u8 {
    #[inline(always)]
    fn from(variant: VVN_A) -> Self {
        variant as _
    }
}
impl VVN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VVN_A> {
        match self.bits {
            0 => Some(VVN_A::_0),
            16 => Some(VVN_A::_10000),
            17 => Some(VVN_A::_10001),
            18 => Some(VVN_A::_10010),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VVN_A::_0
    }
    #[doc = "Checks if the value of the field is `_10000`"]
    #[inline(always)]
    pub fn is_10000(&self) -> bool {
        *self == VVN_A::_10000
    }
    #[doc = "Checks if the value of the field is `_10001`"]
    #[inline(always)]
    pub fn is_10001(&self) -> bool {
        *self == VVN_A::_10001
    }
    #[doc = "Checks if the value of the field is `_10010`"]
    #[inline(always)]
    pub fn is_10010(&self) -> bool {
        *self == VVN_A::_10010
    }
}
impl R {
    #[doc = "Bits 0:7 - Specification Version Number"]
    #[inline(always)]
    pub fn svn(&self) -> SVN_R {
        SVN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Vendor Version Number"]
    #[inline(always)]
    pub fn vvn(&self) -> VVN_R {
        VVN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Host Controller Version\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hostver](index.html) module"]
pub struct HOSTVER_SPEC;
impl crate::RegisterSpec for HOSTVER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hostver::R](R) reader structure"]
impl crate::Readable for HOSTVER_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HOSTVER to value 0x1201"]
impl crate::Resettable for HOSTVER_SPEC {
    const RESET_VALUE: Self::Ux = 0x1201;
}
