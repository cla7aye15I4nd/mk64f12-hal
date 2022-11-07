#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBRSTEN` reader - USBRST Interrupt Enable"]
pub type USBRSTEN_R = crate::BitReader<USBRSTEN_A>;
#[doc = "USBRST Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBRSTEN_A {
    #[doc = "0: Disables the USBRST interrupt."]
    _0 = 0,
    #[doc = "1: Enables the USBRST interrupt."]
    _1 = 1,
}
impl From<USBRSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBRSTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl USBRSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBRSTEN_A {
        match self.bits {
            false => USBRSTEN_A::_0,
            true => USBRSTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBRSTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBRSTEN_A::_1
    }
}
#[doc = "Field `USBRSTEN` writer - USBRST Interrupt Enable"]
pub type USBRSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTEN_SPEC, USBRSTEN_A, O>;
impl<'a, const O: u8> USBRSTEN_W<'a, O> {
    #[doc = "Disables the USBRST interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBRSTEN_A::_0)
    }
    #[doc = "Enables the USBRST interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBRSTEN_A::_1)
    }
}
#[doc = "Field `ERROREN` reader - ERROR Interrupt Enable"]
pub type ERROREN_R = crate::BitReader<ERROREN_A>;
#[doc = "ERROR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERROREN_A {
    #[doc = "0: Disables the ERROR interrupt."]
    _0 = 0,
    #[doc = "1: Enables the ERROR interrupt."]
    _1 = 1,
}
impl From<ERROREN_A> for bool {
    #[inline(always)]
    fn from(variant: ERROREN_A) -> Self {
        variant as u8 != 0
    }
}
impl ERROREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROREN_A {
        match self.bits {
            false => ERROREN_A::_0,
            true => ERROREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERROREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERROREN_A::_1
    }
}
#[doc = "Field `ERROREN` writer - ERROR Interrupt Enable"]
pub type ERROREN_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTEN_SPEC, ERROREN_A, O>;
impl<'a, const O: u8> ERROREN_W<'a, O> {
    #[doc = "Disables the ERROR interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERROREN_A::_0)
    }
    #[doc = "Enables the ERROR interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERROREN_A::_1)
    }
}
#[doc = "Field `SOFTOKEN` reader - SOFTOK Interrupt Enable"]
pub type SOFTOKEN_R = crate::BitReader<SOFTOKEN_A>;
#[doc = "SOFTOK Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFTOKEN_A {
    #[doc = "0: Disbles the SOFTOK interrupt."]
    _0 = 0,
    #[doc = "1: Enables the SOFTOK interrupt."]
    _1 = 1,
}
impl From<SOFTOKEN_A> for bool {
    #[inline(always)]
    fn from(variant: SOFTOKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SOFTOKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFTOKEN_A {
        match self.bits {
            false => SOFTOKEN_A::_0,
            true => SOFTOKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOFTOKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOFTOKEN_A::_1
    }
}
#[doc = "Field `SOFTOKEN` writer - SOFTOK Interrupt Enable"]
pub type SOFTOKEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTEN_SPEC, SOFTOKEN_A, O>;
impl<'a, const O: u8> SOFTOKEN_W<'a, O> {
    #[doc = "Disbles the SOFTOK interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOFTOKEN_A::_0)
    }
    #[doc = "Enables the SOFTOK interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SOFTOKEN_A::_1)
    }
}
#[doc = "Field `TOKDNEEN` reader - TOKDNE Interrupt Enable"]
pub type TOKDNEEN_R = crate::BitReader<TOKDNEEN_A>;
#[doc = "TOKDNE Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOKDNEEN_A {
    #[doc = "0: Disables the TOKDNE interrupt."]
    _0 = 0,
    #[doc = "1: Enables the TOKDNE interrupt."]
    _1 = 1,
}
impl From<TOKDNEEN_A> for bool {
    #[inline(always)]
    fn from(variant: TOKDNEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TOKDNEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOKDNEEN_A {
        match self.bits {
            false => TOKDNEEN_A::_0,
            true => TOKDNEEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOKDNEEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOKDNEEN_A::_1
    }
}
#[doc = "Field `TOKDNEEN` writer - TOKDNE Interrupt Enable"]
pub type TOKDNEEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTEN_SPEC, TOKDNEEN_A, O>;
impl<'a, const O: u8> TOKDNEEN_W<'a, O> {
    #[doc = "Disables the TOKDNE interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOKDNEEN_A::_0)
    }
    #[doc = "Enables the TOKDNE interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOKDNEEN_A::_1)
    }
}
#[doc = "Field `SLEEPEN` reader - SLEEP Interrupt Enable"]
pub type SLEEPEN_R = crate::BitReader<SLEEPEN_A>;
#[doc = "SLEEP Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEEPEN_A {
    #[doc = "0: Disables the SLEEP interrupt."]
    _0 = 0,
    #[doc = "1: Enables the SLEEP interrupt."]
    _1 = 1,
}
impl From<SLEEPEN_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEEPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEPEN_A {
        match self.bits {
            false => SLEEPEN_A::_0,
            true => SLEEPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLEEPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLEEPEN_A::_1
    }
}
#[doc = "Field `SLEEPEN` writer - SLEEP Interrupt Enable"]
pub type SLEEPEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTEN_SPEC, SLEEPEN_A, O>;
impl<'a, const O: u8> SLEEPEN_W<'a, O> {
    #[doc = "Disables the SLEEP interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLEEPEN_A::_0)
    }
    #[doc = "Enables the SLEEP interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLEEPEN_A::_1)
    }
}
#[doc = "Field `RESUMEEN` reader - RESUME Interrupt Enable"]
pub type RESUMEEN_R = crate::BitReader<RESUMEEN_A>;
#[doc = "RESUME Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESUMEEN_A {
    #[doc = "0: Disables the RESUME interrupt."]
    _0 = 0,
    #[doc = "1: Enables the RESUME interrupt."]
    _1 = 1,
}
impl From<RESUMEEN_A> for bool {
    #[inline(always)]
    fn from(variant: RESUMEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RESUMEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RESUMEEN_A {
        match self.bits {
            false => RESUMEEN_A::_0,
            true => RESUMEEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RESUMEEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RESUMEEN_A::_1
    }
}
#[doc = "Field `RESUMEEN` writer - RESUME Interrupt Enable"]
pub type RESUMEEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTEN_SPEC, RESUMEEN_A, O>;
impl<'a, const O: u8> RESUMEEN_W<'a, O> {
    #[doc = "Disables the RESUME interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RESUMEEN_A::_0)
    }
    #[doc = "Enables the RESUME interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RESUMEEN_A::_1)
    }
}
#[doc = "Field `ATTACHEN` reader - ATTACH Interrupt Enable"]
pub type ATTACHEN_R = crate::BitReader<ATTACHEN_A>;
#[doc = "ATTACH Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATTACHEN_A {
    #[doc = "0: Disables the ATTACH interrupt."]
    _0 = 0,
    #[doc = "1: Enables the ATTACH interrupt."]
    _1 = 1,
}
impl From<ATTACHEN_A> for bool {
    #[inline(always)]
    fn from(variant: ATTACHEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ATTACHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATTACHEN_A {
        match self.bits {
            false => ATTACHEN_A::_0,
            true => ATTACHEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATTACHEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATTACHEN_A::_1
    }
}
#[doc = "Field `ATTACHEN` writer - ATTACH Interrupt Enable"]
pub type ATTACHEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTEN_SPEC, ATTACHEN_A, O>;
impl<'a, const O: u8> ATTACHEN_W<'a, O> {
    #[doc = "Disables the ATTACH interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATTACHEN_A::_0)
    }
    #[doc = "Enables the ATTACH interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATTACHEN_A::_1)
    }
}
#[doc = "Field `STALLEN` reader - STALL Interrupt Enable"]
pub type STALLEN_R = crate::BitReader<STALLEN_A>;
#[doc = "STALL Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STALLEN_A {
    #[doc = "0: Diasbles the STALL interrupt."]
    _0 = 0,
    #[doc = "1: Enables the STALL interrupt."]
    _1 = 1,
}
impl From<STALLEN_A> for bool {
    #[inline(always)]
    fn from(variant: STALLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl STALLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STALLEN_A {
        match self.bits {
            false => STALLEN_A::_0,
            true => STALLEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == STALLEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == STALLEN_A::_1
    }
}
#[doc = "Field `STALLEN` writer - STALL Interrupt Enable"]
pub type STALLEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, INTEN_SPEC, STALLEN_A, O>;
impl<'a, const O: u8> STALLEN_W<'a, O> {
    #[doc = "Diasbles the STALL interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(STALLEN_A::_0)
    }
    #[doc = "Enables the STALL interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(STALLEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - USBRST Interrupt Enable"]
    #[inline(always)]
    pub fn usbrsten(&self) -> USBRSTEN_R {
        USBRSTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ERROR Interrupt Enable"]
    #[inline(always)]
    pub fn erroren(&self) -> ERROREN_R {
        ERROREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SOFTOK Interrupt Enable"]
    #[inline(always)]
    pub fn softoken(&self) -> SOFTOKEN_R {
        SOFTOKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TOKDNE Interrupt Enable"]
    #[inline(always)]
    pub fn tokdneen(&self) -> TOKDNEEN_R {
        TOKDNEEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SLEEP Interrupt Enable"]
    #[inline(always)]
    pub fn sleepen(&self) -> SLEEPEN_R {
        SLEEPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RESUME Interrupt Enable"]
    #[inline(always)]
    pub fn resumeen(&self) -> RESUMEEN_R {
        RESUMEEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ATTACH Interrupt Enable"]
    #[inline(always)]
    pub fn attachen(&self) -> ATTACHEN_R {
        ATTACHEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - STALL Interrupt Enable"]
    #[inline(always)]
    pub fn stallen(&self) -> STALLEN_R {
        STALLEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USBRST Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbrsten(&mut self) -> USBRSTEN_W<0> {
        USBRSTEN_W::new(self)
    }
    #[doc = "Bit 1 - ERROR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn erroren(&mut self) -> ERROREN_W<1> {
        ERROREN_W::new(self)
    }
    #[doc = "Bit 2 - SOFTOK Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn softoken(&mut self) -> SOFTOKEN_W<2> {
        SOFTOKEN_W::new(self)
    }
    #[doc = "Bit 3 - TOKDNE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tokdneen(&mut self) -> TOKDNEEN_W<3> {
        TOKDNEEN_W::new(self)
    }
    #[doc = "Bit 4 - SLEEP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sleepen(&mut self) -> SLEEPEN_W<4> {
        SLEEPEN_W::new(self)
    }
    #[doc = "Bit 5 - RESUME Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn resumeen(&mut self) -> RESUMEEN_W<5> {
        RESUMEEN_W::new(self)
    }
    #[doc = "Bit 6 - ATTACH Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn attachen(&mut self) -> ATTACHEN_W<6> {
        ATTACHEN_W::new(self)
    }
    #[doc = "Bit 7 - STALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stallen(&mut self) -> STALLEN_W<7> {
        STALLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
