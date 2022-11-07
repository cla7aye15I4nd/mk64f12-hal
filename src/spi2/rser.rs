#[doc = "Register `RSER` reader"]
pub struct R(crate::R<RSER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSER` writer"]
pub struct W(crate::W<RSER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSER_SPEC>;
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
impl From<crate::W<RSER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RFDF_DIRS` reader - Receive FIFO Drain DMA or Interrupt Request Select"]
pub type RFDF_DIRS_R = crate::BitReader<RFDF_DIRS_A>;
#[doc = "Receive FIFO Drain DMA or Interrupt Request Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFDF_DIRS_A {
    #[doc = "0: Interrupt request."]
    _0 = 0,
    #[doc = "1: DMA request."]
    _1 = 1,
}
impl From<RFDF_DIRS_A> for bool {
    #[inline(always)]
    fn from(variant: RFDF_DIRS_A) -> Self {
        variant as u8 != 0
    }
}
impl RFDF_DIRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFDF_DIRS_A {
        match self.bits {
            false => RFDF_DIRS_A::_0,
            true => RFDF_DIRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFDF_DIRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFDF_DIRS_A::_1
    }
}
#[doc = "Field `RFDF_DIRS` writer - Receive FIFO Drain DMA or Interrupt Request Select"]
pub type RFDF_DIRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSER_SPEC, RFDF_DIRS_A, O>;
impl<'a, const O: u8> RFDF_DIRS_W<'a, O> {
    #[doc = "Interrupt request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFDF_DIRS_A::_0)
    }
    #[doc = "DMA request."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFDF_DIRS_A::_1)
    }
}
#[doc = "Field `RFDF_RE` reader - Receive FIFO Drain Request Enable"]
pub type RFDF_RE_R = crate::BitReader<RFDF_RE_A>;
#[doc = "Receive FIFO Drain Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFDF_RE_A {
    #[doc = "0: RFDF interrupt or DMA requests are disabled."]
    _0 = 0,
    #[doc = "1: RFDF interrupt or DMA requests are enabled."]
    _1 = 1,
}
impl From<RFDF_RE_A> for bool {
    #[inline(always)]
    fn from(variant: RFDF_RE_A) -> Self {
        variant as u8 != 0
    }
}
impl RFDF_RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFDF_RE_A {
        match self.bits {
            false => RFDF_RE_A::_0,
            true => RFDF_RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFDF_RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFDF_RE_A::_1
    }
}
#[doc = "Field `RFDF_RE` writer - Receive FIFO Drain Request Enable"]
pub type RFDF_RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSER_SPEC, RFDF_RE_A, O>;
impl<'a, const O: u8> RFDF_RE_W<'a, O> {
    #[doc = "RFDF interrupt or DMA requests are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFDF_RE_A::_0)
    }
    #[doc = "RFDF interrupt or DMA requests are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFDF_RE_A::_1)
    }
}
#[doc = "Field `RFOF_RE` reader - Receive FIFO Overflow Request Enable"]
pub type RFOF_RE_R = crate::BitReader<RFOF_RE_A>;
#[doc = "Receive FIFO Overflow Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFOF_RE_A {
    #[doc = "0: RFOF interrupt requests are disabled."]
    _0 = 0,
    #[doc = "1: RFOF interrupt requests are enabled."]
    _1 = 1,
}
impl From<RFOF_RE_A> for bool {
    #[inline(always)]
    fn from(variant: RFOF_RE_A) -> Self {
        variant as u8 != 0
    }
}
impl RFOF_RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RFOF_RE_A {
        match self.bits {
            false => RFOF_RE_A::_0,
            true => RFOF_RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RFOF_RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RFOF_RE_A::_1
    }
}
#[doc = "Field `RFOF_RE` writer - Receive FIFO Overflow Request Enable"]
pub type RFOF_RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSER_SPEC, RFOF_RE_A, O>;
impl<'a, const O: u8> RFOF_RE_W<'a, O> {
    #[doc = "RFOF interrupt requests are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RFOF_RE_A::_0)
    }
    #[doc = "RFOF interrupt requests are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RFOF_RE_A::_1)
    }
}
#[doc = "Field `TFFF_DIRS` reader - Transmit FIFO Fill DMA or Interrupt Request Select"]
pub type TFFF_DIRS_R = crate::BitReader<TFFF_DIRS_A>;
#[doc = "Transmit FIFO Fill DMA or Interrupt Request Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFFF_DIRS_A {
    #[doc = "0: TFFF flag generates interrupt requests."]
    _0 = 0,
    #[doc = "1: TFFF flag generates DMA requests."]
    _1 = 1,
}
impl From<TFFF_DIRS_A> for bool {
    #[inline(always)]
    fn from(variant: TFFF_DIRS_A) -> Self {
        variant as u8 != 0
    }
}
impl TFFF_DIRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFFF_DIRS_A {
        match self.bits {
            false => TFFF_DIRS_A::_0,
            true => TFFF_DIRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFFF_DIRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFFF_DIRS_A::_1
    }
}
#[doc = "Field `TFFF_DIRS` writer - Transmit FIFO Fill DMA or Interrupt Request Select"]
pub type TFFF_DIRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSER_SPEC, TFFF_DIRS_A, O>;
impl<'a, const O: u8> TFFF_DIRS_W<'a, O> {
    #[doc = "TFFF flag generates interrupt requests."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFFF_DIRS_A::_0)
    }
    #[doc = "TFFF flag generates DMA requests."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFFF_DIRS_A::_1)
    }
}
#[doc = "Field `TFFF_RE` reader - Transmit FIFO Fill Request Enable"]
pub type TFFF_RE_R = crate::BitReader<TFFF_RE_A>;
#[doc = "Transmit FIFO Fill Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFFF_RE_A {
    #[doc = "0: TFFF interrupts or DMA requests are disabled."]
    _0 = 0,
    #[doc = "1: TFFF interrupts or DMA requests are enabled."]
    _1 = 1,
}
impl From<TFFF_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TFFF_RE_A) -> Self {
        variant as u8 != 0
    }
}
impl TFFF_RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFFF_RE_A {
        match self.bits {
            false => TFFF_RE_A::_0,
            true => TFFF_RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFFF_RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFFF_RE_A::_1
    }
}
#[doc = "Field `TFFF_RE` writer - Transmit FIFO Fill Request Enable"]
pub type TFFF_RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSER_SPEC, TFFF_RE_A, O>;
impl<'a, const O: u8> TFFF_RE_W<'a, O> {
    #[doc = "TFFF interrupts or DMA requests are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFFF_RE_A::_0)
    }
    #[doc = "TFFF interrupts or DMA requests are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFFF_RE_A::_1)
    }
}
#[doc = "Field `TFUF_RE` reader - Transmit FIFO Underflow Request Enable"]
pub type TFUF_RE_R = crate::BitReader<TFUF_RE_A>;
#[doc = "Transmit FIFO Underflow Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFUF_RE_A {
    #[doc = "0: TFUF interrupt requests are disabled."]
    _0 = 0,
    #[doc = "1: TFUF interrupt requests are enabled."]
    _1 = 1,
}
impl From<TFUF_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TFUF_RE_A) -> Self {
        variant as u8 != 0
    }
}
impl TFUF_RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFUF_RE_A {
        match self.bits {
            false => TFUF_RE_A::_0,
            true => TFUF_RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFUF_RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFUF_RE_A::_1
    }
}
#[doc = "Field `TFUF_RE` writer - Transmit FIFO Underflow Request Enable"]
pub type TFUF_RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSER_SPEC, TFUF_RE_A, O>;
impl<'a, const O: u8> TFUF_RE_W<'a, O> {
    #[doc = "TFUF interrupt requests are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFUF_RE_A::_0)
    }
    #[doc = "TFUF interrupt requests are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFUF_RE_A::_1)
    }
}
#[doc = "Field `EOQF_RE` reader - Finished Request Enable"]
pub type EOQF_RE_R = crate::BitReader<EOQF_RE_A>;
#[doc = "Finished Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOQF_RE_A {
    #[doc = "0: EOQF interrupt requests are disabled."]
    _0 = 0,
    #[doc = "1: EOQF interrupt requests are enabled."]
    _1 = 1,
}
impl From<EOQF_RE_A> for bool {
    #[inline(always)]
    fn from(variant: EOQF_RE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOQF_RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOQF_RE_A {
        match self.bits {
            false => EOQF_RE_A::_0,
            true => EOQF_RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOQF_RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOQF_RE_A::_1
    }
}
#[doc = "Field `EOQF_RE` writer - Finished Request Enable"]
pub type EOQF_RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSER_SPEC, EOQF_RE_A, O>;
impl<'a, const O: u8> EOQF_RE_W<'a, O> {
    #[doc = "EOQF interrupt requests are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOQF_RE_A::_0)
    }
    #[doc = "EOQF interrupt requests are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOQF_RE_A::_1)
    }
}
#[doc = "Field `TCF_RE` reader - Transmission Complete Request Enable"]
pub type TCF_RE_R = crate::BitReader<TCF_RE_A>;
#[doc = "Transmission Complete Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCF_RE_A {
    #[doc = "0: TCF interrupt requests are disabled."]
    _0 = 0,
    #[doc = "1: TCF interrupt requests are enabled."]
    _1 = 1,
}
impl From<TCF_RE_A> for bool {
    #[inline(always)]
    fn from(variant: TCF_RE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCF_RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCF_RE_A {
        match self.bits {
            false => TCF_RE_A::_0,
            true => TCF_RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCF_RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCF_RE_A::_1
    }
}
#[doc = "Field `TCF_RE` writer - Transmission Complete Request Enable"]
pub type TCF_RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RSER_SPEC, TCF_RE_A, O>;
impl<'a, const O: u8> TCF_RE_W<'a, O> {
    #[doc = "TCF interrupt requests are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCF_RE_A::_0)
    }
    #[doc = "TCF interrupt requests are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCF_RE_A::_1)
    }
}
impl R {
    #[doc = "Bit 16 - Receive FIFO Drain DMA or Interrupt Request Select"]
    #[inline(always)]
    pub fn rfdf_dirs(&self) -> RFDF_DIRS_R {
        RFDF_DIRS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive FIFO Drain Request Enable"]
    #[inline(always)]
    pub fn rfdf_re(&self) -> RFDF_RE_R {
        RFDF_RE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Receive FIFO Overflow Request Enable"]
    #[inline(always)]
    pub fn rfof_re(&self) -> RFOF_RE_R {
        RFOF_RE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Transmit FIFO Fill DMA or Interrupt Request Select"]
    #[inline(always)]
    pub fn tfff_dirs(&self) -> TFFF_DIRS_R {
        TFFF_DIRS_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Transmit FIFO Fill Request Enable"]
    #[inline(always)]
    pub fn tfff_re(&self) -> TFFF_RE_R {
        TFFF_RE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit FIFO Underflow Request Enable"]
    #[inline(always)]
    pub fn tfuf_re(&self) -> TFUF_RE_R {
        TFUF_RE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Finished Request Enable"]
    #[inline(always)]
    pub fn eoqf_re(&self) -> EOQF_RE_R {
        EOQF_RE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - Transmission Complete Request Enable"]
    #[inline(always)]
    pub fn tcf_re(&self) -> TCF_RE_R {
        TCF_RE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Receive FIFO Drain DMA or Interrupt Request Select"]
    #[inline(always)]
    #[must_use]
    pub fn rfdf_dirs(&mut self) -> RFDF_DIRS_W<16> {
        RFDF_DIRS_W::new(self)
    }
    #[doc = "Bit 17 - Receive FIFO Drain Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfdf_re(&mut self) -> RFDF_RE_W<17> {
        RFDF_RE_W::new(self)
    }
    #[doc = "Bit 19 - Receive FIFO Overflow Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rfof_re(&mut self) -> RFOF_RE_W<19> {
        RFOF_RE_W::new(self)
    }
    #[doc = "Bit 24 - Transmit FIFO Fill DMA or Interrupt Request Select"]
    #[inline(always)]
    #[must_use]
    pub fn tfff_dirs(&mut self) -> TFFF_DIRS_W<24> {
        TFFF_DIRS_W::new(self)
    }
    #[doc = "Bit 25 - Transmit FIFO Fill Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfff_re(&mut self) -> TFFF_RE_W<25> {
        TFFF_RE_W::new(self)
    }
    #[doc = "Bit 27 - Transmit FIFO Underflow Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tfuf_re(&mut self) -> TFUF_RE_W<27> {
        TFUF_RE_W::new(self)
    }
    #[doc = "Bit 28 - Finished Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eoqf_re(&mut self) -> EOQF_RE_W<28> {
        EOQF_RE_W::new(self)
    }
    #[doc = "Bit 31 - Transmission Complete Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcf_re(&mut self) -> TCF_RE_W<31> {
        TCF_RE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA/Interrupt Request Select and Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rser](index.html) module"]
pub struct RSER_SPEC;
impl crate::RegisterSpec for RSER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rser::R](R) reader structure"]
impl crate::Readable for RSER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rser::W](W) writer structure"]
impl crate::Writable for RSER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSER to value 0"]
impl crate::Resettable for RSER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
