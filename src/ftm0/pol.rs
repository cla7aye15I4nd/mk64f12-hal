#[doc = "Register `POL` reader"]
pub struct R(crate::R<POL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POL` writer"]
pub struct W(crate::W<POL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POL_SPEC>;
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
impl From<crate::W<POL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POL0` reader - Channel 0 Polarity"]
pub type POL0_R = crate::BitReader<POL0_A>;
#[doc = "Channel 0 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL0_A {
    #[doc = "0: The channel polarity is active high."]
    _0 = 0,
    #[doc = "1: The channel polarity is active low."]
    _1 = 1,
}
impl From<POL0_A> for bool {
    #[inline(always)]
    fn from(variant: POL0_A) -> Self {
        variant as u8 != 0
    }
}
impl POL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL0_A {
        match self.bits {
            false => POL0_A::_0,
            true => POL0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POL0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POL0_A::_1
    }
}
#[doc = "Field `POL0` writer - Channel 0 Polarity"]
pub type POL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, POL_SPEC, POL0_A, O>;
impl<'a, const O: u8> POL0_W<'a, O> {
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL0_A::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL0_A::_1)
    }
}
#[doc = "Field `POL1` reader - Channel 1 Polarity"]
pub type POL1_R = crate::BitReader<POL1_A>;
#[doc = "Channel 1 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL1_A {
    #[doc = "0: The channel polarity is active high."]
    _0 = 0,
    #[doc = "1: The channel polarity is active low."]
    _1 = 1,
}
impl From<POL1_A> for bool {
    #[inline(always)]
    fn from(variant: POL1_A) -> Self {
        variant as u8 != 0
    }
}
impl POL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL1_A {
        match self.bits {
            false => POL1_A::_0,
            true => POL1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POL1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POL1_A::_1
    }
}
#[doc = "Field `POL1` writer - Channel 1 Polarity"]
pub type POL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, POL_SPEC, POL1_A, O>;
impl<'a, const O: u8> POL1_W<'a, O> {
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL1_A::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL1_A::_1)
    }
}
#[doc = "Field `POL2` reader - Channel 2 Polarity"]
pub type POL2_R = crate::BitReader<POL2_A>;
#[doc = "Channel 2 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL2_A {
    #[doc = "0: The channel polarity is active high."]
    _0 = 0,
    #[doc = "1: The channel polarity is active low."]
    _1 = 1,
}
impl From<POL2_A> for bool {
    #[inline(always)]
    fn from(variant: POL2_A) -> Self {
        variant as u8 != 0
    }
}
impl POL2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL2_A {
        match self.bits {
            false => POL2_A::_0,
            true => POL2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POL2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POL2_A::_1
    }
}
#[doc = "Field `POL2` writer - Channel 2 Polarity"]
pub type POL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, POL_SPEC, POL2_A, O>;
impl<'a, const O: u8> POL2_W<'a, O> {
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL2_A::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL2_A::_1)
    }
}
#[doc = "Field `POL3` reader - Channel 3 Polarity"]
pub type POL3_R = crate::BitReader<POL3_A>;
#[doc = "Channel 3 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL3_A {
    #[doc = "0: The channel polarity is active high."]
    _0 = 0,
    #[doc = "1: The channel polarity is active low."]
    _1 = 1,
}
impl From<POL3_A> for bool {
    #[inline(always)]
    fn from(variant: POL3_A) -> Self {
        variant as u8 != 0
    }
}
impl POL3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL3_A {
        match self.bits {
            false => POL3_A::_0,
            true => POL3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POL3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POL3_A::_1
    }
}
#[doc = "Field `POL3` writer - Channel 3 Polarity"]
pub type POL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, POL_SPEC, POL3_A, O>;
impl<'a, const O: u8> POL3_W<'a, O> {
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL3_A::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL3_A::_1)
    }
}
#[doc = "Field `POL4` reader - Channel 4 Polarity"]
pub type POL4_R = crate::BitReader<POL4_A>;
#[doc = "Channel 4 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL4_A {
    #[doc = "0: The channel polarity is active high."]
    _0 = 0,
    #[doc = "1: The channel polarity is active low."]
    _1 = 1,
}
impl From<POL4_A> for bool {
    #[inline(always)]
    fn from(variant: POL4_A) -> Self {
        variant as u8 != 0
    }
}
impl POL4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL4_A {
        match self.bits {
            false => POL4_A::_0,
            true => POL4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POL4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POL4_A::_1
    }
}
#[doc = "Field `POL4` writer - Channel 4 Polarity"]
pub type POL4_W<'a, const O: u8> = crate::BitWriter<'a, u32, POL_SPEC, POL4_A, O>;
impl<'a, const O: u8> POL4_W<'a, O> {
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL4_A::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL4_A::_1)
    }
}
#[doc = "Field `POL5` reader - Channel 5 Polarity"]
pub type POL5_R = crate::BitReader<POL5_A>;
#[doc = "Channel 5 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL5_A {
    #[doc = "0: The channel polarity is active high."]
    _0 = 0,
    #[doc = "1: The channel polarity is active low."]
    _1 = 1,
}
impl From<POL5_A> for bool {
    #[inline(always)]
    fn from(variant: POL5_A) -> Self {
        variant as u8 != 0
    }
}
impl POL5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL5_A {
        match self.bits {
            false => POL5_A::_0,
            true => POL5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POL5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POL5_A::_1
    }
}
#[doc = "Field `POL5` writer - Channel 5 Polarity"]
pub type POL5_W<'a, const O: u8> = crate::BitWriter<'a, u32, POL_SPEC, POL5_A, O>;
impl<'a, const O: u8> POL5_W<'a, O> {
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL5_A::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL5_A::_1)
    }
}
#[doc = "Field `POL6` reader - Channel 6 Polarity"]
pub type POL6_R = crate::BitReader<POL6_A>;
#[doc = "Channel 6 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL6_A {
    #[doc = "0: The channel polarity is active high."]
    _0 = 0,
    #[doc = "1: The channel polarity is active low."]
    _1 = 1,
}
impl From<POL6_A> for bool {
    #[inline(always)]
    fn from(variant: POL6_A) -> Self {
        variant as u8 != 0
    }
}
impl POL6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL6_A {
        match self.bits {
            false => POL6_A::_0,
            true => POL6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POL6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POL6_A::_1
    }
}
#[doc = "Field `POL6` writer - Channel 6 Polarity"]
pub type POL6_W<'a, const O: u8> = crate::BitWriter<'a, u32, POL_SPEC, POL6_A, O>;
impl<'a, const O: u8> POL6_W<'a, O> {
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL6_A::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL6_A::_1)
    }
}
#[doc = "Field `POL7` reader - Channel 7 Polarity"]
pub type POL7_R = crate::BitReader<POL7_A>;
#[doc = "Channel 7 Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL7_A {
    #[doc = "0: The channel polarity is active high."]
    _0 = 0,
    #[doc = "1: The channel polarity is active low."]
    _1 = 1,
}
impl From<POL7_A> for bool {
    #[inline(always)]
    fn from(variant: POL7_A) -> Self {
        variant as u8 != 0
    }
}
impl POL7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL7_A {
        match self.bits {
            false => POL7_A::_0,
            true => POL7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POL7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POL7_A::_1
    }
}
#[doc = "Field `POL7` writer - Channel 7 Polarity"]
pub type POL7_W<'a, const O: u8> = crate::BitWriter<'a, u32, POL_SPEC, POL7_A, O>;
impl<'a, const O: u8> POL7_W<'a, O> {
    #[doc = "The channel polarity is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POL7_A::_0)
    }
    #[doc = "The channel polarity is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POL7_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Polarity"]
    #[inline(always)]
    pub fn pol0(&self) -> POL0_R {
        POL0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Polarity"]
    #[inline(always)]
    pub fn pol1(&self) -> POL1_R {
        POL1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Polarity"]
    #[inline(always)]
    pub fn pol2(&self) -> POL2_R {
        POL2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Polarity"]
    #[inline(always)]
    pub fn pol3(&self) -> POL3_R {
        POL3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Polarity"]
    #[inline(always)]
    pub fn pol4(&self) -> POL4_R {
        POL4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Polarity"]
    #[inline(always)]
    pub fn pol5(&self) -> POL5_R {
        POL5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Polarity"]
    #[inline(always)]
    pub fn pol6(&self) -> POL6_R {
        POL6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Polarity"]
    #[inline(always)]
    pub fn pol7(&self) -> POL7_R {
        POL7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol0(&mut self) -> POL0_W<0> {
        POL0_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol1(&mut self) -> POL1_W<1> {
        POL1_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol2(&mut self) -> POL2_W<2> {
        POL2_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol3(&mut self) -> POL3_W<3> {
        POL3_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol4(&mut self) -> POL4_W<4> {
        POL4_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol5(&mut self) -> POL5_W<5> {
        POL5_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol6(&mut self) -> POL6_W<6> {
        POL6_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol7(&mut self) -> POL7_W<7> {
        POL7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channels Polarity\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pol](index.html) module"]
pub struct POL_SPEC;
impl crate::RegisterSpec for POL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pol::R](R) reader structure"]
impl crate::Readable for POL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pol::W](W) writer structure"]
impl crate::Writable for POL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POL to value 0"]
impl crate::Resettable for POL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
