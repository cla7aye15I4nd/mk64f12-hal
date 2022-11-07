#[doc = "Register `POEN` reader"]
pub struct R(crate::R<POEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POEN` writer"]
pub struct W(crate::W<POEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POEN_SPEC>;
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
impl From<crate::W<POEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POEN0` reader - PDB Pulse-Out Enable"]
pub type POEN0_R = crate::BitReader<POEN0_A>;
#[doc = "PDB Pulse-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POEN0_A {
    #[doc = "0: PDB Pulse-Out disabled"]
    _0 = 0,
    #[doc = "1: PDB Pulse-Out enabled"]
    _1 = 1,
}
impl From<POEN0_A> for bool {
    #[inline(always)]
    fn from(variant: POEN0_A) -> Self {
        variant as u8 != 0
    }
}
impl POEN0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN0_A {
        match self.bits {
            false => POEN0_A::_0,
            true => POEN0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POEN0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POEN0_A::_1
    }
}
#[doc = "Field `POEN0` writer - PDB Pulse-Out Enable"]
pub type POEN0_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN_SPEC, POEN0_A, O>;
impl<'a, const O: u8> POEN0_W<'a, O> {
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN0_A::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN0_A::_1)
    }
}
#[doc = "Field `POEN1` reader - PDB Pulse-Out Enable"]
pub type POEN1_R = crate::BitReader<POEN1_A>;
#[doc = "PDB Pulse-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POEN1_A {
    #[doc = "0: PDB Pulse-Out disabled"]
    _0 = 0,
    #[doc = "1: PDB Pulse-Out enabled"]
    _1 = 1,
}
impl From<POEN1_A> for bool {
    #[inline(always)]
    fn from(variant: POEN1_A) -> Self {
        variant as u8 != 0
    }
}
impl POEN1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN1_A {
        match self.bits {
            false => POEN1_A::_0,
            true => POEN1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POEN1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POEN1_A::_1
    }
}
#[doc = "Field `POEN1` writer - PDB Pulse-Out Enable"]
pub type POEN1_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN_SPEC, POEN1_A, O>;
impl<'a, const O: u8> POEN1_W<'a, O> {
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN1_A::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN1_A::_1)
    }
}
#[doc = "Field `POEN2` reader - PDB Pulse-Out Enable"]
pub type POEN2_R = crate::BitReader<POEN2_A>;
#[doc = "PDB Pulse-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POEN2_A {
    #[doc = "0: PDB Pulse-Out disabled"]
    _0 = 0,
    #[doc = "1: PDB Pulse-Out enabled"]
    _1 = 1,
}
impl From<POEN2_A> for bool {
    #[inline(always)]
    fn from(variant: POEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl POEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN2_A {
        match self.bits {
            false => POEN2_A::_0,
            true => POEN2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POEN2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POEN2_A::_1
    }
}
#[doc = "Field `POEN2` writer - PDB Pulse-Out Enable"]
pub type POEN2_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN_SPEC, POEN2_A, O>;
impl<'a, const O: u8> POEN2_W<'a, O> {
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN2_A::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN2_A::_1)
    }
}
#[doc = "Field `POEN3` reader - PDB Pulse-Out Enable"]
pub type POEN3_R = crate::BitReader<POEN3_A>;
#[doc = "PDB Pulse-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POEN3_A {
    #[doc = "0: PDB Pulse-Out disabled"]
    _0 = 0,
    #[doc = "1: PDB Pulse-Out enabled"]
    _1 = 1,
}
impl From<POEN3_A> for bool {
    #[inline(always)]
    fn from(variant: POEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl POEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN3_A {
        match self.bits {
            false => POEN3_A::_0,
            true => POEN3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POEN3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POEN3_A::_1
    }
}
#[doc = "Field `POEN3` writer - PDB Pulse-Out Enable"]
pub type POEN3_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN_SPEC, POEN3_A, O>;
impl<'a, const O: u8> POEN3_W<'a, O> {
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN3_A::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN3_A::_1)
    }
}
#[doc = "Field `POEN4` reader - PDB Pulse-Out Enable"]
pub type POEN4_R = crate::BitReader<POEN4_A>;
#[doc = "PDB Pulse-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POEN4_A {
    #[doc = "0: PDB Pulse-Out disabled"]
    _0 = 0,
    #[doc = "1: PDB Pulse-Out enabled"]
    _1 = 1,
}
impl From<POEN4_A> for bool {
    #[inline(always)]
    fn from(variant: POEN4_A) -> Self {
        variant as u8 != 0
    }
}
impl POEN4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN4_A {
        match self.bits {
            false => POEN4_A::_0,
            true => POEN4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POEN4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POEN4_A::_1
    }
}
#[doc = "Field `POEN4` writer - PDB Pulse-Out Enable"]
pub type POEN4_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN_SPEC, POEN4_A, O>;
impl<'a, const O: u8> POEN4_W<'a, O> {
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN4_A::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN4_A::_1)
    }
}
#[doc = "Field `POEN5` reader - PDB Pulse-Out Enable"]
pub type POEN5_R = crate::BitReader<POEN5_A>;
#[doc = "PDB Pulse-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POEN5_A {
    #[doc = "0: PDB Pulse-Out disabled"]
    _0 = 0,
    #[doc = "1: PDB Pulse-Out enabled"]
    _1 = 1,
}
impl From<POEN5_A> for bool {
    #[inline(always)]
    fn from(variant: POEN5_A) -> Self {
        variant as u8 != 0
    }
}
impl POEN5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN5_A {
        match self.bits {
            false => POEN5_A::_0,
            true => POEN5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POEN5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POEN5_A::_1
    }
}
#[doc = "Field `POEN5` writer - PDB Pulse-Out Enable"]
pub type POEN5_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN_SPEC, POEN5_A, O>;
impl<'a, const O: u8> POEN5_W<'a, O> {
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN5_A::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN5_A::_1)
    }
}
#[doc = "Field `POEN6` reader - PDB Pulse-Out Enable"]
pub type POEN6_R = crate::BitReader<POEN6_A>;
#[doc = "PDB Pulse-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POEN6_A {
    #[doc = "0: PDB Pulse-Out disabled"]
    _0 = 0,
    #[doc = "1: PDB Pulse-Out enabled"]
    _1 = 1,
}
impl From<POEN6_A> for bool {
    #[inline(always)]
    fn from(variant: POEN6_A) -> Self {
        variant as u8 != 0
    }
}
impl POEN6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN6_A {
        match self.bits {
            false => POEN6_A::_0,
            true => POEN6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POEN6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POEN6_A::_1
    }
}
#[doc = "Field `POEN6` writer - PDB Pulse-Out Enable"]
pub type POEN6_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN_SPEC, POEN6_A, O>;
impl<'a, const O: u8> POEN6_W<'a, O> {
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN6_A::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN6_A::_1)
    }
}
#[doc = "Field `POEN7` reader - PDB Pulse-Out Enable"]
pub type POEN7_R = crate::BitReader<POEN7_A>;
#[doc = "PDB Pulse-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POEN7_A {
    #[doc = "0: PDB Pulse-Out disabled"]
    _0 = 0,
    #[doc = "1: PDB Pulse-Out enabled"]
    _1 = 1,
}
impl From<POEN7_A> for bool {
    #[inline(always)]
    fn from(variant: POEN7_A) -> Self {
        variant as u8 != 0
    }
}
impl POEN7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POEN7_A {
        match self.bits {
            false => POEN7_A::_0,
            true => POEN7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POEN7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POEN7_A::_1
    }
}
#[doc = "Field `POEN7` writer - PDB Pulse-Out Enable"]
pub type POEN7_W<'a, const O: u8> = crate::BitWriter<'a, u32, POEN_SPEC, POEN7_A, O>;
impl<'a, const O: u8> POEN7_W<'a, O> {
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN7_A::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN7_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen0(&self) -> POEN0_R {
        POEN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen1(&self) -> POEN1_R {
        POEN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen2(&self) -> POEN2_R {
        POEN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen3(&self) -> POEN3_R {
        POEN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen4(&self) -> POEN4_R {
        POEN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen5(&self) -> POEN5_R {
        POEN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen6(&self) -> POEN6_R {
        POEN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen7(&self) -> POEN7_R {
        POEN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDB Pulse-Out Enable"]
    #[inline(always)]
    #[must_use]
    pub fn poen0(&mut self) -> POEN0_W<0> {
        POEN0_W::new(self)
    }
    #[doc = "Bit 1 - PDB Pulse-Out Enable"]
    #[inline(always)]
    #[must_use]
    pub fn poen1(&mut self) -> POEN1_W<1> {
        POEN1_W::new(self)
    }
    #[doc = "Bit 2 - PDB Pulse-Out Enable"]
    #[inline(always)]
    #[must_use]
    pub fn poen2(&mut self) -> POEN2_W<2> {
        POEN2_W::new(self)
    }
    #[doc = "Bit 3 - PDB Pulse-Out Enable"]
    #[inline(always)]
    #[must_use]
    pub fn poen3(&mut self) -> POEN3_W<3> {
        POEN3_W::new(self)
    }
    #[doc = "Bit 4 - PDB Pulse-Out Enable"]
    #[inline(always)]
    #[must_use]
    pub fn poen4(&mut self) -> POEN4_W<4> {
        POEN4_W::new(self)
    }
    #[doc = "Bit 5 - PDB Pulse-Out Enable"]
    #[inline(always)]
    #[must_use]
    pub fn poen5(&mut self) -> POEN5_W<5> {
        POEN5_W::new(self)
    }
    #[doc = "Bit 6 - PDB Pulse-Out Enable"]
    #[inline(always)]
    #[must_use]
    pub fn poen6(&mut self) -> POEN6_W<6> {
        POEN6_W::new(self)
    }
    #[doc = "Bit 7 - PDB Pulse-Out Enable"]
    #[inline(always)]
    #[must_use]
    pub fn poen7(&mut self) -> POEN7_W<7> {
        POEN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pulse-Out n Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [poen](index.html) module"]
pub struct POEN_SPEC;
impl crate::RegisterSpec for POEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [poen::R](R) reader structure"]
impl crate::Readable for POEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [poen::W](W) writer structure"]
impl crate::Writable for POEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POEN to value 0"]
impl crate::Resettable for POEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
