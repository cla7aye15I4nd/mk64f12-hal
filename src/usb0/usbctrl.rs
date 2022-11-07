#[doc = "Register `USBCTRL` reader"]
pub struct R(crate::R<USBCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBCTRL` writer"]
pub struct W(crate::W<USBCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBCTRL_SPEC>;
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
impl From<crate::W<USBCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDE` reader - Enables the weak pulldowns on the USB transceiver."]
pub type PDE_R = crate::BitReader<PDE_A>;
#[doc = "Enables the weak pulldowns on the USB transceiver.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDE_A {
    #[doc = "0: Weak pulldowns are disabled on D+ and D-."]
    _0 = 0,
    #[doc = "1: Weak pulldowns are enabled on D+ and D-."]
    _1 = 1,
}
impl From<PDE_A> for bool {
    #[inline(always)]
    fn from(variant: PDE_A) -> Self {
        variant as u8 != 0
    }
}
impl PDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDE_A {
        match self.bits {
            false => PDE_A::_0,
            true => PDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDE_A::_1
    }
}
#[doc = "Field `PDE` writer - Enables the weak pulldowns on the USB transceiver."]
pub type PDE_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBCTRL_SPEC, PDE_A, O>;
impl<'a, const O: u8> PDE_W<'a, O> {
    #[doc = "Weak pulldowns are disabled on D+ and D-."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDE_A::_0)
    }
    #[doc = "Weak pulldowns are enabled on D+ and D-."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDE_A::_1)
    }
}
#[doc = "Field `SUSP` reader - Places the USB transceiver into the suspend state."]
pub type SUSP_R = crate::BitReader<SUSP_A>;
#[doc = "Places the USB transceiver into the suspend state.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUSP_A {
    #[doc = "0: USB transceiver is not in suspend state."]
    _0 = 0,
    #[doc = "1: USB transceiver is in suspend state."]
    _1 = 1,
}
impl From<SUSP_A> for bool {
    #[inline(always)]
    fn from(variant: SUSP_A) -> Self {
        variant as u8 != 0
    }
}
impl SUSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SUSP_A {
        match self.bits {
            false => SUSP_A::_0,
            true => SUSP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SUSP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SUSP_A::_1
    }
}
#[doc = "Field `SUSP` writer - Places the USB transceiver into the suspend state."]
pub type SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u8, USBCTRL_SPEC, SUSP_A, O>;
impl<'a, const O: u8> SUSP_W<'a, O> {
    #[doc = "USB transceiver is not in suspend state."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SUSP_A::_0)
    }
    #[doc = "USB transceiver is in suspend state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SUSP_A::_1)
    }
}
impl R {
    #[doc = "Bit 6 - Enables the weak pulldowns on the USB transceiver."]
    #[inline(always)]
    pub fn pde(&self) -> PDE_R {
        PDE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Places the USB transceiver into the suspend state."]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Enables the weak pulldowns on the USB transceiver."]
    #[inline(always)]
    #[must_use]
    pub fn pde(&mut self) -> PDE_W<6> {
        PDE_W::new(self)
    }
    #[doc = "Bit 7 - Places the USB transceiver into the suspend state."]
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<7> {
        SUSP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbctrl](index.html) module"]
pub struct USBCTRL_SPEC;
impl crate::RegisterSpec for USBCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbctrl::R](R) reader structure"]
impl crate::Readable for USBCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbctrl::W](W) writer structure"]
impl crate::Writable for USBCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBCTRL to value 0xc0"]
impl crate::Resettable for USBCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0;
}
