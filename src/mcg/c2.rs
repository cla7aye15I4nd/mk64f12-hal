#[doc = "Register `C2` reader"]
pub struct R(crate::R<C2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2` writer"]
pub struct W(crate::W<C2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2_SPEC>;
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
impl From<crate::W<C2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRCS` reader - Internal Reference Clock Select"]
pub type IRCS_R = crate::BitReader<IRCS_A>;
#[doc = "Internal Reference Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRCS_A {
    #[doc = "0: Slow internal reference clock selected."]
    _0 = 0,
    #[doc = "1: Fast internal reference clock selected."]
    _1 = 1,
}
impl From<IRCS_A> for bool {
    #[inline(always)]
    fn from(variant: IRCS_A) -> Self {
        variant as u8 != 0
    }
}
impl IRCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRCS_A {
        match self.bits {
            false => IRCS_A::_0,
            true => IRCS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRCS_A::_1
    }
}
#[doc = "Field `IRCS` writer - Internal Reference Clock Select"]
pub type IRCS_W<'a, const O: u8> = crate::BitWriter<'a, u8, C2_SPEC, IRCS_A, O>;
impl<'a, const O: u8> IRCS_W<'a, O> {
    #[doc = "Slow internal reference clock selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRCS_A::_0)
    }
    #[doc = "Fast internal reference clock selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRCS_A::_1)
    }
}
#[doc = "Field `LP` reader - Low Power Select"]
pub type LP_R = crate::BitReader<LP_A>;
#[doc = "Low Power Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LP_A {
    #[doc = "0: FLL or PLL is not disabled in bypass modes."]
    _0 = 0,
    #[doc = "1: FLL or PLL is disabled in bypass modes (lower power)"]
    _1 = 1,
}
impl From<LP_A> for bool {
    #[inline(always)]
    fn from(variant: LP_A) -> Self {
        variant as u8 != 0
    }
}
impl LP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LP_A {
        match self.bits {
            false => LP_A::_0,
            true => LP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LP_A::_1
    }
}
#[doc = "Field `LP` writer - Low Power Select"]
pub type LP_W<'a, const O: u8> = crate::BitWriter<'a, u8, C2_SPEC, LP_A, O>;
impl<'a, const O: u8> LP_W<'a, O> {
    #[doc = "FLL or PLL is not disabled in bypass modes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LP_A::_0)
    }
    #[doc = "FLL or PLL is disabled in bypass modes (lower power)"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LP_A::_1)
    }
}
#[doc = "Field `EREFS` reader - External Reference Select"]
pub type EREFS_R = crate::BitReader<EREFS_A>;
#[doc = "External Reference Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EREFS_A {
    #[doc = "0: External reference clock requested."]
    _0 = 0,
    #[doc = "1: Oscillator requested."]
    _1 = 1,
}
impl From<EREFS_A> for bool {
    #[inline(always)]
    fn from(variant: EREFS_A) -> Self {
        variant as u8 != 0
    }
}
impl EREFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EREFS_A {
        match self.bits {
            false => EREFS_A::_0,
            true => EREFS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EREFS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EREFS_A::_1
    }
}
#[doc = "Field `EREFS` writer - External Reference Select"]
pub type EREFS_W<'a, const O: u8> = crate::BitWriter<'a, u8, C2_SPEC, EREFS_A, O>;
impl<'a, const O: u8> EREFS_W<'a, O> {
    #[doc = "External reference clock requested."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EREFS_A::_0)
    }
    #[doc = "Oscillator requested."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EREFS_A::_1)
    }
}
#[doc = "Field `HGO` reader - High Gain Oscillator Select"]
pub type HGO_R = crate::BitReader<HGO_A>;
#[doc = "High Gain Oscillator Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HGO_A {
    #[doc = "0: Configure crystal oscillator for low-power operation."]
    _0 = 0,
    #[doc = "1: Configure crystal oscillator for high-gain operation."]
    _1 = 1,
}
impl From<HGO_A> for bool {
    #[inline(always)]
    fn from(variant: HGO_A) -> Self {
        variant as u8 != 0
    }
}
impl HGO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HGO_A {
        match self.bits {
            false => HGO_A::_0,
            true => HGO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HGO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HGO_A::_1
    }
}
#[doc = "Field `HGO` writer - High Gain Oscillator Select"]
pub type HGO_W<'a, const O: u8> = crate::BitWriter<'a, u8, C2_SPEC, HGO_A, O>;
impl<'a, const O: u8> HGO_W<'a, O> {
    #[doc = "Configure crystal oscillator for low-power operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HGO_A::_0)
    }
    #[doc = "Configure crystal oscillator for high-gain operation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HGO_A::_1)
    }
}
#[doc = "Field `RANGE` reader - Frequency Range Select"]
pub type RANGE_R = crate::FieldReader<u8, RANGE_A>;
#[doc = "Frequency Range Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RANGE_A {
    #[doc = "0: Encoding 0 - Low frequency range selected for the crystal oscillator ."]
    _00 = 0,
    #[doc = "1: Encoding 1 - High frequency range selected for the crystal oscillator ."]
    _01 = 1,
}
impl From<RANGE_A> for u8 {
    #[inline(always)]
    fn from(variant: RANGE_A) -> Self {
        variant as _
    }
}
impl RANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RANGE_A> {
        match self.bits {
            0 => Some(RANGE_A::_00),
            1 => Some(RANGE_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RANGE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RANGE_A::_01
    }
}
#[doc = "Field `RANGE` writer - Frequency Range Select"]
pub type RANGE_W<'a, const O: u8> = crate::FieldWriter<'a, u8, C2_SPEC, u8, RANGE_A, 2, O>;
impl<'a, const O: u8> RANGE_W<'a, O> {
    #[doc = "Encoding 0 - Low frequency range selected for the crystal oscillator ."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RANGE_A::_00)
    }
    #[doc = "Encoding 1 - High frequency range selected for the crystal oscillator ."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RANGE_A::_01)
    }
}
#[doc = "Field `FCFTRIM` reader - Fast Internal Reference Clock Fine Trim"]
pub type FCFTRIM_R = crate::BitReader<bool>;
#[doc = "Field `FCFTRIM` writer - Fast Internal Reference Clock Fine Trim"]
pub type FCFTRIM_W<'a, const O: u8> = crate::BitWriter<'a, u8, C2_SPEC, bool, O>;
#[doc = "Field `LOCRE0` reader - Loss of Clock Reset Enable"]
pub type LOCRE0_R = crate::BitReader<LOCRE0_A>;
#[doc = "Loss of Clock Reset Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCRE0_A {
    #[doc = "0: Interrupt request is generated on a loss of OSC0 external reference clock."]
    _0 = 0,
    #[doc = "1: Generate a reset request on a loss of OSC0 external reference clock."]
    _1 = 1,
}
impl From<LOCRE0_A> for bool {
    #[inline(always)]
    fn from(variant: LOCRE0_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCRE0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCRE0_A {
        match self.bits {
            false => LOCRE0_A::_0,
            true => LOCRE0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOCRE0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOCRE0_A::_1
    }
}
#[doc = "Field `LOCRE0` writer - Loss of Clock Reset Enable"]
pub type LOCRE0_W<'a, const O: u8> = crate::BitWriter<'a, u8, C2_SPEC, LOCRE0_A, O>;
impl<'a, const O: u8> LOCRE0_W<'a, O> {
    #[doc = "Interrupt request is generated on a loss of OSC0 external reference clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCRE0_A::_0)
    }
    #[doc = "Generate a reset request on a loss of OSC0 external reference clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCRE0_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Internal Reference Clock Select"]
    #[inline(always)]
    pub fn ircs(&self) -> IRCS_R {
        IRCS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Power Select"]
    #[inline(always)]
    pub fn lp(&self) -> LP_R {
        LP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Reference Select"]
    #[inline(always)]
    pub fn erefs(&self) -> EREFS_R {
        EREFS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - High Gain Oscillator Select"]
    #[inline(always)]
    pub fn hgo(&self) -> HGO_R {
        HGO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Frequency Range Select"]
    #[inline(always)]
    pub fn range(&self) -> RANGE_R {
        RANGE_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Fast Internal Reference Clock Fine Trim"]
    #[inline(always)]
    pub fn fcftrim(&self) -> FCFTRIM_R {
        FCFTRIM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline(always)]
    pub fn locre0(&self) -> LOCRE0_R {
        LOCRE0_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Reference Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn ircs(&mut self) -> IRCS_W<0> {
        IRCS_W::new(self)
    }
    #[doc = "Bit 1 - Low Power Select"]
    #[inline(always)]
    #[must_use]
    pub fn lp(&mut self) -> LP_W<1> {
        LP_W::new(self)
    }
    #[doc = "Bit 2 - External Reference Select"]
    #[inline(always)]
    #[must_use]
    pub fn erefs(&mut self) -> EREFS_W<2> {
        EREFS_W::new(self)
    }
    #[doc = "Bit 3 - High Gain Oscillator Select"]
    #[inline(always)]
    #[must_use]
    pub fn hgo(&mut self) -> HGO_W<3> {
        HGO_W::new(self)
    }
    #[doc = "Bits 4:5 - Frequency Range Select"]
    #[inline(always)]
    #[must_use]
    pub fn range(&mut self) -> RANGE_W<4> {
        RANGE_W::new(self)
    }
    #[doc = "Bit 6 - Fast Internal Reference Clock Fine Trim"]
    #[inline(always)]
    #[must_use]
    pub fn fcftrim(&mut self) -> FCFTRIM_W<6> {
        FCFTRIM_W::new(self)
    }
    #[doc = "Bit 7 - Loss of Clock Reset Enable"]
    #[inline(always)]
    #[must_use]
    pub fn locre0(&mut self) -> LOCRE0_W<7> {
        LOCRE0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCG Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2](index.html) module"]
pub struct C2_SPEC;
impl crate::RegisterSpec for C2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c2::R](R) reader structure"]
impl crate::Readable for C2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2::W](W) writer structure"]
impl crate::Writable for C2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2 to value 0x80"]
impl crate::Resettable for C2_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
