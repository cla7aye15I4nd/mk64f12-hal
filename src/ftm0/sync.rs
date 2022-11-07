#[doc = "Register `SYNC` reader"]
pub struct R(crate::R<SYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYNC` writer"]
pub struct W(crate::W<SYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYNC_SPEC>;
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
impl From<crate::W<SYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTMIN` reader - Minimum Loading Point Enable"]
pub type CNTMIN_R = crate::BitReader<CNTMIN_A>;
#[doc = "Minimum Loading Point Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNTMIN_A {
    #[doc = "0: The minimum loading point is disabled."]
    _0 = 0,
    #[doc = "1: The minimum loading point is enabled."]
    _1 = 1,
}
impl From<CNTMIN_A> for bool {
    #[inline(always)]
    fn from(variant: CNTMIN_A) -> Self {
        variant as u8 != 0
    }
}
impl CNTMIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTMIN_A {
        match self.bits {
            false => CNTMIN_A::_0,
            true => CNTMIN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CNTMIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CNTMIN_A::_1
    }
}
#[doc = "Field `CNTMIN` writer - Minimum Loading Point Enable"]
pub type CNTMIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNC_SPEC, CNTMIN_A, O>;
impl<'a, const O: u8> CNTMIN_W<'a, O> {
    #[doc = "The minimum loading point is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTMIN_A::_0)
    }
    #[doc = "The minimum loading point is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTMIN_A::_1)
    }
}
#[doc = "Field `CNTMAX` reader - Maximum Loading Point Enable"]
pub type CNTMAX_R = crate::BitReader<CNTMAX_A>;
#[doc = "Maximum Loading Point Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNTMAX_A {
    #[doc = "0: The maximum loading point is disabled."]
    _0 = 0,
    #[doc = "1: The maximum loading point is enabled."]
    _1 = 1,
}
impl From<CNTMAX_A> for bool {
    #[inline(always)]
    fn from(variant: CNTMAX_A) -> Self {
        variant as u8 != 0
    }
}
impl CNTMAX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTMAX_A {
        match self.bits {
            false => CNTMAX_A::_0,
            true => CNTMAX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CNTMAX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CNTMAX_A::_1
    }
}
#[doc = "Field `CNTMAX` writer - Maximum Loading Point Enable"]
pub type CNTMAX_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNC_SPEC, CNTMAX_A, O>;
impl<'a, const O: u8> CNTMAX_W<'a, O> {
    #[doc = "The maximum loading point is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTMAX_A::_0)
    }
    #[doc = "The maximum loading point is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTMAX_A::_1)
    }
}
#[doc = "Field `REINIT` reader - FTM Counter Reinitialization By Synchronization (FTM counter synchronization)"]
pub type REINIT_R = crate::BitReader<REINIT_A>;
#[doc = "FTM Counter Reinitialization By Synchronization (FTM counter synchronization)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REINIT_A {
    #[doc = "0: FTM counter continues to count normally."]
    _0 = 0,
    #[doc = "1: FTM counter is updated with its initial value when the selected trigger is detected."]
    _1 = 1,
}
impl From<REINIT_A> for bool {
    #[inline(always)]
    fn from(variant: REINIT_A) -> Self {
        variant as u8 != 0
    }
}
impl REINIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REINIT_A {
        match self.bits {
            false => REINIT_A::_0,
            true => REINIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == REINIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == REINIT_A::_1
    }
}
#[doc = "Field `REINIT` writer - FTM Counter Reinitialization By Synchronization (FTM counter synchronization)"]
pub type REINIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNC_SPEC, REINIT_A, O>;
impl<'a, const O: u8> REINIT_W<'a, O> {
    #[doc = "FTM counter continues to count normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(REINIT_A::_0)
    }
    #[doc = "FTM counter is updated with its initial value when the selected trigger is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(REINIT_A::_1)
    }
}
#[doc = "Field `SYNCHOM` reader - Output Mask Synchronization"]
pub type SYNCHOM_R = crate::BitReader<SYNCHOM_A>;
#[doc = "Output Mask Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNCHOM_A {
    #[doc = "0: OUTMASK register is updated with the value of its buffer in all rising edges of the system clock."]
    _0 = 0,
    #[doc = "1: OUTMASK register is updated with the value of its buffer only by the PWM synchronization."]
    _1 = 1,
}
impl From<SYNCHOM_A> for bool {
    #[inline(always)]
    fn from(variant: SYNCHOM_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNCHOM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNCHOM_A {
        match self.bits {
            false => SYNCHOM_A::_0,
            true => SYNCHOM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SYNCHOM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SYNCHOM_A::_1
    }
}
#[doc = "Field `SYNCHOM` writer - Output Mask Synchronization"]
pub type SYNCHOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNC_SPEC, SYNCHOM_A, O>;
impl<'a, const O: u8> SYNCHOM_W<'a, O> {
    #[doc = "OUTMASK register is updated with the value of its buffer in all rising edges of the system clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYNCHOM_A::_0)
    }
    #[doc = "OUTMASK register is updated with the value of its buffer only by the PWM synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYNCHOM_A::_1)
    }
}
#[doc = "Field `TRIG0` reader - PWM Synchronization Hardware Trigger 0"]
pub type TRIG0_R = crate::BitReader<TRIG0_A>;
#[doc = "PWM Synchronization Hardware Trigger 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG0_A {
    #[doc = "0: Trigger is disabled."]
    _0 = 0,
    #[doc = "1: Trigger is enabled."]
    _1 = 1,
}
impl From<TRIG0_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG0_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG0_A {
        match self.bits {
            false => TRIG0_A::_0,
            true => TRIG0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRIG0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRIG0_A::_1
    }
}
#[doc = "Field `TRIG0` writer - PWM Synchronization Hardware Trigger 0"]
pub type TRIG0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNC_SPEC, TRIG0_A, O>;
impl<'a, const O: u8> TRIG0_W<'a, O> {
    #[doc = "Trigger is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRIG0_A::_0)
    }
    #[doc = "Trigger is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRIG0_A::_1)
    }
}
#[doc = "Field `TRIG1` reader - PWM Synchronization Hardware Trigger 1"]
pub type TRIG1_R = crate::BitReader<TRIG1_A>;
#[doc = "PWM Synchronization Hardware Trigger 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG1_A {
    #[doc = "0: Trigger is disabled."]
    _0 = 0,
    #[doc = "1: Trigger is enabled."]
    _1 = 1,
}
impl From<TRIG1_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG1_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG1_A {
        match self.bits {
            false => TRIG1_A::_0,
            true => TRIG1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRIG1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRIG1_A::_1
    }
}
#[doc = "Field `TRIG1` writer - PWM Synchronization Hardware Trigger 1"]
pub type TRIG1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNC_SPEC, TRIG1_A, O>;
impl<'a, const O: u8> TRIG1_W<'a, O> {
    #[doc = "Trigger is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRIG1_A::_0)
    }
    #[doc = "Trigger is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRIG1_A::_1)
    }
}
#[doc = "Field `TRIG2` reader - PWM Synchronization Hardware Trigger 2"]
pub type TRIG2_R = crate::BitReader<TRIG2_A>;
#[doc = "PWM Synchronization Hardware Trigger 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG2_A {
    #[doc = "0: Trigger is disabled."]
    _0 = 0,
    #[doc = "1: Trigger is enabled."]
    _1 = 1,
}
impl From<TRIG2_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG2_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG2_A {
        match self.bits {
            false => TRIG2_A::_0,
            true => TRIG2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRIG2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRIG2_A::_1
    }
}
#[doc = "Field `TRIG2` writer - PWM Synchronization Hardware Trigger 2"]
pub type TRIG2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNC_SPEC, TRIG2_A, O>;
impl<'a, const O: u8> TRIG2_W<'a, O> {
    #[doc = "Trigger is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRIG2_A::_0)
    }
    #[doc = "Trigger is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRIG2_A::_1)
    }
}
#[doc = "Field `SWSYNC` reader - PWM Synchronization Software Trigger"]
pub type SWSYNC_R = crate::BitReader<SWSYNC_A>;
#[doc = "PWM Synchronization Software Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWSYNC_A {
    #[doc = "0: Software trigger is not selected."]
    _0 = 0,
    #[doc = "1: Software trigger is selected."]
    _1 = 1,
}
impl From<SWSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SWSYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl SWSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWSYNC_A {
        match self.bits {
            false => SWSYNC_A::_0,
            true => SWSYNC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWSYNC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWSYNC_A::_1
    }
}
#[doc = "Field `SWSYNC` writer - PWM Synchronization Software Trigger"]
pub type SWSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SYNC_SPEC, SWSYNC_A, O>;
impl<'a, const O: u8> SWSYNC_W<'a, O> {
    #[doc = "Software trigger is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWSYNC_A::_0)
    }
    #[doc = "Software trigger is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWSYNC_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Minimum Loading Point Enable"]
    #[inline(always)]
    pub fn cntmin(&self) -> CNTMIN_R {
        CNTMIN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Maximum Loading Point Enable"]
    #[inline(always)]
    pub fn cntmax(&self) -> CNTMAX_R {
        CNTMAX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FTM Counter Reinitialization By Synchronization (FTM counter synchronization)"]
    #[inline(always)]
    pub fn reinit(&self) -> REINIT_R {
        REINIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output Mask Synchronization"]
    #[inline(always)]
    pub fn synchom(&self) -> SYNCHOM_R {
        SYNCHOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM Synchronization Hardware Trigger 0"]
    #[inline(always)]
    pub fn trig0(&self) -> TRIG0_R {
        TRIG0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWM Synchronization Hardware Trigger 1"]
    #[inline(always)]
    pub fn trig1(&self) -> TRIG1_R {
        TRIG1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM Synchronization Hardware Trigger 2"]
    #[inline(always)]
    pub fn trig2(&self) -> TRIG2_R {
        TRIG2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PWM Synchronization Software Trigger"]
    #[inline(always)]
    pub fn swsync(&self) -> SWSYNC_R {
        SWSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Minimum Loading Point Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cntmin(&mut self) -> CNTMIN_W<0> {
        CNTMIN_W::new(self)
    }
    #[doc = "Bit 1 - Maximum Loading Point Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cntmax(&mut self) -> CNTMAX_W<1> {
        CNTMAX_W::new(self)
    }
    #[doc = "Bit 2 - FTM Counter Reinitialization By Synchronization (FTM counter synchronization)"]
    #[inline(always)]
    #[must_use]
    pub fn reinit(&mut self) -> REINIT_W<2> {
        REINIT_W::new(self)
    }
    #[doc = "Bit 3 - Output Mask Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn synchom(&mut self) -> SYNCHOM_W<3> {
        SYNCHOM_W::new(self)
    }
    #[doc = "Bit 4 - PWM Synchronization Hardware Trigger 0"]
    #[inline(always)]
    #[must_use]
    pub fn trig0(&mut self) -> TRIG0_W<4> {
        TRIG0_W::new(self)
    }
    #[doc = "Bit 5 - PWM Synchronization Hardware Trigger 1"]
    #[inline(always)]
    #[must_use]
    pub fn trig1(&mut self) -> TRIG1_W<5> {
        TRIG1_W::new(self)
    }
    #[doc = "Bit 6 - PWM Synchronization Hardware Trigger 2"]
    #[inline(always)]
    #[must_use]
    pub fn trig2(&mut self) -> TRIG2_W<6> {
        TRIG2_W::new(self)
    }
    #[doc = "Bit 7 - PWM Synchronization Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn swsync(&mut self) -> SWSYNC_W<7> {
        SWSYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Synchronization\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sync](index.html) module"]
pub struct SYNC_SPEC;
impl crate::RegisterSpec for SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sync::R](R) reader structure"]
impl crate::Readable for SYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sync::W](W) writer structure"]
impl crate::Writable for SYNC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYNC to value 0"]
impl crate::Resettable for SYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
