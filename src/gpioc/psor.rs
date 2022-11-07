#[doc = "Register `PSOR` writer"]
pub struct W(crate::W<PSOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PSOR_SPEC>;
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
impl From<crate::W<PSOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PSOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO0_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO0_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO0` writer - Port Set Output"]
pub type PTSO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO0_AW, O>;
impl<'a, const O: u8> PTSO0_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO0_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO0_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO1_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO1_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO1` writer - Port Set Output"]
pub type PTSO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO1_AW, O>;
impl<'a, const O: u8> PTSO1_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO1_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO1_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO2_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO2_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO2` writer - Port Set Output"]
pub type PTSO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO2_AW, O>;
impl<'a, const O: u8> PTSO2_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO2_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO2_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO3_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO3_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO3` writer - Port Set Output"]
pub type PTSO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO3_AW, O>;
impl<'a, const O: u8> PTSO3_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO3_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO3_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO4_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO4_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO4` writer - Port Set Output"]
pub type PTSO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO4_AW, O>;
impl<'a, const O: u8> PTSO4_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO4_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO4_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO5_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO5_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO5` writer - Port Set Output"]
pub type PTSO5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO5_AW, O>;
impl<'a, const O: u8> PTSO5_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO5_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO5_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO6_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO6_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO6` writer - Port Set Output"]
pub type PTSO6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO6_AW, O>;
impl<'a, const O: u8> PTSO6_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO6_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO6_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO7_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO7_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO7` writer - Port Set Output"]
pub type PTSO7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO7_AW, O>;
impl<'a, const O: u8> PTSO7_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO7_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO7_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO8_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO8_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO8` writer - Port Set Output"]
pub type PTSO8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO8_AW, O>;
impl<'a, const O: u8> PTSO8_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO8_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO8_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO9_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO9_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO9` writer - Port Set Output"]
pub type PTSO9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO9_AW, O>;
impl<'a, const O: u8> PTSO9_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO9_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO9_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO10_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO10_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO10` writer - Port Set Output"]
pub type PTSO10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO10_AW, O>;
impl<'a, const O: u8> PTSO10_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO10_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO10_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO11_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO11_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO11` writer - Port Set Output"]
pub type PTSO11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO11_AW, O>;
impl<'a, const O: u8> PTSO11_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO11_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO11_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO12_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO12_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO12` writer - Port Set Output"]
pub type PTSO12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO12_AW, O>;
impl<'a, const O: u8> PTSO12_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO12_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO12_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO13_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO13_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO13` writer - Port Set Output"]
pub type PTSO13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO13_AW, O>;
impl<'a, const O: u8> PTSO13_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO13_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO13_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO14_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO14_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO14` writer - Port Set Output"]
pub type PTSO14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO14_AW, O>;
impl<'a, const O: u8> PTSO14_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO14_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO14_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO15_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO15_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO15` writer - Port Set Output"]
pub type PTSO15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO15_AW, O>;
impl<'a, const O: u8> PTSO15_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO15_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO15_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO16_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO16_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO16_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO16` writer - Port Set Output"]
pub type PTSO16_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO16_AW, O>;
impl<'a, const O: u8> PTSO16_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO16_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO16_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO17_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO17_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO17_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO17` writer - Port Set Output"]
pub type PTSO17_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO17_AW, O>;
impl<'a, const O: u8> PTSO17_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO17_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO17_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO18_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO18_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO18_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO18` writer - Port Set Output"]
pub type PTSO18_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO18_AW, O>;
impl<'a, const O: u8> PTSO18_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO18_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO18_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO19_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO19_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO19_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO19` writer - Port Set Output"]
pub type PTSO19_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO19_AW, O>;
impl<'a, const O: u8> PTSO19_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO19_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO19_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO20_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO20_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO20_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO20` writer - Port Set Output"]
pub type PTSO20_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO20_AW, O>;
impl<'a, const O: u8> PTSO20_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO20_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO20_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO21_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO21_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO21_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO21` writer - Port Set Output"]
pub type PTSO21_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO21_AW, O>;
impl<'a, const O: u8> PTSO21_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO21_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO21_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO22_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO22_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO22_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO22` writer - Port Set Output"]
pub type PTSO22_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO22_AW, O>;
impl<'a, const O: u8> PTSO22_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO22_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO22_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO23_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO23_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO23_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO23` writer - Port Set Output"]
pub type PTSO23_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO23_AW, O>;
impl<'a, const O: u8> PTSO23_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO23_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO23_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO24_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO24_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO24_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO24` writer - Port Set Output"]
pub type PTSO24_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO24_AW, O>;
impl<'a, const O: u8> PTSO24_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO24_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO24_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO25_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO25_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO25_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO25` writer - Port Set Output"]
pub type PTSO25_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO25_AW, O>;
impl<'a, const O: u8> PTSO25_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO25_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO25_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO26_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO26_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO26_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO26` writer - Port Set Output"]
pub type PTSO26_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO26_AW, O>;
impl<'a, const O: u8> PTSO26_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO26_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO26_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO27_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO27_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO27_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO27` writer - Port Set Output"]
pub type PTSO27_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO27_AW, O>;
impl<'a, const O: u8> PTSO27_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO27_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO27_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO28_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO28_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO28_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO28` writer - Port Set Output"]
pub type PTSO28_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO28_AW, O>;
impl<'a, const O: u8> PTSO28_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO28_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO28_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO29_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO29_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO29_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO29` writer - Port Set Output"]
pub type PTSO29_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO29_AW, O>;
impl<'a, const O: u8> PTSO29_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO29_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO29_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO30_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO30_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO30_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO30` writer - Port Set Output"]
pub type PTSO30_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO30_AW, O>;
impl<'a, const O: u8> PTSO30_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO30_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO30_AW::_1)
    }
}
#[doc = "Port Set Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTSO31_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to logic 1."]
    _1 = 1,
}
impl From<PTSO31_AW> for bool {
    #[inline(always)]
    fn from(variant: PTSO31_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTSO31` writer - Port Set Output"]
pub type PTSO31_W<'a, const O: u8> = crate::BitWriter<'a, u32, PSOR_SPEC, PTSO31_AW, O>;
impl<'a, const O: u8> PTSO31_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTSO31_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to logic 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTSO31_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso0(&mut self) -> PTSO0_W<0> {
        PTSO0_W::new(self)
    }
    #[doc = "Bit 1 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso1(&mut self) -> PTSO1_W<1> {
        PTSO1_W::new(self)
    }
    #[doc = "Bit 2 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso2(&mut self) -> PTSO2_W<2> {
        PTSO2_W::new(self)
    }
    #[doc = "Bit 3 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso3(&mut self) -> PTSO3_W<3> {
        PTSO3_W::new(self)
    }
    #[doc = "Bit 4 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso4(&mut self) -> PTSO4_W<4> {
        PTSO4_W::new(self)
    }
    #[doc = "Bit 5 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso5(&mut self) -> PTSO5_W<5> {
        PTSO5_W::new(self)
    }
    #[doc = "Bit 6 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso6(&mut self) -> PTSO6_W<6> {
        PTSO6_W::new(self)
    }
    #[doc = "Bit 7 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso7(&mut self) -> PTSO7_W<7> {
        PTSO7_W::new(self)
    }
    #[doc = "Bit 8 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso8(&mut self) -> PTSO8_W<8> {
        PTSO8_W::new(self)
    }
    #[doc = "Bit 9 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso9(&mut self) -> PTSO9_W<9> {
        PTSO9_W::new(self)
    }
    #[doc = "Bit 10 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso10(&mut self) -> PTSO10_W<10> {
        PTSO10_W::new(self)
    }
    #[doc = "Bit 11 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso11(&mut self) -> PTSO11_W<11> {
        PTSO11_W::new(self)
    }
    #[doc = "Bit 12 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso12(&mut self) -> PTSO12_W<12> {
        PTSO12_W::new(self)
    }
    #[doc = "Bit 13 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso13(&mut self) -> PTSO13_W<13> {
        PTSO13_W::new(self)
    }
    #[doc = "Bit 14 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso14(&mut self) -> PTSO14_W<14> {
        PTSO14_W::new(self)
    }
    #[doc = "Bit 15 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso15(&mut self) -> PTSO15_W<15> {
        PTSO15_W::new(self)
    }
    #[doc = "Bit 16 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso16(&mut self) -> PTSO16_W<16> {
        PTSO16_W::new(self)
    }
    #[doc = "Bit 17 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso17(&mut self) -> PTSO17_W<17> {
        PTSO17_W::new(self)
    }
    #[doc = "Bit 18 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso18(&mut self) -> PTSO18_W<18> {
        PTSO18_W::new(self)
    }
    #[doc = "Bit 19 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso19(&mut self) -> PTSO19_W<19> {
        PTSO19_W::new(self)
    }
    #[doc = "Bit 20 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso20(&mut self) -> PTSO20_W<20> {
        PTSO20_W::new(self)
    }
    #[doc = "Bit 21 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso21(&mut self) -> PTSO21_W<21> {
        PTSO21_W::new(self)
    }
    #[doc = "Bit 22 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso22(&mut self) -> PTSO22_W<22> {
        PTSO22_W::new(self)
    }
    #[doc = "Bit 23 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso23(&mut self) -> PTSO23_W<23> {
        PTSO23_W::new(self)
    }
    #[doc = "Bit 24 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso24(&mut self) -> PTSO24_W<24> {
        PTSO24_W::new(self)
    }
    #[doc = "Bit 25 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso25(&mut self) -> PTSO25_W<25> {
        PTSO25_W::new(self)
    }
    #[doc = "Bit 26 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso26(&mut self) -> PTSO26_W<26> {
        PTSO26_W::new(self)
    }
    #[doc = "Bit 27 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso27(&mut self) -> PTSO27_W<27> {
        PTSO27_W::new(self)
    }
    #[doc = "Bit 28 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso28(&mut self) -> PTSO28_W<28> {
        PTSO28_W::new(self)
    }
    #[doc = "Bit 29 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso29(&mut self) -> PTSO29_W<29> {
        PTSO29_W::new(self)
    }
    #[doc = "Bit 30 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso30(&mut self) -> PTSO30_W<30> {
        PTSO30_W::new(self)
    }
    #[doc = "Bit 31 - Port Set Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptso31(&mut self) -> PTSO31_W<31> {
        PTSO31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Set Output Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [psor](index.html) module"]
pub struct PSOR_SPEC;
impl crate::RegisterSpec for PSOR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [psor::W](W) writer structure"]
impl crate::Writable for PSOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PSOR to value 0"]
impl crate::Resettable for PSOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
