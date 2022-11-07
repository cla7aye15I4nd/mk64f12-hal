#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTMEN` reader - FTM Enable"]
pub type FTMEN_R = crate::BitReader<FTMEN_A>;
#[doc = "FTM Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTMEN_A {
    #[doc = "0: Only the TPM-compatible registers (first set of registers) can be used without any restriction. Do not use the FTM-specific registers."]
    _0 = 0,
    #[doc = "1: All registers including the FTM-specific registers (second set of registers) are available for use with no restrictions."]
    _1 = 1,
}
impl From<FTMEN_A> for bool {
    #[inline(always)]
    fn from(variant: FTMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FTMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTMEN_A {
        match self.bits {
            false => FTMEN_A::_0,
            true => FTMEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTMEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTMEN_A::_1
    }
}
#[doc = "Field `FTMEN` writer - FTM Enable"]
pub type FTMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, FTMEN_A, O>;
impl<'a, const O: u8> FTMEN_W<'a, O> {
    #[doc = "Only the TPM-compatible registers (first set of registers) can be used without any restriction. Do not use the FTM-specific registers."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTMEN_A::_0)
    }
    #[doc = "All registers including the FTM-specific registers (second set of registers) are available for use with no restrictions."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTMEN_A::_1)
    }
}
#[doc = "Field `INIT` reader - Initialize The Channels Output"]
pub type INIT_R = crate::BitReader<bool>;
#[doc = "Field `INIT` writer - Initialize The Channels Output"]
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, bool, O>;
#[doc = "Field `WPDIS` reader - Write Protection Disable"]
pub type WPDIS_R = crate::BitReader<WPDIS_A>;
#[doc = "Write Protection Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPDIS_A {
    #[doc = "0: Write protection is enabled."]
    _0 = 0,
    #[doc = "1: Write protection is disabled."]
    _1 = 1,
}
impl From<WPDIS_A> for bool {
    #[inline(always)]
    fn from(variant: WPDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl WPDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPDIS_A {
        match self.bits {
            false => WPDIS_A::_0,
            true => WPDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPDIS_A::_1
    }
}
#[doc = "Field `WPDIS` writer - Write Protection Disable"]
pub type WPDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, WPDIS_A, O>;
impl<'a, const O: u8> WPDIS_W<'a, O> {
    #[doc = "Write protection is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPDIS_A::_0)
    }
    #[doc = "Write protection is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPDIS_A::_1)
    }
}
#[doc = "Field `PWMSYNC` reader - PWM Synchronization Mode"]
pub type PWMSYNC_R = crate::BitReader<PWMSYNC_A>;
#[doc = "PWM Synchronization Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PWMSYNC_A {
    #[doc = "0: No restrictions. Software and hardware triggers can be used by MOD, CnV, OUTMASK, and FTM counter synchronization."]
    _0 = 0,
    #[doc = "1: Software trigger can only be used by MOD and CnV synchronization, and hardware triggers can only be used by OUTMASK and FTM counter synchronization."]
    _1 = 1,
}
impl From<PWMSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: PWMSYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl PWMSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWMSYNC_A {
        match self.bits {
            false => PWMSYNC_A::_0,
            true => PWMSYNC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PWMSYNC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PWMSYNC_A::_1
    }
}
#[doc = "Field `PWMSYNC` writer - PWM Synchronization Mode"]
pub type PWMSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, PWMSYNC_A, O>;
impl<'a, const O: u8> PWMSYNC_W<'a, O> {
    #[doc = "No restrictions. Software and hardware triggers can be used by MOD, CnV, OUTMASK, and FTM counter synchronization."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PWMSYNC_A::_0)
    }
    #[doc = "Software trigger can only be used by MOD and CnV synchronization, and hardware triggers can only be used by OUTMASK and FTM counter synchronization."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PWMSYNC_A::_1)
    }
}
#[doc = "Field `CAPTEST` reader - Capture Test Mode Enable"]
pub type CAPTEST_R = crate::BitReader<CAPTEST_A>;
#[doc = "Capture Test Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPTEST_A {
    #[doc = "0: Capture test mode is disabled."]
    _0 = 0,
    #[doc = "1: Capture test mode is enabled."]
    _1 = 1,
}
impl From<CAPTEST_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTEST_A) -> Self {
        variant as u8 != 0
    }
}
impl CAPTEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTEST_A {
        match self.bits {
            false => CAPTEST_A::_0,
            true => CAPTEST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CAPTEST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CAPTEST_A::_1
    }
}
#[doc = "Field `CAPTEST` writer - Capture Test Mode Enable"]
pub type CAPTEST_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, CAPTEST_A, O>;
impl<'a, const O: u8> CAPTEST_W<'a, O> {
    #[doc = "Capture test mode is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAPTEST_A::_0)
    }
    #[doc = "Capture test mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAPTEST_A::_1)
    }
}
#[doc = "Field `FAULTM` reader - Fault Control Mode"]
pub type FAULTM_R = crate::FieldReader<u8, FAULTM_A>;
#[doc = "Fault Control Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FAULTM_A {
    #[doc = "0: Fault control is disabled for all channels."]
    _00 = 0,
    #[doc = "1: Fault control is enabled for even channels only (channels 0, 2, 4, and 6), and the selected mode is the manual fault clearing."]
    _01 = 1,
    #[doc = "2: Fault control is enabled for all channels, and the selected mode is the manual fault clearing."]
    _10 = 2,
    #[doc = "3: Fault control is enabled for all channels, and the selected mode is the automatic fault clearing."]
    _11 = 3,
}
impl From<FAULTM_A> for u8 {
    #[inline(always)]
    fn from(variant: FAULTM_A) -> Self {
        variant as _
    }
}
impl FAULTM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTM_A {
        match self.bits {
            0 => FAULTM_A::_00,
            1 => FAULTM_A::_01,
            2 => FAULTM_A::_10,
            3 => FAULTM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == FAULTM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == FAULTM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == FAULTM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == FAULTM_A::_11
    }
}
#[doc = "Field `FAULTM` writer - Fault Control Mode"]
pub type FAULTM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MODE_SPEC, u8, FAULTM_A, 2, O>;
impl<'a, const O: u8> FAULTM_W<'a, O> {
    #[doc = "Fault control is disabled for all channels."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(FAULTM_A::_00)
    }
    #[doc = "Fault control is enabled for even channels only (channels 0, 2, 4, and 6), and the selected mode is the manual fault clearing."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(FAULTM_A::_01)
    }
    #[doc = "Fault control is enabled for all channels, and the selected mode is the manual fault clearing."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(FAULTM_A::_10)
    }
    #[doc = "Fault control is enabled for all channels, and the selected mode is the automatic fault clearing."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(FAULTM_A::_11)
    }
}
#[doc = "Field `FAULTIE` reader - Fault Interrupt Enable"]
pub type FAULTIE_R = crate::BitReader<FAULTIE_A>;
#[doc = "Fault Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAULTIE_A {
    #[doc = "0: Fault control interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Fault control interrupt is enabled."]
    _1 = 1,
}
impl From<FAULTIE_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl FAULTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTIE_A {
        match self.bits {
            false => FAULTIE_A::_0,
            true => FAULTIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTIE_A::_1
    }
}
#[doc = "Field `FAULTIE` writer - Fault Interrupt Enable"]
pub type FAULTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MODE_SPEC, FAULTIE_A, O>;
impl<'a, const O: u8> FAULTIE_W<'a, O> {
    #[doc = "Fault control interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTIE_A::_0)
    }
    #[doc = "Fault control interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTIE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - FTM Enable"]
    #[inline(always)]
    pub fn ftmen(&self) -> FTMEN_R {
        FTMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Initialize The Channels Output"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write Protection Disable"]
    #[inline(always)]
    pub fn wpdis(&self) -> WPDIS_R {
        WPDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM Synchronization Mode"]
    #[inline(always)]
    pub fn pwmsync(&self) -> PWMSYNC_R {
        PWMSYNC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture Test Mode Enable"]
    #[inline(always)]
    pub fn captest(&self) -> CAPTEST_R {
        CAPTEST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Fault Control Mode"]
    #[inline(always)]
    pub fn faultm(&self) -> FAULTM_R {
        FAULTM_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Fault Interrupt Enable"]
    #[inline(always)]
    pub fn faultie(&self) -> FAULTIE_R {
        FAULTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FTM Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ftmen(&mut self) -> FTMEN_W<0> {
        FTMEN_W::new(self)
    }
    #[doc = "Bit 1 - Initialize The Channels Output"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<1> {
        INIT_W::new(self)
    }
    #[doc = "Bit 2 - Write Protection Disable"]
    #[inline(always)]
    #[must_use]
    pub fn wpdis(&mut self) -> WPDIS_W<2> {
        WPDIS_W::new(self)
    }
    #[doc = "Bit 3 - PWM Synchronization Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pwmsync(&mut self) -> PWMSYNC_W<3> {
        PWMSYNC_W::new(self)
    }
    #[doc = "Bit 4 - Capture Test Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn captest(&mut self) -> CAPTEST_W<4> {
        CAPTEST_W::new(self)
    }
    #[doc = "Bits 5:6 - Fault Control Mode"]
    #[inline(always)]
    #[must_use]
    pub fn faultm(&mut self) -> FAULTM_W<5> {
        FAULTM_W::new(self)
    }
    #[doc = "Bit 7 - Fault Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn faultie(&mut self) -> FAULTIE_W<7> {
        FAULTIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Features Mode Selection\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODE to value 0x04"]
impl crate::Resettable for MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
