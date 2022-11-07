#[doc = "Register `OTGCTL` reader"]
pub struct R(crate::R<OTGCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTGCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTGCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTGCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTGCTL` writer"]
pub struct W(crate::W<OTGCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTGCTL_SPEC>;
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
impl From<crate::W<OTGCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTGCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTGEN` reader - On-The-Go pullup/pulldown resistor enable"]
pub type OTGEN_R = crate::BitReader<OTGEN_A>;
#[doc = "On-The-Go pullup/pulldown resistor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OTGEN_A {
    #[doc = "0: If USB_EN is 1 and HOST_MODE is 0 in the Control Register (CTL), then the D+ Data Line pull-up resistors are enabled. If HOST_MODE is 1 the D+ and D- Data Line pull-down resistors are engaged."]
    _0 = 0,
    #[doc = "1: The pull-up and pull-down controls in this register are used."]
    _1 = 1,
}
impl From<OTGEN_A> for bool {
    #[inline(always)]
    fn from(variant: OTGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OTGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTGEN_A {
        match self.bits {
            false => OTGEN_A::_0,
            true => OTGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OTGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OTGEN_A::_1
    }
}
#[doc = "Field `OTGEN` writer - On-The-Go pullup/pulldown resistor enable"]
pub type OTGEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGCTL_SPEC, OTGEN_A, O>;
impl<'a, const O: u8> OTGEN_W<'a, O> {
    #[doc = "If USB_EN is 1 and HOST_MODE is 0 in the Control Register (CTL), then the D+ Data Line pull-up resistors are enabled. If HOST_MODE is 1 the D+ and D- Data Line pull-down resistors are engaged."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OTGEN_A::_0)
    }
    #[doc = "The pull-up and pull-down controls in this register are used."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OTGEN_A::_1)
    }
}
#[doc = "Field `DMLOW` reader - D- Data Line pull-down resistor enable"]
pub type DMLOW_R = crate::BitReader<DMLOW_A>;
#[doc = "D- Data Line pull-down resistor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMLOW_A {
    #[doc = "0: D- pulldown resistor is not enabled."]
    _0 = 0,
    #[doc = "1: D- pulldown resistor is enabled."]
    _1 = 1,
}
impl From<DMLOW_A> for bool {
    #[inline(always)]
    fn from(variant: DMLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl DMLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMLOW_A {
        match self.bits {
            false => DMLOW_A::_0,
            true => DMLOW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMLOW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMLOW_A::_1
    }
}
#[doc = "Field `DMLOW` writer - D- Data Line pull-down resistor enable"]
pub type DMLOW_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGCTL_SPEC, DMLOW_A, O>;
impl<'a, const O: u8> DMLOW_W<'a, O> {
    #[doc = "D- pulldown resistor is not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMLOW_A::_0)
    }
    #[doc = "D- pulldown resistor is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMLOW_A::_1)
    }
}
#[doc = "Field `DPLOW` reader - D+ Data Line pull-down resistor enable"]
pub type DPLOW_R = crate::BitReader<DPLOW_A>;
#[doc = "D+ Data Line pull-down resistor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPLOW_A {
    #[doc = "0: D+ pulldown resistor is not enabled."]
    _0 = 0,
    #[doc = "1: D+ pulldown resistor is enabled."]
    _1 = 1,
}
impl From<DPLOW_A> for bool {
    #[inline(always)]
    fn from(variant: DPLOW_A) -> Self {
        variant as u8 != 0
    }
}
impl DPLOW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPLOW_A {
        match self.bits {
            false => DPLOW_A::_0,
            true => DPLOW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPLOW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPLOW_A::_1
    }
}
#[doc = "Field `DPLOW` writer - D+ Data Line pull-down resistor enable"]
pub type DPLOW_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGCTL_SPEC, DPLOW_A, O>;
impl<'a, const O: u8> DPLOW_W<'a, O> {
    #[doc = "D+ pulldown resistor is not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPLOW_A::_0)
    }
    #[doc = "D+ pulldown resistor is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPLOW_A::_1)
    }
}
#[doc = "Field `DPHIGH` reader - D+ Data Line pullup resistor enable"]
pub type DPHIGH_R = crate::BitReader<DPHIGH_A>;
#[doc = "D+ Data Line pullup resistor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPHIGH_A {
    #[doc = "0: D+ pullup resistor is not enabled"]
    _0 = 0,
    #[doc = "1: D+ pullup resistor is enabled"]
    _1 = 1,
}
impl From<DPHIGH_A> for bool {
    #[inline(always)]
    fn from(variant: DPHIGH_A) -> Self {
        variant as u8 != 0
    }
}
impl DPHIGH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DPHIGH_A {
        match self.bits {
            false => DPHIGH_A::_0,
            true => DPHIGH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPHIGH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPHIGH_A::_1
    }
}
#[doc = "Field `DPHIGH` writer - D+ Data Line pullup resistor enable"]
pub type DPHIGH_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGCTL_SPEC, DPHIGH_A, O>;
impl<'a, const O: u8> DPHIGH_W<'a, O> {
    #[doc = "D+ pullup resistor is not enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPHIGH_A::_0)
    }
    #[doc = "D+ pullup resistor is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPHIGH_A::_1)
    }
}
impl R {
    #[doc = "Bit 2 - On-The-Go pullup/pulldown resistor enable"]
    #[inline(always)]
    pub fn otgen(&self) -> OTGEN_R {
        OTGEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - D- Data Line pull-down resistor enable"]
    #[inline(always)]
    pub fn dmlow(&self) -> DMLOW_R {
        DMLOW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - D+ Data Line pull-down resistor enable"]
    #[inline(always)]
    pub fn dplow(&self) -> DPLOW_R {
        DPLOW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - D+ Data Line pullup resistor enable"]
    #[inline(always)]
    pub fn dphigh(&self) -> DPHIGH_R {
        DPHIGH_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - On-The-Go pullup/pulldown resistor enable"]
    #[inline(always)]
    #[must_use]
    pub fn otgen(&mut self) -> OTGEN_W<2> {
        OTGEN_W::new(self)
    }
    #[doc = "Bit 4 - D- Data Line pull-down resistor enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmlow(&mut self) -> DMLOW_W<4> {
        DMLOW_W::new(self)
    }
    #[doc = "Bit 5 - D+ Data Line pull-down resistor enable"]
    #[inline(always)]
    #[must_use]
    pub fn dplow(&mut self) -> DPLOW_W<5> {
        DPLOW_W::new(self)
    }
    #[doc = "Bit 7 - D+ Data Line pullup resistor enable"]
    #[inline(always)]
    #[must_use]
    pub fn dphigh(&mut self) -> DPHIGH_W<7> {
        DPHIGH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otgctl](index.html) module"]
pub struct OTGCTL_SPEC;
impl crate::RegisterSpec for OTGCTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [otgctl::R](R) reader structure"]
impl crate::Readable for OTGCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otgctl::W](W) writer structure"]
impl crate::Writable for OTGCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTGCTL to value 0"]
impl crate::Resettable for OTGCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
