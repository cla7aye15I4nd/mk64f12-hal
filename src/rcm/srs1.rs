#[doc = "Register `SRS1` reader"]
pub struct R(crate::R<SRS1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRS1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRS1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRS1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `JTAG` reader - JTAG Generated Reset"]
pub type JTAG_R = crate::BitReader<JTAG_A>;
#[doc = "JTAG Generated Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JTAG_A {
    #[doc = "0: Reset not caused by JTAG"]
    _0 = 0,
    #[doc = "1: Reset caused by JTAG"]
    _1 = 1,
}
impl From<JTAG_A> for bool {
    #[inline(always)]
    fn from(variant: JTAG_A) -> Self {
        variant as u8 != 0
    }
}
impl JTAG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JTAG_A {
        match self.bits {
            false => JTAG_A::_0,
            true => JTAG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == JTAG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == JTAG_A::_1
    }
}
#[doc = "Field `LOCKUP` reader - Core Lockup"]
pub type LOCKUP_R = crate::BitReader<LOCKUP_A>;
#[doc = "Core Lockup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCKUP_A {
    #[doc = "0: Reset not caused by core LOCKUP event"]
    _0 = 0,
    #[doc = "1: Reset caused by core LOCKUP event"]
    _1 = 1,
}
impl From<LOCKUP_A> for bool {
    #[inline(always)]
    fn from(variant: LOCKUP_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCKUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCKUP_A {
        match self.bits {
            false => LOCKUP_A::_0,
            true => LOCKUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOCKUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOCKUP_A::_1
    }
}
#[doc = "Field `SW` reader - Software"]
pub type SW_R = crate::BitReader<SW_A>;
#[doc = "Software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SW_A {
    #[doc = "0: Reset not caused by software setting of SYSRESETREQ bit"]
    _0 = 0,
    #[doc = "1: Reset caused by software setting of SYSRESETREQ bit"]
    _1 = 1,
}
impl From<SW_A> for bool {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as u8 != 0
    }
}
impl SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_A {
        match self.bits {
            false => SW_A::_0,
            true => SW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SW_A::_1
    }
}
#[doc = "Field `MDM_AP` reader - MDM-AP System Reset Request"]
pub type MDM_AP_R = crate::BitReader<MDM_AP_A>;
#[doc = "MDM-AP System Reset Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDM_AP_A {
    #[doc = "0: Reset not caused by host debugger system setting of the System Reset Request bit"]
    _0 = 0,
    #[doc = "1: Reset caused by host debugger system setting of the System Reset Request bit"]
    _1 = 1,
}
impl From<MDM_AP_A> for bool {
    #[inline(always)]
    fn from(variant: MDM_AP_A) -> Self {
        variant as u8 != 0
    }
}
impl MDM_AP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDM_AP_A {
        match self.bits {
            false => MDM_AP_A::_0,
            true => MDM_AP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MDM_AP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MDM_AP_A::_1
    }
}
#[doc = "Field `EZPT` reader - EzPort Reset"]
pub type EZPT_R = crate::BitReader<EZPT_A>;
#[doc = "EzPort Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EZPT_A {
    #[doc = "0: Reset not caused by EzPort receiving the RESET command while the device is in EzPort mode"]
    _0 = 0,
    #[doc = "1: Reset caused by EzPort receiving the RESET command while the device is in EzPort mode"]
    _1 = 1,
}
impl From<EZPT_A> for bool {
    #[inline(always)]
    fn from(variant: EZPT_A) -> Self {
        variant as u8 != 0
    }
}
impl EZPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EZPT_A {
        match self.bits {
            false => EZPT_A::_0,
            true => EZPT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EZPT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EZPT_A::_1
    }
}
#[doc = "Field `SACKERR` reader - Stop Mode Acknowledge Error Reset"]
pub type SACKERR_R = crate::BitReader<SACKERR_A>;
#[doc = "Stop Mode Acknowledge Error Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SACKERR_A {
    #[doc = "0: Reset not caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _0 = 0,
    #[doc = "1: Reset caused by peripheral failure to acknowledge attempt to enter stop mode"]
    _1 = 1,
}
impl From<SACKERR_A> for bool {
    #[inline(always)]
    fn from(variant: SACKERR_A) -> Self {
        variant as u8 != 0
    }
}
impl SACKERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SACKERR_A {
        match self.bits {
            false => SACKERR_A::_0,
            true => SACKERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SACKERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SACKERR_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - JTAG Generated Reset"]
    #[inline(always)]
    pub fn jtag(&self) -> JTAG_R {
        JTAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Core Lockup"]
    #[inline(always)]
    pub fn lockup(&self) -> LOCKUP_R {
        LOCKUP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MDM-AP System Reset Request"]
    #[inline(always)]
    pub fn mdm_ap(&self) -> MDM_AP_R {
        MDM_AP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EzPort Reset"]
    #[inline(always)]
    pub fn ezpt(&self) -> EZPT_R {
        EZPT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop Mode Acknowledge Error Reset"]
    #[inline(always)]
    pub fn sackerr(&self) -> SACKERR_R {
        SACKERR_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "System Reset Status Register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srs1](index.html) module"]
pub struct SRS1_SPEC;
impl crate::RegisterSpec for SRS1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [srs1::R](R) reader structure"]
impl crate::Readable for SRS1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SRS1 to value 0"]
impl crate::Resettable for SRS1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
