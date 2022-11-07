#[doc = "Register `IRQSIGEN` reader"]
pub struct R(crate::R<IRQSIGEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQSIGEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQSIGEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQSIGEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQSIGEN` writer"]
pub struct W(crate::W<IRQSIGEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQSIGEN_SPEC>;
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
impl From<crate::W<IRQSIGEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQSIGEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCIEN` reader - Command Complete Interrupt Enable"]
pub type CCIEN_R = crate::BitReader<CCIEN_A>;
#[doc = "Command Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCIEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<CCIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CCIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIEN_A {
        match self.bits {
            false => CCIEN_A::_0,
            true => CCIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCIEN_A::_1
    }
}
#[doc = "Field `CCIEN` writer - Command Complete Interrupt Enable"]
pub type CCIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSIGEN_SPEC, CCIEN_A, O>;
impl<'a, const O: u8> CCIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCIEN_A::_1)
    }
}
#[doc = "Field `TCIEN` reader - Transfer Complete Interrupt Enable"]
pub type TCIEN_R = crate::BitReader<TCIEN_A>;
#[doc = "Transfer Complete Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<TCIEN_A> for bool {
    #[inline(always)]
    fn from(variant: TCIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIEN_A {
        match self.bits {
            false => TCIEN_A::_0,
            true => TCIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCIEN_A::_1
    }
}
#[doc = "Field `TCIEN` writer - Transfer Complete Interrupt Enable"]
pub type TCIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSIGEN_SPEC, TCIEN_A, O>;
impl<'a, const O: u8> TCIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCIEN_A::_1)
    }
}
#[doc = "Field `BGEIEN` reader - Block Gap Event Interrupt Enable"]
pub type BGEIEN_R = crate::BitReader<BGEIEN_A>;
#[doc = "Block Gap Event Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BGEIEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<BGEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: BGEIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BGEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGEIEN_A {
        match self.bits {
            false => BGEIEN_A::_0,
            true => BGEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BGEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BGEIEN_A::_1
    }
}
#[doc = "Field `BGEIEN` writer - Block Gap Event Interrupt Enable"]
pub type BGEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSIGEN_SPEC, BGEIEN_A, O>;
impl<'a, const O: u8> BGEIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGEIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGEIEN_A::_1)
    }
}
#[doc = "Field `DINTIEN` reader - DMA Interrupt Enable"]
pub type DINTIEN_R = crate::BitReader<DINTIEN_A>;
#[doc = "DMA Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DINTIEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<DINTIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DINTIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DINTIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINTIEN_A {
        match self.bits {
            false => DINTIEN_A::_0,
            true => DINTIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DINTIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DINTIEN_A::_1
    }
}
#[doc = "Field `DINTIEN` writer - DMA Interrupt Enable"]
pub type DINTIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSIGEN_SPEC, DINTIEN_A, O>;
impl<'a, const O: u8> DINTIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINTIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINTIEN_A::_1)
    }
}
#[doc = "Field `BWRIEN` reader - Buffer Write Ready Interrupt Enable"]
pub type BWRIEN_R = crate::BitReader<BWRIEN_A>;
#[doc = "Buffer Write Ready Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWRIEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<BWRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: BWRIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BWRIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWRIEN_A {
        match self.bits {
            false => BWRIEN_A::_0,
            true => BWRIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BWRIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BWRIEN_A::_1
    }
}
#[doc = "Field `BWRIEN` writer - Buffer Write Ready Interrupt Enable"]
pub type BWRIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSIGEN_SPEC, BWRIEN_A, O>;
impl<'a, const O: u8> BWRIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWRIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWRIEN_A::_1)
    }
}
#[doc = "Field `BRRIEN` reader - Buffer Read Ready Interrupt Enable"]
pub type BRRIEN_R = crate::BitReader<BRRIEN_A>;
#[doc = "Buffer Read Ready Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRRIEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<BRRIEN_A> for bool {
    #[inline(always)]
    fn from(variant: BRRIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BRRIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRRIEN_A {
        match self.bits {
            false => BRRIEN_A::_0,
            true => BRRIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRRIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRRIEN_A::_1
    }
}
#[doc = "Field `BRRIEN` writer - Buffer Read Ready Interrupt Enable"]
pub type BRRIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSIGEN_SPEC, BRRIEN_A, O>;
impl<'a, const O: u8> BRRIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRRIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRRIEN_A::_1)
    }
}
#[doc = "Field `CINSIEN` reader - Card Insertion Interrupt Enable"]
pub type CINSIEN_R = crate::BitReader<CINSIEN_A>;
#[doc = "Card Insertion Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINSIEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<CINSIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CINSIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CINSIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINSIEN_A {
        match self.bits {
            false => CINSIEN_A::_0,
            true => CINSIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CINSIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CINSIEN_A::_1
    }
}
#[doc = "Field `CINSIEN` writer - Card Insertion Interrupt Enable"]
pub type CINSIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSIGEN_SPEC, CINSIEN_A, O>;
impl<'a, const O: u8> CINSIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CINSIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CINSIEN_A::_1)
    }
}
#[doc = "Field `CRMIEN` reader - Card Removal Interrupt Enable"]
pub type CRMIEN_R = crate::BitReader<CRMIEN_A>;
#[doc = "Card Removal Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRMIEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<CRMIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRMIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRMIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRMIEN_A {
        match self.bits {
            false => CRMIEN_A::_0,
            true => CRMIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRMIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRMIEN_A::_1
    }
}
#[doc = "Field `CRMIEN` writer - Card Removal Interrupt Enable"]
pub type CRMIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSIGEN_SPEC, CRMIEN_A, O>;
impl<'a, const O: u8> CRMIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRMIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRMIEN_A::_1)
    }
}
#[doc = "Field `CINTIEN` reader - Card Interrupt Enable"]
pub type CINTIEN_R = crate::BitReader<CINTIEN_A>;
#[doc = "Card Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINTIEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<CINTIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CINTIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CINTIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINTIEN_A {
        match self.bits {
            false => CINTIEN_A::_0,
            true => CINTIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CINTIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CINTIEN_A::_1
    }
}
#[doc = "Field `CINTIEN` writer - Card Interrupt Enable"]
pub type CINTIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSIGEN_SPEC, CINTIEN_A, O>;
impl<'a, const O: u8> CINTIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CINTIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CINTIEN_A::_1)
    }
}
#[doc = "Field `CTOEIEN` reader - Command Timeout Error Interrupt Enable"]
pub type CTOEIEN_R = crate::BitReader<CTOEIEN_A>;
#[doc = "Command Timeout Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTOEIEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<CTOEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CTOEIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CTOEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTOEIEN_A {
        match self.bits {
            false => CTOEIEN_A::_0,
            true => CTOEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTOEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTOEIEN_A::_1
    }
}
#[doc = "Field `CTOEIEN` writer - Command Timeout Error Interrupt Enable"]
pub type CTOEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSIGEN_SPEC, CTOEIEN_A, O>;
impl<'a, const O: u8> CTOEIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTOEIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTOEIEN_A::_1)
    }
}
#[doc = "Field `CCEIEN` reader - Command CRC Error Interrupt Enable"]
pub type CCEIEN_R = crate::BitReader<CCEIEN_A>;
#[doc = "Command CRC Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCEIEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<CCEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CCEIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CCEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCEIEN_A {
        match self.bits {
            false => CCEIEN_A::_0,
            true => CCEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCEIEN_A::_1
    }
}
#[doc = "Field `CCEIEN` writer - Command CRC Error Interrupt Enable"]
pub type CCEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSIGEN_SPEC, CCEIEN_A, O>;
impl<'a, const O: u8> CCEIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCEIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCEIEN_A::_1)
    }
}
#[doc = "Field `CEBEIEN` reader - Command End Bit Error Interrupt Enable"]
pub type CEBEIEN_R = crate::BitReader<CEBEIEN_A>;
#[doc = "Command End Bit Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEBEIEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<CEBEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEBEIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CEBEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEBEIEN_A {
        match self.bits {
            false => CEBEIEN_A::_0,
            true => CEBEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CEBEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CEBEIEN_A::_1
    }
}
#[doc = "Field `CEBEIEN` writer - Command End Bit Error Interrupt Enable"]
pub type CEBEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSIGEN_SPEC, CEBEIEN_A, O>;
impl<'a, const O: u8> CEBEIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CEBEIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CEBEIEN_A::_1)
    }
}
#[doc = "Field `CIEIEN` reader - Command Index Error Interrupt Enable"]
pub type CIEIEN_R = crate::BitReader<CIEIEN_A>;
#[doc = "Command Index Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIEIEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<CIEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: CIEIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CIEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIEIEN_A {
        match self.bits {
            false => CIEIEN_A::_0,
            true => CIEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CIEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CIEIEN_A::_1
    }
}
#[doc = "Field `CIEIEN` writer - Command Index Error Interrupt Enable"]
pub type CIEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSIGEN_SPEC, CIEIEN_A, O>;
impl<'a, const O: u8> CIEIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CIEIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CIEIEN_A::_1)
    }
}
#[doc = "Field `DTOEIEN` reader - Data Timeout Error Interrupt Enable"]
pub type DTOEIEN_R = crate::BitReader<DTOEIEN_A>;
#[doc = "Data Timeout Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTOEIEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<DTOEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DTOEIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DTOEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTOEIEN_A {
        match self.bits {
            false => DTOEIEN_A::_0,
            true => DTOEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTOEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTOEIEN_A::_1
    }
}
#[doc = "Field `DTOEIEN` writer - Data Timeout Error Interrupt Enable"]
pub type DTOEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSIGEN_SPEC, DTOEIEN_A, O>;
impl<'a, const O: u8> DTOEIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTOEIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTOEIEN_A::_1)
    }
}
#[doc = "Field `DCEIEN` reader - Data CRC Error Interrupt Enable"]
pub type DCEIEN_R = crate::BitReader<DCEIEN_A>;
#[doc = "Data CRC Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCEIEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<DCEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DCEIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DCEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCEIEN_A {
        match self.bits {
            false => DCEIEN_A::_0,
            true => DCEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCEIEN_A::_1
    }
}
#[doc = "Field `DCEIEN` writer - Data CRC Error Interrupt Enable"]
pub type DCEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSIGEN_SPEC, DCEIEN_A, O>;
impl<'a, const O: u8> DCEIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCEIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCEIEN_A::_1)
    }
}
#[doc = "Field `DEBEIEN` reader - Data End Bit Error Interrupt Enable"]
pub type DEBEIEN_R = crate::BitReader<DEBEIEN_A>;
#[doc = "Data End Bit Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEBEIEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<DEBEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DEBEIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DEBEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBEIEN_A {
        match self.bits {
            false => DEBEIEN_A::_0,
            true => DEBEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DEBEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DEBEIEN_A::_1
    }
}
#[doc = "Field `DEBEIEN` writer - Data End Bit Error Interrupt Enable"]
pub type DEBEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSIGEN_SPEC, DEBEIEN_A, O>;
impl<'a, const O: u8> DEBEIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEBEIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEBEIEN_A::_1)
    }
}
#[doc = "Field `AC12EIEN` reader - Auto CMD12 Error Interrupt Enable"]
pub type AC12EIEN_R = crate::BitReader<AC12EIEN_A>;
#[doc = "Auto CMD12 Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AC12EIEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<AC12EIEN_A> for bool {
    #[inline(always)]
    fn from(variant: AC12EIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AC12EIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12EIEN_A {
        match self.bits {
            false => AC12EIEN_A::_0,
            true => AC12EIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AC12EIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AC12EIEN_A::_1
    }
}
#[doc = "Field `AC12EIEN` writer - Auto CMD12 Error Interrupt Enable"]
pub type AC12EIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSIGEN_SPEC, AC12EIEN_A, O>;
impl<'a, const O: u8> AC12EIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AC12EIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AC12EIEN_A::_1)
    }
}
#[doc = "Field `DMAEIEN` reader - DMA Error Interrupt Enable"]
pub type DMAEIEN_R = crate::BitReader<DMAEIEN_A>;
#[doc = "DMA Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEIEN_A {
    #[doc = "0: Masked"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<DMAEIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEIEN_A {
        match self.bits {
            false => DMAEIEN_A::_0,
            true => DMAEIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAEIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAEIEN_A::_1
    }
}
#[doc = "Field `DMAEIEN` writer - DMA Error Interrupt Enable"]
pub type DMAEIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSIGEN_SPEC, DMAEIEN_A, O>;
impl<'a, const O: u8> DMAEIEN_W<'a, O> {
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEIEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAEIEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Command Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ccien(&self) -> CCIEN_R {
        CCIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn tcien(&self) -> TCIEN_R {
        TCIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event Interrupt Enable"]
    #[inline(always)]
    pub fn bgeien(&self) -> BGEIEN_R {
        BGEIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Enable"]
    #[inline(always)]
    pub fn dintien(&self) -> DINTIEN_R {
        DINTIEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready Interrupt Enable"]
    #[inline(always)]
    pub fn bwrien(&self) -> BWRIEN_R {
        BWRIEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready Interrupt Enable"]
    #[inline(always)]
    pub fn brrien(&self) -> BRRIEN_R {
        BRRIEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion Interrupt Enable"]
    #[inline(always)]
    pub fn cinsien(&self) -> CINSIEN_R {
        CINSIEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal Interrupt Enable"]
    #[inline(always)]
    pub fn crmien(&self) -> CRMIEN_R {
        CRMIEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt Enable"]
    #[inline(always)]
    pub fn cintien(&self) -> CINTIEN_R {
        CINTIEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Command Timeout Error Interrupt Enable"]
    #[inline(always)]
    pub fn ctoeien(&self) -> CTOEIEN_R {
        CTOEIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Command CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn cceien(&self) -> CCEIEN_R {
        CCEIEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn cebeien(&self) -> CEBEIEN_R {
        CEBEIEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command Index Error Interrupt Enable"]
    #[inline(always)]
    pub fn cieien(&self) -> CIEIEN_R {
        CIEIEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error Interrupt Enable"]
    #[inline(always)]
    pub fn dtoeien(&self) -> DTOEIEN_R {
        DTOEIEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error Interrupt Enable"]
    #[inline(always)]
    pub fn dceien(&self) -> DCEIEN_R {
        DCEIEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error Interrupt Enable"]
    #[inline(always)]
    pub fn debeien(&self) -> DEBEIEN_R {
        DEBEIEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Interrupt Enable"]
    #[inline(always)]
    pub fn ac12eien(&self) -> AC12EIEN_R {
        AC12EIEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA Error Interrupt Enable"]
    #[inline(always)]
    pub fn dmaeien(&self) -> DMAEIEN_R {
        DMAEIEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccien(&mut self) -> CCIEN_W<0> {
        CCIEN_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Complete Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcien(&mut self) -> TCIEN_W<1> {
        TCIEN_W::new(self)
    }
    #[doc = "Bit 2 - Block Gap Event Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bgeien(&mut self) -> BGEIEN_W<2> {
        BGEIEN_W::new(self)
    }
    #[doc = "Bit 3 - DMA Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dintien(&mut self) -> DINTIEN_W<3> {
        DINTIEN_W::new(self)
    }
    #[doc = "Bit 4 - Buffer Write Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bwrien(&mut self) -> BWRIEN_W<4> {
        BWRIEN_W::new(self)
    }
    #[doc = "Bit 5 - Buffer Read Ready Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn brrien(&mut self) -> BRRIEN_W<5> {
        BRRIEN_W::new(self)
    }
    #[doc = "Bit 6 - Card Insertion Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cinsien(&mut self) -> CINSIEN_W<6> {
        CINSIEN_W::new(self)
    }
    #[doc = "Bit 7 - Card Removal Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crmien(&mut self) -> CRMIEN_W<7> {
        CRMIEN_W::new(self)
    }
    #[doc = "Bit 8 - Card Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cintien(&mut self) -> CINTIEN_W<8> {
        CINTIEN_W::new(self)
    }
    #[doc = "Bit 16 - Command Timeout Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctoeien(&mut self) -> CTOEIEN_W<16> {
        CTOEIEN_W::new(self)
    }
    #[doc = "Bit 17 - Command CRC Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cceien(&mut self) -> CCEIEN_W<17> {
        CCEIEN_W::new(self)
    }
    #[doc = "Bit 18 - Command End Bit Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cebeien(&mut self) -> CEBEIEN_W<18> {
        CEBEIEN_W::new(self)
    }
    #[doc = "Bit 19 - Command Index Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cieien(&mut self) -> CIEIEN_W<19> {
        CIEIEN_W::new(self)
    }
    #[doc = "Bit 20 - Data Timeout Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dtoeien(&mut self) -> DTOEIEN_W<20> {
        DTOEIEN_W::new(self)
    }
    #[doc = "Bit 21 - Data CRC Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dceien(&mut self) -> DCEIEN_W<21> {
        DCEIEN_W::new(self)
    }
    #[doc = "Bit 22 - Data End Bit Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debeien(&mut self) -> DEBEIEN_W<22> {
        DEBEIEN_W::new(self)
    }
    #[doc = "Bit 24 - Auto CMD12 Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ac12eien(&mut self) -> AC12EIEN_W<24> {
        AC12EIEN_W::new(self)
    }
    #[doc = "Bit 28 - DMA Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaeien(&mut self) -> DMAEIEN_W<28> {
        DMAEIEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Signal Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqsigen](index.html) module"]
pub struct IRQSIGEN_SPEC;
impl crate::RegisterSpec for IRQSIGEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irqsigen::R](R) reader structure"]
impl crate::Readable for IRQSIGEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irqsigen::W](W) writer structure"]
impl crate::Writable for IRQSIGEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQSIGEN to value 0"]
impl crate::Resettable for IRQSIGEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
