#[doc = "Register `C5` reader"]
pub struct R(crate::R<C5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C5` writer"]
pub struct W(crate::W<C5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C5_SPEC>;
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
impl From<crate::W<C5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBKDDMAS` reader - LIN Break Detect DMA Select Bit"]
pub type LBKDDMAS_R = crate::BitReader<LBKDDMAS_A>;
#[doc = "LIN Break Detect DMA Select Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBKDDMAS_A {
    #[doc = "0: If BDH\\[LBKDIE\\]
and S2\\[LBKDIF\\]
are set, the LBKDIF interrupt signal is asserted to request an interrupt service."]
    _0 = 0,
    #[doc = "1: If BDH\\[LBKDIE\\]
and S2\\[LBKDIF\\]
are set, the LBKDIF DMA request signal is asserted to request a DMA transfer."]
    _1 = 1,
}
impl From<LBKDDMAS_A> for bool {
    #[inline(always)]
    fn from(variant: LBKDDMAS_A) -> Self {
        variant as u8 != 0
    }
}
impl LBKDDMAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBKDDMAS_A {
        match self.bits {
            false => LBKDDMAS_A::_0,
            true => LBKDDMAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LBKDDMAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LBKDDMAS_A::_1
    }
}
#[doc = "Field `LBKDDMAS` writer - LIN Break Detect DMA Select Bit"]
pub type LBKDDMAS_W<'a, const O: u8> = crate::BitWriter<'a, u8, C5_SPEC, LBKDDMAS_A, O>;
impl<'a, const O: u8> LBKDDMAS_W<'a, O> {
    #[doc = "If BDH\\[LBKDIE\\]
and S2\\[LBKDIF\\]
are set, the LBKDIF interrupt signal is asserted to request an interrupt service."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDDMAS_A::_0)
    }
    #[doc = "If BDH\\[LBKDIE\\]
and S2\\[LBKDIF\\]
are set, the LBKDIF DMA request signal is asserted to request a DMA transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDDMAS_A::_1)
    }
}
#[doc = "Field `ILDMAS` reader - Idle Line DMA Select"]
pub type ILDMAS_R = crate::BitReader<ILDMAS_A>;
#[doc = "Idle Line DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILDMAS_A {
    #[doc = "0: If C2\\[ILIE\\]
and S1\\[IDLE\\]
are set, the IDLE interrupt request signal is asserted to request an interrupt service."]
    _0 = 0,
    #[doc = "1: If C2\\[ILIE\\]
and S1\\[IDLE\\]
are set, the IDLE DMA request signal is asserted to request a DMA transfer."]
    _1 = 1,
}
impl From<ILDMAS_A> for bool {
    #[inline(always)]
    fn from(variant: ILDMAS_A) -> Self {
        variant as u8 != 0
    }
}
impl ILDMAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILDMAS_A {
        match self.bits {
            false => ILDMAS_A::_0,
            true => ILDMAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILDMAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILDMAS_A::_1
    }
}
#[doc = "Field `ILDMAS` writer - Idle Line DMA Select"]
pub type ILDMAS_W<'a, const O: u8> = crate::BitWriter<'a, u8, C5_SPEC, ILDMAS_A, O>;
impl<'a, const O: u8> ILDMAS_W<'a, O> {
    #[doc = "If C2\\[ILIE\\]
and S1\\[IDLE\\]
are set, the IDLE interrupt request signal is asserted to request an interrupt service."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILDMAS_A::_0)
    }
    #[doc = "If C2\\[ILIE\\]
and S1\\[IDLE\\]
are set, the IDLE DMA request signal is asserted to request a DMA transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILDMAS_A::_1)
    }
}
#[doc = "Field `RDMAS` reader - Receiver Full DMA Select"]
pub type RDMAS_R = crate::BitReader<RDMAS_A>;
#[doc = "Receiver Full DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDMAS_A {
    #[doc = "0: If C2\\[RIE\\]
and S1\\[RDRF\\]
are set, the RDFR interrupt request signal is asserted to request an interrupt service."]
    _0 = 0,
    #[doc = "1: If C2\\[RIE\\]
and S1\\[RDRF\\]
are set, the RDRF DMA request signal is asserted to request a DMA transfer."]
    _1 = 1,
}
impl From<RDMAS_A> for bool {
    #[inline(always)]
    fn from(variant: RDMAS_A) -> Self {
        variant as u8 != 0
    }
}
impl RDMAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDMAS_A {
        match self.bits {
            false => RDMAS_A::_0,
            true => RDMAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDMAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDMAS_A::_1
    }
}
#[doc = "Field `RDMAS` writer - Receiver Full DMA Select"]
pub type RDMAS_W<'a, const O: u8> = crate::BitWriter<'a, u8, C5_SPEC, RDMAS_A, O>;
impl<'a, const O: u8> RDMAS_W<'a, O> {
    #[doc = "If C2\\[RIE\\]
and S1\\[RDRF\\]
are set, the RDFR interrupt request signal is asserted to request an interrupt service."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDMAS_A::_0)
    }
    #[doc = "If C2\\[RIE\\]
and S1\\[RDRF\\]
are set, the RDRF DMA request signal is asserted to request a DMA transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDMAS_A::_1)
    }
}
#[doc = "Field `TCDMAS` reader - Transmission Complete DMA Select"]
pub type TCDMAS_R = crate::BitReader<TCDMAS_A>;
#[doc = "Transmission Complete DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCDMAS_A {
    #[doc = "0: If C2\\[TCIE\\]
is set and the S1\\[TC\\]
flag is set, the TC interrupt request signal is asserted to request an interrupt service."]
    _0 = 0,
    #[doc = "1: If C2\\[TCIE\\]
is set and the S1\\[TC\\]
flag is set, the TC DMA request signal is asserted to request a DMA transfer."]
    _1 = 1,
}
impl From<TCDMAS_A> for bool {
    #[inline(always)]
    fn from(variant: TCDMAS_A) -> Self {
        variant as u8 != 0
    }
}
impl TCDMAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCDMAS_A {
        match self.bits {
            false => TCDMAS_A::_0,
            true => TCDMAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCDMAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCDMAS_A::_1
    }
}
#[doc = "Field `TCDMAS` writer - Transmission Complete DMA Select"]
pub type TCDMAS_W<'a, const O: u8> = crate::BitWriter<'a, u8, C5_SPEC, TCDMAS_A, O>;
impl<'a, const O: u8> TCDMAS_W<'a, O> {
    #[doc = "If C2\\[TCIE\\]
is set and the S1\\[TC\\]
flag is set, the TC interrupt request signal is asserted to request an interrupt service."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCDMAS_A::_0)
    }
    #[doc = "If C2\\[TCIE\\]
is set and the S1\\[TC\\]
flag is set, the TC DMA request signal is asserted to request a DMA transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCDMAS_A::_1)
    }
}
#[doc = "Field `TDMAS` reader - Transmitter DMA Select"]
pub type TDMAS_R = crate::BitReader<TDMAS_A>;
#[doc = "Transmitter DMA Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDMAS_A {
    #[doc = "0: If C2\\[TIE\\]
is set and the S1\\[TDRE\\]
flag is set, the TDRE interrupt request signal is asserted to request interrupt service."]
    _0 = 0,
    #[doc = "1: If C2\\[TIE\\]
is set and the S1\\[TDRE\\]
flag is set, the TDRE DMA request signal is asserted to request a DMA transfer."]
    _1 = 1,
}
impl From<TDMAS_A> for bool {
    #[inline(always)]
    fn from(variant: TDMAS_A) -> Self {
        variant as u8 != 0
    }
}
impl TDMAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDMAS_A {
        match self.bits {
            false => TDMAS_A::_0,
            true => TDMAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TDMAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TDMAS_A::_1
    }
}
#[doc = "Field `TDMAS` writer - Transmitter DMA Select"]
pub type TDMAS_W<'a, const O: u8> = crate::BitWriter<'a, u8, C5_SPEC, TDMAS_A, O>;
impl<'a, const O: u8> TDMAS_W<'a, O> {
    #[doc = "If C2\\[TIE\\]
is set and the S1\\[TDRE\\]
flag is set, the TDRE interrupt request signal is asserted to request interrupt service."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TDMAS_A::_0)
    }
    #[doc = "If C2\\[TIE\\]
is set and the S1\\[TDRE\\]
flag is set, the TDRE DMA request signal is asserted to request a DMA transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TDMAS_A::_1)
    }
}
impl R {
    #[doc = "Bit 3 - LIN Break Detect DMA Select Bit"]
    #[inline(always)]
    pub fn lbkddmas(&self) -> LBKDDMAS_R {
        LBKDDMAS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Idle Line DMA Select"]
    #[inline(always)]
    pub fn ildmas(&self) -> ILDMAS_R {
        ILDMAS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receiver Full DMA Select"]
    #[inline(always)]
    pub fn rdmas(&self) -> RDMAS_R {
        RDMAS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission Complete DMA Select"]
    #[inline(always)]
    pub fn tcdmas(&self) -> TCDMAS_R {
        TCDMAS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmitter DMA Select"]
    #[inline(always)]
    pub fn tdmas(&self) -> TDMAS_R {
        TDMAS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - LIN Break Detect DMA Select Bit"]
    #[inline(always)]
    #[must_use]
    pub fn lbkddmas(&mut self) -> LBKDDMAS_W<3> {
        LBKDDMAS_W::new(self)
    }
    #[doc = "Bit 4 - Idle Line DMA Select"]
    #[inline(always)]
    #[must_use]
    pub fn ildmas(&mut self) -> ILDMAS_W<4> {
        ILDMAS_W::new(self)
    }
    #[doc = "Bit 5 - Receiver Full DMA Select"]
    #[inline(always)]
    #[must_use]
    pub fn rdmas(&mut self) -> RDMAS_W<5> {
        RDMAS_W::new(self)
    }
    #[doc = "Bit 6 - Transmission Complete DMA Select"]
    #[inline(always)]
    #[must_use]
    pub fn tcdmas(&mut self) -> TCDMAS_W<6> {
        TCDMAS_W::new(self)
    }
    #[doc = "Bit 7 - Transmitter DMA Select"]
    #[inline(always)]
    #[must_use]
    pub fn tdmas(&mut self) -> TDMAS_W<7> {
        TDMAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Control Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c5](index.html) module"]
pub struct C5_SPEC;
impl crate::RegisterSpec for C5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c5::R](R) reader structure"]
impl crate::Readable for C5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c5::W](W) writer structure"]
impl crate::Writable for C5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C5 to value 0"]
impl crate::Resettable for C5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
