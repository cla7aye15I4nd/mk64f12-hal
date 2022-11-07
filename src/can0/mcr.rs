#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAXMB` reader - Number Of The Last Message Buffer"]
pub type MAXMB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAXMB` writer - Number Of The Last Message Buffer"]
pub type MAXMB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR_SPEC, u8, u8, 7, O>;
#[doc = "Field `IDAM` reader - ID Acceptance Mode"]
pub type IDAM_R = crate::FieldReader<u8, IDAM_A>;
#[doc = "ID Acceptance Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDAM_A {
    #[doc = "0: Format A: One full ID (standard and extended) per ID Filter Table element."]
    _00 = 0,
    #[doc = "1: Format B: Two full standard IDs or two partial 14-bit (standard and extended) IDs per ID Filter Table element."]
    _01 = 1,
    #[doc = "2: Format C: Four partial 8-bit Standard IDs per ID Filter Table element."]
    _10 = 2,
    #[doc = "3: Format D: All frames rejected."]
    _11 = 3,
}
impl From<IDAM_A> for u8 {
    #[inline(always)]
    fn from(variant: IDAM_A) -> Self {
        variant as _
    }
}
impl IDAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDAM_A {
        match self.bits {
            0 => IDAM_A::_00,
            1 => IDAM_A::_01,
            2 => IDAM_A::_10,
            3 => IDAM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == IDAM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == IDAM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == IDAM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == IDAM_A::_11
    }
}
#[doc = "Field `IDAM` writer - ID Acceptance Mode"]
pub type IDAM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MCR_SPEC, u8, IDAM_A, 2, O>;
impl<'a, const O: u8> IDAM_W<'a, O> {
    #[doc = "Format A: One full ID (standard and extended) per ID Filter Table element."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(IDAM_A::_00)
    }
    #[doc = "Format B: Two full standard IDs or two partial 14-bit (standard and extended) IDs per ID Filter Table element."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(IDAM_A::_01)
    }
    #[doc = "Format C: Four partial 8-bit Standard IDs per ID Filter Table element."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(IDAM_A::_10)
    }
    #[doc = "Format D: All frames rejected."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(IDAM_A::_11)
    }
}
#[doc = "Field `AEN` reader - Abort Enable"]
pub type AEN_R = crate::BitReader<AEN_A>;
#[doc = "Abort Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AEN_A {
    #[doc = "0: Abort disabled."]
    _0 = 0,
    #[doc = "1: Abort enabled."]
    _1 = 1,
}
impl From<AEN_A> for bool {
    #[inline(always)]
    fn from(variant: AEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AEN_A {
        match self.bits {
            false => AEN_A::_0,
            true => AEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AEN_A::_1
    }
}
#[doc = "Field `AEN` writer - Abort Enable"]
pub type AEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, AEN_A, O>;
impl<'a, const O: u8> AEN_W<'a, O> {
    #[doc = "Abort disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AEN_A::_0)
    }
    #[doc = "Abort enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AEN_A::_1)
    }
}
#[doc = "Field `LPRIOEN` reader - Local Priority Enable"]
pub type LPRIOEN_R = crate::BitReader<LPRIOEN_A>;
#[doc = "Local Priority Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPRIOEN_A {
    #[doc = "0: Local Priority disabled."]
    _0 = 0,
    #[doc = "1: Local Priority enabled."]
    _1 = 1,
}
impl From<LPRIOEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPRIOEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LPRIOEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPRIOEN_A {
        match self.bits {
            false => LPRIOEN_A::_0,
            true => LPRIOEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPRIOEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPRIOEN_A::_1
    }
}
#[doc = "Field `LPRIOEN` writer - Local Priority Enable"]
pub type LPRIOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, LPRIOEN_A, O>;
impl<'a, const O: u8> LPRIOEN_W<'a, O> {
    #[doc = "Local Priority disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPRIOEN_A::_0)
    }
    #[doc = "Local Priority enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPRIOEN_A::_1)
    }
}
#[doc = "Field `IRMQ` reader - Individual Rx Masking And Queue Enable"]
pub type IRMQ_R = crate::BitReader<IRMQ_A>;
#[doc = "Individual Rx Masking And Queue Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRMQ_A {
    #[doc = "0: Individual Rx masking and queue feature are disabled. For backward compatibility with legacy applications, the reading of C/S word locks the MB even if it is EMPTY."]
    _0 = 0,
    #[doc = "1: Individual Rx masking and queue feature are enabled."]
    _1 = 1,
}
impl From<IRMQ_A> for bool {
    #[inline(always)]
    fn from(variant: IRMQ_A) -> Self {
        variant as u8 != 0
    }
}
impl IRMQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRMQ_A {
        match self.bits {
            false => IRMQ_A::_0,
            true => IRMQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRMQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRMQ_A::_1
    }
}
#[doc = "Field `IRMQ` writer - Individual Rx Masking And Queue Enable"]
pub type IRMQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, IRMQ_A, O>;
impl<'a, const O: u8> IRMQ_W<'a, O> {
    #[doc = "Individual Rx masking and queue feature are disabled. For backward compatibility with legacy applications, the reading of C/S word locks the MB even if it is EMPTY."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRMQ_A::_0)
    }
    #[doc = "Individual Rx masking and queue feature are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRMQ_A::_1)
    }
}
#[doc = "Field `SRXDIS` reader - Self Reception Disable"]
pub type SRXDIS_R = crate::BitReader<SRXDIS_A>;
#[doc = "Self Reception Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRXDIS_A {
    #[doc = "0: Self reception enabled."]
    _0 = 0,
    #[doc = "1: Self reception disabled."]
    _1 = 1,
}
impl From<SRXDIS_A> for bool {
    #[inline(always)]
    fn from(variant: SRXDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl SRXDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRXDIS_A {
        match self.bits {
            false => SRXDIS_A::_0,
            true => SRXDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRXDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRXDIS_A::_1
    }
}
#[doc = "Field `SRXDIS` writer - Self Reception Disable"]
pub type SRXDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, SRXDIS_A, O>;
impl<'a, const O: u8> SRXDIS_W<'a, O> {
    #[doc = "Self reception enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRXDIS_A::_0)
    }
    #[doc = "Self reception disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRXDIS_A::_1)
    }
}
#[doc = "Field `WAKSRC` reader - Wake Up Source"]
pub type WAKSRC_R = crate::BitReader<WAKSRC_A>;
#[doc = "Wake Up Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKSRC_A {
    #[doc = "0: FlexCAN uses the unfiltered Rx input to detect recessive to dominant edges on the CAN bus."]
    _0 = 0,
    #[doc = "1: FlexCAN uses the filtered Rx input to detect recessive to dominant edges on the CAN bus."]
    _1 = 1,
}
impl From<WAKSRC_A> for bool {
    #[inline(always)]
    fn from(variant: WAKSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKSRC_A {
        match self.bits {
            false => WAKSRC_A::_0,
            true => WAKSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WAKSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAKSRC_A::_1
    }
}
#[doc = "Field `WAKSRC` writer - Wake Up Source"]
pub type WAKSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, WAKSRC_A, O>;
impl<'a, const O: u8> WAKSRC_W<'a, O> {
    #[doc = "FlexCAN uses the unfiltered Rx input to detect recessive to dominant edges on the CAN bus."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKSRC_A::_0)
    }
    #[doc = "FlexCAN uses the filtered Rx input to detect recessive to dominant edges on the CAN bus."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKSRC_A::_1)
    }
}
#[doc = "Field `LPMACK` reader - Low-Power Mode Acknowledge"]
pub type LPMACK_R = crate::BitReader<LPMACK_A>;
#[doc = "Low-Power Mode Acknowledge\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPMACK_A {
    #[doc = "0: FlexCAN is not in a low-power mode."]
    _0 = 0,
    #[doc = "1: FlexCAN is in a low-power mode."]
    _1 = 1,
}
impl From<LPMACK_A> for bool {
    #[inline(always)]
    fn from(variant: LPMACK_A) -> Self {
        variant as u8 != 0
    }
}
impl LPMACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPMACK_A {
        match self.bits {
            false => LPMACK_A::_0,
            true => LPMACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPMACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPMACK_A::_1
    }
}
#[doc = "Field `WRNEN` reader - Warning Interrupt Enable"]
pub type WRNEN_R = crate::BitReader<WRNEN_A>;
#[doc = "Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRNEN_A {
    #[doc = "0: TWRNINT and RWRNINT bits are zero, independent of the values in the error counters."]
    _0 = 0,
    #[doc = "1: TWRNINT and RWRNINT bits are set when the respective error counter transitions from less than 96 to greater than or equal to 96."]
    _1 = 1,
}
impl From<WRNEN_A> for bool {
    #[inline(always)]
    fn from(variant: WRNEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WRNEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRNEN_A {
        match self.bits {
            false => WRNEN_A::_0,
            true => WRNEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WRNEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WRNEN_A::_1
    }
}
#[doc = "Field `WRNEN` writer - Warning Interrupt Enable"]
pub type WRNEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, WRNEN_A, O>;
impl<'a, const O: u8> WRNEN_W<'a, O> {
    #[doc = "TWRNINT and RWRNINT bits are zero, independent of the values in the error counters."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WRNEN_A::_0)
    }
    #[doc = "TWRNINT and RWRNINT bits are set when the respective error counter transitions from less than 96 to greater than or equal to 96."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WRNEN_A::_1)
    }
}
#[doc = "Field `SLFWAK` reader - Self Wake Up"]
pub type SLFWAK_R = crate::BitReader<SLFWAK_A>;
#[doc = "Self Wake Up\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLFWAK_A {
    #[doc = "0: FlexCAN Self Wake Up feature is disabled."]
    _0 = 0,
    #[doc = "1: FlexCAN Self Wake Up feature is enabled."]
    _1 = 1,
}
impl From<SLFWAK_A> for bool {
    #[inline(always)]
    fn from(variant: SLFWAK_A) -> Self {
        variant as u8 != 0
    }
}
impl SLFWAK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLFWAK_A {
        match self.bits {
            false => SLFWAK_A::_0,
            true => SLFWAK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLFWAK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLFWAK_A::_1
    }
}
#[doc = "Field `SLFWAK` writer - Self Wake Up"]
pub type SLFWAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, SLFWAK_A, O>;
impl<'a, const O: u8> SLFWAK_W<'a, O> {
    #[doc = "FlexCAN Self Wake Up feature is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLFWAK_A::_0)
    }
    #[doc = "FlexCAN Self Wake Up feature is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLFWAK_A::_1)
    }
}
#[doc = "Field `SUPV` reader - Supervisor Mode"]
pub type SUPV_R = crate::BitReader<SUPV_A>;
#[doc = "Supervisor Mode\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUPV_A {
    #[doc = "0: FlexCAN is in User mode. Affected registers allow both Supervisor and Unrestricted accesses ."]
    _0 = 0,
    #[doc = "1: FlexCAN is in Supervisor mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location ."]
    _1 = 1,
}
impl From<SUPV_A> for bool {
    #[inline(always)]
    fn from(variant: SUPV_A) -> Self {
        variant as u8 != 0
    }
}
impl SUPV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUPV_A {
        match self.bits {
            false => SUPV_A::_0,
            true => SUPV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SUPV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SUPV_A::_1
    }
}
#[doc = "Field `SUPV` writer - Supervisor Mode"]
pub type SUPV_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, SUPV_A, O>;
impl<'a, const O: u8> SUPV_W<'a, O> {
    #[doc = "FlexCAN is in User mode. Affected registers allow both Supervisor and Unrestricted accesses ."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUPV_A::_0)
    }
    #[doc = "FlexCAN is in Supervisor mode. Affected registers allow only Supervisor access. Unrestricted access behaves as though the access was done to an unimplemented register location ."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUPV_A::_1)
    }
}
#[doc = "Field `FRZACK` reader - Freeze Mode Acknowledge"]
pub type FRZACK_R = crate::BitReader<FRZACK_A>;
#[doc = "Freeze Mode Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRZACK_A {
    #[doc = "0: FlexCAN not in Freeze mode, prescaler running."]
    _0 = 0,
    #[doc = "1: FlexCAN in Freeze mode, prescaler stopped."]
    _1 = 1,
}
impl From<FRZACK_A> for bool {
    #[inline(always)]
    fn from(variant: FRZACK_A) -> Self {
        variant as u8 != 0
    }
}
impl FRZACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRZACK_A {
        match self.bits {
            false => FRZACK_A::_0,
            true => FRZACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRZACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRZACK_A::_1
    }
}
#[doc = "Field `SOFTRST` reader - Soft Reset"]
pub type SOFTRST_R = crate::BitReader<SOFTRST_A>;
#[doc = "Soft Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFTRST_A {
    #[doc = "0: No reset request."]
    _0 = 0,
    #[doc = "1: Resets the registers affected by soft reset."]
    _1 = 1,
}
impl From<SOFTRST_A> for bool {
    #[inline(always)]
    fn from(variant: SOFTRST_A) -> Self {
        variant as u8 != 0
    }
}
impl SOFTRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFTRST_A {
        match self.bits {
            false => SOFTRST_A::_0,
            true => SOFTRST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOFTRST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOFTRST_A::_1
    }
}
#[doc = "Field `SOFTRST` writer - Soft Reset"]
pub type SOFTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, SOFTRST_A, O>;
impl<'a, const O: u8> SOFTRST_W<'a, O> {
    #[doc = "No reset request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOFTRST_A::_0)
    }
    #[doc = "Resets the registers affected by soft reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOFTRST_A::_1)
    }
}
#[doc = "Field `WAKMSK` reader - Wake Up Interrupt Mask"]
pub type WAKMSK_R = crate::BitReader<WAKMSK_A>;
#[doc = "Wake Up Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKMSK_A {
    #[doc = "0: Wake Up Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Wake Up Interrupt is enabled."]
    _1 = 1,
}
impl From<WAKMSK_A> for bool {
    #[inline(always)]
    fn from(variant: WAKMSK_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKMSK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKMSK_A {
        match self.bits {
            false => WAKMSK_A::_0,
            true => WAKMSK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WAKMSK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAKMSK_A::_1
    }
}
#[doc = "Field `WAKMSK` writer - Wake Up Interrupt Mask"]
pub type WAKMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, WAKMSK_A, O>;
impl<'a, const O: u8> WAKMSK_W<'a, O> {
    #[doc = "Wake Up Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKMSK_A::_0)
    }
    #[doc = "Wake Up Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKMSK_A::_1)
    }
}
#[doc = "Field `NOTRDY` reader - FlexCAN Not Ready"]
pub type NOTRDY_R = crate::BitReader<NOTRDY_A>;
#[doc = "FlexCAN Not Ready\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOTRDY_A {
    #[doc = "0: FlexCAN module is either in Normal mode, Listen-Only mode or Loop-Back mode."]
    _0 = 0,
    #[doc = "1: FlexCAN module is either in Disable mode , Stop mode or Freeze mode."]
    _1 = 1,
}
impl From<NOTRDY_A> for bool {
    #[inline(always)]
    fn from(variant: NOTRDY_A) -> Self {
        variant as u8 != 0
    }
}
impl NOTRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOTRDY_A {
        match self.bits {
            false => NOTRDY_A::_0,
            true => NOTRDY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NOTRDY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NOTRDY_A::_1
    }
}
#[doc = "Field `HALT` reader - Halt FlexCAN"]
pub type HALT_R = crate::BitReader<HALT_A>;
#[doc = "Halt FlexCAN\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALT_A {
    #[doc = "0: No Freeze mode request."]
    _0 = 0,
    #[doc = "1: Enters Freeze mode if the FRZ bit is asserted."]
    _1 = 1,
}
impl From<HALT_A> for bool {
    #[inline(always)]
    fn from(variant: HALT_A) -> Self {
        variant as u8 != 0
    }
}
impl HALT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALT_A {
        match self.bits {
            false => HALT_A::_0,
            true => HALT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HALT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HALT_A::_1
    }
}
#[doc = "Field `HALT` writer - Halt FlexCAN"]
pub type HALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, HALT_A, O>;
impl<'a, const O: u8> HALT_W<'a, O> {
    #[doc = "No Freeze mode request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HALT_A::_0)
    }
    #[doc = "Enters Freeze mode if the FRZ bit is asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HALT_A::_1)
    }
}
#[doc = "Field `RFEN` reader - Rx FIFO Enable"]
pub type RFEN_R = crate::BitReader<RFEN_A>;
#[doc = "Rx FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFEN_A {
    #[doc = "0: Rx FIFO not enabled."]
    _0 = 0,
    #[doc = "1: Rx FIFO enabled."]
    _1 = 1,
}
impl From<RFEN_A> for bool {
    #[inline(always)]
    fn from(variant: RFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFEN_A {
        match self.bits {
            false => RFEN_A::_0,
            true => RFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFEN_A::_1
    }
}
#[doc = "Field `RFEN` writer - Rx FIFO Enable"]
pub type RFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, RFEN_A, O>;
impl<'a, const O: u8> RFEN_W<'a, O> {
    #[doc = "Rx FIFO not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFEN_A::_0)
    }
    #[doc = "Rx FIFO enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFEN_A::_1)
    }
}
#[doc = "Field `FRZ` reader - Freeze Enable"]
pub type FRZ_R = crate::BitReader<FRZ_A>;
#[doc = "Freeze Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRZ_A {
    #[doc = "0: Not enabled to enter Freeze mode."]
    _0 = 0,
    #[doc = "1: Enabled to enter Freeze mode."]
    _1 = 1,
}
impl From<FRZ_A> for bool {
    #[inline(always)]
    fn from(variant: FRZ_A) -> Self {
        variant as u8 != 0
    }
}
impl FRZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRZ_A {
        match self.bits {
            false => FRZ_A::_0,
            true => FRZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRZ_A::_1
    }
}
#[doc = "Field `FRZ` writer - Freeze Enable"]
pub type FRZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, FRZ_A, O>;
impl<'a, const O: u8> FRZ_W<'a, O> {
    #[doc = "Not enabled to enter Freeze mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRZ_A::_0)
    }
    #[doc = "Enabled to enter Freeze mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRZ_A::_1)
    }
}
#[doc = "Field `MDIS` reader - Module Disable"]
pub type MDIS_R = crate::BitReader<MDIS_A>;
#[doc = "Module Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDIS_A {
    #[doc = "0: Enable the FlexCAN module."]
    _0 = 0,
    #[doc = "1: Disable the FlexCAN module."]
    _1 = 1,
}
impl From<MDIS_A> for bool {
    #[inline(always)]
    fn from(variant: MDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl MDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIS_A {
        match self.bits {
            false => MDIS_A::_0,
            true => MDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MDIS_A::_1
    }
}
#[doc = "Field `MDIS` writer - Module Disable"]
pub type MDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MDIS_A, O>;
impl<'a, const O: u8> MDIS_W<'a, O> {
    #[doc = "Enable the FlexCAN module."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MDIS_A::_0)
    }
    #[doc = "Disable the FlexCAN module."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MDIS_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:6 - Number Of The Last Message Buffer"]
    #[inline(always)]
    pub fn maxmb(&self) -> MAXMB_R {
        MAXMB_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - ID Acceptance Mode"]
    #[inline(always)]
    pub fn idam(&self) -> IDAM_R {
        IDAM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Abort Enable"]
    #[inline(always)]
    pub fn aen(&self) -> AEN_R {
        AEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Local Priority Enable"]
    #[inline(always)]
    pub fn lprioen(&self) -> LPRIOEN_R {
        LPRIOEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Individual Rx Masking And Queue Enable"]
    #[inline(always)]
    pub fn irmq(&self) -> IRMQ_R {
        IRMQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Self Reception Disable"]
    #[inline(always)]
    pub fn srxdis(&self) -> SRXDIS_R {
        SRXDIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Wake Up Source"]
    #[inline(always)]
    pub fn waksrc(&self) -> WAKSRC_R {
        WAKSRC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Low-Power Mode Acknowledge"]
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Warning Interrupt Enable"]
    #[inline(always)]
    pub fn wrnen(&self) -> WRNEN_R {
        WRNEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Self Wake Up"]
    #[inline(always)]
    pub fn slfwak(&self) -> SLFWAK_R {
        SLFWAK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Supervisor Mode"]
    #[inline(always)]
    pub fn supv(&self) -> SUPV_R {
        SUPV_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Freeze Mode Acknowledge"]
    #[inline(always)]
    pub fn frzack(&self) -> FRZACK_R {
        FRZACK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Soft Reset"]
    #[inline(always)]
    pub fn softrst(&self) -> SOFTRST_R {
        SOFTRST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Wake Up Interrupt Mask"]
    #[inline(always)]
    pub fn wakmsk(&self) -> WAKMSK_R {
        WAKMSK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - FlexCAN Not Ready"]
    #[inline(always)]
    pub fn notrdy(&self) -> NOTRDY_R {
        NOTRDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Halt FlexCAN"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Rx FIFO Enable"]
    #[inline(always)]
    pub fn rfen(&self) -> RFEN_R {
        RFEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Freeze Enable"]
    #[inline(always)]
    pub fn frz(&self) -> FRZ_R {
        FRZ_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&self) -> MDIS_R {
        MDIS_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Number Of The Last Message Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn maxmb(&mut self) -> MAXMB_W<0> {
        MAXMB_W::new(self)
    }
    #[doc = "Bits 8:9 - ID Acceptance Mode"]
    #[inline(always)]
    #[must_use]
    pub fn idam(&mut self) -> IDAM_W<8> {
        IDAM_W::new(self)
    }
    #[doc = "Bit 12 - Abort Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aen(&mut self) -> AEN_W<12> {
        AEN_W::new(self)
    }
    #[doc = "Bit 13 - Local Priority Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lprioen(&mut self) -> LPRIOEN_W<13> {
        LPRIOEN_W::new(self)
    }
    #[doc = "Bit 16 - Individual Rx Masking And Queue Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irmq(&mut self) -> IRMQ_W<16> {
        IRMQ_W::new(self)
    }
    #[doc = "Bit 17 - Self Reception Disable"]
    #[inline(always)]
    #[must_use]
    pub fn srxdis(&mut self) -> SRXDIS_W<17> {
        SRXDIS_W::new(self)
    }
    #[doc = "Bit 19 - Wake Up Source"]
    #[inline(always)]
    #[must_use]
    pub fn waksrc(&mut self) -> WAKSRC_W<19> {
        WAKSRC_W::new(self)
    }
    #[doc = "Bit 21 - Warning Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wrnen(&mut self) -> WRNEN_W<21> {
        WRNEN_W::new(self)
    }
    #[doc = "Bit 22 - Self Wake Up"]
    #[inline(always)]
    #[must_use]
    pub fn slfwak(&mut self) -> SLFWAK_W<22> {
        SLFWAK_W::new(self)
    }
    #[doc = "Bit 23 - Supervisor Mode"]
    #[inline(always)]
    #[must_use]
    pub fn supv(&mut self) -> SUPV_W<23> {
        SUPV_W::new(self)
    }
    #[doc = "Bit 25 - Soft Reset"]
    #[inline(always)]
    #[must_use]
    pub fn softrst(&mut self) -> SOFTRST_W<25> {
        SOFTRST_W::new(self)
    }
    #[doc = "Bit 26 - Wake Up Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn wakmsk(&mut self) -> WAKMSK_W<26> {
        WAKMSK_W::new(self)
    }
    #[doc = "Bit 28 - Halt FlexCAN"]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HALT_W<28> {
        HALT_W::new(self)
    }
    #[doc = "Bit 29 - Rx FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfen(&mut self) -> RFEN_W<29> {
        RFEN_W::new(self)
    }
    #[doc = "Bit 30 - Freeze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frz(&mut self) -> FRZ_W<30> {
        FRZ_W::new(self)
    }
    #[doc = "Bit 31 - Module Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mdis(&mut self) -> MDIS_W<31> {
        MDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCR to value 0xd890_000f"]
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xd890_000f;
}
