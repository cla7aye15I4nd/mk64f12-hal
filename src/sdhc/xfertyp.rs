#[doc = "Register `XFERTYP` reader"]
pub struct R(crate::R<XFERTYP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XFERTYP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XFERTYP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XFERTYP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XFERTYP` writer"]
pub struct W(crate::W<XFERTYP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XFERTYP_SPEC>;
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
impl From<crate::W<XFERTYP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XFERTYP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::_0,
            true => DMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAEN_A::_1
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, XFERTYP_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEN_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAEN_A::_1)
    }
}
#[doc = "Field `BCEN` reader - Block Count Enable"]
pub type BCEN_R = crate::BitReader<BCEN_A>;
#[doc = "Block Count Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCEN_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<BCEN_A> for bool {
    #[inline(always)]
    fn from(variant: BCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCEN_A {
        match self.bits {
            false => BCEN_A::_0,
            true => BCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCEN_A::_1
    }
}
#[doc = "Field `BCEN` writer - Block Count Enable"]
pub type BCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, XFERTYP_SPEC, BCEN_A, O>;
impl<'a, const O: u8> BCEN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCEN_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCEN_A::_1)
    }
}
#[doc = "Field `AC12EN` reader - Auto CMD12 Enable"]
pub type AC12EN_R = crate::BitReader<AC12EN_A>;
#[doc = "Auto CMD12 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AC12EN_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<AC12EN_A> for bool {
    #[inline(always)]
    fn from(variant: AC12EN_A) -> Self {
        variant as u8 != 0
    }
}
impl AC12EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12EN_A {
        match self.bits {
            false => AC12EN_A::_0,
            true => AC12EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AC12EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AC12EN_A::_1
    }
}
#[doc = "Field `AC12EN` writer - Auto CMD12 Enable"]
pub type AC12EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, XFERTYP_SPEC, AC12EN_A, O>;
impl<'a, const O: u8> AC12EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AC12EN_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AC12EN_A::_1)
    }
}
#[doc = "Field `DTDSEL` reader - Data Transfer Direction Select"]
pub type DTDSEL_R = crate::BitReader<DTDSEL_A>;
#[doc = "Data Transfer Direction Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTDSEL_A {
    #[doc = "0: Write host to card."]
    _0 = 0,
    #[doc = "1: Read card to host."]
    _1 = 1,
}
impl From<DTDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DTDSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DTDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTDSEL_A {
        match self.bits {
            false => DTDSEL_A::_0,
            true => DTDSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTDSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTDSEL_A::_1
    }
}
#[doc = "Field `DTDSEL` writer - Data Transfer Direction Select"]
pub type DTDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, XFERTYP_SPEC, DTDSEL_A, O>;
impl<'a, const O: u8> DTDSEL_W<'a, O> {
    #[doc = "Write host to card."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTDSEL_A::_0)
    }
    #[doc = "Read card to host."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTDSEL_A::_1)
    }
}
#[doc = "Field `MSBSEL` reader - Multi/Single Block Select"]
pub type MSBSEL_R = crate::BitReader<MSBSEL_A>;
#[doc = "Multi/Single Block Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSBSEL_A {
    #[doc = "0: Single block."]
    _0 = 0,
    #[doc = "1: Multiple blocks."]
    _1 = 1,
}
impl From<MSBSEL_A> for bool {
    #[inline(always)]
    fn from(variant: MSBSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl MSBSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSBSEL_A {
        match self.bits {
            false => MSBSEL_A::_0,
            true => MSBSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSBSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSBSEL_A::_1
    }
}
#[doc = "Field `MSBSEL` writer - Multi/Single Block Select"]
pub type MSBSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, XFERTYP_SPEC, MSBSEL_A, O>;
impl<'a, const O: u8> MSBSEL_W<'a, O> {
    #[doc = "Single block."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSBSEL_A::_0)
    }
    #[doc = "Multiple blocks."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSBSEL_A::_1)
    }
}
#[doc = "Field `RSPTYP` reader - Response Type Select"]
pub type RSPTYP_R = crate::FieldReader<u8, RSPTYP_A>;
#[doc = "Response Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSPTYP_A {
    #[doc = "0: No response."]
    _00 = 0,
    #[doc = "1: Response length 136."]
    _01 = 1,
    #[doc = "2: Response length 48."]
    _10 = 2,
    #[doc = "3: Response length 48, check busy after response."]
    _11 = 3,
}
impl From<RSPTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: RSPTYP_A) -> Self {
        variant as _
    }
}
impl RSPTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPTYP_A {
        match self.bits {
            0 => RSPTYP_A::_00,
            1 => RSPTYP_A::_01,
            2 => RSPTYP_A::_10,
            3 => RSPTYP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RSPTYP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RSPTYP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RSPTYP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RSPTYP_A::_11
    }
}
#[doc = "Field `RSPTYP` writer - Response Type Select"]
pub type RSPTYP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, XFERTYP_SPEC, u8, RSPTYP_A, 2, O>;
impl<'a, const O: u8> RSPTYP_W<'a, O> {
    #[doc = "No response."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RSPTYP_A::_00)
    }
    #[doc = "Response length 136."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RSPTYP_A::_01)
    }
    #[doc = "Response length 48."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RSPTYP_A::_10)
    }
    #[doc = "Response length 48, check busy after response."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RSPTYP_A::_11)
    }
}
#[doc = "Field `CCCEN` reader - Command CRC Check Enable"]
pub type CCCEN_R = crate::BitReader<CCCEN_A>;
#[doc = "Command CRC Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCCEN_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<CCCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CCCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCCEN_A {
        match self.bits {
            false => CCCEN_A::_0,
            true => CCCEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCCEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCCEN_A::_1
    }
}
#[doc = "Field `CCCEN` writer - Command CRC Check Enable"]
pub type CCCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, XFERTYP_SPEC, CCCEN_A, O>;
impl<'a, const O: u8> CCCEN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCCEN_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCCEN_A::_1)
    }
}
#[doc = "Field `CICEN` reader - Command Index Check Enable"]
pub type CICEN_R = crate::BitReader<CICEN_A>;
#[doc = "Command Index Check Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CICEN_A {
    #[doc = "0: Disable"]
    _0 = 0,
    #[doc = "1: Enable"]
    _1 = 1,
}
impl From<CICEN_A> for bool {
    #[inline(always)]
    fn from(variant: CICEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CICEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CICEN_A {
        match self.bits {
            false => CICEN_A::_0,
            true => CICEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CICEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CICEN_A::_1
    }
}
#[doc = "Field `CICEN` writer - Command Index Check Enable"]
pub type CICEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, XFERTYP_SPEC, CICEN_A, O>;
impl<'a, const O: u8> CICEN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CICEN_A::_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CICEN_A::_1)
    }
}
#[doc = "Field `DPSEL` reader - Data Present Select"]
pub type DPSEL_R = crate::BitReader<DPSEL_A>;
#[doc = "Data Present Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPSEL_A {
    #[doc = "0: No data present."]
    _0 = 0,
    #[doc = "1: Data present."]
    _1 = 1,
}
impl From<DPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DPSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DPSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPSEL_A {
        match self.bits {
            false => DPSEL_A::_0,
            true => DPSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPSEL_A::_1
    }
}
#[doc = "Field `DPSEL` writer - Data Present Select"]
pub type DPSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, XFERTYP_SPEC, DPSEL_A, O>;
impl<'a, const O: u8> DPSEL_W<'a, O> {
    #[doc = "No data present."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPSEL_A::_0)
    }
    #[doc = "Data present."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPSEL_A::_1)
    }
}
#[doc = "Field `CMDTYP` reader - Command Type"]
pub type CMDTYP_R = crate::FieldReader<u8, CMDTYP_A>;
#[doc = "Command Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMDTYP_A {
    #[doc = "0: Normal other commands."]
    _00 = 0,
    #[doc = "1: Suspend CMD52 for writing bus suspend in CCCR."]
    _01 = 1,
    #[doc = "2: Resume CMD52 for writing function select in CCCR."]
    _10 = 2,
    #[doc = "3: Abort CMD12, CMD52 for writing I/O abort in CCCR."]
    _11 = 3,
}
impl From<CMDTYP_A> for u8 {
    #[inline(always)]
    fn from(variant: CMDTYP_A) -> Self {
        variant as _
    }
}
impl CMDTYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMDTYP_A {
        match self.bits {
            0 => CMDTYP_A::_00,
            1 => CMDTYP_A::_01,
            2 => CMDTYP_A::_10,
            3 => CMDTYP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMDTYP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMDTYP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMDTYP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CMDTYP_A::_11
    }
}
#[doc = "Field `CMDTYP` writer - Command Type"]
pub type CMDTYP_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, XFERTYP_SPEC, u8, CMDTYP_A, 2, O>;
impl<'a, const O: u8> CMDTYP_W<'a, O> {
    #[doc = "Normal other commands."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMDTYP_A::_00)
    }
    #[doc = "Suspend CMD52 for writing bus suspend in CCCR."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMDTYP_A::_01)
    }
    #[doc = "Resume CMD52 for writing function select in CCCR."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMDTYP_A::_10)
    }
    #[doc = "Abort CMD12, CMD52 for writing I/O abort in CCCR."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMDTYP_A::_11)
    }
}
#[doc = "Field `CMDINX` reader - Command Index"]
pub type CMDINX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CMDINX` writer - Command Index"]
pub type CMDINX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, XFERTYP_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    pub fn bcen(&self) -> BCEN_R {
        BCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto CMD12 Enable"]
    #[inline(always)]
    pub fn ac12en(&self) -> AC12EN_R {
        AC12EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline(always)]
    pub fn dtdsel(&self) -> DTDSEL_R {
        DTDSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi/Single Block Select"]
    #[inline(always)]
    pub fn msbsel(&self) -> MSBSEL_R {
        MSBSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline(always)]
    pub fn rsptyp(&self) -> RSPTYP_R {
        RSPTYP_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline(always)]
    pub fn cccen(&self) -> CCCEN_R {
        CCCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline(always)]
    pub fn cicen(&self) -> CICEN_R {
        CICEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline(always)]
    pub fn dpsel(&self) -> DPSEL_R {
        DPSEL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline(always)]
    pub fn cmdtyp(&self) -> CMDTYP_R {
        CMDTYP_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline(always)]
    pub fn cmdinx(&self) -> CMDINX_R {
        CMDINX_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<0> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 1 - Block Count Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bcen(&mut self) -> BCEN_W<1> {
        BCEN_W::new(self)
    }
    #[doc = "Bit 2 - Auto CMD12 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ac12en(&mut self) -> AC12EN_W<2> {
        AC12EN_W::new(self)
    }
    #[doc = "Bit 4 - Data Transfer Direction Select"]
    #[inline(always)]
    #[must_use]
    pub fn dtdsel(&mut self) -> DTDSEL_W<4> {
        DTDSEL_W::new(self)
    }
    #[doc = "Bit 5 - Multi/Single Block Select"]
    #[inline(always)]
    #[must_use]
    pub fn msbsel(&mut self) -> MSBSEL_W<5> {
        MSBSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - Response Type Select"]
    #[inline(always)]
    #[must_use]
    pub fn rsptyp(&mut self) -> RSPTYP_W<16> {
        RSPTYP_W::new(self)
    }
    #[doc = "Bit 19 - Command CRC Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cccen(&mut self) -> CCCEN_W<19> {
        CCCEN_W::new(self)
    }
    #[doc = "Bit 20 - Command Index Check Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cicen(&mut self) -> CICEN_W<20> {
        CICEN_W::new(self)
    }
    #[doc = "Bit 21 - Data Present Select"]
    #[inline(always)]
    #[must_use]
    pub fn dpsel(&mut self) -> DPSEL_W<21> {
        DPSEL_W::new(self)
    }
    #[doc = "Bits 22:23 - Command Type"]
    #[inline(always)]
    #[must_use]
    pub fn cmdtyp(&mut self) -> CMDTYP_W<22> {
        CMDTYP_W::new(self)
    }
    #[doc = "Bits 24:29 - Command Index"]
    #[inline(always)]
    #[must_use]
    pub fn cmdinx(&mut self) -> CMDINX_W<24> {
        CMDINX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transfer Type register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xfertyp](index.html) module"]
pub struct XFERTYP_SPEC;
impl crate::RegisterSpec for XFERTYP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xfertyp::R](R) reader structure"]
impl crate::Readable for XFERTYP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xfertyp::W](W) writer structure"]
impl crate::Writable for XFERTYP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XFERTYP to value 0"]
impl crate::Resettable for XFERTYP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
