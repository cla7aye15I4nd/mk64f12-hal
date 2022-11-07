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
#[doc = "Field `LOCS0` reader - OSC0 Loss of Clock Status"]
pub type LOCS0_R = crate::BitReader<LOCS0_A>;
#[doc = "OSC0 Loss of Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOCS0_A {
    #[doc = "0: Loss of OSC0 has not occurred."]
    _0 = 0,
    #[doc = "1: Loss of OSC0 has occurred."]
    _1 = 1,
}
impl From<LOCS0_A> for bool {
    #[inline(always)]
    fn from(variant: LOCS0_A) -> Self {
        variant as u8 != 0
    }
}
impl LOCS0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOCS0_A {
        match self.bits {
            false => LOCS0_A::_0,
            true => LOCS0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOCS0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOCS0_A::_1
    }
}
#[doc = "Field `LOCS0` writer - OSC0 Loss of Clock Status"]
pub type LOCS0_W<'a, const O: u8> = crate::BitWriter<'a, u8, SC_SPEC, LOCS0_A, O>;
impl<'a, const O: u8> LOCS0_W<'a, O> {
    #[doc = "Loss of OSC0 has not occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOCS0_A::_0)
    }
    #[doc = "Loss of OSC0 has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOCS0_A::_1)
    }
}
#[doc = "Field `FCRDIV` reader - Fast Clock Internal Reference Divider"]
pub type FCRDIV_R = crate::FieldReader<u8, FCRDIV_A>;
#[doc = "Fast Clock Internal Reference Divider\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FCRDIV_A {
    #[doc = "0: Divide Factor is 1"]
    _000 = 0,
    #[doc = "1: Divide Factor is 2."]
    _001 = 1,
    #[doc = "2: Divide Factor is 4."]
    _010 = 2,
    #[doc = "3: Divide Factor is 8."]
    _011 = 3,
    #[doc = "4: Divide Factor is 16"]
    _100 = 4,
    #[doc = "5: Divide Factor is 32"]
    _101 = 5,
    #[doc = "6: Divide Factor is 64"]
    _110 = 6,
    #[doc = "7: Divide Factor is 128."]
    _111 = 7,
}
impl From<FCRDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: FCRDIV_A) -> Self {
        variant as _
    }
}
impl FCRDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCRDIV_A {
        match self.bits {
            0 => FCRDIV_A::_000,
            1 => FCRDIV_A::_001,
            2 => FCRDIV_A::_010,
            3 => FCRDIV_A::_011,
            4 => FCRDIV_A::_100,
            5 => FCRDIV_A::_101,
            6 => FCRDIV_A::_110,
            7 => FCRDIV_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == FCRDIV_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == FCRDIV_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == FCRDIV_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == FCRDIV_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == FCRDIV_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == FCRDIV_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == FCRDIV_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == FCRDIV_A::_111
    }
}
#[doc = "Field `FCRDIV` writer - Fast Clock Internal Reference Divider"]
pub type FCRDIV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, SC_SPEC, u8, FCRDIV_A, 3, O>;
impl<'a, const O: u8> FCRDIV_W<'a, O> {
    #[doc = "Divide Factor is 1"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(FCRDIV_A::_000)
    }
    #[doc = "Divide Factor is 2."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(FCRDIV_A::_001)
    }
    #[doc = "Divide Factor is 4."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(FCRDIV_A::_010)
    }
    #[doc = "Divide Factor is 8."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(FCRDIV_A::_011)
    }
    #[doc = "Divide Factor is 16"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(FCRDIV_A::_100)
    }
    #[doc = "Divide Factor is 32"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(FCRDIV_A::_101)
    }
    #[doc = "Divide Factor is 64"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(FCRDIV_A::_110)
    }
    #[doc = "Divide Factor is 128."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(FCRDIV_A::_111)
    }
}
#[doc = "Field `FLTPRSRV` reader - FLL Filter Preserve Enable"]
pub type FLTPRSRV_R = crate::BitReader<FLTPRSRV_A>;
#[doc = "FLL Filter Preserve Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLTPRSRV_A {
    #[doc = "0: FLL filter and FLL frequency will reset on changes to currect clock mode."]
    _0 = 0,
    #[doc = "1: Fll filter and FLL frequency retain their previous values during new clock mode change."]
    _1 = 1,
}
impl From<FLTPRSRV_A> for bool {
    #[inline(always)]
    fn from(variant: FLTPRSRV_A) -> Self {
        variant as u8 != 0
    }
}
impl FLTPRSRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLTPRSRV_A {
        match self.bits {
            false => FLTPRSRV_A::_0,
            true => FLTPRSRV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLTPRSRV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLTPRSRV_A::_1
    }
}
#[doc = "Field `FLTPRSRV` writer - FLL Filter Preserve Enable"]
pub type FLTPRSRV_W<'a, const O: u8> = crate::BitWriter<'a, u8, SC_SPEC, FLTPRSRV_A, O>;
impl<'a, const O: u8> FLTPRSRV_W<'a, O> {
    #[doc = "FLL filter and FLL frequency will reset on changes to currect clock mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FLTPRSRV_A::_0)
    }
    #[doc = "Fll filter and FLL frequency retain their previous values during new clock mode change."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLTPRSRV_A::_1)
    }
}
#[doc = "Field `ATMF` reader - Automatic Trim Machine Fail Flag"]
pub type ATMF_R = crate::BitReader<ATMF_A>;
#[doc = "Automatic Trim Machine Fail Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATMF_A {
    #[doc = "0: Automatic Trim Machine completed normally."]
    _0 = 0,
    #[doc = "1: Automatic Trim Machine failed."]
    _1 = 1,
}
impl From<ATMF_A> for bool {
    #[inline(always)]
    fn from(variant: ATMF_A) -> Self {
        variant as u8 != 0
    }
}
impl ATMF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATMF_A {
        match self.bits {
            false => ATMF_A::_0,
            true => ATMF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATMF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATMF_A::_1
    }
}
#[doc = "Field `ATMF` writer - Automatic Trim Machine Fail Flag"]
pub type ATMF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SC_SPEC, ATMF_A, O>;
impl<'a, const O: u8> ATMF_W<'a, O> {
    #[doc = "Automatic Trim Machine completed normally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATMF_A::_0)
    }
    #[doc = "Automatic Trim Machine failed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATMF_A::_1)
    }
}
#[doc = "Field `ATMS` reader - Automatic Trim Machine Select"]
pub type ATMS_R = crate::BitReader<ATMS_A>;
#[doc = "Automatic Trim Machine Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATMS_A {
    #[doc = "0: 32 kHz Internal Reference Clock selected."]
    _0 = 0,
    #[doc = "1: 4 MHz Internal Reference Clock selected."]
    _1 = 1,
}
impl From<ATMS_A> for bool {
    #[inline(always)]
    fn from(variant: ATMS_A) -> Self {
        variant as u8 != 0
    }
}
impl ATMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATMS_A {
        match self.bits {
            false => ATMS_A::_0,
            true => ATMS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATMS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATMS_A::_1
    }
}
#[doc = "Field `ATMS` writer - Automatic Trim Machine Select"]
pub type ATMS_W<'a, const O: u8> = crate::BitWriter<'a, u8, SC_SPEC, ATMS_A, O>;
impl<'a, const O: u8> ATMS_W<'a, O> {
    #[doc = "32 kHz Internal Reference Clock selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATMS_A::_0)
    }
    #[doc = "4 MHz Internal Reference Clock selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATMS_A::_1)
    }
}
#[doc = "Field `ATME` reader - Automatic Trim Machine Enable"]
pub type ATME_R = crate::BitReader<ATME_A>;
#[doc = "Automatic Trim Machine Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ATME_A {
    #[doc = "0: Auto Trim Machine disabled."]
    _0 = 0,
    #[doc = "1: Auto Trim Machine enabled."]
    _1 = 1,
}
impl From<ATME_A> for bool {
    #[inline(always)]
    fn from(variant: ATME_A) -> Self {
        variant as u8 != 0
    }
}
impl ATME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ATME_A {
        match self.bits {
            false => ATME_A::_0,
            true => ATME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ATME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ATME_A::_1
    }
}
#[doc = "Field `ATME` writer - Automatic Trim Machine Enable"]
pub type ATME_W<'a, const O: u8> = crate::BitWriter<'a, u8, SC_SPEC, ATME_A, O>;
impl<'a, const O: u8> ATME_W<'a, O> {
    #[doc = "Auto Trim Machine disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ATME_A::_0)
    }
    #[doc = "Auto Trim Machine enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ATME_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - OSC0 Loss of Clock Status"]
    #[inline(always)]
    pub fn locs0(&self) -> LOCS0_R {
        LOCS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Fast Clock Internal Reference Divider"]
    #[inline(always)]
    pub fn fcrdiv(&self) -> FCRDIV_R {
        FCRDIV_R::new((self.bits >> 1) & 7)
    }
    #[doc = "Bit 4 - FLL Filter Preserve Enable"]
    #[inline(always)]
    pub fn fltprsrv(&self) -> FLTPRSRV_R {
        FLTPRSRV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Automatic Trim Machine Fail Flag"]
    #[inline(always)]
    pub fn atmf(&self) -> ATMF_R {
        ATMF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Automatic Trim Machine Select"]
    #[inline(always)]
    pub fn atms(&self) -> ATMS_R {
        ATMS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Automatic Trim Machine Enable"]
    #[inline(always)]
    pub fn atme(&self) -> ATME_R {
        ATME_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OSC0 Loss of Clock Status"]
    #[inline(always)]
    #[must_use]
    pub fn locs0(&mut self) -> LOCS0_W<0> {
        LOCS0_W::new(self)
    }
    #[doc = "Bits 1:3 - Fast Clock Internal Reference Divider"]
    #[inline(always)]
    #[must_use]
    pub fn fcrdiv(&mut self) -> FCRDIV_W<1> {
        FCRDIV_W::new(self)
    }
    #[doc = "Bit 4 - FLL Filter Preserve Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fltprsrv(&mut self) -> FLTPRSRV_W<4> {
        FLTPRSRV_W::new(self)
    }
    #[doc = "Bit 5 - Automatic Trim Machine Fail Flag"]
    #[inline(always)]
    #[must_use]
    pub fn atmf(&mut self) -> ATMF_W<5> {
        ATMF_W::new(self)
    }
    #[doc = "Bit 6 - Automatic Trim Machine Select"]
    #[inline(always)]
    #[must_use]
    pub fn atms(&mut self) -> ATMS_W<6> {
        ATMS_W::new(self)
    }
    #[doc = "Bit 7 - Automatic Trim Machine Enable"]
    #[inline(always)]
    #[must_use]
    pub fn atme(&mut self) -> ATME_W<7> {
        ATME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MCG Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc](index.html) module"]
pub struct SC_SPEC;
impl crate::RegisterSpec for SC_SPEC {
    type Ux = u8;
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
#[doc = "`reset()` method sets SC to value 0x02"]
impl crate::Resettable for SC_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
