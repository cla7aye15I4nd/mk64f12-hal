#[doc = "Register `BDH` reader"]
pub struct R(crate::R<BDH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDH` writer"]
pub struct W(crate::W<BDH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDH_SPEC>;
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
impl From<crate::W<BDH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBR` reader - UART Baud Rate Bits"]
pub type SBR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SBR` writer - UART Baud Rate Bits"]
pub type SBR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, BDH_SPEC, u8, u8, 5, O>;
#[doc = "Field `SBNS` reader - Stop Bit Number Select"]
pub type SBNS_R = crate::BitReader<SBNS_A>;
#[doc = "Stop Bit Number Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBNS_A {
    #[doc = "0: Data frame consists of a single stop bit."]
    _0 = 0,
    #[doc = "1: Data frame consists of two stop bits."]
    _1 = 1,
}
impl From<SBNS_A> for bool {
    #[inline(always)]
    fn from(variant: SBNS_A) -> Self {
        variant as u8 != 0
    }
}
impl SBNS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBNS_A {
        match self.bits {
            false => SBNS_A::_0,
            true => SBNS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBNS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBNS_A::_1
    }
}
#[doc = "Field `SBNS` writer - Stop Bit Number Select"]
pub type SBNS_W<'a, const O: u8> = crate::BitWriter<'a, u8, BDH_SPEC, SBNS_A, O>;
impl<'a, const O: u8> SBNS_W<'a, O> {
    #[doc = "Data frame consists of a single stop bit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBNS_A::_0)
    }
    #[doc = "Data frame consists of two stop bits."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBNS_A::_1)
    }
}
#[doc = "Field `RXEDGIE` reader - RxD Input Active Edge Interrupt Enable"]
pub type RXEDGIE_R = crate::BitReader<RXEDGIE_A>;
#[doc = "RxD Input Active Edge Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXEDGIE_A {
    #[doc = "0: Hardware interrupts from RXEDGIF disabled using polling."]
    _0 = 0,
    #[doc = "1: RXEDGIF interrupt request enabled."]
    _1 = 1,
}
impl From<RXEDGIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXEDGIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXEDGIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEDGIE_A {
        match self.bits {
            false => RXEDGIE_A::_0,
            true => RXEDGIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXEDGIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXEDGIE_A::_1
    }
}
#[doc = "Field `RXEDGIE` writer - RxD Input Active Edge Interrupt Enable"]
pub type RXEDGIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, BDH_SPEC, RXEDGIE_A, O>;
impl<'a, const O: u8> RXEDGIE_W<'a, O> {
    #[doc = "Hardware interrupts from RXEDGIF disabled using polling."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXEDGIE_A::_0)
    }
    #[doc = "RXEDGIF interrupt request enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXEDGIE_A::_1)
    }
}
#[doc = "Field `LBKDIE` reader - LIN Break Detect Interrupt or DMA Request Enable"]
pub type LBKDIE_R = crate::BitReader<LBKDIE_A>;
#[doc = "LIN Break Detect Interrupt or DMA Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBKDIE_A {
    #[doc = "0: LBKDIF interrupt and DMA transfer requests disabled."]
    _0 = 0,
    #[doc = "1: LBKDIF interrupt or DMA transfer requests enabled."]
    _1 = 1,
}
impl From<LBKDIE_A> for bool {
    #[inline(always)]
    fn from(variant: LBKDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LBKDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBKDIE_A {
        match self.bits {
            false => LBKDIE_A::_0,
            true => LBKDIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LBKDIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LBKDIE_A::_1
    }
}
#[doc = "Field `LBKDIE` writer - LIN Break Detect Interrupt or DMA Request Enable"]
pub type LBKDIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, BDH_SPEC, LBKDIE_A, O>;
impl<'a, const O: u8> LBKDIE_W<'a, O> {
    #[doc = "LBKDIF interrupt and DMA transfer requests disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDIE_A::_0)
    }
    #[doc = "LBKDIF interrupt or DMA transfer requests enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDIE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - UART Baud Rate Bits"]
    #[inline(always)]
    pub fn sbr(&self) -> SBR_R {
        SBR_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5 - Stop Bit Number Select"]
    #[inline(always)]
    pub fn sbns(&self) -> SBNS_R {
        SBNS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RxD Input Active Edge Interrupt Enable"]
    #[inline(always)]
    pub fn rxedgie(&self) -> RXEDGIE_R {
        RXEDGIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LIN Break Detect Interrupt or DMA Request Enable"]
    #[inline(always)]
    pub fn lbkdie(&self) -> LBKDIE_R {
        LBKDIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - UART Baud Rate Bits"]
    #[inline(always)]
    #[must_use]
    pub fn sbr(&mut self) -> SBR_W<0> {
        SBR_W::new(self)
    }
    #[doc = "Bit 5 - Stop Bit Number Select"]
    #[inline(always)]
    #[must_use]
    pub fn sbns(&mut self) -> SBNS_W<5> {
        SBNS_W::new(self)
    }
    #[doc = "Bit 6 - RxD Input Active Edge Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxedgie(&mut self) -> RXEDGIE_W<6> {
        RXEDGIE_W::new(self)
    }
    #[doc = "Bit 7 - LIN Break Detect Interrupt or DMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lbkdie(&mut self) -> LBKDIE_W<7> {
        LBKDIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Baud Rate Registers: High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdh](index.html) module"]
pub struct BDH_SPEC;
impl crate::RegisterSpec for BDH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [bdh::R](R) reader structure"]
impl crate::Readable for BDH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdh::W](W) writer structure"]
impl crate::Writable for BDH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BDH to value 0"]
impl crate::Resettable for BDH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
