#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SC16P` reader - Oscillator 16 pF Capacitor Load Configure"]
pub type SC16P_R = crate::BitReader<SC16P_A>;
#[doc = "Oscillator 16 pF Capacitor Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SC16P_A {
    #[doc = "0: Disable the selection."]
    _0 = 0,
    #[doc = "1: Add 16 pF capacitor to the oscillator load."]
    _1 = 1,
}
impl From<SC16P_A> for bool {
    #[inline(always)]
    fn from(variant: SC16P_A) -> Self {
        variant as u8 != 0
    }
}
impl SC16P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SC16P_A {
        match self.bits {
            false => SC16P_A::_0,
            true => SC16P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SC16P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SC16P_A::_1
    }
}
#[doc = "Field `SC16P` writer - Oscillator 16 pF Capacitor Load Configure"]
pub type SC16P_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR_SPEC, SC16P_A, O>;
impl<'a, const O: u8> SC16P_W<'a, O> {
    #[doc = "Disable the selection."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC16P_A::_0)
    }
    #[doc = "Add 16 pF capacitor to the oscillator load."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC16P_A::_1)
    }
}
#[doc = "Field `SC8P` reader - Oscillator 8 pF Capacitor Load Configure"]
pub type SC8P_R = crate::BitReader<SC8P_A>;
#[doc = "Oscillator 8 pF Capacitor Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SC8P_A {
    #[doc = "0: Disable the selection."]
    _0 = 0,
    #[doc = "1: Add 8 pF capacitor to the oscillator load."]
    _1 = 1,
}
impl From<SC8P_A> for bool {
    #[inline(always)]
    fn from(variant: SC8P_A) -> Self {
        variant as u8 != 0
    }
}
impl SC8P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SC8P_A {
        match self.bits {
            false => SC8P_A::_0,
            true => SC8P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SC8P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SC8P_A::_1
    }
}
#[doc = "Field `SC8P` writer - Oscillator 8 pF Capacitor Load Configure"]
pub type SC8P_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR_SPEC, SC8P_A, O>;
impl<'a, const O: u8> SC8P_W<'a, O> {
    #[doc = "Disable the selection."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC8P_A::_0)
    }
    #[doc = "Add 8 pF capacitor to the oscillator load."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC8P_A::_1)
    }
}
#[doc = "Field `SC4P` reader - Oscillator 4 pF Capacitor Load Configure"]
pub type SC4P_R = crate::BitReader<SC4P_A>;
#[doc = "Oscillator 4 pF Capacitor Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SC4P_A {
    #[doc = "0: Disable the selection."]
    _0 = 0,
    #[doc = "1: Add 4 pF capacitor to the oscillator load."]
    _1 = 1,
}
impl From<SC4P_A> for bool {
    #[inline(always)]
    fn from(variant: SC4P_A) -> Self {
        variant as u8 != 0
    }
}
impl SC4P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SC4P_A {
        match self.bits {
            false => SC4P_A::_0,
            true => SC4P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SC4P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SC4P_A::_1
    }
}
#[doc = "Field `SC4P` writer - Oscillator 4 pF Capacitor Load Configure"]
pub type SC4P_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR_SPEC, SC4P_A, O>;
impl<'a, const O: u8> SC4P_W<'a, O> {
    #[doc = "Disable the selection."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC4P_A::_0)
    }
    #[doc = "Add 4 pF capacitor to the oscillator load."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC4P_A::_1)
    }
}
#[doc = "Field `SC2P` reader - Oscillator 2 pF Capacitor Load Configure"]
pub type SC2P_R = crate::BitReader<SC2P_A>;
#[doc = "Oscillator 2 pF Capacitor Load Configure\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SC2P_A {
    #[doc = "0: Disable the selection."]
    _0 = 0,
    #[doc = "1: Add 2 pF capacitor to the oscillator load."]
    _1 = 1,
}
impl From<SC2P_A> for bool {
    #[inline(always)]
    fn from(variant: SC2P_A) -> Self {
        variant as u8 != 0
    }
}
impl SC2P_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SC2P_A {
        match self.bits {
            false => SC2P_A::_0,
            true => SC2P_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SC2P_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SC2P_A::_1
    }
}
#[doc = "Field `SC2P` writer - Oscillator 2 pF Capacitor Load Configure"]
pub type SC2P_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR_SPEC, SC2P_A, O>;
impl<'a, const O: u8> SC2P_W<'a, O> {
    #[doc = "Disable the selection."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SC2P_A::_0)
    }
    #[doc = "Add 2 pF capacitor to the oscillator load."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SC2P_A::_1)
    }
}
#[doc = "Field `EREFSTEN` reader - External Reference Stop Enable"]
pub type EREFSTEN_R = crate::BitReader<EREFSTEN_A>;
#[doc = "External Reference Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EREFSTEN_A {
    #[doc = "0: External reference clock is disabled in Stop mode."]
    _0 = 0,
    #[doc = "1: External reference clock stays enabled in Stop mode if ERCLKEN is set before entering Stop mode."]
    _1 = 1,
}
impl From<EREFSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: EREFSTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl EREFSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EREFSTEN_A {
        match self.bits {
            false => EREFSTEN_A::_0,
            true => EREFSTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EREFSTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EREFSTEN_A::_1
    }
}
#[doc = "Field `EREFSTEN` writer - External Reference Stop Enable"]
pub type EREFSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR_SPEC, EREFSTEN_A, O>;
impl<'a, const O: u8> EREFSTEN_W<'a, O> {
    #[doc = "External reference clock is disabled in Stop mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EREFSTEN_A::_0)
    }
    #[doc = "External reference clock stays enabled in Stop mode if ERCLKEN is set before entering Stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EREFSTEN_A::_1)
    }
}
#[doc = "Field `ERCLKEN` reader - External Reference Enable"]
pub type ERCLKEN_R = crate::BitReader<ERCLKEN_A>;
#[doc = "External Reference Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERCLKEN_A {
    #[doc = "0: External reference clock is inactive."]
    _0 = 0,
    #[doc = "1: External reference clock is enabled."]
    _1 = 1,
}
impl From<ERCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: ERCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ERCLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERCLKEN_A {
        match self.bits {
            false => ERCLKEN_A::_0,
            true => ERCLKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERCLKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERCLKEN_A::_1
    }
}
#[doc = "Field `ERCLKEN` writer - External Reference Enable"]
pub type ERCLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR_SPEC, ERCLKEN_A, O>;
impl<'a, const O: u8> ERCLKEN_W<'a, O> {
    #[doc = "External reference clock is inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERCLKEN_A::_0)
    }
    #[doc = "External reference clock is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERCLKEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Oscillator 16 pF Capacitor Load Configure"]
    #[inline(always)]
    pub fn sc16p(&self) -> SC16P_R {
        SC16P_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Oscillator 8 pF Capacitor Load Configure"]
    #[inline(always)]
    pub fn sc8p(&self) -> SC8P_R {
        SC8P_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Oscillator 4 pF Capacitor Load Configure"]
    #[inline(always)]
    pub fn sc4p(&self) -> SC4P_R {
        SC4P_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Oscillator 2 pF Capacitor Load Configure"]
    #[inline(always)]
    pub fn sc2p(&self) -> SC2P_R {
        SC2P_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - External Reference Stop Enable"]
    #[inline(always)]
    pub fn erefsten(&self) -> EREFSTEN_R {
        EREFSTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - External Reference Enable"]
    #[inline(always)]
    pub fn erclken(&self) -> ERCLKEN_R {
        ERCLKEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Oscillator 16 pF Capacitor Load Configure"]
    #[inline(always)]
    #[must_use]
    pub fn sc16p(&mut self) -> SC16P_W<0> {
        SC16P_W::new(self)
    }
    #[doc = "Bit 1 - Oscillator 8 pF Capacitor Load Configure"]
    #[inline(always)]
    #[must_use]
    pub fn sc8p(&mut self) -> SC8P_W<1> {
        SC8P_W::new(self)
    }
    #[doc = "Bit 2 - Oscillator 4 pF Capacitor Load Configure"]
    #[inline(always)]
    #[must_use]
    pub fn sc4p(&mut self) -> SC4P_W<2> {
        SC4P_W::new(self)
    }
    #[doc = "Bit 3 - Oscillator 2 pF Capacitor Load Configure"]
    #[inline(always)]
    #[must_use]
    pub fn sc2p(&mut self) -> SC2P_W<3> {
        SC2P_W::new(self)
    }
    #[doc = "Bit 5 - External Reference Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn erefsten(&mut self) -> EREFSTEN_W<5> {
        EREFSTEN_W::new(self)
    }
    #[doc = "Bit 7 - External Reference Enable"]
    #[inline(always)]
    #[must_use]
    pub fn erclken(&mut self) -> ERCLKEN_W<7> {
        ERCLKEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
