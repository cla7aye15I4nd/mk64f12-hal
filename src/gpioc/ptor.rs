#[doc = "Register `PTOR` writer"]
pub struct W(crate::W<PTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTOR_SPEC>;
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
impl From<crate::W<PTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO0_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO0_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO0` writer - Port Toggle Output"]
pub type PTTO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO0_AW, O>;
impl<'a, const O: u8> PTTO0_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO0_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO0_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO1_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO1_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO1` writer - Port Toggle Output"]
pub type PTTO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO1_AW, O>;
impl<'a, const O: u8> PTTO1_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO1_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO1_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO2_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO2_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO2` writer - Port Toggle Output"]
pub type PTTO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO2_AW, O>;
impl<'a, const O: u8> PTTO2_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO2_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO2_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO3_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO3_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO3` writer - Port Toggle Output"]
pub type PTTO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO3_AW, O>;
impl<'a, const O: u8> PTTO3_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO3_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO3_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO4_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO4_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO4` writer - Port Toggle Output"]
pub type PTTO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO4_AW, O>;
impl<'a, const O: u8> PTTO4_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO4_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO4_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO5_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO5_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO5` writer - Port Toggle Output"]
pub type PTTO5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO5_AW, O>;
impl<'a, const O: u8> PTTO5_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO5_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO5_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO6_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO6_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO6` writer - Port Toggle Output"]
pub type PTTO6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO6_AW, O>;
impl<'a, const O: u8> PTTO6_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO6_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO6_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO7_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO7_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO7` writer - Port Toggle Output"]
pub type PTTO7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO7_AW, O>;
impl<'a, const O: u8> PTTO7_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO7_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO7_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO8_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO8_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO8` writer - Port Toggle Output"]
pub type PTTO8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO8_AW, O>;
impl<'a, const O: u8> PTTO8_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO8_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO8_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO9_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO9_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO9` writer - Port Toggle Output"]
pub type PTTO9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO9_AW, O>;
impl<'a, const O: u8> PTTO9_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO9_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO9_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO10_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO10_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO10` writer - Port Toggle Output"]
pub type PTTO10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO10_AW, O>;
impl<'a, const O: u8> PTTO10_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO10_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO10_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO11_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO11_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO11` writer - Port Toggle Output"]
pub type PTTO11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO11_AW, O>;
impl<'a, const O: u8> PTTO11_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO11_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO11_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO12_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO12_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO12` writer - Port Toggle Output"]
pub type PTTO12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO12_AW, O>;
impl<'a, const O: u8> PTTO12_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO12_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO12_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO13_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO13_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO13` writer - Port Toggle Output"]
pub type PTTO13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO13_AW, O>;
impl<'a, const O: u8> PTTO13_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO13_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO13_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO14_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO14_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO14` writer - Port Toggle Output"]
pub type PTTO14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO14_AW, O>;
impl<'a, const O: u8> PTTO14_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO14_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO14_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO15_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO15_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO15` writer - Port Toggle Output"]
pub type PTTO15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO15_AW, O>;
impl<'a, const O: u8> PTTO15_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO15_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO15_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO16_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO16_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO16_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO16` writer - Port Toggle Output"]
pub type PTTO16_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO16_AW, O>;
impl<'a, const O: u8> PTTO16_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO16_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO16_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO17_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO17_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO17_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO17` writer - Port Toggle Output"]
pub type PTTO17_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO17_AW, O>;
impl<'a, const O: u8> PTTO17_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO17_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO17_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO18_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO18_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO18_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO18` writer - Port Toggle Output"]
pub type PTTO18_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO18_AW, O>;
impl<'a, const O: u8> PTTO18_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO18_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO18_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO19_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO19_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO19_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO19` writer - Port Toggle Output"]
pub type PTTO19_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO19_AW, O>;
impl<'a, const O: u8> PTTO19_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO19_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO19_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO20_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO20_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO20_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO20` writer - Port Toggle Output"]
pub type PTTO20_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO20_AW, O>;
impl<'a, const O: u8> PTTO20_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO20_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO20_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO21_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO21_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO21_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO21` writer - Port Toggle Output"]
pub type PTTO21_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO21_AW, O>;
impl<'a, const O: u8> PTTO21_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO21_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO21_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO22_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO22_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO22_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO22` writer - Port Toggle Output"]
pub type PTTO22_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO22_AW, O>;
impl<'a, const O: u8> PTTO22_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO22_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO22_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO23_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO23_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO23_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO23` writer - Port Toggle Output"]
pub type PTTO23_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO23_AW, O>;
impl<'a, const O: u8> PTTO23_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO23_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO23_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO24_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO24_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO24_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO24` writer - Port Toggle Output"]
pub type PTTO24_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO24_AW, O>;
impl<'a, const O: u8> PTTO24_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO24_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO24_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO25_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO25_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO25_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO25` writer - Port Toggle Output"]
pub type PTTO25_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO25_AW, O>;
impl<'a, const O: u8> PTTO25_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO25_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO25_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO26_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO26_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO26_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO26` writer - Port Toggle Output"]
pub type PTTO26_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO26_AW, O>;
impl<'a, const O: u8> PTTO26_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO26_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO26_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO27_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO27_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO27_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO27` writer - Port Toggle Output"]
pub type PTTO27_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO27_AW, O>;
impl<'a, const O: u8> PTTO27_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO27_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO27_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO28_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO28_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO28_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO28` writer - Port Toggle Output"]
pub type PTTO28_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO28_AW, O>;
impl<'a, const O: u8> PTTO28_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO28_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO28_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO29_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO29_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO29_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO29` writer - Port Toggle Output"]
pub type PTTO29_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO29_AW, O>;
impl<'a, const O: u8> PTTO29_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO29_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO29_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO30_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO30_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO30_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO30` writer - Port Toggle Output"]
pub type PTTO30_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO30_AW, O>;
impl<'a, const O: u8> PTTO30_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO30_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO30_AW::_1)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTTO31_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO31_AW> for bool {
    #[inline(always)]
    fn from(variant: PTTO31_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTTO31` writer - Port Toggle Output"]
pub type PTTO31_W<'a, const O: u8> = crate::BitWriter<'a, u32, PTOR_SPEC, PTTO31_AW, O>;
impl<'a, const O: u8> PTTO31_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO31_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO31_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto0(&mut self) -> PTTO0_W<0> {
        PTTO0_W::new(self)
    }
    #[doc = "Bit 1 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto1(&mut self) -> PTTO1_W<1> {
        PTTO1_W::new(self)
    }
    #[doc = "Bit 2 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto2(&mut self) -> PTTO2_W<2> {
        PTTO2_W::new(self)
    }
    #[doc = "Bit 3 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto3(&mut self) -> PTTO3_W<3> {
        PTTO3_W::new(self)
    }
    #[doc = "Bit 4 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto4(&mut self) -> PTTO4_W<4> {
        PTTO4_W::new(self)
    }
    #[doc = "Bit 5 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto5(&mut self) -> PTTO5_W<5> {
        PTTO5_W::new(self)
    }
    #[doc = "Bit 6 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto6(&mut self) -> PTTO6_W<6> {
        PTTO6_W::new(self)
    }
    #[doc = "Bit 7 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto7(&mut self) -> PTTO7_W<7> {
        PTTO7_W::new(self)
    }
    #[doc = "Bit 8 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto8(&mut self) -> PTTO8_W<8> {
        PTTO8_W::new(self)
    }
    #[doc = "Bit 9 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto9(&mut self) -> PTTO9_W<9> {
        PTTO9_W::new(self)
    }
    #[doc = "Bit 10 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto10(&mut self) -> PTTO10_W<10> {
        PTTO10_W::new(self)
    }
    #[doc = "Bit 11 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto11(&mut self) -> PTTO11_W<11> {
        PTTO11_W::new(self)
    }
    #[doc = "Bit 12 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto12(&mut self) -> PTTO12_W<12> {
        PTTO12_W::new(self)
    }
    #[doc = "Bit 13 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto13(&mut self) -> PTTO13_W<13> {
        PTTO13_W::new(self)
    }
    #[doc = "Bit 14 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto14(&mut self) -> PTTO14_W<14> {
        PTTO14_W::new(self)
    }
    #[doc = "Bit 15 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto15(&mut self) -> PTTO15_W<15> {
        PTTO15_W::new(self)
    }
    #[doc = "Bit 16 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto16(&mut self) -> PTTO16_W<16> {
        PTTO16_W::new(self)
    }
    #[doc = "Bit 17 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto17(&mut self) -> PTTO17_W<17> {
        PTTO17_W::new(self)
    }
    #[doc = "Bit 18 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto18(&mut self) -> PTTO18_W<18> {
        PTTO18_W::new(self)
    }
    #[doc = "Bit 19 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto19(&mut self) -> PTTO19_W<19> {
        PTTO19_W::new(self)
    }
    #[doc = "Bit 20 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto20(&mut self) -> PTTO20_W<20> {
        PTTO20_W::new(self)
    }
    #[doc = "Bit 21 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto21(&mut self) -> PTTO21_W<21> {
        PTTO21_W::new(self)
    }
    #[doc = "Bit 22 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto22(&mut self) -> PTTO22_W<22> {
        PTTO22_W::new(self)
    }
    #[doc = "Bit 23 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto23(&mut self) -> PTTO23_W<23> {
        PTTO23_W::new(self)
    }
    #[doc = "Bit 24 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto24(&mut self) -> PTTO24_W<24> {
        PTTO24_W::new(self)
    }
    #[doc = "Bit 25 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto25(&mut self) -> PTTO25_W<25> {
        PTTO25_W::new(self)
    }
    #[doc = "Bit 26 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto26(&mut self) -> PTTO26_W<26> {
        PTTO26_W::new(self)
    }
    #[doc = "Bit 27 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto27(&mut self) -> PTTO27_W<27> {
        PTTO27_W::new(self)
    }
    #[doc = "Bit 28 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto28(&mut self) -> PTTO28_W<28> {
        PTTO28_W::new(self)
    }
    #[doc = "Bit 29 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto29(&mut self) -> PTTO29_W<29> {
        PTTO29_W::new(self)
    }
    #[doc = "Bit 30 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto30(&mut self) -> PTTO30_W<30> {
        PTTO30_W::new(self)
    }
    #[doc = "Bit 31 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto31(&mut self) -> PTTO31_W<31> {
        PTTO31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Toggle Output Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptor](index.html) module"]
pub struct PTOR_SPEC;
impl crate::RegisterSpec for PTOR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ptor::W](W) writer structure"]
impl crate::Writable for PTOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTOR to value 0"]
impl crate::Resettable for PTOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
