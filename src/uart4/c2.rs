#[doc = "Register `C2` reader"]
pub struct R(crate::R<C2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2` writer"]
pub struct W(crate::W<C2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2_SPEC>;
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
impl From<crate::W<C2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SBK` reader - Send Break"]
pub type SBK_R = crate::BitReader<SBK_A>;
#[doc = "Send Break\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBK_A {
    #[doc = "0: Normal transmitter operation."]
    _0 = 0,
    #[doc = "1: Queue break characters to be sent."]
    _1 = 1,
}
impl From<SBK_A> for bool {
    #[inline(always)]
    fn from(variant: SBK_A) -> Self {
        variant as u8 != 0
    }
}
impl SBK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBK_A {
        match self.bits {
            false => SBK_A::_0,
            true => SBK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBK_A::_1
    }
}
#[doc = "Field `SBK` writer - Send Break"]
pub type SBK_W<'a, const O: u8> = crate::BitWriter<'a, u8, C2_SPEC, SBK_A, O>;
impl<'a, const O: u8> SBK_W<'a, O> {
    #[doc = "Normal transmitter operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBK_A::_0)
    }
    #[doc = "Queue break characters to be sent."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBK_A::_1)
    }
}
#[doc = "Field `RWU` reader - Receiver Wakeup Control"]
pub type RWU_R = crate::BitReader<RWU_A>;
#[doc = "Receiver Wakeup Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWU_A {
    #[doc = "0: Normal operation."]
    _0 = 0,
    #[doc = "1: RWU enables the wakeup function and inhibits further receiver interrupt requests. Normally, hardware wakes the receiver by automatically clearing RWU."]
    _1 = 1,
}
impl From<RWU_A> for bool {
    #[inline(always)]
    fn from(variant: RWU_A) -> Self {
        variant as u8 != 0
    }
}
impl RWU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWU_A {
        match self.bits {
            false => RWU_A::_0,
            true => RWU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWU_A::_1
    }
}
#[doc = "Field `RWU` writer - Receiver Wakeup Control"]
pub type RWU_W<'a, const O: u8> = crate::BitWriter<'a, u8, C2_SPEC, RWU_A, O>;
impl<'a, const O: u8> RWU_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWU_A::_0)
    }
    #[doc = "RWU enables the wakeup function and inhibits further receiver interrupt requests. Normally, hardware wakes the receiver by automatically clearing RWU."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWU_A::_1)
    }
}
#[doc = "Field `RE` reader - Receiver Enable"]
pub type RE_R = crate::BitReader<RE_A>;
#[doc = "Receiver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE_A {
    #[doc = "0: Receiver off."]
    _0 = 0,
    #[doc = "1: Receiver on."]
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
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u8, C2_SPEC, RE_A, O>;
impl<'a, const O: u8> RE_W<'a, O> {
    #[doc = "Receiver off."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RE_A::_0)
    }
    #[doc = "Receiver on."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RE_A::_1)
    }
}
#[doc = "Field `TE` reader - Transmitter Enable"]
pub type TE_R = crate::BitReader<TE_A>;
#[doc = "Transmitter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TE_A {
    #[doc = "0: Transmitter off."]
    _0 = 0,
    #[doc = "1: Transmitter on."]
    _1 = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
impl TE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::_0,
            true => TE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TE_A::_1
    }
}
#[doc = "Field `TE` writer - Transmitter Enable"]
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u8, C2_SPEC, TE_A, O>;
impl<'a, const O: u8> TE_W<'a, O> {
    #[doc = "Transmitter off."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TE_A::_0)
    }
    #[doc = "Transmitter on."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TE_A::_1)
    }
}
#[doc = "Field `ILIE` reader - Idle Line Interrupt DMA Transfer Enable"]
pub type ILIE_R = crate::BitReader<ILIE_A>;
#[doc = "Idle Line Interrupt DMA Transfer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILIE_A {
    #[doc = "0: IDLE interrupt requests disabled. and DMA transfer"]
    _0 = 0,
    #[doc = "1: IDLE interrupt requests enabled. or DMA transfer"]
    _1 = 1,
}
impl From<ILIE_A> for bool {
    #[inline(always)]
    fn from(variant: ILIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ILIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILIE_A {
        match self.bits {
            false => ILIE_A::_0,
            true => ILIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILIE_A::_1
    }
}
#[doc = "Field `ILIE` writer - Idle Line Interrupt DMA Transfer Enable"]
pub type ILIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, C2_SPEC, ILIE_A, O>;
impl<'a, const O: u8> ILIE_W<'a, O> {
    #[doc = "IDLE interrupt requests disabled. and DMA transfer"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILIE_A::_0)
    }
    #[doc = "IDLE interrupt requests enabled. or DMA transfer"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILIE_A::_1)
    }
}
#[doc = "Field `RIE` reader - Receiver Full Interrupt or DMA Transfer Enable"]
pub type RIE_R = crate::BitReader<RIE_A>;
#[doc = "Receiver Full Interrupt or DMA Transfer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIE_A {
    #[doc = "0: RDRF interrupt and DMA transfer requests disabled."]
    _0 = 0,
    #[doc = "1: RDRF interrupt or DMA transfer requests enabled."]
    _1 = 1,
}
impl From<RIE_A> for bool {
    #[inline(always)]
    fn from(variant: RIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIE_A {
        match self.bits {
            false => RIE_A::_0,
            true => RIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RIE_A::_1
    }
}
#[doc = "Field `RIE` writer - Receiver Full Interrupt or DMA Transfer Enable"]
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, C2_SPEC, RIE_A, O>;
impl<'a, const O: u8> RIE_W<'a, O> {
    #[doc = "RDRF interrupt and DMA transfer requests disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RIE_A::_0)
    }
    #[doc = "RDRF interrupt or DMA transfer requests enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RIE_A::_1)
    }
}
#[doc = "Field `TCIE` reader - Transmission Complete Interrupt or DMA Transfer Enable"]
pub type TCIE_R = crate::BitReader<TCIE_A>;
#[doc = "Transmission Complete Interrupt or DMA Transfer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE_A {
    #[doc = "0: TC interrupt and DMA transfer requests disabled."]
    _0 = 0,
    #[doc = "1: TC interrupt or DMA transfer requests enabled."]
    _1 = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::_0,
            true => TCIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCIE_A::_1
    }
}
#[doc = "Field `TCIE` writer - Transmission Complete Interrupt or DMA Transfer Enable"]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, C2_SPEC, TCIE_A, O>;
impl<'a, const O: u8> TCIE_W<'a, O> {
    #[doc = "TC interrupt and DMA transfer requests disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCIE_A::_0)
    }
    #[doc = "TC interrupt or DMA transfer requests enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCIE_A::_1)
    }
}
#[doc = "Field `TIE` reader - Transmitter Interrupt or DMA Transfer Enable."]
pub type TIE_R = crate::BitReader<TIE_A>;
#[doc = "Transmitter Interrupt or DMA Transfer Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE_A {
    #[doc = "0: TDRE interrupt and DMA transfer requests disabled."]
    _0 = 0,
    #[doc = "1: TDRE interrupt or DMA transfer requests enabled."]
    _1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::_0,
            true => TIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE_A::_1
    }
}
#[doc = "Field `TIE` writer - Transmitter Interrupt or DMA Transfer Enable."]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, C2_SPEC, TIE_A, O>;
impl<'a, const O: u8> TIE_W<'a, O> {
    #[doc = "TDRE interrupt and DMA transfer requests disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE_A::_0)
    }
    #[doc = "TDRE interrupt or DMA transfer requests enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Send Break"]
    #[inline(always)]
    pub fn sbk(&self) -> SBK_R {
        SBK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver Wakeup Control"]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Idle Line Interrupt DMA Transfer Enable"]
    #[inline(always)]
    pub fn ilie(&self) -> ILIE_R {
        ILIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receiver Full Interrupt or DMA Transfer Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission Complete Interrupt or DMA Transfer Enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmitter Interrupt or DMA Transfer Enable."]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Send Break"]
    #[inline(always)]
    #[must_use]
    pub fn sbk(&mut self) -> SBK_W<0> {
        SBK_W::new(self)
    }
    #[doc = "Bit 1 - Receiver Wakeup Control"]
    #[inline(always)]
    #[must_use]
    pub fn rwu(&mut self) -> RWU_W<1> {
        RWU_W::new(self)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<2> {
        RE_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<3> {
        TE_W::new(self)
    }
    #[doc = "Bit 4 - Idle Line Interrupt DMA Transfer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ilie(&mut self) -> ILIE_W<4> {
        ILIE_W::new(self)
    }
    #[doc = "Bit 5 - Receiver Full Interrupt or DMA Transfer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<5> {
        RIE_W::new(self)
    }
    #[doc = "Bit 6 - Transmission Complete Interrupt or DMA Transfer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<6> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 7 - Transmitter Interrupt or DMA Transfer Enable."]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<7> {
        TIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2](index.html) module"]
pub struct C2_SPEC;
impl crate::RegisterSpec for C2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c2::R](R) reader structure"]
impl crate::Readable for C2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2::W](W) writer structure"]
impl crate::Writable for C2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2 to value 0"]
impl crate::Resettable for C2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
