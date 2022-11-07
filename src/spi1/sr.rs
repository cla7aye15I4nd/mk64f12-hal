#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POPNXTPTR` reader - Pop Next Pointer"]
pub type POPNXTPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXCTR` reader - RX FIFO Counter"]
pub type RXCTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXNXTPTR` reader - Transmit Next Pointer"]
pub type TXNXTPTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXCTR` reader - TX FIFO Counter"]
pub type TXCTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFDF` reader - Receive FIFO Drain Flag"]
pub type RFDF_R = crate::BitReader<RFDF_A>;
#[doc = "Receive FIFO Drain Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFDF_A {
    #[doc = "0: RX FIFO is empty."]
    _0 = 0,
    #[doc = "1: RX FIFO is not empty."]
    _1 = 1,
}
impl From<RFDF_A> for bool {
    #[inline(always)]
    fn from(variant: RFDF_A) -> Self {
        variant as u8 != 0
    }
}
impl RFDF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFDF_A {
        match self.bits {
            false => RFDF_A::_0,
            true => RFDF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFDF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFDF_A::_1
    }
}
#[doc = "Field `RFDF` writer - Receive FIFO Drain Flag"]
pub type RFDF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, RFDF_A, O>;
impl<'a, const O: u8> RFDF_W<'a, O> {
    #[doc = "RX FIFO is empty."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFDF_A::_0)
    }
    #[doc = "RX FIFO is not empty."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFDF_A::_1)
    }
}
#[doc = "Field `RFOF` reader - Receive FIFO Overflow Flag"]
pub type RFOF_R = crate::BitReader<RFOF_A>;
#[doc = "Receive FIFO Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFOF_A {
    #[doc = "0: No Rx FIFO overflow."]
    _0 = 0,
    #[doc = "1: Rx FIFO overflow has occurred."]
    _1 = 1,
}
impl From<RFOF_A> for bool {
    #[inline(always)]
    fn from(variant: RFOF_A) -> Self {
        variant as u8 != 0
    }
}
impl RFOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFOF_A {
        match self.bits {
            false => RFOF_A::_0,
            true => RFOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFOF_A::_1
    }
}
#[doc = "Field `RFOF` writer - Receive FIFO Overflow Flag"]
pub type RFOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, RFOF_A, O>;
impl<'a, const O: u8> RFOF_W<'a, O> {
    #[doc = "No Rx FIFO overflow."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFOF_A::_0)
    }
    #[doc = "Rx FIFO overflow has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFOF_A::_1)
    }
}
#[doc = "Field `TFFF` reader - Transmit FIFO Fill Flag"]
pub type TFFF_R = crate::BitReader<TFFF_A>;
#[doc = "Transmit FIFO Fill Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFFF_A {
    #[doc = "0: TX FIFO is full."]
    _0 = 0,
    #[doc = "1: TX FIFO is not full."]
    _1 = 1,
}
impl From<TFFF_A> for bool {
    #[inline(always)]
    fn from(variant: TFFF_A) -> Self {
        variant as u8 != 0
    }
}
impl TFFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFFF_A {
        match self.bits {
            false => TFFF_A::_0,
            true => TFFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFFF_A::_1
    }
}
#[doc = "Field `TFFF` writer - Transmit FIFO Fill Flag"]
pub type TFFF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, TFFF_A, O>;
impl<'a, const O: u8> TFFF_W<'a, O> {
    #[doc = "TX FIFO is full."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFFF_A::_0)
    }
    #[doc = "TX FIFO is not full."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFFF_A::_1)
    }
}
#[doc = "Field `TFUF` reader - Transmit FIFO Underflow Flag"]
pub type TFUF_R = crate::BitReader<TFUF_A>;
#[doc = "Transmit FIFO Underflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFUF_A {
    #[doc = "0: No TX FIFO underflow."]
    _0 = 0,
    #[doc = "1: TX FIFO underflow has occurred."]
    _1 = 1,
}
impl From<TFUF_A> for bool {
    #[inline(always)]
    fn from(variant: TFUF_A) -> Self {
        variant as u8 != 0
    }
}
impl TFUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFUF_A {
        match self.bits {
            false => TFUF_A::_0,
            true => TFUF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFUF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFUF_A::_1
    }
}
#[doc = "Field `TFUF` writer - Transmit FIFO Underflow Flag"]
pub type TFUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, TFUF_A, O>;
impl<'a, const O: u8> TFUF_W<'a, O> {
    #[doc = "No TX FIFO underflow."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFUF_A::_0)
    }
    #[doc = "TX FIFO underflow has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFUF_A::_1)
    }
}
#[doc = "Field `EOQF` reader - End of Queue Flag"]
pub type EOQF_R = crate::BitReader<EOQF_A>;
#[doc = "End of Queue Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOQF_A {
    #[doc = "0: EOQ is not set in the executing command."]
    _0 = 0,
    #[doc = "1: EOQ is set in the executing SPI command."]
    _1 = 1,
}
impl From<EOQF_A> for bool {
    #[inline(always)]
    fn from(variant: EOQF_A) -> Self {
        variant as u8 != 0
    }
}
impl EOQF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOQF_A {
        match self.bits {
            false => EOQF_A::_0,
            true => EOQF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOQF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOQF_A::_1
    }
}
#[doc = "Field `EOQF` writer - End of Queue Flag"]
pub type EOQF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, EOQF_A, O>;
impl<'a, const O: u8> EOQF_W<'a, O> {
    #[doc = "EOQ is not set in the executing command."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOQF_A::_0)
    }
    #[doc = "EOQ is set in the executing SPI command."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOQF_A::_1)
    }
}
#[doc = "Field `TXRXS` reader - TX and RX Status"]
pub type TXRXS_R = crate::BitReader<TXRXS_A>;
#[doc = "TX and RX Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXRXS_A {
    #[doc = "0: Transmit and receive operations are disabled (The module is in Stopped state)."]
    _0 = 0,
    #[doc = "1: Transmit and receive operations are enabled (The module is in Running state)."]
    _1 = 1,
}
impl From<TXRXS_A> for bool {
    #[inline(always)]
    fn from(variant: TXRXS_A) -> Self {
        variant as u8 != 0
    }
}
impl TXRXS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXRXS_A {
        match self.bits {
            false => TXRXS_A::_0,
            true => TXRXS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXRXS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXRXS_A::_1
    }
}
#[doc = "Field `TXRXS` writer - TX and RX Status"]
pub type TXRXS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, TXRXS_A, O>;
impl<'a, const O: u8> TXRXS_W<'a, O> {
    #[doc = "Transmit and receive operations are disabled (The module is in Stopped state)."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXRXS_A::_0)
    }
    #[doc = "Transmit and receive operations are enabled (The module is in Running state)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXRXS_A::_1)
    }
}
#[doc = "Field `TCF` reader - Transfer Complete Flag"]
pub type TCF_R = crate::BitReader<TCF_A>;
#[doc = "Transfer Complete Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCF_A {
    #[doc = "0: Transfer not complete."]
    _0 = 0,
    #[doc = "1: Transfer complete."]
    _1 = 1,
}
impl From<TCF_A> for bool {
    #[inline(always)]
    fn from(variant: TCF_A) -> Self {
        variant as u8 != 0
    }
}
impl TCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCF_A {
        match self.bits {
            false => TCF_A::_0,
            true => TCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCF_A::_1
    }
}
#[doc = "Field `TCF` writer - Transfer Complete Flag"]
pub type TCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, TCF_A, O>;
impl<'a, const O: u8> TCF_W<'a, O> {
    #[doc = "Transfer not complete."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCF_A::_0)
    }
    #[doc = "Transfer complete."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCF_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Pop Next Pointer"]
    #[inline(always)]
    pub fn popnxtptr(&self) -> POPNXTPTR_R {
        POPNXTPTR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RX FIFO Counter"]
    #[inline(always)]
    pub fn rxctr(&self) -> RXCTR_R {
        RXCTR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Transmit Next Pointer"]
    #[inline(always)]
    pub fn txnxtptr(&self) -> TXNXTPTR_R {
        TXNXTPTR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - TX FIFO Counter"]
    #[inline(always)]
    pub fn txctr(&self) -> TXCTR_R {
        TXCTR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - Receive FIFO Drain Flag"]
    #[inline(always)]
    pub fn rfdf(&self) -> RFDF_R {
        RFDF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO Overflow Flag"]
    #[inline(always)]
    pub fn rfof(&self) -> RFOF_R {
        RFOF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 25 - Transmit FIFO Fill Flag"]
    #[inline(always)]
    pub fn tfff(&self) -> TFFF_R {
        TFFF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit FIFO Underflow Flag"]
    #[inline(always)]
    pub fn tfuf(&self) -> TFUF_R {
        TFUF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - End of Queue Flag"]
    #[inline(always)]
    pub fn eoqf(&self) -> EOQF_R {
        EOQF_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - TX and RX Status"]
    #[inline(always)]
    pub fn txrxs(&self) -> TXRXS_R {
        TXRXS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Transfer Complete Flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Receive FIFO Drain Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rfdf(&mut self) -> RFDF_W<17> {
        RFDF_W::new(self)
    }
    #[doc = "Bit 19 - Receive FIFO Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rfof(&mut self) -> RFOF_W<19> {
        RFOF_W::new(self)
    }
    #[doc = "Bit 25 - Transmit FIFO Fill Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tfff(&mut self) -> TFFF_W<25> {
        TFFF_W::new(self)
    }
    #[doc = "Bit 27 - Transmit FIFO Underflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tfuf(&mut self) -> TFUF_W<27> {
        TFUF_W::new(self)
    }
    #[doc = "Bit 28 - End of Queue Flag"]
    #[inline(always)]
    #[must_use]
    pub fn eoqf(&mut self) -> EOQF_W<28> {
        EOQF_W::new(self)
    }
    #[doc = "Bit 30 - TX and RX Status"]
    #[inline(always)]
    #[must_use]
    pub fn txrxs(&mut self) -> TXRXS_W<30> {
        TXRXS_W::new(self)
    }
    #[doc = "Bit 31 - Transfer Complete Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<31> {
        TCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0x0200_0000"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200_0000;
}
