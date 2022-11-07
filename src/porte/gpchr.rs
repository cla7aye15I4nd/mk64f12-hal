#[doc = "Register `GPCHR` writer"]
pub struct W(crate::W<GPCHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPCHR_SPEC>;
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
impl From<crate::W<GPCHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPCHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPWD` writer - Global Pin Write Data"]
pub type GPWD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPCHR_SPEC, u16, u16, 16, O>;
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPWE0_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GPWE0_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPWE0` writer - Global Pin Write Enable"]
pub type GPWE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPCHR_SPEC, GPWE0_AW, O>;
impl<'a, const O: u8> GPWE0_W<'a, O> {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE0_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE0_AW::_1)
    }
}
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPWE1_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GPWE1_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPWE1` writer - Global Pin Write Enable"]
pub type GPWE1_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPCHR_SPEC, GPWE1_AW, O>;
impl<'a, const O: u8> GPWE1_W<'a, O> {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE1_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE1_AW::_1)
    }
}
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPWE2_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GPWE2_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPWE2` writer - Global Pin Write Enable"]
pub type GPWE2_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPCHR_SPEC, GPWE2_AW, O>;
impl<'a, const O: u8> GPWE2_W<'a, O> {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE2_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE2_AW::_1)
    }
}
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPWE3_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GPWE3_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPWE3` writer - Global Pin Write Enable"]
pub type GPWE3_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPCHR_SPEC, GPWE3_AW, O>;
impl<'a, const O: u8> GPWE3_W<'a, O> {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE3_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE3_AW::_1)
    }
}
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPWE4_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GPWE4_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPWE4` writer - Global Pin Write Enable"]
pub type GPWE4_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPCHR_SPEC, GPWE4_AW, O>;
impl<'a, const O: u8> GPWE4_W<'a, O> {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE4_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE4_AW::_1)
    }
}
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPWE5_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GPWE5_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPWE5` writer - Global Pin Write Enable"]
pub type GPWE5_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPCHR_SPEC, GPWE5_AW, O>;
impl<'a, const O: u8> GPWE5_W<'a, O> {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE5_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE5_AW::_1)
    }
}
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPWE6_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GPWE6_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPWE6` writer - Global Pin Write Enable"]
pub type GPWE6_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPCHR_SPEC, GPWE6_AW, O>;
impl<'a, const O: u8> GPWE6_W<'a, O> {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE6_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE6_AW::_1)
    }
}
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPWE7_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GPWE7_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPWE7` writer - Global Pin Write Enable"]
pub type GPWE7_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPCHR_SPEC, GPWE7_AW, O>;
impl<'a, const O: u8> GPWE7_W<'a, O> {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE7_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE7_AW::_1)
    }
}
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPWE8_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GPWE8_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPWE8` writer - Global Pin Write Enable"]
pub type GPWE8_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPCHR_SPEC, GPWE8_AW, O>;
impl<'a, const O: u8> GPWE8_W<'a, O> {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE8_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE8_AW::_1)
    }
}
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPWE9_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GPWE9_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPWE9` writer - Global Pin Write Enable"]
pub type GPWE9_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPCHR_SPEC, GPWE9_AW, O>;
impl<'a, const O: u8> GPWE9_W<'a, O> {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE9_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE9_AW::_1)
    }
}
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPWE10_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GPWE10_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPWE10` writer - Global Pin Write Enable"]
pub type GPWE10_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPCHR_SPEC, GPWE10_AW, O>;
impl<'a, const O: u8> GPWE10_W<'a, O> {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE10_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE10_AW::_1)
    }
}
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPWE11_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GPWE11_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPWE11` writer - Global Pin Write Enable"]
pub type GPWE11_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPCHR_SPEC, GPWE11_AW, O>;
impl<'a, const O: u8> GPWE11_W<'a, O> {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE11_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE11_AW::_1)
    }
}
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPWE12_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GPWE12_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPWE12` writer - Global Pin Write Enable"]
pub type GPWE12_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPCHR_SPEC, GPWE12_AW, O>;
impl<'a, const O: u8> GPWE12_W<'a, O> {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE12_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE12_AW::_1)
    }
}
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPWE13_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GPWE13_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPWE13` writer - Global Pin Write Enable"]
pub type GPWE13_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPCHR_SPEC, GPWE13_AW, O>;
impl<'a, const O: u8> GPWE13_W<'a, O> {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE13_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE13_AW::_1)
    }
}
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPWE14_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GPWE14_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPWE14` writer - Global Pin Write Enable"]
pub type GPWE14_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPCHR_SPEC, GPWE14_AW, O>;
impl<'a, const O: u8> GPWE14_W<'a, O> {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE14_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE14_AW::_1)
    }
}
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPWE15_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GPWE15_AW> for bool {
    #[inline(always)]
    fn from(variant: GPWE15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GPWE15` writer - Global Pin Write Enable"]
pub type GPWE15_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPCHR_SPEC, GPWE15_AW, O>;
impl<'a, const O: u8> GPWE15_W<'a, O> {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE15_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE15_AW::_1)
    }
}
impl W {
    #[doc = "Bits 0:15 - Global Pin Write Data"]
    #[inline(always)]
    #[must_use]
    pub fn gpwd(&mut self) -> GPWD_W<0> {
        GPWD_W::new(self)
    }
    #[doc = "Bit 16 - Global Pin Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpwe0(&mut self) -> GPWE0_W<16> {
        GPWE0_W::new(self)
    }
    #[doc = "Bit 17 - Global Pin Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpwe1(&mut self) -> GPWE1_W<17> {
        GPWE1_W::new(self)
    }
    #[doc = "Bit 18 - Global Pin Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpwe2(&mut self) -> GPWE2_W<18> {
        GPWE2_W::new(self)
    }
    #[doc = "Bit 19 - Global Pin Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpwe3(&mut self) -> GPWE3_W<19> {
        GPWE3_W::new(self)
    }
    #[doc = "Bit 20 - Global Pin Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpwe4(&mut self) -> GPWE4_W<20> {
        GPWE4_W::new(self)
    }
    #[doc = "Bit 21 - Global Pin Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpwe5(&mut self) -> GPWE5_W<21> {
        GPWE5_W::new(self)
    }
    #[doc = "Bit 22 - Global Pin Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpwe6(&mut self) -> GPWE6_W<22> {
        GPWE6_W::new(self)
    }
    #[doc = "Bit 23 - Global Pin Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpwe7(&mut self) -> GPWE7_W<23> {
        GPWE7_W::new(self)
    }
    #[doc = "Bit 24 - Global Pin Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpwe8(&mut self) -> GPWE8_W<24> {
        GPWE8_W::new(self)
    }
    #[doc = "Bit 25 - Global Pin Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpwe9(&mut self) -> GPWE9_W<25> {
        GPWE9_W::new(self)
    }
    #[doc = "Bit 26 - Global Pin Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpwe10(&mut self) -> GPWE10_W<26> {
        GPWE10_W::new(self)
    }
    #[doc = "Bit 27 - Global Pin Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpwe11(&mut self) -> GPWE11_W<27> {
        GPWE11_W::new(self)
    }
    #[doc = "Bit 28 - Global Pin Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpwe12(&mut self) -> GPWE12_W<28> {
        GPWE12_W::new(self)
    }
    #[doc = "Bit 29 - Global Pin Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpwe13(&mut self) -> GPWE13_W<29> {
        GPWE13_W::new(self)
    }
    #[doc = "Bit 30 - Global Pin Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpwe14(&mut self) -> GPWE14_W<30> {
        GPWE14_W::new(self)
    }
    #[doc = "Bit 31 - Global Pin Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpwe15(&mut self) -> GPWE15_W<31> {
        GPWE15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Pin Control High Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpchr](index.html) module"]
pub struct GPCHR_SPEC;
impl crate::RegisterSpec for GPCHR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpchr::W](W) writer structure"]
impl crate::Writable for GPCHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPCHR to value 0"]
impl crate::Resettable for GPCHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
