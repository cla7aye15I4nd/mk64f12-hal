#[doc = "Register `PRSSTAT` reader"]
pub struct R(crate::R<PRSSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CIHB` reader - Command Inhibit (CMD)"]
pub type CIHB_R = crate::BitReader<CIHB_A>;
#[doc = "Command Inhibit (CMD)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIHB_A {
    #[doc = "0: Can issue command using only CMD line."]
    _0 = 0,
    #[doc = "1: Cannot issue command."]
    _1 = 1,
}
impl From<CIHB_A> for bool {
    #[inline(always)]
    fn from(variant: CIHB_A) -> Self {
        variant as u8 != 0
    }
}
impl CIHB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIHB_A {
        match self.bits {
            false => CIHB_A::_0,
            true => CIHB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CIHB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CIHB_A::_1
    }
}
#[doc = "Field `CDIHB` reader - Command Inhibit (DAT)"]
pub type CDIHB_R = crate::BitReader<CDIHB_A>;
#[doc = "Command Inhibit (DAT)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CDIHB_A {
    #[doc = "0: Can issue command which uses the DAT line."]
    _0 = 0,
    #[doc = "1: Cannot issue command which uses the DAT line."]
    _1 = 1,
}
impl From<CDIHB_A> for bool {
    #[inline(always)]
    fn from(variant: CDIHB_A) -> Self {
        variant as u8 != 0
    }
}
impl CDIHB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CDIHB_A {
        match self.bits {
            false => CDIHB_A::_0,
            true => CDIHB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CDIHB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CDIHB_A::_1
    }
}
#[doc = "Field `DLA` reader - Data Line Active"]
pub type DLA_R = crate::BitReader<DLA_A>;
#[doc = "Data Line Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLA_A {
    #[doc = "0: DAT line inactive."]
    _0 = 0,
    #[doc = "1: DAT line active."]
    _1 = 1,
}
impl From<DLA_A> for bool {
    #[inline(always)]
    fn from(variant: DLA_A) -> Self {
        variant as u8 != 0
    }
}
impl DLA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DLA_A {
        match self.bits {
            false => DLA_A::_0,
            true => DLA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DLA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DLA_A::_1
    }
}
#[doc = "Field `SDSTB` reader - SD Clock Stable"]
pub type SDSTB_R = crate::BitReader<SDSTB_A>;
#[doc = "SD Clock Stable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDSTB_A {
    #[doc = "0: Clock is changing frequency and not stable."]
    _0 = 0,
    #[doc = "1: Clock is stable."]
    _1 = 1,
}
impl From<SDSTB_A> for bool {
    #[inline(always)]
    fn from(variant: SDSTB_A) -> Self {
        variant as u8 != 0
    }
}
impl SDSTB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDSTB_A {
        match self.bits {
            false => SDSTB_A::_0,
            true => SDSTB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDSTB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDSTB_A::_1
    }
}
#[doc = "Field `IPGOFF` reader - Bus Clock Gated Off Internally"]
pub type IPGOFF_R = crate::BitReader<IPGOFF_A>;
#[doc = "Bus Clock Gated Off Internally\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPGOFF_A {
    #[doc = "0: Bus clock is active."]
    _0 = 0,
    #[doc = "1: Bus clock is gated off."]
    _1 = 1,
}
impl From<IPGOFF_A> for bool {
    #[inline(always)]
    fn from(variant: IPGOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl IPGOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPGOFF_A {
        match self.bits {
            false => IPGOFF_A::_0,
            true => IPGOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPGOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPGOFF_A::_1
    }
}
#[doc = "Field `HCKOFF` reader - System Clock Gated Off Internally"]
pub type HCKOFF_R = crate::BitReader<HCKOFF_A>;
#[doc = "System Clock Gated Off Internally\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HCKOFF_A {
    #[doc = "0: System clock is active."]
    _0 = 0,
    #[doc = "1: System clock is gated off."]
    _1 = 1,
}
impl From<HCKOFF_A> for bool {
    #[inline(always)]
    fn from(variant: HCKOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl HCKOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HCKOFF_A {
        match self.bits {
            false => HCKOFF_A::_0,
            true => HCKOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HCKOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HCKOFF_A::_1
    }
}
#[doc = "Field `PEROFF` reader - SDHC clock Gated Off Internally"]
pub type PEROFF_R = crate::BitReader<PEROFF_A>;
#[doc = "SDHC clock Gated Off Internally\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEROFF_A {
    #[doc = "0: SDHC clock is active."]
    _0 = 0,
    #[doc = "1: SDHC clock is gated off."]
    _1 = 1,
}
impl From<PEROFF_A> for bool {
    #[inline(always)]
    fn from(variant: PEROFF_A) -> Self {
        variant as u8 != 0
    }
}
impl PEROFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEROFF_A {
        match self.bits {
            false => PEROFF_A::_0,
            true => PEROFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PEROFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PEROFF_A::_1
    }
}
#[doc = "Field `SDOFF` reader - SD Clock Gated Off Internally"]
pub type SDOFF_R = crate::BitReader<SDOFF_A>;
#[doc = "SD Clock Gated Off Internally\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDOFF_A {
    #[doc = "0: SD clock is active."]
    _0 = 0,
    #[doc = "1: SD clock is gated off."]
    _1 = 1,
}
impl From<SDOFF_A> for bool {
    #[inline(always)]
    fn from(variant: SDOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl SDOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDOFF_A {
        match self.bits {
            false => SDOFF_A::_0,
            true => SDOFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDOFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDOFF_A::_1
    }
}
#[doc = "Field `WTA` reader - Write Transfer Active"]
pub type WTA_R = crate::BitReader<WTA_A>;
#[doc = "Write Transfer Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WTA_A {
    #[doc = "0: No valid data."]
    _0 = 0,
    #[doc = "1: Transferring data."]
    _1 = 1,
}
impl From<WTA_A> for bool {
    #[inline(always)]
    fn from(variant: WTA_A) -> Self {
        variant as u8 != 0
    }
}
impl WTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTA_A {
        match self.bits {
            false => WTA_A::_0,
            true => WTA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WTA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WTA_A::_1
    }
}
#[doc = "Field `RTA` reader - Read Transfer Active"]
pub type RTA_R = crate::BitReader<RTA_A>;
#[doc = "Read Transfer Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTA_A {
    #[doc = "0: No valid data."]
    _0 = 0,
    #[doc = "1: Transferring data."]
    _1 = 1,
}
impl From<RTA_A> for bool {
    #[inline(always)]
    fn from(variant: RTA_A) -> Self {
        variant as u8 != 0
    }
}
impl RTA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTA_A {
        match self.bits {
            false => RTA_A::_0,
            true => RTA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RTA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RTA_A::_1
    }
}
#[doc = "Field `BWEN` reader - Buffer Write Enable"]
pub type BWEN_R = crate::BitReader<BWEN_A>;
#[doc = "Buffer Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWEN_A {
    #[doc = "0: Write disable, the buffer can hold valid data less than the write watermark level."]
    _0 = 0,
    #[doc = "1: Write enable, the buffer can hold valid data greater than the write watermark level."]
    _1 = 1,
}
impl From<BWEN_A> for bool {
    #[inline(always)]
    fn from(variant: BWEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BWEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWEN_A {
        match self.bits {
            false => BWEN_A::_0,
            true => BWEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BWEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BWEN_A::_1
    }
}
#[doc = "Field `BREN` reader - Buffer Read Enable"]
pub type BREN_R = crate::BitReader<BREN_A>;
#[doc = "Buffer Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BREN_A {
    #[doc = "0: Read disable, valid data less than the watermark level exist in the buffer."]
    _0 = 0,
    #[doc = "1: Read enable, valid data greater than the watermark level exist in the buffer."]
    _1 = 1,
}
impl From<BREN_A> for bool {
    #[inline(always)]
    fn from(variant: BREN_A) -> Self {
        variant as u8 != 0
    }
}
impl BREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BREN_A {
        match self.bits {
            false => BREN_A::_0,
            true => BREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BREN_A::_1
    }
}
#[doc = "Field `CINS` reader - Card Inserted"]
pub type CINS_R = crate::BitReader<CINS_A>;
#[doc = "Card Inserted\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINS_A {
    #[doc = "0: Power on reset or no card."]
    _0 = 0,
    #[doc = "1: Card inserted."]
    _1 = 1,
}
impl From<CINS_A> for bool {
    #[inline(always)]
    fn from(variant: CINS_A) -> Self {
        variant as u8 != 0
    }
}
impl CINS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINS_A {
        match self.bits {
            false => CINS_A::_0,
            true => CINS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CINS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CINS_A::_1
    }
}
#[doc = "Field `CLSL` reader - CMD Line Signal Level"]
pub type CLSL_R = crate::BitReader<bool>;
#[doc = "Field `DLSL` reader - DAT Line Signal Level"]
pub type DLSL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Command Inhibit (CMD)"]
    #[inline(always)]
    pub fn cihb(&self) -> CIHB_R {
        CIHB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command Inhibit (DAT)"]
    #[inline(always)]
    pub fn cdihb(&self) -> CDIHB_R {
        CDIHB_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Line Active"]
    #[inline(always)]
    pub fn dla(&self) -> DLA_R {
        DLA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SD Clock Stable"]
    #[inline(always)]
    pub fn sdstb(&self) -> SDSTB_R {
        SDSTB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bus Clock Gated Off Internally"]
    #[inline(always)]
    pub fn ipgoff(&self) -> IPGOFF_R {
        IPGOFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - System Clock Gated Off Internally"]
    #[inline(always)]
    pub fn hckoff(&self) -> HCKOFF_R {
        HCKOFF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SDHC clock Gated Off Internally"]
    #[inline(always)]
    pub fn peroff(&self) -> PEROFF_R {
        PEROFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SD Clock Gated Off Internally"]
    #[inline(always)]
    pub fn sdoff(&self) -> SDOFF_R {
        SDOFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write Transfer Active"]
    #[inline(always)]
    pub fn wta(&self) -> WTA_R {
        WTA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read Transfer Active"]
    #[inline(always)]
    pub fn rta(&self) -> RTA_R {
        RTA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Buffer Write Enable"]
    #[inline(always)]
    pub fn bwen(&self) -> BWEN_R {
        BWEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Buffer Read Enable"]
    #[inline(always)]
    pub fn bren(&self) -> BREN_R {
        BREN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Card Inserted"]
    #[inline(always)]
    pub fn cins(&self) -> CINS_R {
        CINS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 23 - CMD Line Signal Level"]
    #[inline(always)]
    pub fn clsl(&self) -> CLSL_R {
        CLSL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - DAT Line Signal Level"]
    #[inline(always)]
    pub fn dlsl(&self) -> DLSL_R {
        DLSL_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Present State register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prsstat](index.html) module"]
pub struct PRSSTAT_SPEC;
impl crate::RegisterSpec for PRSSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prsstat::R](R) reader structure"]
impl crate::Readable for PRSSTAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PRSSTAT to value 0"]
impl crate::Resettable for PRSSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
