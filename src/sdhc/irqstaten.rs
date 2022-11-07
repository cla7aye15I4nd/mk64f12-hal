#[doc = "Register `IRQSTATEN` reader"]
pub struct R(crate::R<IRQSTATEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQSTATEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQSTATEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQSTATEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQSTATEN` writer"]
pub struct W(crate::W<IRQSTATEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQSTATEN_SPEC>;
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
impl From<crate::W<IRQSTATEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQSTATEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCSEN` reader - Command Complete Status Enable"]
pub type CCSEN_R = crate::BitReader<CCSEN_A>;
#[doc = "Command Complete Status Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCSEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<CCSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CCSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCSEN_A {
        match self.bits {
            false => CCSEN_A::_0,
            true => CCSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCSEN_A::_1
    }
}
#[doc = "Field `CCSEN` writer - Command Complete Status Enable"]
pub type CCSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATEN_SPEC, CCSEN_A, O>;
impl<'a, const O: u8> CCSEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCSEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCSEN_A::_1)
    }
}
#[doc = "Field `TCSEN` reader - Transfer Complete Status Enable"]
pub type TCSEN_R = crate::BitReader<TCSEN_A>;
#[doc = "Transfer Complete Status Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCSEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<TCSEN_A> for bool {
    #[inline(always)]
    fn from(variant: TCSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TCSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCSEN_A {
        match self.bits {
            false => TCSEN_A::_0,
            true => TCSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCSEN_A::_1
    }
}
#[doc = "Field `TCSEN` writer - Transfer Complete Status Enable"]
pub type TCSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATEN_SPEC, TCSEN_A, O>;
impl<'a, const O: u8> TCSEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCSEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCSEN_A::_1)
    }
}
#[doc = "Field `BGESEN` reader - Block Gap Event Status Enable"]
pub type BGESEN_R = crate::BitReader<BGESEN_A>;
#[doc = "Block Gap Event Status Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BGESEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<BGESEN_A> for bool {
    #[inline(always)]
    fn from(variant: BGESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BGESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGESEN_A {
        match self.bits {
            false => BGESEN_A::_0,
            true => BGESEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BGESEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BGESEN_A::_1
    }
}
#[doc = "Field `BGESEN` writer - Block Gap Event Status Enable"]
pub type BGESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATEN_SPEC, BGESEN_A, O>;
impl<'a, const O: u8> BGESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGESEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGESEN_A::_1)
    }
}
#[doc = "Field `DINTSEN` reader - DMA Interrupt Status Enable"]
pub type DINTSEN_R = crate::BitReader<DINTSEN_A>;
#[doc = "DMA Interrupt Status Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DINTSEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<DINTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: DINTSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DINTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINTSEN_A {
        match self.bits {
            false => DINTSEN_A::_0,
            true => DINTSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DINTSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DINTSEN_A::_1
    }
}
#[doc = "Field `DINTSEN` writer - DMA Interrupt Status Enable"]
pub type DINTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATEN_SPEC, DINTSEN_A, O>;
impl<'a, const O: u8> DINTSEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINTSEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINTSEN_A::_1)
    }
}
#[doc = "Field `BWRSEN` reader - Buffer Write Ready Status Enable"]
pub type BWRSEN_R = crate::BitReader<BWRSEN_A>;
#[doc = "Buffer Write Ready Status Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWRSEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<BWRSEN_A> for bool {
    #[inline(always)]
    fn from(variant: BWRSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BWRSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWRSEN_A {
        match self.bits {
            false => BWRSEN_A::_0,
            true => BWRSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BWRSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BWRSEN_A::_1
    }
}
#[doc = "Field `BWRSEN` writer - Buffer Write Ready Status Enable"]
pub type BWRSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATEN_SPEC, BWRSEN_A, O>;
impl<'a, const O: u8> BWRSEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWRSEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWRSEN_A::_1)
    }
}
#[doc = "Field `BRRSEN` reader - Buffer Read Ready Status Enable"]
pub type BRRSEN_R = crate::BitReader<BRRSEN_A>;
#[doc = "Buffer Read Ready Status Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRRSEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<BRRSEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRRSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BRRSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRRSEN_A {
        match self.bits {
            false => BRRSEN_A::_0,
            true => BRRSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRRSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRRSEN_A::_1
    }
}
#[doc = "Field `BRRSEN` writer - Buffer Read Ready Status Enable"]
pub type BRRSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATEN_SPEC, BRRSEN_A, O>;
impl<'a, const O: u8> BRRSEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRRSEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRRSEN_A::_1)
    }
}
#[doc = "Field `CINSEN` reader - Card Insertion Status Enable"]
pub type CINSEN_R = crate::BitReader<CINSEN_A>;
#[doc = "Card Insertion Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINSEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<CINSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CINSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CINSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINSEN_A {
        match self.bits {
            false => CINSEN_A::_0,
            true => CINSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CINSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CINSEN_A::_1
    }
}
#[doc = "Field `CINSEN` writer - Card Insertion Status Enable"]
pub type CINSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATEN_SPEC, CINSEN_A, O>;
impl<'a, const O: u8> CINSEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CINSEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CINSEN_A::_1)
    }
}
#[doc = "Field `CRMSEN` reader - Card Removal Status Enable"]
pub type CRMSEN_R = crate::BitReader<CRMSEN_A>;
#[doc = "Card Removal Status Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRMSEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<CRMSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRMSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRMSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRMSEN_A {
        match self.bits {
            false => CRMSEN_A::_0,
            true => CRMSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRMSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRMSEN_A::_1
    }
}
#[doc = "Field `CRMSEN` writer - Card Removal Status Enable"]
pub type CRMSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATEN_SPEC, CRMSEN_A, O>;
impl<'a, const O: u8> CRMSEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRMSEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRMSEN_A::_1)
    }
}
#[doc = "Field `CINTSEN` reader - Card Interrupt Status Enable"]
pub type CINTSEN_R = crate::BitReader<CINTSEN_A>;
#[doc = "Card Interrupt Status Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINTSEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<CINTSEN_A> for bool {
    #[inline(always)]
    fn from(variant: CINTSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CINTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINTSEN_A {
        match self.bits {
            false => CINTSEN_A::_0,
            true => CINTSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CINTSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CINTSEN_A::_1
    }
}
#[doc = "Field `CINTSEN` writer - Card Interrupt Status Enable"]
pub type CINTSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATEN_SPEC, CINTSEN_A, O>;
impl<'a, const O: u8> CINTSEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CINTSEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CINTSEN_A::_1)
    }
}
#[doc = "Field `CTOESEN` reader - Command Timeout Error Status Enable"]
pub type CTOESEN_R = crate::BitReader<CTOESEN_A>;
#[doc = "Command Timeout Error Status Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTOESEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<CTOESEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTOESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CTOESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTOESEN_A {
        match self.bits {
            false => CTOESEN_A::_0,
            true => CTOESEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTOESEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTOESEN_A::_1
    }
}
#[doc = "Field `CTOESEN` writer - Command Timeout Error Status Enable"]
pub type CTOESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATEN_SPEC, CTOESEN_A, O>;
impl<'a, const O: u8> CTOESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTOESEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTOESEN_A::_1)
    }
}
#[doc = "Field `CCESEN` reader - Command CRC Error Status Enable"]
pub type CCESEN_R = crate::BitReader<CCESEN_A>;
#[doc = "Command CRC Error Status Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCESEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<CCESEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CCESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCESEN_A {
        match self.bits {
            false => CCESEN_A::_0,
            true => CCESEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCESEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCESEN_A::_1
    }
}
#[doc = "Field `CCESEN` writer - Command CRC Error Status Enable"]
pub type CCESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATEN_SPEC, CCESEN_A, O>;
impl<'a, const O: u8> CCESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCESEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCESEN_A::_1)
    }
}
#[doc = "Field `CEBESEN` reader - Command End Bit Error Status Enable"]
pub type CEBESEN_R = crate::BitReader<CEBESEN_A>;
#[doc = "Command End Bit Error Status Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEBESEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<CEBESEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEBESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CEBESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEBESEN_A {
        match self.bits {
            false => CEBESEN_A::_0,
            true => CEBESEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CEBESEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CEBESEN_A::_1
    }
}
#[doc = "Field `CEBESEN` writer - Command End Bit Error Status Enable"]
pub type CEBESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATEN_SPEC, CEBESEN_A, O>;
impl<'a, const O: u8> CEBESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CEBESEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CEBESEN_A::_1)
    }
}
#[doc = "Field `CIESEN` reader - Command Index Error Status Enable"]
pub type CIESEN_R = crate::BitReader<CIESEN_A>;
#[doc = "Command Index Error Status Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIESEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<CIESEN_A> for bool {
    #[inline(always)]
    fn from(variant: CIESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CIESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIESEN_A {
        match self.bits {
            false => CIESEN_A::_0,
            true => CIESEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CIESEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CIESEN_A::_1
    }
}
#[doc = "Field `CIESEN` writer - Command Index Error Status Enable"]
pub type CIESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATEN_SPEC, CIESEN_A, O>;
impl<'a, const O: u8> CIESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CIESEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CIESEN_A::_1)
    }
}
#[doc = "Field `DTOESEN` reader - Data Timeout Error Status Enable"]
pub type DTOESEN_R = crate::BitReader<DTOESEN_A>;
#[doc = "Data Timeout Error Status Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTOESEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<DTOESEN_A> for bool {
    #[inline(always)]
    fn from(variant: DTOESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DTOESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTOESEN_A {
        match self.bits {
            false => DTOESEN_A::_0,
            true => DTOESEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTOESEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTOESEN_A::_1
    }
}
#[doc = "Field `DTOESEN` writer - Data Timeout Error Status Enable"]
pub type DTOESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATEN_SPEC, DTOESEN_A, O>;
impl<'a, const O: u8> DTOESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTOESEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTOESEN_A::_1)
    }
}
#[doc = "Field `DCESEN` reader - Data CRC Error Status Enable"]
pub type DCESEN_R = crate::BitReader<DCESEN_A>;
#[doc = "Data CRC Error Status Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCESEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<DCESEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DCESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCESEN_A {
        match self.bits {
            false => DCESEN_A::_0,
            true => DCESEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCESEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCESEN_A::_1
    }
}
#[doc = "Field `DCESEN` writer - Data CRC Error Status Enable"]
pub type DCESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATEN_SPEC, DCESEN_A, O>;
impl<'a, const O: u8> DCESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCESEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCESEN_A::_1)
    }
}
#[doc = "Field `DEBESEN` reader - Data End Bit Error Status Enable"]
pub type DEBESEN_R = crate::BitReader<DEBESEN_A>;
#[doc = "Data End Bit Error Status Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEBESEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<DEBESEN_A> for bool {
    #[inline(always)]
    fn from(variant: DEBESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DEBESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBESEN_A {
        match self.bits {
            false => DEBESEN_A::_0,
            true => DEBESEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DEBESEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DEBESEN_A::_1
    }
}
#[doc = "Field `DEBESEN` writer - Data End Bit Error Status Enable"]
pub type DEBESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATEN_SPEC, DEBESEN_A, O>;
impl<'a, const O: u8> DEBESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEBESEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEBESEN_A::_1)
    }
}
#[doc = "Field `AC12ESEN` reader - Auto CMD12 Error Status Enable"]
pub type AC12ESEN_R = crate::BitReader<AC12ESEN_A>;
#[doc = "Auto CMD12 Error Status Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AC12ESEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<AC12ESEN_A> for bool {
    #[inline(always)]
    fn from(variant: AC12ESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AC12ESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12ESEN_A {
        match self.bits {
            false => AC12ESEN_A::_0,
            true => AC12ESEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AC12ESEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AC12ESEN_A::_1
    }
}
#[doc = "Field `AC12ESEN` writer - Auto CMD12 Error Status Enable"]
pub type AC12ESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATEN_SPEC, AC12ESEN_A, O>;
impl<'a, const O: u8> AC12ESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AC12ESEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AC12ESEN_A::_1)
    }
}
#[doc = "Field `DMAESEN` reader - DMA Error Status Enable"]
pub type DMAESEN_R = crate::BitReader<DMAESEN_A>;
#[doc = "DMA Error Status Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAESEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<DMAESEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAESEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAESEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAESEN_A {
        match self.bits {
            false => DMAESEN_A::_0,
            true => DMAESEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAESEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAESEN_A::_1
    }
}
#[doc = "Field `DMAESEN` writer - DMA Error Status Enable"]
pub type DMAESEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTATEN_SPEC, DMAESEN_A, O>;
impl<'a, const O: u8> DMAESEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAESEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAESEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline(always)]
    pub fn ccsen(&self) -> CCSEN_R {
        CCSEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline(always)]
    pub fn tcsen(&self) -> TCSEN_R {
        TCSEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline(always)]
    pub fn bgesen(&self) -> BGESEN_R {
        BGESEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable"]
    #[inline(always)]
    pub fn dintsen(&self) -> DINTSEN_R {
        DINTSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline(always)]
    pub fn bwrsen(&self) -> BWRSEN_R {
        BWRSEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline(always)]
    pub fn brrsen(&self) -> BRRSEN_R {
        BRRSEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable"]
    #[inline(always)]
    pub fn cinsen(&self) -> CINSEN_R {
        CINSEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal Status Enable"]
    #[inline(always)]
    pub fn crmsen(&self) -> CRMSEN_R {
        CRMSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable"]
    #[inline(always)]
    pub fn cintsen(&self) -> CINTSEN_R {
        CINTSEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Command Timeout Error Status Enable"]
    #[inline(always)]
    pub fn ctoesen(&self) -> CTOESEN_R {
        CTOESEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Command CRC Error Status Enable"]
    #[inline(always)]
    pub fn ccesen(&self) -> CCESEN_R {
        CCESEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error Status Enable"]
    #[inline(always)]
    pub fn cebesen(&self) -> CEBESEN_R {
        CEBESEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command Index Error Status Enable"]
    #[inline(always)]
    pub fn ciesen(&self) -> CIESEN_R {
        CIESEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error Status Enable"]
    #[inline(always)]
    pub fn dtoesen(&self) -> DTOESEN_R {
        DTOESEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error Status Enable"]
    #[inline(always)]
    pub fn dcesen(&self) -> DCESEN_R {
        DCESEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error Status Enable"]
    #[inline(always)]
    pub fn debesen(&self) -> DEBESEN_R {
        DEBESEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Status Enable"]
    #[inline(always)]
    pub fn ac12esen(&self) -> AC12ESEN_R {
        AC12ESEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA Error Status Enable"]
    #[inline(always)]
    pub fn dmaesen(&self) -> DMAESEN_R {
        DMAESEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccsen(&mut self) -> CCSEN_W<0> {
        CCSEN_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Complete Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcsen(&mut self) -> TCSEN_W<1> {
        TCSEN_W::new(self)
    }
    #[doc = "Bit 2 - Block Gap Event Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bgesen(&mut self) -> BGESEN_W<2> {
        BGESEN_W::new(self)
    }
    #[doc = "Bit 3 - DMA Interrupt Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dintsen(&mut self) -> DINTSEN_W<3> {
        DINTSEN_W::new(self)
    }
    #[doc = "Bit 4 - Buffer Write Ready Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bwrsen(&mut self) -> BWRSEN_W<4> {
        BWRSEN_W::new(self)
    }
    #[doc = "Bit 5 - Buffer Read Ready Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn brrsen(&mut self) -> BRRSEN_W<5> {
        BRRSEN_W::new(self)
    }
    #[doc = "Bit 6 - Card Insertion Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cinsen(&mut self) -> CINSEN_W<6> {
        CINSEN_W::new(self)
    }
    #[doc = "Bit 7 - Card Removal Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crmsen(&mut self) -> CRMSEN_W<7> {
        CRMSEN_W::new(self)
    }
    #[doc = "Bit 8 - Card Interrupt Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cintsen(&mut self) -> CINTSEN_W<8> {
        CINTSEN_W::new(self)
    }
    #[doc = "Bit 16 - Command Timeout Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctoesen(&mut self) -> CTOESEN_W<16> {
        CTOESEN_W::new(self)
    }
    #[doc = "Bit 17 - Command CRC Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccesen(&mut self) -> CCESEN_W<17> {
        CCESEN_W::new(self)
    }
    #[doc = "Bit 18 - Command End Bit Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cebesen(&mut self) -> CEBESEN_W<18> {
        CEBESEN_W::new(self)
    }
    #[doc = "Bit 19 - Command Index Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ciesen(&mut self) -> CIESEN_W<19> {
        CIESEN_W::new(self)
    }
    #[doc = "Bit 20 - Data Timeout Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtoesen(&mut self) -> DTOESEN_W<20> {
        DTOESEN_W::new(self)
    }
    #[doc = "Bit 21 - Data CRC Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcesen(&mut self) -> DCESEN_W<21> {
        DCESEN_W::new(self)
    }
    #[doc = "Bit 22 - Data End Bit Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debesen(&mut self) -> DEBESEN_W<22> {
        DEBESEN_W::new(self)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ac12esen(&mut self) -> AC12ESEN_W<24> {
        AC12ESEN_W::new(self)
    }
    #[doc = "Bit 28 - DMA Error Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaesen(&mut self) -> DMAESEN_W<28> {
        DMAESEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqstaten](index.html) module"]
pub struct IRQSTATEN_SPEC;
impl crate::RegisterSpec for IRQSTATEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irqstaten::R](R) reader structure"]
impl crate::Readable for IRQSTATEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irqstaten::W](W) writer structure"]
impl crate::Writable for IRQSTATEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQSTATEN to value 0x117f_013f"]
impl crate::Resettable for IRQSTATEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x117f_013f;
}
