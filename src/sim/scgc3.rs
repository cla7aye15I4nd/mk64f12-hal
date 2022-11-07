#[doc = "Register `SCGC3` reader"]
pub struct R(crate::R<SCGC3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGC3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGC3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGC3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGC3` writer"]
pub struct W(crate::W<SCGC3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGC3_SPEC>;
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
impl From<crate::W<SCGC3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGC3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RNGA` reader - RNGA Clock Gate Control"]
pub type RNGA_R = crate::BitReader<RNGA_A>;
#[doc = "RNGA Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RNGA_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<RNGA_A> for bool {
    #[inline(always)]
    fn from(variant: RNGA_A) -> Self {
        variant as u8 != 0
    }
}
impl RNGA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNGA_A {
        match self.bits {
            false => RNGA_A::_0,
            true => RNGA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RNGA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RNGA_A::_1
    }
}
#[doc = "Field `RNGA` writer - RNGA Clock Gate Control"]
pub type RNGA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC3_SPEC, RNGA_A, O>;
impl<'a, const O: u8> RNGA_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RNGA_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RNGA_A::_1)
    }
}
#[doc = "Field `SPI2` reader - SPI2 Clock Gate Control"]
pub type SPI2_R = crate::BitReader<SPI2_A>;
#[doc = "SPI2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI2_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<SPI2_A> for bool {
    #[inline(always)]
    fn from(variant: SPI2_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI2_A {
        match self.bits {
            false => SPI2_A::_0,
            true => SPI2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPI2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPI2_A::_1
    }
}
#[doc = "Field `SPI2` writer - SPI2 Clock Gate Control"]
pub type SPI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC3_SPEC, SPI2_A, O>;
impl<'a, const O: u8> SPI2_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI2_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI2_A::_1)
    }
}
#[doc = "Field `SDHC` reader - SDHC Clock Gate Control"]
pub type SDHC_R = crate::BitReader<SDHC_A>;
#[doc = "SDHC Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDHC_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<SDHC_A> for bool {
    #[inline(always)]
    fn from(variant: SDHC_A) -> Self {
        variant as u8 != 0
    }
}
impl SDHC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDHC_A {
        match self.bits {
            false => SDHC_A::_0,
            true => SDHC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SDHC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SDHC_A::_1
    }
}
#[doc = "Field `SDHC` writer - SDHC Clock Gate Control"]
pub type SDHC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC3_SPEC, SDHC_A, O>;
impl<'a, const O: u8> SDHC_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SDHC_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SDHC_A::_1)
    }
}
#[doc = "Field `FTM2` reader - FTM2 Clock Gate Control"]
pub type FTM2_R = crate::BitReader<FTM2_A>;
#[doc = "FTM2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM2_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<FTM2_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2_A {
        match self.bits {
            false => FTM2_A::_0,
            true => FTM2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM2_A::_1
    }
}
#[doc = "Field `FTM2` writer - FTM2 Clock Gate Control"]
pub type FTM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC3_SPEC, FTM2_A, O>;
impl<'a, const O: u8> FTM2_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2_A::_1)
    }
}
#[doc = "Field `FTM3` reader - FTM3 Clock Gate Control"]
pub type FTM3_R = crate::BitReader<FTM3_A>;
#[doc = "FTM3 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM3_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<FTM3_A> for bool {
    #[inline(always)]
    fn from(variant: FTM3_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM3_A {
        match self.bits {
            false => FTM3_A::_0,
            true => FTM3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM3_A::_1
    }
}
#[doc = "Field `FTM3` writer - FTM3 Clock Gate Control"]
pub type FTM3_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC3_SPEC, FTM3_A, O>;
impl<'a, const O: u8> FTM3_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM3_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM3_A::_1)
    }
}
#[doc = "Field `ADC1` reader - ADC1 Clock Gate Control"]
pub type ADC1_R = crate::BitReader<ADC1_A>;
#[doc = "ADC1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC1_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<ADC1_A> for bool {
    #[inline(always)]
    fn from(variant: ADC1_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC1_A {
        match self.bits {
            false => ADC1_A::_0,
            true => ADC1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADC1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADC1_A::_1
    }
}
#[doc = "Field `ADC1` writer - ADC1 Clock Gate Control"]
pub type ADC1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC3_SPEC, ADC1_A, O>;
impl<'a, const O: u8> ADC1_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC1_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC1_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - RNGA Clock Gate Control"]
    #[inline(always)]
    pub fn rnga(&self) -> RNGA_R {
        RNGA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 12 - SPI2 Clock Gate Control"]
    #[inline(always)]
    pub fn spi2(&self) -> SPI2_R {
        SPI2_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 17 - SDHC Clock Gate Control"]
    #[inline(always)]
    pub fn sdhc(&self) -> SDHC_R {
        SDHC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - FTM2 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm2(&self) -> FTM2_R {
        FTM2_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - FTM3 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm3(&self) -> FTM3_R {
        FTM3_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - ADC1 Clock Gate Control"]
    #[inline(always)]
    pub fn adc1(&self) -> ADC1_R {
        ADC1_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RNGA Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn rnga(&mut self) -> RNGA_W<0> {
        RNGA_W::new(self)
    }
    #[doc = "Bit 12 - SPI2 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn spi2(&mut self) -> SPI2_W<12> {
        SPI2_W::new(self)
    }
    #[doc = "Bit 17 - SDHC Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn sdhc(&mut self) -> SDHC_W<17> {
        SDHC_W::new(self)
    }
    #[doc = "Bit 24 - FTM2 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn ftm2(&mut self) -> FTM2_W<24> {
        FTM2_W::new(self)
    }
    #[doc = "Bit 25 - FTM3 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn ftm3(&mut self) -> FTM3_W<25> {
        FTM3_W::new(self)
    }
    #[doc = "Bit 27 - ADC1 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn adc1(&mut self) -> ADC1_W<27> {
        ADC1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Gating Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgc3](index.html) module"]
pub struct SCGC3_SPEC;
impl crate::RegisterSpec for SCGC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgc3::R](R) reader structure"]
impl crate::Readable for SCGC3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgc3::W](W) writer structure"]
impl crate::Writable for SCGC3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGC3 to value 0"]
impl crate::Resettable for SCGC3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
