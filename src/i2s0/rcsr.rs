#[doc = "Register `RCSR` reader"]
pub struct R(crate::R<RCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCSR` writer"]
pub struct W(crate::W<RCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCSR_SPEC>;
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
impl From<crate::W<RCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRDE` reader - FIFO Request DMA Enable"]
pub type FRDE_R = crate::BitReader<FRDE_A>;
#[doc = "FIFO Request DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRDE_A {
    #[doc = "0: Disables the DMA request."]
    _0 = 0,
    #[doc = "1: Enables the DMA request."]
    _1 = 1,
}
impl From<FRDE_A> for bool {
    #[inline(always)]
    fn from(variant: FRDE_A) -> Self {
        variant as u8 != 0
    }
}
impl FRDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRDE_A {
        match self.bits {
            false => FRDE_A::_0,
            true => FRDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRDE_A::_1
    }
}
#[doc = "Field `FRDE` writer - FIFO Request DMA Enable"]
pub type FRDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCSR_SPEC, FRDE_A, O>;
impl<'a, const O: u8> FRDE_W<'a, O> {
    #[doc = "Disables the DMA request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRDE_A::_0)
    }
    #[doc = "Enables the DMA request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRDE_A::_1)
    }
}
#[doc = "Field `FWDE` reader - FIFO Warning DMA Enable"]
pub type FWDE_R = crate::BitReader<FWDE_A>;
#[doc = "FIFO Warning DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWDE_A {
    #[doc = "0: Disables the DMA request."]
    _0 = 0,
    #[doc = "1: Enables the DMA request."]
    _1 = 1,
}
impl From<FWDE_A> for bool {
    #[inline(always)]
    fn from(variant: FWDE_A) -> Self {
        variant as u8 != 0
    }
}
impl FWDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWDE_A {
        match self.bits {
            false => FWDE_A::_0,
            true => FWDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FWDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FWDE_A::_1
    }
}
#[doc = "Field `FWDE` writer - FIFO Warning DMA Enable"]
pub type FWDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCSR_SPEC, FWDE_A, O>;
impl<'a, const O: u8> FWDE_W<'a, O> {
    #[doc = "Disables the DMA request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FWDE_A::_0)
    }
    #[doc = "Enables the DMA request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FWDE_A::_1)
    }
}
#[doc = "Field `FRIE` reader - FIFO Request Interrupt Enable"]
pub type FRIE_R = crate::BitReader<FRIE_A>;
#[doc = "FIFO Request Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRIE_A {
    #[doc = "0: Disables the interrupt."]
    _0 = 0,
    #[doc = "1: Enables the interrupt."]
    _1 = 1,
}
impl From<FRIE_A> for bool {
    #[inline(always)]
    fn from(variant: FRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl FRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRIE_A {
        match self.bits {
            false => FRIE_A::_0,
            true => FRIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRIE_A::_1
    }
}
#[doc = "Field `FRIE` writer - FIFO Request Interrupt Enable"]
pub type FRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCSR_SPEC, FRIE_A, O>;
impl<'a, const O: u8> FRIE_W<'a, O> {
    #[doc = "Disables the interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FRIE_A::_0)
    }
    #[doc = "Enables the interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FRIE_A::_1)
    }
}
#[doc = "Field `FWIE` reader - FIFO Warning Interrupt Enable"]
pub type FWIE_R = crate::BitReader<FWIE_A>;
#[doc = "FIFO Warning Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWIE_A {
    #[doc = "0: Disables the interrupt."]
    _0 = 0,
    #[doc = "1: Enables the interrupt."]
    _1 = 1,
}
impl From<FWIE_A> for bool {
    #[inline(always)]
    fn from(variant: FWIE_A) -> Self {
        variant as u8 != 0
    }
}
impl FWIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWIE_A {
        match self.bits {
            false => FWIE_A::_0,
            true => FWIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FWIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FWIE_A::_1
    }
}
#[doc = "Field `FWIE` writer - FIFO Warning Interrupt Enable"]
pub type FWIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCSR_SPEC, FWIE_A, O>;
impl<'a, const O: u8> FWIE_W<'a, O> {
    #[doc = "Disables the interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FWIE_A::_0)
    }
    #[doc = "Enables the interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FWIE_A::_1)
    }
}
#[doc = "Field `FEIE` reader - FIFO Error Interrupt Enable"]
pub type FEIE_R = crate::BitReader<FEIE_A>;
#[doc = "FIFO Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEIE_A {
    #[doc = "0: Disables the interrupt."]
    _0 = 0,
    #[doc = "1: Enables the interrupt."]
    _1 = 1,
}
impl From<FEIE_A> for bool {
    #[inline(always)]
    fn from(variant: FEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl FEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIE_A {
        match self.bits {
            false => FEIE_A::_0,
            true => FEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FEIE_A::_1
    }
}
#[doc = "Field `FEIE` writer - FIFO Error Interrupt Enable"]
pub type FEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCSR_SPEC, FEIE_A, O>;
impl<'a, const O: u8> FEIE_W<'a, O> {
    #[doc = "Disables the interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEIE_A::_0)
    }
    #[doc = "Enables the interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEIE_A::_1)
    }
}
#[doc = "Field `SEIE` reader - Sync Error Interrupt Enable"]
pub type SEIE_R = crate::BitReader<SEIE_A>;
#[doc = "Sync Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEIE_A {
    #[doc = "0: Disables interrupt."]
    _0 = 0,
    #[doc = "1: Enables interrupt."]
    _1 = 1,
}
impl From<SEIE_A> for bool {
    #[inline(always)]
    fn from(variant: SEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEIE_A {
        match self.bits {
            false => SEIE_A::_0,
            true => SEIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEIE_A::_1
    }
}
#[doc = "Field `SEIE` writer - Sync Error Interrupt Enable"]
pub type SEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCSR_SPEC, SEIE_A, O>;
impl<'a, const O: u8> SEIE_W<'a, O> {
    #[doc = "Disables interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEIE_A::_0)
    }
    #[doc = "Enables interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEIE_A::_1)
    }
}
#[doc = "Field `WSIE` reader - Word Start Interrupt Enable"]
pub type WSIE_R = crate::BitReader<WSIE_A>;
#[doc = "Word Start Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WSIE_A {
    #[doc = "0: Disables interrupt."]
    _0 = 0,
    #[doc = "1: Enables interrupt."]
    _1 = 1,
}
impl From<WSIE_A> for bool {
    #[inline(always)]
    fn from(variant: WSIE_A) -> Self {
        variant as u8 != 0
    }
}
impl WSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WSIE_A {
        match self.bits {
            false => WSIE_A::_0,
            true => WSIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WSIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WSIE_A::_1
    }
}
#[doc = "Field `WSIE` writer - Word Start Interrupt Enable"]
pub type WSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCSR_SPEC, WSIE_A, O>;
impl<'a, const O: u8> WSIE_W<'a, O> {
    #[doc = "Disables interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WSIE_A::_0)
    }
    #[doc = "Enables interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WSIE_A::_1)
    }
}
#[doc = "Field `FRF` reader - FIFO Request Flag"]
pub type FRF_R = crate::BitReader<FRF_A>;
#[doc = "FIFO Request Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRF_A {
    #[doc = "0: Receive FIFO watermark not reached."]
    _0 = 0,
    #[doc = "1: Receive FIFO watermark has been reached."]
    _1 = 1,
}
impl From<FRF_A> for bool {
    #[inline(always)]
    fn from(variant: FRF_A) -> Self {
        variant as u8 != 0
    }
}
impl FRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRF_A {
        match self.bits {
            false => FRF_A::_0,
            true => FRF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FRF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FRF_A::_1
    }
}
#[doc = "Field `FWF` reader - FIFO Warning Flag"]
pub type FWF_R = crate::BitReader<FWF_A>;
#[doc = "FIFO Warning Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FWF_A {
    #[doc = "0: No enabled receive FIFO is full."]
    _0 = 0,
    #[doc = "1: Enabled receive FIFO is full."]
    _1 = 1,
}
impl From<FWF_A> for bool {
    #[inline(always)]
    fn from(variant: FWF_A) -> Self {
        variant as u8 != 0
    }
}
impl FWF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FWF_A {
        match self.bits {
            false => FWF_A::_0,
            true => FWF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FWF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FWF_A::_1
    }
}
#[doc = "Field `FEF` reader - FIFO Error Flag"]
pub type FEF_R = crate::BitReader<FEF_A>;
#[doc = "FIFO Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEF_A {
    #[doc = "0: Receive overflow not detected."]
    _0 = 0,
    #[doc = "1: Receive overflow detected."]
    _1 = 1,
}
impl From<FEF_A> for bool {
    #[inline(always)]
    fn from(variant: FEF_A) -> Self {
        variant as u8 != 0
    }
}
impl FEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEF_A {
        match self.bits {
            false => FEF_A::_0,
            true => FEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FEF_A::_1
    }
}
#[doc = "Field `FEF` writer - FIFO Error Flag"]
pub type FEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCSR_SPEC, FEF_A, O>;
impl<'a, const O: u8> FEF_W<'a, O> {
    #[doc = "Receive overflow not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FEF_A::_0)
    }
    #[doc = "Receive overflow detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FEF_A::_1)
    }
}
#[doc = "Field `SEF` reader - Sync Error Flag"]
pub type SEF_R = crate::BitReader<SEF_A>;
#[doc = "Sync Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEF_A {
    #[doc = "0: Sync error not detected."]
    _0 = 0,
    #[doc = "1: Frame sync error detected."]
    _1 = 1,
}
impl From<SEF_A> for bool {
    #[inline(always)]
    fn from(variant: SEF_A) -> Self {
        variant as u8 != 0
    }
}
impl SEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEF_A {
        match self.bits {
            false => SEF_A::_0,
            true => SEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SEF_A::_1
    }
}
#[doc = "Field `SEF` writer - Sync Error Flag"]
pub type SEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCSR_SPEC, SEF_A, O>;
impl<'a, const O: u8> SEF_W<'a, O> {
    #[doc = "Sync error not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SEF_A::_0)
    }
    #[doc = "Frame sync error detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SEF_A::_1)
    }
}
#[doc = "Field `WSF` reader - Word Start Flag"]
pub type WSF_R = crate::BitReader<WSF_A>;
#[doc = "Word Start Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WSF_A {
    #[doc = "0: Start of word not detected."]
    _0 = 0,
    #[doc = "1: Start of word detected."]
    _1 = 1,
}
impl From<WSF_A> for bool {
    #[inline(always)]
    fn from(variant: WSF_A) -> Self {
        variant as u8 != 0
    }
}
impl WSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WSF_A {
        match self.bits {
            false => WSF_A::_0,
            true => WSF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WSF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WSF_A::_1
    }
}
#[doc = "Field `WSF` writer - Word Start Flag"]
pub type WSF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCSR_SPEC, WSF_A, O>;
impl<'a, const O: u8> WSF_W<'a, O> {
    #[doc = "Start of word not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WSF_A::_0)
    }
    #[doc = "Start of word detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WSF_A::_1)
    }
}
#[doc = "Field `SR` reader - Software Reset"]
pub type SR_R = crate::BitReader<SR_A>;
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR_A {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Software reset."]
    _1 = 1,
}
impl From<SR_A> for bool {
    #[inline(always)]
    fn from(variant: SR_A) -> Self {
        variant as u8 != 0
    }
}
impl SR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR_A {
        match self.bits {
            false => SR_A::_0,
            true => SR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SR_A::_1
    }
}
#[doc = "Field `SR` writer - Software Reset"]
pub type SR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCSR_SPEC, SR_A, O>;
impl<'a, const O: u8> SR_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SR_A::_0)
    }
    #[doc = "Software reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SR_A::_1)
    }
}
#[doc = "FIFO Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FR_AW {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: FIFO reset."]
    _1 = 1,
}
impl From<FR_AW> for bool {
    #[inline(always)]
    fn from(variant: FR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FR` writer - FIFO Reset"]
pub type FR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCSR_SPEC, FR_AW, O>;
impl<'a, const O: u8> FR_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FR_AW::_0)
    }
    #[doc = "FIFO reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FR_AW::_1)
    }
}
#[doc = "Field `BCE` reader - Bit Clock Enable"]
pub type BCE_R = crate::BitReader<BCE_A>;
#[doc = "Bit Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCE_A {
    #[doc = "0: Receive bit clock is disabled."]
    _0 = 0,
    #[doc = "1: Receive bit clock is enabled."]
    _1 = 1,
}
impl From<BCE_A> for bool {
    #[inline(always)]
    fn from(variant: BCE_A) -> Self {
        variant as u8 != 0
    }
}
impl BCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCE_A {
        match self.bits {
            false => BCE_A::_0,
            true => BCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BCE_A::_1
    }
}
#[doc = "Field `BCE` writer - Bit Clock Enable"]
pub type BCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCSR_SPEC, BCE_A, O>;
impl<'a, const O: u8> BCE_W<'a, O> {
    #[doc = "Receive bit clock is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BCE_A::_0)
    }
    #[doc = "Receive bit clock is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BCE_A::_1)
    }
}
#[doc = "Field `DBGE` reader - Debug Enable"]
pub type DBGE_R = crate::BitReader<DBGE_A>;
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGE_A {
    #[doc = "0: Receiver is disabled in Debug mode, after completing the current frame."]
    _0 = 0,
    #[doc = "1: Receiver is enabled in Debug mode."]
    _1 = 1,
}
impl From<DBGE_A> for bool {
    #[inline(always)]
    fn from(variant: DBGE_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGE_A {
        match self.bits {
            false => DBGE_A::_0,
            true => DBGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGE_A::_1
    }
}
#[doc = "Field `DBGE` writer - Debug Enable"]
pub type DBGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCSR_SPEC, DBGE_A, O>;
impl<'a, const O: u8> DBGE_W<'a, O> {
    #[doc = "Receiver is disabled in Debug mode, after completing the current frame."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGE_A::_0)
    }
    #[doc = "Receiver is enabled in Debug mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGE_A::_1)
    }
}
#[doc = "Field `STOPE` reader - Stop Enable"]
pub type STOPE_R = crate::BitReader<STOPE_A>;
#[doc = "Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPE_A {
    #[doc = "0: Receiver disabled in Stop mode."]
    _0 = 0,
    #[doc = "1: Receiver enabled in Stop mode."]
    _1 = 1,
}
impl From<STOPE_A> for bool {
    #[inline(always)]
    fn from(variant: STOPE_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPE_A {
        match self.bits {
            false => STOPE_A::_0,
            true => STOPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STOPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STOPE_A::_1
    }
}
#[doc = "Field `STOPE` writer - Stop Enable"]
pub type STOPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCSR_SPEC, STOPE_A, O>;
impl<'a, const O: u8> STOPE_W<'a, O> {
    #[doc = "Receiver disabled in Stop mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STOPE_A::_0)
    }
    #[doc = "Receiver enabled in Stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STOPE_A::_1)
    }
}
#[doc = "Field `RE` reader - Receiver Enable"]
pub type RE_R = crate::BitReader<RE_A>;
#[doc = "Receiver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE_A {
    #[doc = "0: Receiver is disabled."]
    _0 = 0,
    #[doc = "1: Receiver is enabled, or receiver has been disabled and has not yet reached end of frame."]
    _1 = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
impl RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::_0,
            true => RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RE_A::_1
    }
}
#[doc = "Field `RE` writer - Receiver Enable"]
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCSR_SPEC, RE_A, O>;
impl<'a, const O: u8> RE_W<'a, O> {
    #[doc = "Receiver is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RE_A::_0)
    }
    #[doc = "Receiver is enabled, or receiver has been disabled and has not yet reached end of frame."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - FIFO Request DMA Enable"]
    #[inline(always)]
    pub fn frde(&self) -> FRDE_R {
        FRDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Warning DMA Enable"]
    #[inline(always)]
    pub fn fwde(&self) -> FWDE_R {
        FWDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - FIFO Request Interrupt Enable"]
    #[inline(always)]
    pub fn frie(&self) -> FRIE_R {
        FRIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FIFO Warning Interrupt Enable"]
    #[inline(always)]
    pub fn fwie(&self) -> FWIE_R {
        FWIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Sync Error Interrupt Enable"]
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Word Start Interrupt Enable"]
    #[inline(always)]
    pub fn wsie(&self) -> WSIE_R {
        WSIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - FIFO Request Flag"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FIFO Warning Flag"]
    #[inline(always)]
    pub fn fwf(&self) -> FWF_R {
        FWF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FIFO Error Flag"]
    #[inline(always)]
    pub fn fef(&self) -> FEF_R {
        FEF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Sync Error Flag"]
    #[inline(always)]
    pub fn sef(&self) -> SEF_R {
        SEF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Word Start Flag"]
    #[inline(always)]
    pub fn wsf(&self) -> WSF_R {
        WSF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Software Reset"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Bit Clock Enable"]
    #[inline(always)]
    pub fn bce(&self) -> BCE_R {
        BCE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Debug Enable"]
    #[inline(always)]
    pub fn dbge(&self) -> DBGE_R {
        DBGE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Stop Enable"]
    #[inline(always)]
    pub fn stope(&self) -> STOPE_R {
        STOPE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO Request DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frde(&mut self) -> FRDE_W<0> {
        FRDE_W::new(self)
    }
    #[doc = "Bit 1 - FIFO Warning DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fwde(&mut self) -> FWDE_W<1> {
        FWDE_W::new(self)
    }
    #[doc = "Bit 8 - FIFO Request Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn frie(&mut self) -> FRIE_W<8> {
        FRIE_W::new(self)
    }
    #[doc = "Bit 9 - FIFO Warning Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fwie(&mut self) -> FWIE_W<9> {
        FWIE_W::new(self)
    }
    #[doc = "Bit 10 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FEIE_W<10> {
        FEIE_W::new(self)
    }
    #[doc = "Bit 11 - Sync Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn seie(&mut self) -> SEIE_W<11> {
        SEIE_W::new(self)
    }
    #[doc = "Bit 12 - Word Start Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wsie(&mut self) -> WSIE_W<12> {
        WSIE_W::new(self)
    }
    #[doc = "Bit 18 - FIFO Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fef(&mut self) -> FEF_W<18> {
        FEF_W::new(self)
    }
    #[doc = "Bit 19 - Sync Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sef(&mut self) -> SEF_W<19> {
        SEF_W::new(self)
    }
    #[doc = "Bit 20 - Word Start Flag"]
    #[inline(always)]
    #[must_use]
    pub fn wsf(&mut self) -> WSF_W<20> {
        WSF_W::new(self)
    }
    #[doc = "Bit 24 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<24> {
        SR_W::new(self)
    }
    #[doc = "Bit 25 - FIFO Reset"]
    #[inline(always)]
    #[must_use]
    pub fn fr(&mut self) -> FR_W<25> {
        FR_W::new(self)
    }
    #[doc = "Bit 28 - Bit Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bce(&mut self) -> BCE_W<28> {
        BCE_W::new(self)
    }
    #[doc = "Bit 29 - Debug Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbge(&mut self) -> DBGE_W<29> {
        DBGE_W::new(self)
    }
    #[doc = "Bit 30 - Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stope(&mut self) -> STOPE_W<30> {
        STOPE_W::new(self)
    }
    #[doc = "Bit 31 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<31> {
        RE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI Receive Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcsr](index.html) module"]
pub struct RCSR_SPEC;
impl crate::RegisterSpec for RCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcsr::R](R) reader structure"]
impl crate::Readable for RCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcsr::W](W) writer structure"]
impl crate::Writable for RCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCSR to value 0"]
impl crate::Resettable for RCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
