#[doc = "Register `HTCAPBLT` reader"]
pub struct R(crate::R<HTCAPBLT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HTCAPBLT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HTCAPBLT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HTCAPBLT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MBL` reader - Max Block Length"]
pub type MBL_R = crate::FieldReader<u8, MBL_A>;
#[doc = "Max Block Length\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MBL_A {
    #[doc = "0: 512 bytes"]
    _000 = 0,
    #[doc = "1: 1024 bytes"]
    _001 = 1,
    #[doc = "2: 2048 bytes"]
    _010 = 2,
    #[doc = "3: 4096 bytes"]
    _011 = 3,
}
impl From<MBL_A> for u8 {
    #[inline(always)]
    fn from(variant: MBL_A) -> Self {
        variant as _
    }
}
impl MBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MBL_A> {
        match self.bits {
            0 => Some(MBL_A::_000),
            1 => Some(MBL_A::_001),
            2 => Some(MBL_A::_010),
            3 => Some(MBL_A::_011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == MBL_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == MBL_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == MBL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == MBL_A::_011
    }
}
#[doc = "Field `ADMAS` reader - ADMA Support"]
pub type ADMAS_R = crate::BitReader<ADMAS_A>;
#[doc = "ADMA Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADMAS_A {
    #[doc = "0: Advanced DMA not supported."]
    _0 = 0,
    #[doc = "1: Advanced DMA supported."]
    _1 = 1,
}
impl From<ADMAS_A> for bool {
    #[inline(always)]
    fn from(variant: ADMAS_A) -> Self {
        variant as u8 != 0
    }
}
impl ADMAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADMAS_A {
        match self.bits {
            false => ADMAS_A::_0,
            true => ADMAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADMAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADMAS_A::_1
    }
}
#[doc = "Field `HSS` reader - High Speed Support"]
pub type HSS_R = crate::BitReader<HSS_A>;
#[doc = "High Speed Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HSS_A {
    #[doc = "0: High speed not supported."]
    _0 = 0,
    #[doc = "1: High speed supported."]
    _1 = 1,
}
impl From<HSS_A> for bool {
    #[inline(always)]
    fn from(variant: HSS_A) -> Self {
        variant as u8 != 0
    }
}
impl HSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HSS_A {
        match self.bits {
            false => HSS_A::_0,
            true => HSS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HSS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HSS_A::_1
    }
}
#[doc = "Field `DMAS` reader - DMA Support"]
pub type DMAS_R = crate::BitReader<DMAS_A>;
#[doc = "DMA Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAS_A {
    #[doc = "0: DMA not supported."]
    _0 = 0,
    #[doc = "1: DMA supported."]
    _1 = 1,
}
impl From<DMAS_A> for bool {
    #[inline(always)]
    fn from(variant: DMAS_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAS_A {
        match self.bits {
            false => DMAS_A::_0,
            true => DMAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAS_A::_1
    }
}
#[doc = "Field `SRS` reader - Suspend/Resume Support"]
pub type SRS_R = crate::BitReader<SRS_A>;
#[doc = "Suspend/Resume Support\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRS_A {
    #[doc = "0: Not supported."]
    _0 = 0,
    #[doc = "1: Supported."]
    _1 = 1,
}
impl From<SRS_A> for bool {
    #[inline(always)]
    fn from(variant: SRS_A) -> Self {
        variant as u8 != 0
    }
}
impl SRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRS_A {
        match self.bits {
            false => SRS_A::_0,
            true => SRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRS_A::_1
    }
}
#[doc = "Field `VS33` reader - Voltage Support 3.3 V"]
pub type VS33_R = crate::BitReader<VS33_A>;
#[doc = "Voltage Support 3.3 V\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VS33_A {
    #[doc = "0: 3.3 V not supported."]
    _0 = 0,
    #[doc = "1: 3.3 V supported."]
    _1 = 1,
}
impl From<VS33_A> for bool {
    #[inline(always)]
    fn from(variant: VS33_A) -> Self {
        variant as u8 != 0
    }
}
impl VS33_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VS33_A {
        match self.bits {
            false => VS33_A::_0,
            true => VS33_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VS33_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VS33_A::_1
    }
}
impl R {
    #[doc = "Bits 16:18 - Max Block Length"]
    #[inline(always)]
    pub fn mbl(&self) -> MBL_R {
        MBL_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - ADMA Support"]
    #[inline(always)]
    pub fn admas(&self) -> ADMAS_R {
        ADMAS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - High Speed Support"]
    #[inline(always)]
    pub fn hss(&self) -> HSS_R {
        HSS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMA Support"]
    #[inline(always)]
    pub fn dmas(&self) -> DMAS_R {
        DMAS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Suspend/Resume Support"]
    #[inline(always)]
    pub fn srs(&self) -> SRS_R {
        SRS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Voltage Support 3.3 V"]
    #[inline(always)]
    pub fn vs33(&self) -> VS33_R {
        VS33_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[doc = "Host Controller Capabilities\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [htcapblt](index.html) module"]
pub struct HTCAPBLT_SPEC;
impl crate::RegisterSpec for HTCAPBLT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [htcapblt::R](R) reader structure"]
impl crate::Readable for HTCAPBLT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HTCAPBLT to value 0x07f3_0000"]
impl crate::Resettable for HTCAPBLT_SPEC {
    const RESET_VALUE: Self::Ux = 0x07f3_0000;
}
