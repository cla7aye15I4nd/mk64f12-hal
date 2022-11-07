#[doc = "Register `IRQSTAT` reader"]
pub struct R(crate::R<IRQSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRQSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRQSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRQSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRQSTAT` writer"]
pub struct W(crate::W<IRQSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRQSTAT_SPEC>;
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
impl From<crate::W<IRQSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRQSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC` reader - Command Complete"]
pub type CC_R = crate::BitReader<CC_A>;
#[doc = "Command Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC_A {
    #[doc = "0: Command not complete."]
    _0 = 0,
    #[doc = "1: Command complete."]
    _1 = 1,
}
impl From<CC_A> for bool {
    #[inline(always)]
    fn from(variant: CC_A) -> Self {
        variant as u8 != 0
    }
}
impl CC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CC_A {
        match self.bits {
            false => CC_A::_0,
            true => CC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CC_A::_1
    }
}
#[doc = "Field `CC` writer - Command Complete"]
pub type CC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTAT_SPEC, CC_A, O>;
impl<'a, const O: u8> CC_W<'a, O> {
    #[doc = "Command not complete."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CC_A::_0)
    }
    #[doc = "Command complete."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CC_A::_1)
    }
}
#[doc = "Field `TC` reader - Transfer Complete"]
pub type TC_R = crate::BitReader<TC_A>;
#[doc = "Transfer Complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC_A {
    #[doc = "0: Transfer not complete."]
    _0 = 0,
    #[doc = "1: Transfer complete."]
    _1 = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
impl TC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::_0,
            true => TC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TC_A::_1
    }
}
#[doc = "Field `TC` writer - Transfer Complete"]
pub type TC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTAT_SPEC, TC_A, O>;
impl<'a, const O: u8> TC_W<'a, O> {
    #[doc = "Transfer not complete."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TC_A::_0)
    }
    #[doc = "Transfer complete."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TC_A::_1)
    }
}
#[doc = "Field `BGE` reader - Block Gap Event"]
pub type BGE_R = crate::BitReader<BGE_A>;
#[doc = "Block Gap Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BGE_A {
    #[doc = "0: No block gap event."]
    _0 = 0,
    #[doc = "1: Transaction stopped at block gap."]
    _1 = 1,
}
impl From<BGE_A> for bool {
    #[inline(always)]
    fn from(variant: BGE_A) -> Self {
        variant as u8 != 0
    }
}
impl BGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BGE_A {
        match self.bits {
            false => BGE_A::_0,
            true => BGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BGE_A::_1
    }
}
#[doc = "Field `BGE` writer - Block Gap Event"]
pub type BGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTAT_SPEC, BGE_A, O>;
impl<'a, const O: u8> BGE_W<'a, O> {
    #[doc = "No block gap event."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BGE_A::_0)
    }
    #[doc = "Transaction stopped at block gap."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BGE_A::_1)
    }
}
#[doc = "Field `DINT` reader - DMA Interrupt"]
pub type DINT_R = crate::BitReader<DINT_A>;
#[doc = "DMA Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DINT_A {
    #[doc = "0: No DMA Interrupt."]
    _0 = 0,
    #[doc = "1: DMA Interrupt is generated."]
    _1 = 1,
}
impl From<DINT_A> for bool {
    #[inline(always)]
    fn from(variant: DINT_A) -> Self {
        variant as u8 != 0
    }
}
impl DINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DINT_A {
        match self.bits {
            false => DINT_A::_0,
            true => DINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DINT_A::_1
    }
}
#[doc = "Field `DINT` writer - DMA Interrupt"]
pub type DINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTAT_SPEC, DINT_A, O>;
impl<'a, const O: u8> DINT_W<'a, O> {
    #[doc = "No DMA Interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DINT_A::_0)
    }
    #[doc = "DMA Interrupt is generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DINT_A::_1)
    }
}
#[doc = "Field `BWR` reader - Buffer Write Ready"]
pub type BWR_R = crate::BitReader<BWR_A>;
#[doc = "Buffer Write Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWR_A {
    #[doc = "0: Not ready to write buffer."]
    _0 = 0,
    #[doc = "1: Ready to write buffer."]
    _1 = 1,
}
impl From<BWR_A> for bool {
    #[inline(always)]
    fn from(variant: BWR_A) -> Self {
        variant as u8 != 0
    }
}
impl BWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWR_A {
        match self.bits {
            false => BWR_A::_0,
            true => BWR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BWR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BWR_A::_1
    }
}
#[doc = "Field `BWR` writer - Buffer Write Ready"]
pub type BWR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTAT_SPEC, BWR_A, O>;
impl<'a, const O: u8> BWR_W<'a, O> {
    #[doc = "Not ready to write buffer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWR_A::_0)
    }
    #[doc = "Ready to write buffer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWR_A::_1)
    }
}
#[doc = "Field `BRR` reader - Buffer Read Ready"]
pub type BRR_R = crate::BitReader<BRR_A>;
#[doc = "Buffer Read Ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRR_A {
    #[doc = "0: Not ready to read buffer."]
    _0 = 0,
    #[doc = "1: Ready to read buffer."]
    _1 = 1,
}
impl From<BRR_A> for bool {
    #[inline(always)]
    fn from(variant: BRR_A) -> Self {
        variant as u8 != 0
    }
}
impl BRR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRR_A {
        match self.bits {
            false => BRR_A::_0,
            true => BRR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRR_A::_1
    }
}
#[doc = "Field `BRR` writer - Buffer Read Ready"]
pub type BRR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTAT_SPEC, BRR_A, O>;
impl<'a, const O: u8> BRR_W<'a, O> {
    #[doc = "Not ready to read buffer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRR_A::_0)
    }
    #[doc = "Ready to read buffer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRR_A::_1)
    }
}
#[doc = "Field `CINS` reader - Card Insertion"]
pub type CINS_R = crate::BitReader<CINS_A>;
#[doc = "Card Insertion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINS_A {
    #[doc = "0: Card state unstable or removed."]
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
#[doc = "Field `CINS` writer - Card Insertion"]
pub type CINS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTAT_SPEC, CINS_A, O>;
impl<'a, const O: u8> CINS_W<'a, O> {
    #[doc = "Card state unstable or removed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CINS_A::_0)
    }
    #[doc = "Card inserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CINS_A::_1)
    }
}
#[doc = "Field `CRM` reader - Card Removal"]
pub type CRM_R = crate::BitReader<CRM_A>;
#[doc = "Card Removal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRM_A {
    #[doc = "0: Card state unstable or inserted."]
    _0 = 0,
    #[doc = "1: Card removed."]
    _1 = 1,
}
impl From<CRM_A> for bool {
    #[inline(always)]
    fn from(variant: CRM_A) -> Self {
        variant as u8 != 0
    }
}
impl CRM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRM_A {
        match self.bits {
            false => CRM_A::_0,
            true => CRM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRM_A::_1
    }
}
#[doc = "Field `CRM` writer - Card Removal"]
pub type CRM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTAT_SPEC, CRM_A, O>;
impl<'a, const O: u8> CRM_W<'a, O> {
    #[doc = "Card state unstable or inserted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRM_A::_0)
    }
    #[doc = "Card removed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRM_A::_1)
    }
}
#[doc = "Field `CINT` reader - Card Interrupt"]
pub type CINT_R = crate::BitReader<CINT_A>;
#[doc = "Card Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CINT_A {
    #[doc = "0: No Card Interrupt."]
    _0 = 0,
    #[doc = "1: Generate Card Interrupt."]
    _1 = 1,
}
impl From<CINT_A> for bool {
    #[inline(always)]
    fn from(variant: CINT_A) -> Self {
        variant as u8 != 0
    }
}
impl CINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CINT_A {
        match self.bits {
            false => CINT_A::_0,
            true => CINT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CINT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CINT_A::_1
    }
}
#[doc = "Field `CINT` writer - Card Interrupt"]
pub type CINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTAT_SPEC, CINT_A, O>;
impl<'a, const O: u8> CINT_W<'a, O> {
    #[doc = "No Card Interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CINT_A::_0)
    }
    #[doc = "Generate Card Interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CINT_A::_1)
    }
}
#[doc = "Field `CTOE` reader - Command Timeout Error"]
pub type CTOE_R = crate::BitReader<CTOE_A>;
#[doc = "Command Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTOE_A {
    #[doc = "0: No error."]
    _0 = 0,
    #[doc = "1: Time out."]
    _1 = 1,
}
impl From<CTOE_A> for bool {
    #[inline(always)]
    fn from(variant: CTOE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTOE_A {
        match self.bits {
            false => CTOE_A::_0,
            true => CTOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTOE_A::_1
    }
}
#[doc = "Field `CTOE` writer - Command Timeout Error"]
pub type CTOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTAT_SPEC, CTOE_A, O>;
impl<'a, const O: u8> CTOE_W<'a, O> {
    #[doc = "No error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTOE_A::_0)
    }
    #[doc = "Time out."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTOE_A::_1)
    }
}
#[doc = "Field `CCE` reader - Command CRC Error"]
pub type CCE_R = crate::BitReader<CCE_A>;
#[doc = "Command CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCE_A {
    #[doc = "0: No error."]
    _0 = 0,
    #[doc = "1: CRC Error generated."]
    _1 = 1,
}
impl From<CCE_A> for bool {
    #[inline(always)]
    fn from(variant: CCE_A) -> Self {
        variant as u8 != 0
    }
}
impl CCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCE_A {
        match self.bits {
            false => CCE_A::_0,
            true => CCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCE_A::_1
    }
}
#[doc = "Field `CCE` writer - Command CRC Error"]
pub type CCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTAT_SPEC, CCE_A, O>;
impl<'a, const O: u8> CCE_W<'a, O> {
    #[doc = "No error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCE_A::_0)
    }
    #[doc = "CRC Error generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCE_A::_1)
    }
}
#[doc = "Field `CEBE` reader - Command End Bit Error"]
pub type CEBE_R = crate::BitReader<CEBE_A>;
#[doc = "Command End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEBE_A {
    #[doc = "0: No error."]
    _0 = 0,
    #[doc = "1: End Bit Error generated."]
    _1 = 1,
}
impl From<CEBE_A> for bool {
    #[inline(always)]
    fn from(variant: CEBE_A) -> Self {
        variant as u8 != 0
    }
}
impl CEBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEBE_A {
        match self.bits {
            false => CEBE_A::_0,
            true => CEBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CEBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CEBE_A::_1
    }
}
#[doc = "Field `CEBE` writer - Command End Bit Error"]
pub type CEBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTAT_SPEC, CEBE_A, O>;
impl<'a, const O: u8> CEBE_W<'a, O> {
    #[doc = "No error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CEBE_A::_0)
    }
    #[doc = "End Bit Error generated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CEBE_A::_1)
    }
}
#[doc = "Field `CIE` reader - Command Index Error"]
pub type CIE_R = crate::BitReader<CIE_A>;
#[doc = "Command Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIE_A {
    #[doc = "0: No error."]
    _0 = 0,
    #[doc = "1: Error."]
    _1 = 1,
}
impl From<CIE_A> for bool {
    #[inline(always)]
    fn from(variant: CIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIE_A {
        match self.bits {
            false => CIE_A::_0,
            true => CIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CIE_A::_1
    }
}
#[doc = "Field `CIE` writer - Command Index Error"]
pub type CIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTAT_SPEC, CIE_A, O>;
impl<'a, const O: u8> CIE_W<'a, O> {
    #[doc = "No error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CIE_A::_0)
    }
    #[doc = "Error."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CIE_A::_1)
    }
}
#[doc = "Field `DTOE` reader - Data Timeout Error"]
pub type DTOE_R = crate::BitReader<DTOE_A>;
#[doc = "Data Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTOE_A {
    #[doc = "0: No error."]
    _0 = 0,
    #[doc = "1: Time out."]
    _1 = 1,
}
impl From<DTOE_A> for bool {
    #[inline(always)]
    fn from(variant: DTOE_A) -> Self {
        variant as u8 != 0
    }
}
impl DTOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTOE_A {
        match self.bits {
            false => DTOE_A::_0,
            true => DTOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DTOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DTOE_A::_1
    }
}
#[doc = "Field `DTOE` writer - Data Timeout Error"]
pub type DTOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTAT_SPEC, DTOE_A, O>;
impl<'a, const O: u8> DTOE_W<'a, O> {
    #[doc = "No error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DTOE_A::_0)
    }
    #[doc = "Time out."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DTOE_A::_1)
    }
}
#[doc = "Field `DCE` reader - Data CRC Error"]
pub type DCE_R = crate::BitReader<DCE_A>;
#[doc = "Data CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCE_A {
    #[doc = "0: No error."]
    _0 = 0,
    #[doc = "1: Error."]
    _1 = 1,
}
impl From<DCE_A> for bool {
    #[inline(always)]
    fn from(variant: DCE_A) -> Self {
        variant as u8 != 0
    }
}
impl DCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCE_A {
        match self.bits {
            false => DCE_A::_0,
            true => DCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DCE_A::_1
    }
}
#[doc = "Field `DCE` writer - Data CRC Error"]
pub type DCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTAT_SPEC, DCE_A, O>;
impl<'a, const O: u8> DCE_W<'a, O> {
    #[doc = "No error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DCE_A::_0)
    }
    #[doc = "Error."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DCE_A::_1)
    }
}
#[doc = "Field `DEBE` reader - Data End Bit Error"]
pub type DEBE_R = crate::BitReader<DEBE_A>;
#[doc = "Data End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEBE_A {
    #[doc = "0: No error."]
    _0 = 0,
    #[doc = "1: Error."]
    _1 = 1,
}
impl From<DEBE_A> for bool {
    #[inline(always)]
    fn from(variant: DEBE_A) -> Self {
        variant as u8 != 0
    }
}
impl DEBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEBE_A {
        match self.bits {
            false => DEBE_A::_0,
            true => DEBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DEBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DEBE_A::_1
    }
}
#[doc = "Field `DEBE` writer - Data End Bit Error"]
pub type DEBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTAT_SPEC, DEBE_A, O>;
impl<'a, const O: u8> DEBE_W<'a, O> {
    #[doc = "No error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DEBE_A::_0)
    }
    #[doc = "Error."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DEBE_A::_1)
    }
}
#[doc = "Field `AC12E` reader - Auto CMD12 Error"]
pub type AC12E_R = crate::BitReader<AC12E_A>;
#[doc = "Auto CMD12 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AC12E_A {
    #[doc = "0: No error."]
    _0 = 0,
    #[doc = "1: Error."]
    _1 = 1,
}
impl From<AC12E_A> for bool {
    #[inline(always)]
    fn from(variant: AC12E_A) -> Self {
        variant as u8 != 0
    }
}
impl AC12E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12E_A {
        match self.bits {
            false => AC12E_A::_0,
            true => AC12E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AC12E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AC12E_A::_1
    }
}
#[doc = "Field `AC12E` writer - Auto CMD12 Error"]
pub type AC12E_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTAT_SPEC, AC12E_A, O>;
impl<'a, const O: u8> AC12E_W<'a, O> {
    #[doc = "No error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AC12E_A::_0)
    }
    #[doc = "Error."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AC12E_A::_1)
    }
}
#[doc = "Field `DMAE` reader - DMA Error"]
pub type DMAE_R = crate::BitReader<DMAE_A>;
#[doc = "DMA Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAE_A {
    #[doc = "0: No error."]
    _0 = 0,
    #[doc = "1: Error."]
    _1 = 1,
}
impl From<DMAE_A> for bool {
    #[inline(always)]
    fn from(variant: DMAE_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAE_A {
        match self.bits {
            false => DMAE_A::_0,
            true => DMAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAE_A::_1
    }
}
#[doc = "Field `DMAE` writer - DMA Error"]
pub type DMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IRQSTAT_SPEC, DMAE_A, O>;
impl<'a, const O: u8> DMAE_W<'a, O> {
    #[doc = "No error."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAE_A::_0)
    }
    #[doc = "Error."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    pub fn cc(&self) -> CC_R {
        CC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    pub fn bge(&self) -> BGE_R {
        BGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    pub fn dint(&self) -> DINT_R {
        DINT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    pub fn bwr(&self) -> BWR_R {
        BWR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    pub fn cins(&self) -> CINS_R {
        CINS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    pub fn crm(&self) -> CRM_R {
        CRM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline(always)]
    pub fn cint(&self) -> CINT_R {
        CINT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Command Timeout Error"]
    #[inline(always)]
    pub fn ctoe(&self) -> CTOE_R {
        CTOE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Command CRC Error"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Command End Bit Error"]
    #[inline(always)]
    pub fn cebe(&self) -> CEBE_R {
        CEBE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Command Index Error"]
    #[inline(always)]
    pub fn cie(&self) -> CIE_R {
        CIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Data Timeout Error"]
    #[inline(always)]
    pub fn dtoe(&self) -> DTOE_R {
        DTOE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Data CRC Error"]
    #[inline(always)]
    pub fn dce(&self) -> DCE_R {
        DCE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data End Bit Error"]
    #[inline(always)]
    pub fn debe(&self) -> DEBE_R {
        DEBE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto CMD12 Error"]
    #[inline(always)]
    pub fn ac12e(&self) -> AC12E_R {
        AC12E_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - DMA Error"]
    #[inline(always)]
    pub fn dmae(&self) -> DMAE_R {
        DMAE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command Complete"]
    #[inline(always)]
    #[must_use]
    pub fn cc(&mut self) -> CC_W<0> {
        CC_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Complete"]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TC_W<1> {
        TC_W::new(self)
    }
    #[doc = "Bit 2 - Block Gap Event"]
    #[inline(always)]
    #[must_use]
    pub fn bge(&mut self) -> BGE_W<2> {
        BGE_W::new(self)
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn dint(&mut self) -> DINT_W<3> {
        DINT_W::new(self)
    }
    #[doc = "Bit 4 - Buffer Write Ready"]
    #[inline(always)]
    #[must_use]
    pub fn bwr(&mut self) -> BWR_W<4> {
        BWR_W::new(self)
    }
    #[doc = "Bit 5 - Buffer Read Ready"]
    #[inline(always)]
    #[must_use]
    pub fn brr(&mut self) -> BRR_W<5> {
        BRR_W::new(self)
    }
    #[doc = "Bit 6 - Card Insertion"]
    #[inline(always)]
    #[must_use]
    pub fn cins(&mut self) -> CINS_W<6> {
        CINS_W::new(self)
    }
    #[doc = "Bit 7 - Card Removal"]
    #[inline(always)]
    #[must_use]
    pub fn crm(&mut self) -> CRM_W<7> {
        CRM_W::new(self)
    }
    #[doc = "Bit 8 - Card Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cint(&mut self) -> CINT_W<8> {
        CINT_W::new(self)
    }
    #[doc = "Bit 16 - Command Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn ctoe(&mut self) -> CTOE_W<16> {
        CTOE_W::new(self)
    }
    #[doc = "Bit 17 - Command CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CCE_W<17> {
        CCE_W::new(self)
    }
    #[doc = "Bit 18 - Command End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn cebe(&mut self) -> CEBE_W<18> {
        CEBE_W::new(self)
    }
    #[doc = "Bit 19 - Command Index Error"]
    #[inline(always)]
    #[must_use]
    pub fn cie(&mut self) -> CIE_W<19> {
        CIE_W::new(self)
    }
    #[doc = "Bit 20 - Data Timeout Error"]
    #[inline(always)]
    #[must_use]
    pub fn dtoe(&mut self) -> DTOE_W<20> {
        DTOE_W::new(self)
    }
    #[doc = "Bit 21 - Data CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn dce(&mut self) -> DCE_W<21> {
        DCE_W::new(self)
    }
    #[doc = "Bit 22 - Data End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn debe(&mut self) -> DEBE_W<22> {
        DEBE_W::new(self)
    }
    #[doc = "Bit 24 - Auto CMD12 Error"]
    #[inline(always)]
    #[must_use]
    pub fn ac12e(&mut self) -> AC12E_W<24> {
        AC12E_W::new(self)
    }
    #[doc = "Bit 28 - DMA Error"]
    #[inline(always)]
    #[must_use]
    pub fn dmae(&mut self) -> DMAE_W<28> {
        DMAE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irqstat](index.html) module"]
pub struct IRQSTAT_SPEC;
impl crate::RegisterSpec for IRQSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irqstat::R](R) reader structure"]
impl crate::Readable for IRQSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irqstat::W](W) writer structure"]
impl crate::Writable for IRQSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IRQSTAT to value 0"]
impl crate::Resettable for IRQSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
