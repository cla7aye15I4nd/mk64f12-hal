#[doc = "Register `C0` reader"]
pub struct R(crate::R<C0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C0` writer"]
pub struct W(crate::W<C0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C0_SPEC>;
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
impl From<crate::W<C0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACBBIEN` reader - DAC Buffer Read Pointer Bottom Flag Interrupt Enable"]
pub type DACBBIEN_R = crate::BitReader<DACBBIEN_A>;
#[doc = "DAC Buffer Read Pointer Bottom Flag Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACBBIEN_A {
    #[doc = "0: The DAC buffer read pointer bottom flag interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The DAC buffer read pointer bottom flag interrupt is enabled."]
    _1 = 1,
}
impl From<DACBBIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACBBIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DACBBIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBBIEN_A {
        match self.bits {
            false => DACBBIEN_A::_0,
            true => DACBBIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACBBIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACBBIEN_A::_1
    }
}
#[doc = "Field `DACBBIEN` writer - DAC Buffer Read Pointer Bottom Flag Interrupt Enable"]
pub type DACBBIEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, C0_SPEC, DACBBIEN_A, O>;
impl<'a, const O: u8> DACBBIEN_W<'a, O> {
    #[doc = "The DAC buffer read pointer bottom flag interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBBIEN_A::_0)
    }
    #[doc = "The DAC buffer read pointer bottom flag interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBBIEN_A::_1)
    }
}
#[doc = "Field `DACBTIEN` reader - DAC Buffer Read Pointer Top Flag Interrupt Enable"]
pub type DACBTIEN_R = crate::BitReader<DACBTIEN_A>;
#[doc = "DAC Buffer Read Pointer Top Flag Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACBTIEN_A {
    #[doc = "0: The DAC buffer read pointer top flag interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The DAC buffer read pointer top flag interrupt is enabled."]
    _1 = 1,
}
impl From<DACBTIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACBTIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DACBTIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBTIEN_A {
        match self.bits {
            false => DACBTIEN_A::_0,
            true => DACBTIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACBTIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACBTIEN_A::_1
    }
}
#[doc = "Field `DACBTIEN` writer - DAC Buffer Read Pointer Top Flag Interrupt Enable"]
pub type DACBTIEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, C0_SPEC, DACBTIEN_A, O>;
impl<'a, const O: u8> DACBTIEN_W<'a, O> {
    #[doc = "The DAC buffer read pointer top flag interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBTIEN_A::_0)
    }
    #[doc = "The DAC buffer read pointer top flag interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBTIEN_A::_1)
    }
}
#[doc = "Field `DACBWIEN` reader - DAC Buffer Watermark Interrupt Enable"]
pub type DACBWIEN_R = crate::BitReader<DACBWIEN_A>;
#[doc = "DAC Buffer Watermark Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACBWIEN_A {
    #[doc = "0: The DAC buffer watermark interrupt is disabled."]
    _0 = 0,
    #[doc = "1: The DAC buffer watermark interrupt is enabled."]
    _1 = 1,
}
impl From<DACBWIEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACBWIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DACBWIEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBWIEN_A {
        match self.bits {
            false => DACBWIEN_A::_0,
            true => DACBWIEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACBWIEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACBWIEN_A::_1
    }
}
#[doc = "Field `DACBWIEN` writer - DAC Buffer Watermark Interrupt Enable"]
pub type DACBWIEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, C0_SPEC, DACBWIEN_A, O>;
impl<'a, const O: u8> DACBWIEN_W<'a, O> {
    #[doc = "The DAC buffer watermark interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBWIEN_A::_0)
    }
    #[doc = "The DAC buffer watermark interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBWIEN_A::_1)
    }
}
#[doc = "Field `LPEN` reader - DAC Low Power Control"]
pub type LPEN_R = crate::BitReader<LPEN_A>;
#[doc = "DAC Low Power Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPEN_A {
    #[doc = "0: High-Power mode"]
    _0 = 0,
    #[doc = "1: Low-Power mode"]
    _1 = 1,
}
impl From<LPEN_A> for bool {
    #[inline(always)]
    fn from(variant: LPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPEN_A {
        match self.bits {
            false => LPEN_A::_0,
            true => LPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LPEN_A::_1
    }
}
#[doc = "Field `LPEN` writer - DAC Low Power Control"]
pub type LPEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, C0_SPEC, LPEN_A, O>;
impl<'a, const O: u8> LPEN_W<'a, O> {
    #[doc = "High-Power mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LPEN_A::_0)
    }
    #[doc = "Low-Power mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LPEN_A::_1)
    }
}
#[doc = "DAC Software Trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACSWTRG_AW {
    #[doc = "0: The DAC soft trigger is not valid."]
    _0 = 0,
    #[doc = "1: The DAC soft trigger is valid."]
    _1 = 1,
}
impl From<DACSWTRG_AW> for bool {
    #[inline(always)]
    fn from(variant: DACSWTRG_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DACSWTRG` writer - DAC Software Trigger"]
pub type DACSWTRG_W<'a, const O: u8> = crate::BitWriter<'a, u8, C0_SPEC, DACSWTRG_AW, O>;
impl<'a, const O: u8> DACSWTRG_W<'a, O> {
    #[doc = "The DAC soft trigger is not valid."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACSWTRG_AW::_0)
    }
    #[doc = "The DAC soft trigger is valid."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACSWTRG_AW::_1)
    }
}
#[doc = "Field `DACTRGSEL` reader - DAC Trigger Select"]
pub type DACTRGSEL_R = crate::BitReader<DACTRGSEL_A>;
#[doc = "DAC Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACTRGSEL_A {
    #[doc = "0: The DAC hardware trigger is selected."]
    _0 = 0,
    #[doc = "1: The DAC software trigger is selected."]
    _1 = 1,
}
impl From<DACTRGSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DACTRGSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DACTRGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACTRGSEL_A {
        match self.bits {
            false => DACTRGSEL_A::_0,
            true => DACTRGSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACTRGSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACTRGSEL_A::_1
    }
}
#[doc = "Field `DACTRGSEL` writer - DAC Trigger Select"]
pub type DACTRGSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, C0_SPEC, DACTRGSEL_A, O>;
impl<'a, const O: u8> DACTRGSEL_W<'a, O> {
    #[doc = "The DAC hardware trigger is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACTRGSEL_A::_0)
    }
    #[doc = "The DAC software trigger is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACTRGSEL_A::_1)
    }
}
#[doc = "Field `DACRFS` reader - DAC Reference Select"]
pub type DACRFS_R = crate::BitReader<DACRFS_A>;
#[doc = "DAC Reference Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACRFS_A {
    #[doc = "0: The DAC selects DACREF_1 as the reference voltage."]
    _0 = 0,
    #[doc = "1: The DAC selects DACREF_2 as the reference voltage."]
    _1 = 1,
}
impl From<DACRFS_A> for bool {
    #[inline(always)]
    fn from(variant: DACRFS_A) -> Self {
        variant as u8 != 0
    }
}
impl DACRFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACRFS_A {
        match self.bits {
            false => DACRFS_A::_0,
            true => DACRFS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACRFS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACRFS_A::_1
    }
}
#[doc = "Field `DACRFS` writer - DAC Reference Select"]
pub type DACRFS_W<'a, const O: u8> = crate::BitWriter<'a, u8, C0_SPEC, DACRFS_A, O>;
impl<'a, const O: u8> DACRFS_W<'a, O> {
    #[doc = "The DAC selects DACREF_1 as the reference voltage."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACRFS_A::_0)
    }
    #[doc = "The DAC selects DACREF_2 as the reference voltage."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACRFS_A::_1)
    }
}
#[doc = "Field `DACEN` reader - DAC Enable"]
pub type DACEN_R = crate::BitReader<DACEN_A>;
#[doc = "DAC Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACEN_A {
    #[doc = "0: The DAC system is disabled."]
    _0 = 0,
    #[doc = "1: The DAC system is enabled."]
    _1 = 1,
}
impl From<DACEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DACEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACEN_A {
        match self.bits {
            false => DACEN_A::_0,
            true => DACEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACEN_A::_1
    }
}
#[doc = "Field `DACEN` writer - DAC Enable"]
pub type DACEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, C0_SPEC, DACEN_A, O>;
impl<'a, const O: u8> DACEN_W<'a, O> {
    #[doc = "The DAC system is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACEN_A::_0)
    }
    #[doc = "The DAC system is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DAC Buffer Read Pointer Bottom Flag Interrupt Enable"]
    #[inline(always)]
    pub fn dacbbien(&self) -> DACBBIEN_R {
        DACBBIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC Buffer Read Pointer Top Flag Interrupt Enable"]
    #[inline(always)]
    pub fn dacbtien(&self) -> DACBTIEN_R {
        DACBTIEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DAC Buffer Watermark Interrupt Enable"]
    #[inline(always)]
    pub fn dacbwien(&self) -> DACBWIEN_R {
        DACBWIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DAC Low Power Control"]
    #[inline(always)]
    pub fn lpen(&self) -> LPEN_R {
        LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - DAC Trigger Select"]
    #[inline(always)]
    pub fn dactrgsel(&self) -> DACTRGSEL_R {
        DACTRGSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DAC Reference Select"]
    #[inline(always)]
    pub fn dacrfs(&self) -> DACRFS_R {
        DACRFS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC Buffer Read Pointer Bottom Flag Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacbbien(&mut self) -> DACBBIEN_W<0> {
        DACBBIEN_W::new(self)
    }
    #[doc = "Bit 1 - DAC Buffer Read Pointer Top Flag Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacbtien(&mut self) -> DACBTIEN_W<1> {
        DACBTIEN_W::new(self)
    }
    #[doc = "Bit 2 - DAC Buffer Watermark Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacbwien(&mut self) -> DACBWIEN_W<2> {
        DACBWIEN_W::new(self)
    }
    #[doc = "Bit 3 - DAC Low Power Control"]
    #[inline(always)]
    #[must_use]
    pub fn lpen(&mut self) -> LPEN_W<3> {
        LPEN_W::new(self)
    }
    #[doc = "Bit 4 - DAC Software Trigger"]
    #[inline(always)]
    #[must_use]
    pub fn dacswtrg(&mut self) -> DACSWTRG_W<4> {
        DACSWTRG_W::new(self)
    }
    #[doc = "Bit 5 - DAC Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn dactrgsel(&mut self) -> DACTRGSEL_W<5> {
        DACTRGSEL_W::new(self)
    }
    #[doc = "Bit 6 - DAC Reference Select"]
    #[inline(always)]
    #[must_use]
    pub fn dacrfs(&mut self) -> DACRFS_W<6> {
        DACRFS_W::new(self)
    }
    #[doc = "Bit 7 - DAC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacen(&mut self) -> DACEN_W<7> {
        DACEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c0](index.html) module"]
pub struct C0_SPEC;
impl crate::RegisterSpec for C0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c0::R](R) reader structure"]
impl crate::Readable for C0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c0::W](W) writer structure"]
impl crate::Writable for C0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C0 to value 0"]
impl crate::Resettable for C0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
