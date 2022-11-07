#[doc = "Register `TCR3` reader"]
pub struct R(crate::R<TCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCR3` writer"]
pub struct W(crate::W<TCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCR3_SPEC>;
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
impl From<crate::W<TCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDFL` reader - Word Flag Configuration"]
pub type WDFL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDFL` writer - Word Flag Configuration"]
pub type WDFL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR3_SPEC, u8, u8, 5, O>;
#[doc = "Field `TCE0` reader - Transmit Channel Enable"]
pub type TCE0_R = crate::BitReader<TCE0_A>;
#[doc = "Transmit Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCE0_A {
    #[doc = "0: Transmit data channel N is disabled."]
    _0 = 0,
    #[doc = "1: Transmit data channel N is enabled."]
    _1 = 1,
}
impl From<TCE0_A> for bool {
    #[inline(always)]
    fn from(variant: TCE0_A) -> Self {
        variant as u8 != 0
    }
}
impl TCE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCE0_A {
        match self.bits {
            false => TCE0_A::_0,
            true => TCE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCE0_A::_1
    }
}
#[doc = "Field `TCE0` writer - Transmit Channel Enable"]
pub type TCE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR3_SPEC, TCE0_A, O>;
impl<'a, const O: u8> TCE0_W<'a, O> {
    #[doc = "Transmit data channel N is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCE0_A::_0)
    }
    #[doc = "Transmit data channel N is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCE0_A::_1)
    }
}
#[doc = "Field `TCE1` reader - Transmit Channel Enable"]
pub type TCE1_R = crate::BitReader<TCE1_A>;
#[doc = "Transmit Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCE1_A {
    #[doc = "0: Transmit data channel N is disabled."]
    _0 = 0,
    #[doc = "1: Transmit data channel N is enabled."]
    _1 = 1,
}
impl From<TCE1_A> for bool {
    #[inline(always)]
    fn from(variant: TCE1_A) -> Self {
        variant as u8 != 0
    }
}
impl TCE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCE1_A {
        match self.bits {
            false => TCE1_A::_0,
            true => TCE1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCE1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCE1_A::_1
    }
}
#[doc = "Field `TCE1` writer - Transmit Channel Enable"]
pub type TCE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR3_SPEC, TCE1_A, O>;
impl<'a, const O: u8> TCE1_W<'a, O> {
    #[doc = "Transmit data channel N is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCE1_A::_0)
    }
    #[doc = "Transmit data channel N is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCE1_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - Word Flag Configuration"]
    #[inline(always)]
    pub fn wdfl(&self) -> WDFL_R {
        WDFL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Transmit Channel Enable"]
    #[inline(always)]
    pub fn tce0(&self) -> TCE0_R {
        TCE0_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transmit Channel Enable"]
    #[inline(always)]
    pub fn tce1(&self) -> TCE1_R {
        TCE1_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Word Flag Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn wdfl(&mut self) -> WDFL_W<0> {
        WDFL_W::new(self)
    }
    #[doc = "Bit 16 - Transmit Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tce0(&mut self) -> TCE0_W<16> {
        TCE0_W::new(self)
    }
    #[doc = "Bit 17 - Transmit Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tce1(&mut self) -> TCE1_W<17> {
        TCE1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI Transmit Configuration 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr3](index.html) module"]
pub struct TCR3_SPEC;
impl crate::RegisterSpec for TCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcr3::R](R) reader structure"]
impl crate::Readable for TCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcr3::W](W) writer structure"]
impl crate::Writable for TCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCR3 to value 0"]
impl crate::Resettable for TCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
