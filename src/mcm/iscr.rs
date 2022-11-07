#[doc = "Register `ISCR` reader"]
pub struct R(crate::R<ISCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISCR` writer"]
pub struct W(crate::W<ISCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISCR_SPEC>;
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
impl From<crate::W<ISCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQ` reader - Normal Interrupt Pending"]
pub type IRQ_R = crate::BitReader<IRQ_A>;
#[doc = "Normal Interrupt Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRQ_A {
    #[doc = "0: No pending interrupt"]
    _0 = 0,
    #[doc = "1: Due to the ETB counter expiring, a normal interrupt is pending"]
    _1 = 1,
}
impl From<IRQ_A> for bool {
    #[inline(always)]
    fn from(variant: IRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl IRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRQ_A {
        match self.bits {
            false => IRQ_A::_0,
            true => IRQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRQ_A::_1
    }
}
#[doc = "Field `IRQ` writer - Normal Interrupt Pending"]
pub type IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, IRQ_A, O>;
impl<'a, const O: u8> IRQ_W<'a, O> {
    #[doc = "No pending interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRQ_A::_0)
    }
    #[doc = "Due to the ETB counter expiring, a normal interrupt is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRQ_A::_1)
    }
}
#[doc = "Field `NMI` reader - Non-maskable Interrupt Pending"]
pub type NMI_R = crate::BitReader<NMI_A>;
#[doc = "Non-maskable Interrupt Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NMI_A {
    #[doc = "0: No pending NMI"]
    _0 = 0,
    #[doc = "1: Due to the ETB counter expiring, an NMI is pending"]
    _1 = 1,
}
impl From<NMI_A> for bool {
    #[inline(always)]
    fn from(variant: NMI_A) -> Self {
        variant as u8 != 0
    }
}
impl NMI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NMI_A {
        match self.bits {
            false => NMI_A::_0,
            true => NMI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NMI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NMI_A::_1
    }
}
#[doc = "Field `NMI` writer - Non-maskable Interrupt Pending"]
pub type NMI_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, NMI_A, O>;
impl<'a, const O: u8> NMI_W<'a, O> {
    #[doc = "No pending NMI"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NMI_A::_0)
    }
    #[doc = "Due to the ETB counter expiring, an NMI is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NMI_A::_1)
    }
}
#[doc = "Field `DHREQ` reader - Debug Halt Request Indicator"]
pub type DHREQ_R = crate::BitReader<DHREQ_A>;
#[doc = "Debug Halt Request Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DHREQ_A {
    #[doc = "0: No debug halt request"]
    _0 = 0,
    #[doc = "1: Debug halt request initiated"]
    _1 = 1,
}
impl From<DHREQ_A> for bool {
    #[inline(always)]
    fn from(variant: DHREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl DHREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DHREQ_A {
        match self.bits {
            false => DHREQ_A::_0,
            true => DHREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DHREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DHREQ_A::_1
    }
}
#[doc = "Field `FIOC` reader - FPU invalid operation interrupt status"]
pub type FIOC_R = crate::BitReader<FIOC_A>;
#[doc = "FPU invalid operation interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIOC_A {
    #[doc = "0: No interrupt"]
    _0 = 0,
    #[doc = "1: Interrupt occurred"]
    _1 = 1,
}
impl From<FIOC_A> for bool {
    #[inline(always)]
    fn from(variant: FIOC_A) -> Self {
        variant as u8 != 0
    }
}
impl FIOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIOC_A {
        match self.bits {
            false => FIOC_A::_0,
            true => FIOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIOC_A::_1
    }
}
#[doc = "Field `FDZC` reader - FPU divide-by-zero interrupt status"]
pub type FDZC_R = crate::BitReader<FDZC_A>;
#[doc = "FPU divide-by-zero interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDZC_A {
    #[doc = "0: No interrupt"]
    _0 = 0,
    #[doc = "1: Interrupt occurred"]
    _1 = 1,
}
impl From<FDZC_A> for bool {
    #[inline(always)]
    fn from(variant: FDZC_A) -> Self {
        variant as u8 != 0
    }
}
impl FDZC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDZC_A {
        match self.bits {
            false => FDZC_A::_0,
            true => FDZC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FDZC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FDZC_A::_1
    }
}
#[doc = "Field `FOFC` reader - FPU overflow interrupt status"]
pub type FOFC_R = crate::BitReader<FOFC_A>;
#[doc = "FPU overflow interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOFC_A {
    #[doc = "0: No interrupt"]
    _0 = 0,
    #[doc = "1: Interrupt occurred"]
    _1 = 1,
}
impl From<FOFC_A> for bool {
    #[inline(always)]
    fn from(variant: FOFC_A) -> Self {
        variant as u8 != 0
    }
}
impl FOFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOFC_A {
        match self.bits {
            false => FOFC_A::_0,
            true => FOFC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FOFC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FOFC_A::_1
    }
}
#[doc = "Field `FUFC` reader - FPU underflow interrupt status"]
pub type FUFC_R = crate::BitReader<FUFC_A>;
#[doc = "FPU underflow interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUFC_A {
    #[doc = "0: No interrupt"]
    _0 = 0,
    #[doc = "1: Interrupt occurred"]
    _1 = 1,
}
impl From<FUFC_A> for bool {
    #[inline(always)]
    fn from(variant: FUFC_A) -> Self {
        variant as u8 != 0
    }
}
impl FUFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FUFC_A {
        match self.bits {
            false => FUFC_A::_0,
            true => FUFC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FUFC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FUFC_A::_1
    }
}
#[doc = "Field `FIXC` reader - FPU inexact interrupt status"]
pub type FIXC_R = crate::BitReader<FIXC_A>;
#[doc = "FPU inexact interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIXC_A {
    #[doc = "0: No interrupt"]
    _0 = 0,
    #[doc = "1: Interrupt occurred"]
    _1 = 1,
}
impl From<FIXC_A> for bool {
    #[inline(always)]
    fn from(variant: FIXC_A) -> Self {
        variant as u8 != 0
    }
}
impl FIXC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIXC_A {
        match self.bits {
            false => FIXC_A::_0,
            true => FIXC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIXC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIXC_A::_1
    }
}
#[doc = "Field `FIDC` reader - FPU input denormal interrupt status"]
pub type FIDC_R = crate::BitReader<FIDC_A>;
#[doc = "FPU input denormal interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIDC_A {
    #[doc = "0: No interrupt"]
    _0 = 0,
    #[doc = "1: Interrupt occurred"]
    _1 = 1,
}
impl From<FIDC_A> for bool {
    #[inline(always)]
    fn from(variant: FIDC_A) -> Self {
        variant as u8 != 0
    }
}
impl FIDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIDC_A {
        match self.bits {
            false => FIDC_A::_0,
            true => FIDC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIDC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIDC_A::_1
    }
}
#[doc = "Field `FIOCE` reader - FPU invalid operation interrupt enable"]
pub type FIOCE_R = crate::BitReader<FIOCE_A>;
#[doc = "FPU invalid operation interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIOCE_A {
    #[doc = "0: Disable interrupt"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<FIOCE_A> for bool {
    #[inline(always)]
    fn from(variant: FIOCE_A) -> Self {
        variant as u8 != 0
    }
}
impl FIOCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIOCE_A {
        match self.bits {
            false => FIOCE_A::_0,
            true => FIOCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIOCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIOCE_A::_1
    }
}
#[doc = "Field `FIOCE` writer - FPU invalid operation interrupt enable"]
pub type FIOCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, FIOCE_A, O>;
impl<'a, const O: u8> FIOCE_W<'a, O> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIOCE_A::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIOCE_A::_1)
    }
}
#[doc = "Field `FDZCE` reader - FPU divide-by-zero interrupt enable"]
pub type FDZCE_R = crate::BitReader<FDZCE_A>;
#[doc = "FPU divide-by-zero interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDZCE_A {
    #[doc = "0: Disable interrupt"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<FDZCE_A> for bool {
    #[inline(always)]
    fn from(variant: FDZCE_A) -> Self {
        variant as u8 != 0
    }
}
impl FDZCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDZCE_A {
        match self.bits {
            false => FDZCE_A::_0,
            true => FDZCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FDZCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FDZCE_A::_1
    }
}
#[doc = "Field `FDZCE` writer - FPU divide-by-zero interrupt enable"]
pub type FDZCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, FDZCE_A, O>;
impl<'a, const O: u8> FDZCE_W<'a, O> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FDZCE_A::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FDZCE_A::_1)
    }
}
#[doc = "Field `FOFCE` reader - FPU overflow interrupt enable"]
pub type FOFCE_R = crate::BitReader<FOFCE_A>;
#[doc = "FPU overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOFCE_A {
    #[doc = "0: Disable interrupt"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<FOFCE_A> for bool {
    #[inline(always)]
    fn from(variant: FOFCE_A) -> Self {
        variant as u8 != 0
    }
}
impl FOFCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOFCE_A {
        match self.bits {
            false => FOFCE_A::_0,
            true => FOFCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FOFCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FOFCE_A::_1
    }
}
#[doc = "Field `FOFCE` writer - FPU overflow interrupt enable"]
pub type FOFCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, FOFCE_A, O>;
impl<'a, const O: u8> FOFCE_W<'a, O> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FOFCE_A::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FOFCE_A::_1)
    }
}
#[doc = "Field `FUFCE` reader - FPU underflow interrupt enable"]
pub type FUFCE_R = crate::BitReader<FUFCE_A>;
#[doc = "FPU underflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUFCE_A {
    #[doc = "0: Disable interrupt"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<FUFCE_A> for bool {
    #[inline(always)]
    fn from(variant: FUFCE_A) -> Self {
        variant as u8 != 0
    }
}
impl FUFCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FUFCE_A {
        match self.bits {
            false => FUFCE_A::_0,
            true => FUFCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FUFCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FUFCE_A::_1
    }
}
#[doc = "Field `FUFCE` writer - FPU underflow interrupt enable"]
pub type FUFCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, FUFCE_A, O>;
impl<'a, const O: u8> FUFCE_W<'a, O> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FUFCE_A::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FUFCE_A::_1)
    }
}
#[doc = "Field `FIXCE` reader - FPU inexact interrupt enable"]
pub type FIXCE_R = crate::BitReader<FIXCE_A>;
#[doc = "FPU inexact interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIXCE_A {
    #[doc = "0: Disable interrupt"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<FIXCE_A> for bool {
    #[inline(always)]
    fn from(variant: FIXCE_A) -> Self {
        variant as u8 != 0
    }
}
impl FIXCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIXCE_A {
        match self.bits {
            false => FIXCE_A::_0,
            true => FIXCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIXCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIXCE_A::_1
    }
}
#[doc = "Field `FIXCE` writer - FPU inexact interrupt enable"]
pub type FIXCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, FIXCE_A, O>;
impl<'a, const O: u8> FIXCE_W<'a, O> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIXCE_A::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIXCE_A::_1)
    }
}
#[doc = "Field `FIDCE` reader - FPU input denormal interrupt enable"]
pub type FIDCE_R = crate::BitReader<FIDCE_A>;
#[doc = "FPU input denormal interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIDCE_A {
    #[doc = "0: Disable interrupt"]
    _0 = 0,
    #[doc = "1: Enable interrupt"]
    _1 = 1,
}
impl From<FIDCE_A> for bool {
    #[inline(always)]
    fn from(variant: FIDCE_A) -> Self {
        variant as u8 != 0
    }
}
impl FIDCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIDCE_A {
        match self.bits {
            false => FIDCE_A::_0,
            true => FIDCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FIDCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FIDCE_A::_1
    }
}
#[doc = "Field `FIDCE` writer - FPU input denormal interrupt enable"]
pub type FIDCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, FIDCE_A, O>;
impl<'a, const O: u8> FIDCE_W<'a, O> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FIDCE_A::_0)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FIDCE_A::_1)
    }
}
impl R {
    #[doc = "Bit 1 - Normal Interrupt Pending"]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Non-maskable Interrupt Pending"]
    #[inline(always)]
    pub fn nmi(&self) -> NMI_R {
        NMI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Debug Halt Request Indicator"]
    #[inline(always)]
    pub fn dhreq(&self) -> DHREQ_R {
        DHREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - FPU invalid operation interrupt status"]
    #[inline(always)]
    pub fn fioc(&self) -> FIOC_R {
        FIOC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FPU divide-by-zero interrupt status"]
    #[inline(always)]
    pub fn fdzc(&self) -> FDZC_R {
        FDZC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FPU overflow interrupt status"]
    #[inline(always)]
    pub fn fofc(&self) -> FOFC_R {
        FOFC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FPU underflow interrupt status"]
    #[inline(always)]
    pub fn fufc(&self) -> FUFC_R {
        FUFC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FPU inexact interrupt status"]
    #[inline(always)]
    pub fn fixc(&self) -> FIXC_R {
        FIXC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - FPU input denormal interrupt status"]
    #[inline(always)]
    pub fn fidc(&self) -> FIDC_R {
        FIDC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 24 - FPU invalid operation interrupt enable"]
    #[inline(always)]
    pub fn fioce(&self) -> FIOCE_R {
        FIOCE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - FPU divide-by-zero interrupt enable"]
    #[inline(always)]
    pub fn fdzce(&self) -> FDZCE_R {
        FDZCE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FPU overflow interrupt enable"]
    #[inline(always)]
    pub fn fofce(&self) -> FOFCE_R {
        FOFCE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - FPU underflow interrupt enable"]
    #[inline(always)]
    pub fn fufce(&self) -> FUFCE_R {
        FUFCE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - FPU inexact interrupt enable"]
    #[inline(always)]
    pub fn fixce(&self) -> FIXCE_R {
        FIXCE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - FPU input denormal interrupt enable"]
    #[inline(always)]
    pub fn fidce(&self) -> FIDCE_R {
        FIDCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Normal Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn irq(&mut self) -> IRQ_W<1> {
        IRQ_W::new(self)
    }
    #[doc = "Bit 2 - Non-maskable Interrupt Pending"]
    #[inline(always)]
    #[must_use]
    pub fn nmi(&mut self) -> NMI_W<2> {
        NMI_W::new(self)
    }
    #[doc = "Bit 24 - FPU invalid operation interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fioce(&mut self) -> FIOCE_W<24> {
        FIOCE_W::new(self)
    }
    #[doc = "Bit 25 - FPU divide-by-zero interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdzce(&mut self) -> FDZCE_W<25> {
        FDZCE_W::new(self)
    }
    #[doc = "Bit 26 - FPU overflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fofce(&mut self) -> FOFCE_W<26> {
        FOFCE_W::new(self)
    }
    #[doc = "Bit 27 - FPU underflow interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fufce(&mut self) -> FUFCE_W<27> {
        FUFCE_W::new(self)
    }
    #[doc = "Bit 28 - FPU inexact interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fixce(&mut self) -> FIXCE_W<28> {
        FIXCE_W::new(self)
    }
    #[doc = "Bit 31 - FPU input denormal interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn fidce(&mut self) -> FIDCE_W<31> {
        FIDCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iscr](index.html) module"]
pub struct ISCR_SPEC;
impl crate::RegisterSpec for ISCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iscr::R](R) reader structure"]
impl crate::Readable for ISCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iscr::W](W) writer structure"]
impl crate::Writable for ISCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISCR to value 0"]
impl crate::Resettable for ISCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
