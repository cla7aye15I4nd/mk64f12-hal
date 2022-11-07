#[doc = "Register `C1` reader"]
pub struct R(crate::R<C1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1` writer"]
pub struct W(crate::W<C1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_SPEC>;
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
impl From<crate::W<C1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PT` reader - Parity Type"]
pub type PT_R = crate::BitReader<PT_A>;
#[doc = "Parity Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PT_A {
    #[doc = "0: Even parity."]
    _0 = 0,
    #[doc = "1: Odd parity."]
    _1 = 1,
}
impl From<PT_A> for bool {
    #[inline(always)]
    fn from(variant: PT_A) -> Self {
        variant as u8 != 0
    }
}
impl PT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT_A {
        match self.bits {
            false => PT_A::_0,
            true => PT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PT_A::_1
    }
}
#[doc = "Field `PT` writer - Parity Type"]
pub type PT_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, PT_A, O>;
impl<'a, const O: u8> PT_W<'a, O> {
    #[doc = "Even parity."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PT_A::_0)
    }
    #[doc = "Odd parity."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PT_A::_1)
    }
}
#[doc = "Field `PE` reader - Parity Enable"]
pub type PE_R = crate::BitReader<PE_A>;
#[doc = "Parity Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE_A {
    #[doc = "0: Parity function disabled."]
    _0 = 0,
    #[doc = "1: Parity function enabled."]
    _1 = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
impl PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::_0,
            true => PE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PE_A::_1
    }
}
#[doc = "Field `PE` writer - Parity Enable"]
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, PE_A, O>;
impl<'a, const O: u8> PE_W<'a, O> {
    #[doc = "Parity function disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PE_A::_0)
    }
    #[doc = "Parity function enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PE_A::_1)
    }
}
#[doc = "Field `ILT` reader - Idle Line Type Select"]
pub type ILT_R = crate::BitReader<ILT_A>;
#[doc = "Idle Line Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILT_A {
    #[doc = "0: Idle character bit count starts after start bit."]
    _0 = 0,
    #[doc = "1: Idle character bit count starts after stop bit."]
    _1 = 1,
}
impl From<ILT_A> for bool {
    #[inline(always)]
    fn from(variant: ILT_A) -> Self {
        variant as u8 != 0
    }
}
impl ILT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILT_A {
        match self.bits {
            false => ILT_A::_0,
            true => ILT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILT_A::_1
    }
}
#[doc = "Field `ILT` writer - Idle Line Type Select"]
pub type ILT_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, ILT_A, O>;
impl<'a, const O: u8> ILT_W<'a, O> {
    #[doc = "Idle character bit count starts after start bit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILT_A::_0)
    }
    #[doc = "Idle character bit count starts after stop bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILT_A::_1)
    }
}
#[doc = "Field `WAKE` reader - Receiver Wakeup Method Select"]
pub type WAKE_R = crate::BitReader<WAKE_A>;
#[doc = "Receiver Wakeup Method Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKE_A {
    #[doc = "0: Idle line wakeup."]
    _0 = 0,
    #[doc = "1: Address mark wakeup."]
    _1 = 1,
}
impl From<WAKE_A> for bool {
    #[inline(always)]
    fn from(variant: WAKE_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKE_A {
        match self.bits {
            false => WAKE_A::_0,
            true => WAKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WAKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAKE_A::_1
    }
}
#[doc = "Field `WAKE` writer - Receiver Wakeup Method Select"]
pub type WAKE_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, WAKE_A, O>;
impl<'a, const O: u8> WAKE_W<'a, O> {
    #[doc = "Idle line wakeup."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKE_A::_0)
    }
    #[doc = "Address mark wakeup."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKE_A::_1)
    }
}
#[doc = "Field `M` reader - 9-bit or 8-bit Mode Select"]
pub type M_R = crate::BitReader<M_A>;
#[doc = "9-bit or 8-bit Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_A {
    #[doc = "0: Normal-start + 8 data bits (MSB/LSB first as determined by MSBF) + stop."]
    _0 = 0,
    #[doc = "1: Use-start + 9 data bits (MSB/LSB first as determined by MSBF) + stop."]
    _1 = 1,
}
impl From<M_A> for bool {
    #[inline(always)]
    fn from(variant: M_A) -> Self {
        variant as u8 != 0
    }
}
impl M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_A {
        match self.bits {
            false => M_A::_0,
            true => M_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M_A::_1
    }
}
#[doc = "Field `M` writer - 9-bit or 8-bit Mode Select"]
pub type M_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, M_A, O>;
impl<'a, const O: u8> M_W<'a, O> {
    #[doc = "Normal-start + 8 data bits (MSB/LSB first as determined by MSBF) + stop."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M_A::_0)
    }
    #[doc = "Use-start + 9 data bits (MSB/LSB first as determined by MSBF) + stop."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M_A::_1)
    }
}
#[doc = "Field `RSRC` reader - Receiver Source Select"]
pub type RSRC_R = crate::BitReader<RSRC_A>;
#[doc = "Receiver Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSRC_A {
    #[doc = "0: Selects internal loop back mode. The receiver input is internally connected to transmitter output."]
    _0 = 0,
    #[doc = "1: Single wire UART mode where the receiver input is connected to the transmit pin input signal."]
    _1 = 1,
}
impl From<RSRC_A> for bool {
    #[inline(always)]
    fn from(variant: RSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl RSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSRC_A {
        match self.bits {
            false => RSRC_A::_0,
            true => RSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSRC_A::_1
    }
}
#[doc = "Field `RSRC` writer - Receiver Source Select"]
pub type RSRC_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, RSRC_A, O>;
impl<'a, const O: u8> RSRC_W<'a, O> {
    #[doc = "Selects internal loop back mode. The receiver input is internally connected to transmitter output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSRC_A::_0)
    }
    #[doc = "Single wire UART mode where the receiver input is connected to the transmit pin input signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSRC_A::_1)
    }
}
#[doc = "Field `UARTSWAI` reader - UART Stops in Wait Mode"]
pub type UARTSWAI_R = crate::BitReader<UARTSWAI_A>;
#[doc = "UART Stops in Wait Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UARTSWAI_A {
    #[doc = "0: UART clock continues to run in Wait mode."]
    _0 = 0,
    #[doc = "1: UART clock freezes while CPU is in Wait mode."]
    _1 = 1,
}
impl From<UARTSWAI_A> for bool {
    #[inline(always)]
    fn from(variant: UARTSWAI_A) -> Self {
        variant as u8 != 0
    }
}
impl UARTSWAI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UARTSWAI_A {
        match self.bits {
            false => UARTSWAI_A::_0,
            true => UARTSWAI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UARTSWAI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UARTSWAI_A::_1
    }
}
#[doc = "Field `UARTSWAI` writer - UART Stops in Wait Mode"]
pub type UARTSWAI_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, UARTSWAI_A, O>;
impl<'a, const O: u8> UARTSWAI_W<'a, O> {
    #[doc = "UART clock continues to run in Wait mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UARTSWAI_A::_0)
    }
    #[doc = "UART clock freezes while CPU is in Wait mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UARTSWAI_A::_1)
    }
}
#[doc = "Field `LOOPS` reader - Loop Mode Select"]
pub type LOOPS_R = crate::BitReader<LOOPS_A>;
#[doc = "Loop Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOOPS_A {
    #[doc = "0: Normal operation."]
    _0 = 0,
    #[doc = "1: Loop mode where transmitter output is internally connected to receiver input. The receiver input is determined by RSRC."]
    _1 = 1,
}
impl From<LOOPS_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPS_A) -> Self {
        variant as u8 != 0
    }
}
impl LOOPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPS_A {
        match self.bits {
            false => LOOPS_A::_0,
            true => LOOPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOOPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOOPS_A::_1
    }
}
#[doc = "Field `LOOPS` writer - Loop Mode Select"]
pub type LOOPS_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, LOOPS_A, O>;
impl<'a, const O: u8> LOOPS_W<'a, O> {
    #[doc = "Normal operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOOPS_A::_0)
    }
    #[doc = "Loop mode where transmitter output is internally connected to receiver input. The receiver input is determined by RSRC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOOPS_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Parity Type"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline(always)]
    pub fn ilt(&self) -> ILT_R {
        ILT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 9-bit or 8-bit Mode Select"]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline(always)]
    pub fn rsrc(&self) -> RSRC_R {
        RSRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - UART Stops in Wait Mode"]
    #[inline(always)]
    pub fn uartswai(&self) -> UARTSWAI_R {
        UARTSWAI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline(always)]
    pub fn loops(&self) -> LOOPS_R {
        LOOPS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Type"]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PT_W<0> {
        PT_W::new(self)
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<1> {
        PE_W::new(self)
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline(always)]
    #[must_use]
    pub fn ilt(&mut self) -> ILT_W<2> {
        ILT_W::new(self)
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WAKE_W<3> {
        WAKE_W::new(self)
    }
    #[doc = "Bit 4 - 9-bit or 8-bit Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> M_W<4> {
        M_W::new(self)
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn rsrc(&mut self) -> RSRC_W<5> {
        RSRC_W::new(self)
    }
    #[doc = "Bit 6 - UART Stops in Wait Mode"]
    #[inline(always)]
    #[must_use]
    pub fn uartswai(&mut self) -> UARTSWAI_W<6> {
        UARTSWAI_W::new(self)
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn loops(&mut self) -> LOOPS_W<7> {
        LOOPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1](index.html) module"]
pub struct C1_SPEC;
impl crate::RegisterSpec for C1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c1::R](R) reader structure"]
impl crate::Readable for C1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1::W](W) writer structure"]
impl crate::Writable for C1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1 to value 0"]
impl crate::Resettable for C1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
