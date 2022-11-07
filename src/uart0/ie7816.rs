#[doc = "Register `IE7816` reader"]
pub struct R(crate::R<IE7816_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IE7816_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IE7816_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IE7816_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IE7816` writer"]
pub struct W(crate::W<IE7816_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IE7816_SPEC>;
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
impl From<crate::W<IE7816_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IE7816_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXTE` reader - Receive Threshold Exceeded Interrupt Enable"]
pub type RXTE_R = crate::BitReader<RXTE_A>;
#[doc = "Receive Threshold Exceeded Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXTE_A {
    #[doc = "0: The assertion of IS7816\\[RXT\\]
does not result in the generation of an interrupt."]
    _0 = 0,
    #[doc = "1: The assertion of IS7816\\[RXT\\]
results in the generation of an interrupt."]
    _1 = 1,
}
impl From<RXTE_A> for bool {
    #[inline(always)]
    fn from(variant: RXTE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXTE_A {
        match self.bits {
            false => RXTE_A::_0,
            true => RXTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXTE_A::_1
    }
}
#[doc = "Field `RXTE` writer - Receive Threshold Exceeded Interrupt Enable"]
pub type RXTE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IE7816_SPEC, RXTE_A, O>;
impl<'a, const O: u8> RXTE_W<'a, O> {
    #[doc = "The assertion of IS7816\\[RXT\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXTE_A::_0)
    }
    #[doc = "The assertion of IS7816\\[RXT\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXTE_A::_1)
    }
}
#[doc = "Field `TXTE` reader - Transmit Threshold Exceeded Interrupt Enable"]
pub type TXTE_R = crate::BitReader<TXTE_A>;
#[doc = "Transmit Threshold Exceeded Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXTE_A {
    #[doc = "0: The assertion of IS7816\\[TXT\\]
does not result in the generation of an interrupt."]
    _0 = 0,
    #[doc = "1: The assertion of IS7816\\[TXT\\]
results in the generation of an interrupt."]
    _1 = 1,
}
impl From<TXTE_A> for bool {
    #[inline(always)]
    fn from(variant: TXTE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXTE_A {
        match self.bits {
            false => TXTE_A::_0,
            true => TXTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXTE_A::_1
    }
}
#[doc = "Field `TXTE` writer - Transmit Threshold Exceeded Interrupt Enable"]
pub type TXTE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IE7816_SPEC, TXTE_A, O>;
impl<'a, const O: u8> TXTE_W<'a, O> {
    #[doc = "The assertion of IS7816\\[TXT\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXTE_A::_0)
    }
    #[doc = "The assertion of IS7816\\[TXT\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXTE_A::_1)
    }
}
#[doc = "Field `GTVE` reader - Guard Timer Violated Interrupt Enable"]
pub type GTVE_R = crate::BitReader<GTVE_A>;
#[doc = "Guard Timer Violated Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GTVE_A {
    #[doc = "0: The assertion of IS7816\\[GTV\\]
does not result in the generation of an interrupt."]
    _0 = 0,
    #[doc = "1: The assertion of IS7816\\[GTV\\]
results in the generation of an interrupt."]
    _1 = 1,
}
impl From<GTVE_A> for bool {
    #[inline(always)]
    fn from(variant: GTVE_A) -> Self {
        variant as u8 != 0
    }
}
impl GTVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GTVE_A {
        match self.bits {
            false => GTVE_A::_0,
            true => GTVE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GTVE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GTVE_A::_1
    }
}
#[doc = "Field `GTVE` writer - Guard Timer Violated Interrupt Enable"]
pub type GTVE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IE7816_SPEC, GTVE_A, O>;
impl<'a, const O: u8> GTVE_W<'a, O> {
    #[doc = "The assertion of IS7816\\[GTV\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GTVE_A::_0)
    }
    #[doc = "The assertion of IS7816\\[GTV\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GTVE_A::_1)
    }
}
#[doc = "Field `INITDE` reader - Initial Character Detected Interrupt Enable"]
pub type INITDE_R = crate::BitReader<INITDE_A>;
#[doc = "Initial Character Detected Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITDE_A {
    #[doc = "0: The assertion of IS7816\\[INITD\\]
does not result in the generation of an interrupt."]
    _0 = 0,
    #[doc = "1: The assertion of IS7816\\[INITD\\]
results in the generation of an interrupt."]
    _1 = 1,
}
impl From<INITDE_A> for bool {
    #[inline(always)]
    fn from(variant: INITDE_A) -> Self {
        variant as u8 != 0
    }
}
impl INITDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITDE_A {
        match self.bits {
            false => INITDE_A::_0,
            true => INITDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INITDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INITDE_A::_1
    }
}
#[doc = "Field `INITDE` writer - Initial Character Detected Interrupt Enable"]
pub type INITDE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IE7816_SPEC, INITDE_A, O>;
impl<'a, const O: u8> INITDE_W<'a, O> {
    #[doc = "The assertion of IS7816\\[INITD\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INITDE_A::_0)
    }
    #[doc = "The assertion of IS7816\\[INITD\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INITDE_A::_1)
    }
}
#[doc = "Field `BWTE` reader - Block Wait Timer Interrupt Enable"]
pub type BWTE_R = crate::BitReader<BWTE_A>;
#[doc = "Block Wait Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWTE_A {
    #[doc = "0: The assertion of IS7816\\[BWT\\]
does not result in the generation of an interrupt."]
    _0 = 0,
    #[doc = "1: The assertion of IS7816\\[BWT\\]
results in the generation of an interrupt."]
    _1 = 1,
}
impl From<BWTE_A> for bool {
    #[inline(always)]
    fn from(variant: BWTE_A) -> Self {
        variant as u8 != 0
    }
}
impl BWTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWTE_A {
        match self.bits {
            false => BWTE_A::_0,
            true => BWTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BWTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BWTE_A::_1
    }
}
#[doc = "Field `BWTE` writer - Block Wait Timer Interrupt Enable"]
pub type BWTE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IE7816_SPEC, BWTE_A, O>;
impl<'a, const O: u8> BWTE_W<'a, O> {
    #[doc = "The assertion of IS7816\\[BWT\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWTE_A::_0)
    }
    #[doc = "The assertion of IS7816\\[BWT\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWTE_A::_1)
    }
}
#[doc = "Field `CWTE` reader - Character Wait Timer Interrupt Enable"]
pub type CWTE_R = crate::BitReader<CWTE_A>;
#[doc = "Character Wait Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWTE_A {
    #[doc = "0: The assertion of IS7816\\[CWT\\]
does not result in the generation of an interrupt."]
    _0 = 0,
    #[doc = "1: The assertion of IS7816\\[CWT\\]
results in the generation of an interrupt."]
    _1 = 1,
}
impl From<CWTE_A> for bool {
    #[inline(always)]
    fn from(variant: CWTE_A) -> Self {
        variant as u8 != 0
    }
}
impl CWTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CWTE_A {
        match self.bits {
            false => CWTE_A::_0,
            true => CWTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CWTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CWTE_A::_1
    }
}
#[doc = "Field `CWTE` writer - Character Wait Timer Interrupt Enable"]
pub type CWTE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IE7816_SPEC, CWTE_A, O>;
impl<'a, const O: u8> CWTE_W<'a, O> {
    #[doc = "The assertion of IS7816\\[CWT\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CWTE_A::_0)
    }
    #[doc = "The assertion of IS7816\\[CWT\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CWTE_A::_1)
    }
}
#[doc = "Field `WTE` reader - Wait Timer Interrupt Enable"]
pub type WTE_R = crate::BitReader<WTE_A>;
#[doc = "Wait Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WTE_A {
    #[doc = "0: The assertion of IS7816\\[WT\\]
does not result in the generation of an interrupt."]
    _0 = 0,
    #[doc = "1: The assertion of IS7816\\[WT\\]
results in the generation of an interrupt."]
    _1 = 1,
}
impl From<WTE_A> for bool {
    #[inline(always)]
    fn from(variant: WTE_A) -> Self {
        variant as u8 != 0
    }
}
impl WTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTE_A {
        match self.bits {
            false => WTE_A::_0,
            true => WTE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WTE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WTE_A::_1
    }
}
#[doc = "Field `WTE` writer - Wait Timer Interrupt Enable"]
pub type WTE_W<'a, const O: u8> = crate::BitWriter<'a, u8, IE7816_SPEC, WTE_A, O>;
impl<'a, const O: u8> WTE_W<'a, O> {
    #[doc = "The assertion of IS7816\\[WT\\]
does not result in the generation of an interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WTE_A::_0)
    }
    #[doc = "The assertion of IS7816\\[WT\\]
results in the generation of an interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WTE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive Threshold Exceeded Interrupt Enable"]
    #[inline(always)]
    pub fn rxte(&self) -> RXTE_R {
        RXTE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Threshold Exceeded Interrupt Enable"]
    #[inline(always)]
    pub fn txte(&self) -> TXTE_R {
        TXTE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Guard Timer Violated Interrupt Enable"]
    #[inline(always)]
    pub fn gtve(&self) -> GTVE_R {
        GTVE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Initial Character Detected Interrupt Enable"]
    #[inline(always)]
    pub fn initde(&self) -> INITDE_R {
        INITDE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Block Wait Timer Interrupt Enable"]
    #[inline(always)]
    pub fn bwte(&self) -> BWTE_R {
        BWTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Character Wait Timer Interrupt Enable"]
    #[inline(always)]
    pub fn cwte(&self) -> CWTE_R {
        CWTE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wait Timer Interrupt Enable"]
    #[inline(always)]
    pub fn wte(&self) -> WTE_R {
        WTE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Threshold Exceeded Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxte(&mut self) -> RXTE_W<0> {
        RXTE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Threshold Exceeded Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txte(&mut self) -> TXTE_W<1> {
        TXTE_W::new(self)
    }
    #[doc = "Bit 2 - Guard Timer Violated Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gtve(&mut self) -> GTVE_W<2> {
        GTVE_W::new(self)
    }
    #[doc = "Bit 4 - Initial Character Detected Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn initde(&mut self) -> INITDE_W<4> {
        INITDE_W::new(self)
    }
    #[doc = "Bit 5 - Block Wait Timer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bwte(&mut self) -> BWTE_W<5> {
        BWTE_W::new(self)
    }
    #[doc = "Bit 6 - Character Wait Timer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cwte(&mut self) -> CWTE_W<6> {
        CWTE_W::new(self)
    }
    #[doc = "Bit 7 - Wait Timer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wte(&mut self) -> WTE_W<7> {
        WTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART 7816 Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ie7816](index.html) module"]
pub struct IE7816_SPEC;
impl crate::RegisterSpec for IE7816_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ie7816::R](R) reader structure"]
impl crate::Readable for IE7816_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ie7816::W](W) writer structure"]
impl crate::Writable for IE7816_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IE7816 to value 0"]
impl crate::Resettable for IE7816_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
