#[doc = "Register `RCR3` reader"]
pub struct R(crate::R<RCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCR3` writer"]
pub struct W(crate::W<RCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCR3_SPEC>;
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
impl From<crate::W<RCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDFL` reader - Word Flag Configuration"]
pub type WDFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDFL` writer - Word Flag Configuration"]
pub type WDFL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `RCE0` reader - Receive Channel Enable"]
pub type RCE0_R = crate::BitReader<RCE0_A>;
#[doc = "Receive Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCE0_A {
    #[doc = "0: Receive data channel N is disabled."]
    _0 = 0,
    #[doc = "1: Receive data channel N is enabled."]
    _1 = 1,
}
impl From<RCE0_A> for bool {
    #[inline(always)]
    fn from(variant: RCE0_A) -> Self {
        variant as u8 != 0
    }
}
impl RCE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCE0_A {
        match self.bits {
            false => RCE0_A::_0,
            true => RCE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCE0_A::_1
    }
}
#[doc = "Field `RCE0` writer - Receive Channel Enable"]
pub type RCE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR3_SPEC, RCE0_A, O>;
impl<'a, const O: u8> RCE0_W<'a, O> {
    #[doc = "Receive data channel N is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCE0_A::_0)
    }
    #[doc = "Receive data channel N is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCE0_A::_1)
    }
}
#[doc = "Field `RCE1` reader - Receive Channel Enable"]
pub type RCE1_R = crate::BitReader<RCE1_A>;
#[doc = "Receive Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCE1_A {
    #[doc = "0: Receive data channel N is disabled."]
    _0 = 0,
    #[doc = "1: Receive data channel N is enabled."]
    _1 = 1,
}
impl From<RCE1_A> for bool {
    #[inline(always)]
    fn from(variant: RCE1_A) -> Self {
        variant as u8 != 0
    }
}
impl RCE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCE1_A {
        match self.bits {
            false => RCE1_A::_0,
            true => RCE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RCE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RCE1_A::_1
    }
}
#[doc = "Field `RCE1` writer - Receive Channel Enable"]
pub type RCE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR3_SPEC, RCE1_A, O>;
impl<'a, const O: u8> RCE1_W<'a, O> {
    #[doc = "Receive data channel N is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RCE1_A::_0)
    }
    #[doc = "Receive data channel N is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RCE1_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Word Flag Configuration"]
    #[inline(always)]
    pub fn wdfl(&self) -> WDFL_R {
        WDFL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Receive Channel Enable"]
    #[inline(always)]
    pub fn rce0(&self) -> RCE0_R {
        RCE0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receive Channel Enable"]
    #[inline(always)]
    pub fn rce1(&self) -> RCE1_R {
        RCE1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Word Flag Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn wdfl(&mut self) -> WDFL_W<0> {
        WDFL_W::new(self)
    }
    #[doc = "Bit 16 - Receive Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rce0(&mut self) -> RCE0_W<16> {
        RCE0_W::new(self)
    }
    #[doc = "Bit 17 - Receive Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rce1(&mut self) -> RCE1_W<17> {
        RCE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI Receive Configuration 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr3](index.html) module"]
pub struct RCR3_SPEC;
impl crate::RegisterSpec for RCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcr3::R](R) reader structure"]
impl crate::Readable for RCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcr3::W](W) writer structure"]
impl crate::Writable for RCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCR3 to value 0"]
impl crate::Resettable for RCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
