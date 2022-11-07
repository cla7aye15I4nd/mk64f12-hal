#[doc = "Register `SC` reader"]
pub struct R(crate::R<SC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC` writer"]
pub struct W(crate::W<SC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC_SPEC>;
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
impl From<crate::W<SC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PS` reader - Prescale Factor Selection"]
pub type PS_R = crate::FieldReader<u8, PS_A>;
#[doc = "Prescale Factor Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PS_A {
    #[doc = "0: Divide by 1"]
    _000 = 0,
    #[doc = "1: Divide by 2"]
    _001 = 1,
    #[doc = "2: Divide by 4"]
    _010 = 2,
    #[doc = "3: Divide by 8"]
    _011 = 3,
    #[doc = "4: Divide by 16"]
    _100 = 4,
    #[doc = "5: Divide by 32"]
    _101 = 5,
    #[doc = "6: Divide by 64"]
    _110 = 6,
    #[doc = "7: Divide by 128"]
    _111 = 7,
}
impl From<PS_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as _
    }
}
impl PS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            0 => PS_A::_000,
            1 => PS_A::_001,
            2 => PS_A::_010,
            3 => PS_A::_011,
            4 => PS_A::_100,
            5 => PS_A::_101,
            6 => PS_A::_110,
            7 => PS_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PS_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PS_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PS_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PS_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PS_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PS_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == PS_A::_111
    }
}
#[doc = "Field `PS` writer - Prescale Factor Selection"]
pub type PS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SC_SPEC, u8, PS_A, 3, O>;
impl<'a, const O: u8> PS_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PS_A::_000)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PS_A::_001)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PS_A::_010)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PS_A::_011)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PS_A::_100)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PS_A::_101)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PS_A::_110)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PS_A::_111)
    }
}
#[doc = "Field `CLKS` reader - Clock Source Selection"]
pub type CLKS_R = crate::FieldReader<u8, CLKS_A>;
#[doc = "Clock Source Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKS_A {
    #[doc = "0: No clock selected. This in effect disables the FTM counter."]
    _00 = 0,
    #[doc = "1: System clock"]
    _01 = 1,
    #[doc = "2: Fixed frequency clock"]
    _10 = 2,
    #[doc = "3: External clock"]
    _11 = 3,
}
impl From<CLKS_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKS_A) -> Self {
        variant as _
    }
}
impl CLKS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKS_A {
        match self.bits {
            0 => CLKS_A::_00,
            1 => CLKS_A::_01,
            2 => CLKS_A::_10,
            3 => CLKS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CLKS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CLKS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CLKS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CLKS_A::_11
    }
}
#[doc = "Field `CLKS` writer - Clock Source Selection"]
pub type CLKS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, SC_SPEC, u8, CLKS_A, 2, O>;
impl<'a, const O: u8> CLKS_W<'a, O> {
    #[doc = "No clock selected. This in effect disables the FTM counter."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CLKS_A::_00)
    }
    #[doc = "System clock"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CLKS_A::_01)
    }
    #[doc = "Fixed frequency clock"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CLKS_A::_10)
    }
    #[doc = "External clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CLKS_A::_11)
    }
}
#[doc = "Field `CPWMS` reader - Center-Aligned PWM Select"]
pub type CPWMS_R = crate::BitReader<CPWMS_A>;
#[doc = "Center-Aligned PWM Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPWMS_A {
    #[doc = "0: FTM counter operates in Up Counting mode."]
    _0 = 0,
    #[doc = "1: FTM counter operates in Up-Down Counting mode."]
    _1 = 1,
}
impl From<CPWMS_A> for bool {
    #[inline(always)]
    fn from(variant: CPWMS_A) -> Self {
        variant as u8 != 0
    }
}
impl CPWMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPWMS_A {
        match self.bits {
            false => CPWMS_A::_0,
            true => CPWMS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPWMS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPWMS_A::_1
    }
}
#[doc = "Field `CPWMS` writer - Center-Aligned PWM Select"]
pub type CPWMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC_SPEC, CPWMS_A, O>;
impl<'a, const O: u8> CPWMS_W<'a, O> {
    #[doc = "FTM counter operates in Up Counting mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPWMS_A::_0)
    }
    #[doc = "FTM counter operates in Up-Down Counting mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPWMS_A::_1)
    }
}
#[doc = "Field `TOIE` reader - Timer Overflow Interrupt Enable"]
pub type TOIE_R = crate::BitReader<TOIE_A>;
#[doc = "Timer Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOIE_A {
    #[doc = "0: Disable TOF interrupts. Use software polling."]
    _0 = 0,
    #[doc = "1: Enable TOF interrupts. An interrupt is generated when TOF equals one."]
    _1 = 1,
}
impl From<TOIE_A> for bool {
    #[inline(always)]
    fn from(variant: TOIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOIE_A {
        match self.bits {
            false => TOIE_A::_0,
            true => TOIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOIE_A::_1
    }
}
#[doc = "Field `TOIE` writer - Timer Overflow Interrupt Enable"]
pub type TOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC_SPEC, TOIE_A, O>;
impl<'a, const O: u8> TOIE_W<'a, O> {
    #[doc = "Disable TOF interrupts. Use software polling."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOIE_A::_0)
    }
    #[doc = "Enable TOF interrupts. An interrupt is generated when TOF equals one."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOIE_A::_1)
    }
}
#[doc = "Field `TOF` reader - Timer Overflow Flag"]
pub type TOF_R = crate::BitReader<TOF_A>;
#[doc = "Timer Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOF_A {
    #[doc = "0: FTM counter has not overflowed."]
    _0 = 0,
    #[doc = "1: FTM counter has overflowed."]
    _1 = 1,
}
impl From<TOF_A> for bool {
    #[inline(always)]
    fn from(variant: TOF_A) -> Self {
        variant as u8 != 0
    }
}
impl TOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOF_A {
        match self.bits {
            false => TOF_A::_0,
            true => TOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOF_A::_1
    }
}
#[doc = "Field `TOF` writer - Timer Overflow Flag"]
pub type TOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC_SPEC, TOF_A, O>;
impl<'a, const O: u8> TOF_W<'a, O> {
    #[doc = "FTM counter has not overflowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOF_A::_0)
    }
    #[doc = "FTM counter has overflowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOF_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Prescale Factor Selection"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Clock Source Selection"]
    #[inline(always)]
    pub fn clks(&self) -> CLKS_R {
        CLKS_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Center-Aligned PWM Select"]
    #[inline(always)]
    pub fn cpwms(&self) -> CPWMS_R {
        CPWMS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer Overflow Flag"]
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescale Factor Selection"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<0> {
        PS_W::new(self)
    }
    #[doc = "Bits 3:4 - Clock Source Selection"]
    #[inline(always)]
    #[must_use]
    pub fn clks(&mut self) -> CLKS_W<3> {
        CLKS_W::new(self)
    }
    #[doc = "Bit 5 - Center-Aligned PWM Select"]
    #[inline(always)]
    #[must_use]
    pub fn cpwms(&mut self) -> CPWMS_W<5> {
        CPWMS_W::new(self)
    }
    #[doc = "Bit 6 - Timer Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toie(&mut self) -> TOIE_W<6> {
        TOIE_W::new(self)
    }
    #[doc = "Bit 7 - Timer Overflow Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tof(&mut self) -> TOF_W<7> {
        TOF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status And Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc](index.html) module"]
pub struct SC_SPEC;
impl crate::RegisterSpec for SC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc::R](R) reader structure"]
impl crate::Readable for SC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc::W](W) writer structure"]
impl crate::Writable for SC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SC to value 0"]
impl crate::Resettable for SC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
