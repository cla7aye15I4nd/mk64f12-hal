#[doc = "Register `SC3` reader"]
pub struct R(crate::R<SC3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SC3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SC3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC3` writer"]
pub struct W(crate::W<SC3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC3_SPEC>;
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
impl From<crate::W<SC3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SC3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVGS` reader - Hardware Average Select"]
pub type AVGS_R = crate::FieldReader<u8, AVGS_A>;
#[doc = "Hardware Average Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AVGS_A {
    #[doc = "0: 4 samples averaged."]
    _00 = 0,
    #[doc = "1: 8 samples averaged."]
    _01 = 1,
    #[doc = "2: 16 samples averaged."]
    _10 = 2,
    #[doc = "3: 32 samples averaged."]
    _11 = 3,
}
impl From<AVGS_A> for u8 {
    #[inline(always)]
    fn from(variant: AVGS_A) -> Self {
        variant as _
    }
}
impl AVGS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVGS_A {
        match self.bits {
            0 => AVGS_A::_00,
            1 => AVGS_A::_01,
            2 => AVGS_A::_10,
            3 => AVGS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == AVGS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == AVGS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == AVGS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == AVGS_A::_11
    }
}
#[doc = "Field `AVGS` writer - Hardware Average Select"]
pub type AVGS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SC3_SPEC, u8, AVGS_A, 2, O>;
impl<'a, const O: u8> AVGS_W<'a, O> {
    #[doc = "4 samples averaged."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(AVGS_A::_00)
    }
    #[doc = "8 samples averaged."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(AVGS_A::_01)
    }
    #[doc = "16 samples averaged."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(AVGS_A::_10)
    }
    #[doc = "32 samples averaged."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(AVGS_A::_11)
    }
}
#[doc = "Field `AVGE` reader - Hardware Average Enable"]
pub type AVGE_R = crate::BitReader<AVGE_A>;
#[doc = "Hardware Average Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVGE_A {
    #[doc = "0: Hardware average function disabled."]
    _0 = 0,
    #[doc = "1: Hardware average function enabled."]
    _1 = 1,
}
impl From<AVGE_A> for bool {
    #[inline(always)]
    fn from(variant: AVGE_A) -> Self {
        variant as u8 != 0
    }
}
impl AVGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVGE_A {
        match self.bits {
            false => AVGE_A::_0,
            true => AVGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AVGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AVGE_A::_1
    }
}
#[doc = "Field `AVGE` writer - Hardware Average Enable"]
pub type AVGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC3_SPEC, AVGE_A, O>;
impl<'a, const O: u8> AVGE_W<'a, O> {
    #[doc = "Hardware average function disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVGE_A::_0)
    }
    #[doc = "Hardware average function enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVGE_A::_1)
    }
}
#[doc = "Field `ADCO` reader - Continuous Conversion Enable"]
pub type ADCO_R = crate::BitReader<ADCO_A>;
#[doc = "Continuous Conversion Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCO_A {
    #[doc = "0: One conversion or one set of conversions if the hardware average function is enabled, that is, AVGE=1, after initiating a conversion."]
    _0 = 0,
    #[doc = "1: Continuous conversions or sets of conversions if the hardware average function is enabled, that is, AVGE=1, after initiating a conversion."]
    _1 = 1,
}
impl From<ADCO_A> for bool {
    #[inline(always)]
    fn from(variant: ADCO_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCO_A {
        match self.bits {
            false => ADCO_A::_0,
            true => ADCO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADCO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADCO_A::_1
    }
}
#[doc = "Field `ADCO` writer - Continuous Conversion Enable"]
pub type ADCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC3_SPEC, ADCO_A, O>;
impl<'a, const O: u8> ADCO_W<'a, O> {
    #[doc = "One conversion or one set of conversions if the hardware average function is enabled, that is, AVGE=1, after initiating a conversion."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADCO_A::_0)
    }
    #[doc = "Continuous conversions or sets of conversions if the hardware average function is enabled, that is, AVGE=1, after initiating a conversion."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADCO_A::_1)
    }
}
#[doc = "Field `CALF` reader - Calibration Failed Flag"]
pub type CALF_R = crate::BitReader<CALF_A>;
#[doc = "Calibration Failed Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALF_A {
    #[doc = "0: Calibration completed normally."]
    _0 = 0,
    #[doc = "1: Calibration failed. ADC accuracy specifications are not guaranteed."]
    _1 = 1,
}
impl From<CALF_A> for bool {
    #[inline(always)]
    fn from(variant: CALF_A) -> Self {
        variant as u8 != 0
    }
}
impl CALF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALF_A {
        match self.bits {
            false => CALF_A::_0,
            true => CALF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CALF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CALF_A::_1
    }
}
#[doc = "Field `CALF` writer - Calibration Failed Flag"]
pub type CALF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC3_SPEC, CALF_A, O>;
impl<'a, const O: u8> CALF_W<'a, O> {
    #[doc = "Calibration completed normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CALF_A::_0)
    }
    #[doc = "Calibration failed. ADC accuracy specifications are not guaranteed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CALF_A::_1)
    }
}
#[doc = "Field `CAL` reader - Calibration"]
pub type CAL_R = crate::BitReader<bool>;
#[doc = "Field `CAL` writer - Calibration"]
pub type CAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC3_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Hardware Average Select"]
    #[inline(always)]
    pub fn avgs(&self) -> AVGS_R {
        AVGS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Hardware Average Enable"]
    #[inline(always)]
    pub fn avge(&self) -> AVGE_R {
        AVGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Continuous Conversion Enable"]
    #[inline(always)]
    pub fn adco(&self) -> ADCO_R {
        ADCO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Calibration Failed Flag"]
    #[inline(always)]
    pub fn calf(&self) -> CALF_R {
        CALF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Calibration"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Hardware Average Select"]
    #[inline(always)]
    #[must_use]
    pub fn avgs(&mut self) -> AVGS_W<0> {
        AVGS_W::new(self)
    }
    #[doc = "Bit 2 - Hardware Average Enable"]
    #[inline(always)]
    #[must_use]
    pub fn avge(&mut self) -> AVGE_W<2> {
        AVGE_W::new(self)
    }
    #[doc = "Bit 3 - Continuous Conversion Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adco(&mut self) -> ADCO_W<3> {
        ADCO_W::new(self)
    }
    #[doc = "Bit 6 - Calibration Failed Flag"]
    #[inline(always)]
    #[must_use]
    pub fn calf(&mut self) -> CALF_W<6> {
        CALF_W::new(self)
    }
    #[doc = "Bit 7 - Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CAL_W<7> {
        CAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status and Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc3](index.html) module"]
pub struct SC3_SPEC;
impl crate::RegisterSpec for SC3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc3::R](R) reader structure"]
impl crate::Readable for SC3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc3::W](W) writer structure"]
impl crate::Writable for SC3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SC3 to value 0"]
impl crate::Resettable for SC3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
