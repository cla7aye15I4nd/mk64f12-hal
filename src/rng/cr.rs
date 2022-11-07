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
#[doc = "Field `GO` reader - Go"]
pub type GO_R = crate::BitReader<GO_A>;
#[doc = "Go\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GO_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<GO_A> for bool {
    #[inline(always)]
    fn from(variant: GO_A) -> Self {
        variant as u8 != 0
    }
}
impl GO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GO_A {
        match self.bits {
            false => GO_A::_0,
            true => GO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GO_A::_1
    }
}
#[doc = "Field `GO` writer - Go"]
pub type GO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, GO_A, O>;
impl<'a, const O: u8> GO_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GO_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GO_A::_1)
    }
}
#[doc = "Field `HA` reader - High Assurance"]
pub type HA_R = crate::BitReader<HA_A>;
#[doc = "High Assurance\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HA_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<HA_A> for bool {
    #[inline(always)]
    fn from(variant: HA_A) -> Self {
        variant as u8 != 0
    }
}
impl HA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HA_A {
        match self.bits {
            false => HA_A::_0,
            true => HA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HA_A::_1
    }
}
#[doc = "Field `HA` writer - High Assurance"]
pub type HA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, HA_A, O>;
impl<'a, const O: u8> HA_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HA_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HA_A::_1)
    }
}
#[doc = "Field `INTM` reader - Interrupt Mask"]
pub type INTM_R = crate::BitReader<INTM_A>;
#[doc = "Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTM_A {
    #[doc = "0: Not masked"]
    _0 = 0,
    #[doc = "1: Masked"]
    _1 = 1,
}
impl From<INTM_A> for bool {
    #[inline(always)]
    fn from(variant: INTM_A) -> Self {
        variant as u8 != 0
    }
}
impl INTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTM_A {
        match self.bits {
            false => INTM_A::_0,
            true => INTM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INTM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INTM_A::_1
    }
}
#[doc = "Field `INTM` writer - Interrupt Mask"]
pub type INTM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, INTM_A, O>;
impl<'a, const O: u8> INTM_W<'a, O> {
    #[doc = "Not masked"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTM_A::_0)
    }
    #[doc = "Masked"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTM_A::_1)
    }
}
#[doc = "Clear Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRI_AW {
    #[doc = "0: Do not clear the interrupt."]
    _0 = 0,
    #[doc = "1: Clear the interrupt. When you write 1 to this field, RNGA then resets the error-interrupt indicator (SR\\[ERRI\\]). This bit always reads as 0."]
    _1 = 1,
}
impl From<CLRI_AW> for bool {
    #[inline(always)]
    fn from(variant: CLRI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLRI` writer - Clear Interrupt"]
pub type CLRI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, CLRI_AW, O>;
impl<'a, const O: u8> CLRI_W<'a, O> {
    #[doc = "Do not clear the interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CLRI_AW::_0)
    }
    #[doc = "Clear the interrupt. When you write 1 to this field, RNGA then resets the error-interrupt indicator (SR\\[ERRI\\]). This bit always reads as 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CLRI_AW::_1)
    }
}
#[doc = "Field `SLP` reader - Sleep"]
pub type SLP_R = crate::BitReader<SLP_A>;
#[doc = "Sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLP_A {
    #[doc = "0: Normal mode"]
    _0 = 0,
    #[doc = "1: Sleep (low-power) mode"]
    _1 = 1,
}
impl From<SLP_A> for bool {
    #[inline(always)]
    fn from(variant: SLP_A) -> Self {
        variant as u8 != 0
    }
}
impl SLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLP_A {
        match self.bits {
            false => SLP_A::_0,
            true => SLP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLP_A::_1
    }
}
#[doc = "Field `SLP` writer - Sleep"]
pub type SLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, SLP_A, O>;
impl<'a, const O: u8> SLP_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLP_A::_0)
    }
    #[doc = "Sleep (low-power) mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Go"]
    #[inline(always)]
    pub fn go(&self) -> GO_R {
        GO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High Assurance"]
    #[inline(always)]
    pub fn ha(&self) -> HA_R {
        HA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Mask"]
    #[inline(always)]
    pub fn intm(&self) -> INTM_R {
        INTM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Sleep"]
    #[inline(always)]
    pub fn slp(&self) -> SLP_R {
        SLP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Go"]
    #[inline(always)]
    #[must_use]
    pub fn go(&mut self) -> GO_W<0> {
        GO_W::new(self)
    }
    #[doc = "Bit 1 - High Assurance"]
    #[inline(always)]
    #[must_use]
    pub fn ha(&mut self) -> HA_W<1> {
        HA_W::new(self)
    }
    #[doc = "Bit 2 - Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn intm(&mut self) -> INTM_W<2> {
        INTM_W::new(self)
    }
    #[doc = "Bit 3 - Clear Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn clri(&mut self) -> CLRI_W<3> {
        CLRI_W::new(self)
    }
    #[doc = "Bit 4 - Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn slp(&mut self) -> SLP_W<4> {
        SLP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RNGA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
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
