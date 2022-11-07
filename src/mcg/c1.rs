#[doc = "Register `C1` reader"]
pub struct R(crate::R<C1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1` writer"]
pub struct W(crate::W<C1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_SPEC>;
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
impl From<crate::W<C1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IREFSTEN` reader - Internal Reference Stop Enable"]
pub type IREFSTEN_R = crate::BitReader<IREFSTEN_A>;
#[doc = "Internal Reference Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREFSTEN_A {
    #[doc = "0: Internal reference clock is disabled in Stop mode."]
    _0 = 0,
    #[doc = "1: Internal reference clock is enabled in Stop mode if IRCLKEN is set or if MCG is in FEI, FBI, or BLPI modes before entering Stop mode."]
    _1 = 1,
}
impl From<IREFSTEN_A> for bool {
    #[inline(always)]
    fn from(variant: IREFSTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IREFSTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREFSTEN_A {
        match self.bits {
            false => IREFSTEN_A::_0,
            true => IREFSTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IREFSTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IREFSTEN_A::_1
    }
}
#[doc = "Field `IREFSTEN` writer - Internal Reference Stop Enable"]
pub type IREFSTEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, IREFSTEN_A, O>;
impl<'a, const O: u8> IREFSTEN_W<'a, O> {
    #[doc = "Internal reference clock is disabled in Stop mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IREFSTEN_A::_0)
    }
    #[doc = "Internal reference clock is enabled in Stop mode if IRCLKEN is set or if MCG is in FEI, FBI, or BLPI modes before entering Stop mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IREFSTEN_A::_1)
    }
}
#[doc = "Field `IRCLKEN` reader - Internal Reference Clock Enable"]
pub type IRCLKEN_R = crate::BitReader<IRCLKEN_A>;
#[doc = "Internal Reference Clock Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRCLKEN_A {
    #[doc = "0: MCGIRCLK inactive."]
    _0 = 0,
    #[doc = "1: MCGIRCLK active."]
    _1 = 1,
}
impl From<IRCLKEN_A> for bool {
    #[inline(always)]
    fn from(variant: IRCLKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IRCLKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRCLKEN_A {
        match self.bits {
            false => IRCLKEN_A::_0,
            true => IRCLKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRCLKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRCLKEN_A::_1
    }
}
#[doc = "Field `IRCLKEN` writer - Internal Reference Clock Enable"]
pub type IRCLKEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, IRCLKEN_A, O>;
impl<'a, const O: u8> IRCLKEN_W<'a, O> {
    #[doc = "MCGIRCLK inactive."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IRCLKEN_A::_0)
    }
    #[doc = "MCGIRCLK active."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IRCLKEN_A::_1)
    }
}
#[doc = "Field `IREFS` reader - Internal Reference Select"]
pub type IREFS_R = crate::BitReader<IREFS_A>;
#[doc = "Internal Reference Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREFS_A {
    #[doc = "0: External reference clock is selected."]
    _0 = 0,
    #[doc = "1: The slow internal reference clock is selected."]
    _1 = 1,
}
impl From<IREFS_A> for bool {
    #[inline(always)]
    fn from(variant: IREFS_A) -> Self {
        variant as u8 != 0
    }
}
impl IREFS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREFS_A {
        match self.bits {
            false => IREFS_A::_0,
            true => IREFS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IREFS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IREFS_A::_1
    }
}
#[doc = "Field `IREFS` writer - Internal Reference Select"]
pub type IREFS_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, IREFS_A, O>;
impl<'a, const O: u8> IREFS_W<'a, O> {
    #[doc = "External reference clock is selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IREFS_A::_0)
    }
    #[doc = "The slow internal reference clock is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IREFS_A::_1)
    }
}
#[doc = "Field `FRDIV` reader - FLL External Reference Divider"]
pub type FRDIV_R = crate::FieldReader<u8, FRDIV_A>;
#[doc = "FLL External Reference Divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FRDIV_A {
    #[doc = "0: If RANGE = 0 or OSCSEL=1 , Divide Factor is 1; for all other RANGE values, Divide Factor is 32."]
    _000 = 0,
    #[doc = "1: If RANGE = 0 or OSCSEL=1 , Divide Factor is 2; for all other RANGE values, Divide Factor is 64."]
    _001 = 1,
    #[doc = "2: If RANGE = 0 or OSCSEL=1 , Divide Factor is 4; for all other RANGE values, Divide Factor is 128."]
    _010 = 2,
    #[doc = "3: If RANGE = 0 or OSCSEL=1 , Divide Factor is 8; for all other RANGE values, Divide Factor is 256."]
    _011 = 3,
    #[doc = "4: If RANGE = 0 or OSCSEL=1 , Divide Factor is 16; for all other RANGE values, Divide Factor is 512."]
    _100 = 4,
    #[doc = "5: If RANGE = 0 or OSCSEL=1 , Divide Factor is 32; for all other RANGE values, Divide Factor is 1024."]
    _101 = 5,
    #[doc = "6: If RANGE = 0 or OSCSEL=1 , Divide Factor is 64; for all other RANGE values, Divide Factor is 1280 ."]
    _110 = 6,
    #[doc = "7: If RANGE = 0 or OSCSEL=1 , Divide Factor is 128; for all other RANGE values, Divide Factor is 1536 ."]
    _111 = 7,
}
impl From<FRDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: FRDIV_A) -> Self {
        variant as _
    }
}
impl FRDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRDIV_A {
        match self.bits {
            0 => FRDIV_A::_000,
            1 => FRDIV_A::_001,
            2 => FRDIV_A::_010,
            3 => FRDIV_A::_011,
            4 => FRDIV_A::_100,
            5 => FRDIV_A::_101,
            6 => FRDIV_A::_110,
            7 => FRDIV_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FRDIV_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FRDIV_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == FRDIV_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == FRDIV_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == FRDIV_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == FRDIV_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == FRDIV_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == FRDIV_A::_111
    }
}
#[doc = "Field `FRDIV` writer - FLL External Reference Divider"]
pub type FRDIV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, C1_SPEC, u8, FRDIV_A, 3, O>;
impl<'a, const O: u8> FRDIV_W<'a, O> {
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 1; for all other RANGE values, Divide Factor is 32."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FRDIV_A::_000)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 2; for all other RANGE values, Divide Factor is 64."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FRDIV_A::_001)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 4; for all other RANGE values, Divide Factor is 128."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(FRDIV_A::_010)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 8; for all other RANGE values, Divide Factor is 256."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(FRDIV_A::_011)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 16; for all other RANGE values, Divide Factor is 512."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(FRDIV_A::_100)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 32; for all other RANGE values, Divide Factor is 1024."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(FRDIV_A::_101)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 64; for all other RANGE values, Divide Factor is 1280 ."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(FRDIV_A::_110)
    }
    #[doc = "If RANGE = 0 or OSCSEL=1 , Divide Factor is 128; for all other RANGE values, Divide Factor is 1536 ."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(FRDIV_A::_111)
    }
}
#[doc = "Field `CLKS` reader - Clock Source Select"]
pub type CLKS_R = crate::FieldReader<u8, CLKS_A>;
#[doc = "Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKS_A {
    #[doc = "0: Encoding 0 - Output of FLL or PLL is selected (depends on PLLS control bit)."]
    _00 = 0,
    #[doc = "1: Encoding 1 - Internal reference clock is selected."]
    _01 = 1,
    #[doc = "2: Encoding 2 - External reference clock is selected."]
    _10 = 2,
    #[doc = "3: Encoding 3 - Reserved."]
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
#[doc = "Field `CLKS` writer - Clock Source Select"]
pub type CLKS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, C1_SPEC, u8, CLKS_A, 2, O>;
impl<'a, const O: u8> CLKS_W<'a, O> {
    #[doc = "Encoding 0 - Output of FLL or PLL is selected (depends on PLLS control bit)."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CLKS_A::_00)
    }
    #[doc = "Encoding 1 - Internal reference clock is selected."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CLKS_A::_01)
    }
    #[doc = "Encoding 2 - External reference clock is selected."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CLKS_A::_10)
    }
    #[doc = "Encoding 3 - Reserved."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CLKS_A::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Internal Reference Stop Enable"]
    #[inline(always)]
    pub fn irefsten(&self) -> IREFSTEN_R {
        IREFSTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Reference Clock Enable"]
    #[inline(always)]
    pub fn irclken(&self) -> IRCLKEN_R {
        IRCLKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Internal Reference Select"]
    #[inline(always)]
    pub fn irefs(&self) -> IREFS_R {
        IREFS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - FLL External Reference Divider"]
    #[inline(always)]
    pub fn frdiv(&self) -> FRDIV_R {
        FRDIV_R::new((self.bits >> 3) & 7)
    }
    #[doc = "Bits 6:7 - Clock Source Select"]
    #[inline(always)]
    pub fn clks(&self) -> CLKS_R {
        CLKS_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Reference Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irefsten(&mut self) -> IREFSTEN_W<0> {
        IREFSTEN_W::new(self)
    }
    #[doc = "Bit 1 - Internal Reference Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn irclken(&mut self) -> IRCLKEN_W<1> {
        IRCLKEN_W::new(self)
    }
    #[doc = "Bit 2 - Internal Reference Select"]
    #[inline(always)]
    #[must_use]
    pub fn irefs(&mut self) -> IREFS_W<2> {
        IREFS_W::new(self)
    }
    #[doc = "Bits 3:5 - FLL External Reference Divider"]
    #[inline(always)]
    #[must_use]
    pub fn frdiv(&mut self) -> FRDIV_W<3> {
        FRDIV_W::new(self)
    }
    #[doc = "Bits 6:7 - Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn clks(&mut self) -> CLKS_W<6> {
        CLKS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCG Control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1](index.html) module"]
pub struct C1_SPEC;
impl crate::RegisterSpec for C1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c1::R](R) reader structure"]
impl crate::Readable for C1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1::W](W) writer structure"]
impl crate::Writable for C1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1 to value 0x04"]
impl crate::Resettable for C1_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
