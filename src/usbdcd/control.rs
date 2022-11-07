#[doc = "Register `CONTROL` reader"]
pub struct R(crate::R<CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONTROL` writer"]
pub struct W(crate::W<CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONTROL_SPEC>;
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
impl From<crate::W<CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Interrupt Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IACK_AW {
    #[doc = "0: Do not clear the interrupt."]
    _0 = 0,
    #[doc = "1: Clear the IF bit (interrupt flag)."]
    _1 = 1,
}
impl From<IACK_AW> for bool {
    #[inline(always)]
    fn from(variant: IACK_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IACK` writer - Interrupt Acknowledge"]
pub type IACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, IACK_AW, O>;
impl<'a, const O: u8> IACK_W<'a, O> {
    #[doc = "Do not clear the interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IACK_AW::_0)
    }
    #[doc = "Clear the IF bit (interrupt flag)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IACK_AW::_1)
    }
}
#[doc = "Field `IF` reader - Interrupt Flag"]
pub type IF_R = crate::BitReader<IF_A>;
#[doc = "Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IF_A {
    #[doc = "0: No interrupt is pending."]
    _0 = 0,
    #[doc = "1: An interrupt is pending."]
    _1 = 1,
}
impl From<IF_A> for bool {
    #[inline(always)]
    fn from(variant: IF_A) -> Self {
        variant as u8 != 0
    }
}
impl IF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IF_A {
        match self.bits {
            false => IF_A::_0,
            true => IF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IF_A::_1
    }
}
#[doc = "Field `IE` reader - Interrupt Enable"]
pub type IE_R = crate::BitReader<IE_A>;
#[doc = "Interrupt Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IE_A {
    #[doc = "0: Disable interrupts to the system."]
    _0 = 0,
    #[doc = "1: Enable interrupts to the system."]
    _1 = 1,
}
impl From<IE_A> for bool {
    #[inline(always)]
    fn from(variant: IE_A) -> Self {
        variant as u8 != 0
    }
}
impl IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IE_A {
        match self.bits {
            false => IE_A::_0,
            true => IE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IE_A::_1
    }
}
#[doc = "Field `IE` writer - Interrupt Enable"]
pub type IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, IE_A, O>;
impl<'a, const O: u8> IE_W<'a, O> {
    #[doc = "Disable interrupts to the system."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IE_A::_0)
    }
    #[doc = "Enable interrupts to the system."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IE_A::_1)
    }
}
#[doc = "Field `BC12` reader - BC1.2 compatibility. This bit cannot be changed after start detection."]
pub type BC12_R = crate::BitReader<BC12_A>;
#[doc = "BC1.2 compatibility. This bit cannot be changed after start detection.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BC12_A {
    #[doc = "0: Compatible with BC1.1 (default)"]
    _0 = 0,
    #[doc = "1: Compatible with BC1.2"]
    _1 = 1,
}
impl From<BC12_A> for bool {
    #[inline(always)]
    fn from(variant: BC12_A) -> Self {
        variant as u8 != 0
    }
}
impl BC12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BC12_A {
        match self.bits {
            false => BC12_A::_0,
            true => BC12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BC12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BC12_A::_1
    }
}
#[doc = "Field `BC12` writer - BC1.2 compatibility. This bit cannot be changed after start detection."]
pub type BC12_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, BC12_A, O>;
impl<'a, const O: u8> BC12_W<'a, O> {
    #[doc = "Compatible with BC1.1 (default)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BC12_A::_0)
    }
    #[doc = "Compatible with BC1.2"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BC12_A::_1)
    }
}
#[doc = "Start Change Detection Sequence\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_AW {
    #[doc = "0: Do not start the sequence. Writes of this value have no effect."]
    _0 = 0,
    #[doc = "1: Initiate the charger detection sequence. If the sequence is already running, writes of this value have no effect."]
    _1 = 1,
}
impl From<START_AW> for bool {
    #[inline(always)]
    fn from(variant: START_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` writer - Start Change Detection Sequence"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, START_AW, O>;
impl<'a, const O: u8> START_W<'a, O> {
    #[doc = "Do not start the sequence. Writes of this value have no effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(START_AW::_0)
    }
    #[doc = "Initiate the charger detection sequence. If the sequence is already running, writes of this value have no effect."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(START_AW::_1)
    }
}
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SR_AW {
    #[doc = "0: Do not perform a software reset."]
    _0 = 0,
    #[doc = "1: Perform a software reset."]
    _1 = 1,
}
impl From<SR_AW> for bool {
    #[inline(always)]
    fn from(variant: SR_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR` writer - Software Reset"]
pub type SR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONTROL_SPEC, SR_AW, O>;
impl<'a, const O: u8> SR_W<'a, O> {
    #[doc = "Do not perform a software reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SR_AW::_0)
    }
    #[doc = "Perform a software reset."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SR_AW::_1)
    }
}
impl R {
    #[doc = "Bit 8 - Interrupt Flag"]
    #[inline(always)]
    pub fn if_(&self) -> IF_R {
        IF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - BC1.2 compatibility. This bit cannot be changed after start detection."]
    #[inline(always)]
    pub fn bc12(&self) -> BC12_R {
        BC12_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Acknowledge"]
    #[inline(always)]
    #[must_use]
    pub fn iack(&mut self) -> IACK_W<0> {
        IACK_W::new(self)
    }
    #[doc = "Bit 16 - Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IE_W<16> {
        IE_W::new(self)
    }
    #[doc = "Bit 17 - BC1.2 compatibility. This bit cannot be changed after start detection."]
    #[inline(always)]
    #[must_use]
    pub fn bc12(&mut self) -> BC12_W<17> {
        BC12_W::new(self)
    }
    #[doc = "Bit 24 - Start Change Detection Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<24> {
        START_W::new(self)
    }
    #[doc = "Bit 25 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sr(&mut self) -> SR_W<25> {
        SR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [control](index.html) module"]
pub struct CONTROL_SPEC;
impl crate::RegisterSpec for CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [control::R](R) reader structure"]
impl crate::Readable for CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [control::W](W) writer structure"]
impl crate::Writable for CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0x0001_0000"]
impl crate::Resettable for CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
