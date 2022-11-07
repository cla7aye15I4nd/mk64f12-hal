#[doc = "Register `PCOR` writer"]
pub struct W(crate::W<PCOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCOR_SPEC>;
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
impl From<crate::W<PCOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO0_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO0_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO0_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO0` writer - Port Clear Output"]
pub type PTCO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO0_AW, O>;
impl<'a, const O: u8> PTCO0_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO0_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO0_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO1_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO1_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO1_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO1` writer - Port Clear Output"]
pub type PTCO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO1_AW, O>;
impl<'a, const O: u8> PTCO1_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO1_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO1_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO2_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO2_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO2_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO2` writer - Port Clear Output"]
pub type PTCO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO2_AW, O>;
impl<'a, const O: u8> PTCO2_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO2_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO2_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO3_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO3_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO3_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO3` writer - Port Clear Output"]
pub type PTCO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO3_AW, O>;
impl<'a, const O: u8> PTCO3_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO3_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO3_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO4_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO4_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO4_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO4` writer - Port Clear Output"]
pub type PTCO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO4_AW, O>;
impl<'a, const O: u8> PTCO4_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO4_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO4_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO5_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO5_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO5_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO5` writer - Port Clear Output"]
pub type PTCO5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO5_AW, O>;
impl<'a, const O: u8> PTCO5_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO5_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO5_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO6_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO6_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO6_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO6` writer - Port Clear Output"]
pub type PTCO6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO6_AW, O>;
impl<'a, const O: u8> PTCO6_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO6_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO6_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO7_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO7_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO7_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO7` writer - Port Clear Output"]
pub type PTCO7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO7_AW, O>;
impl<'a, const O: u8> PTCO7_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO7_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO7_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO8_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO8_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO8_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO8` writer - Port Clear Output"]
pub type PTCO8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO8_AW, O>;
impl<'a, const O: u8> PTCO8_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO8_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO8_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO9_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO9_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO9_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO9` writer - Port Clear Output"]
pub type PTCO9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO9_AW, O>;
impl<'a, const O: u8> PTCO9_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO9_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO9_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO10_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO10_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO10_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO10` writer - Port Clear Output"]
pub type PTCO10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO10_AW, O>;
impl<'a, const O: u8> PTCO10_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO10_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO10_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO11_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO11_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO11_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO11` writer - Port Clear Output"]
pub type PTCO11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO11_AW, O>;
impl<'a, const O: u8> PTCO11_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO11_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO11_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO12_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO12_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO12_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO12` writer - Port Clear Output"]
pub type PTCO12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO12_AW, O>;
impl<'a, const O: u8> PTCO12_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO12_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO12_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO13_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO13_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO13_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO13` writer - Port Clear Output"]
pub type PTCO13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO13_AW, O>;
impl<'a, const O: u8> PTCO13_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO13_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO13_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO14_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO14_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO14_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO14` writer - Port Clear Output"]
pub type PTCO14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO14_AW, O>;
impl<'a, const O: u8> PTCO14_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO14_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO14_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO15_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO15_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO15_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO15` writer - Port Clear Output"]
pub type PTCO15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO15_AW, O>;
impl<'a, const O: u8> PTCO15_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO15_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO15_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO16_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO16_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO16_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO16` writer - Port Clear Output"]
pub type PTCO16_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO16_AW, O>;
impl<'a, const O: u8> PTCO16_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO16_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO16_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO17_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO17_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO17_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO17` writer - Port Clear Output"]
pub type PTCO17_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO17_AW, O>;
impl<'a, const O: u8> PTCO17_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO17_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO17_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO18_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO18_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO18_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO18` writer - Port Clear Output"]
pub type PTCO18_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO18_AW, O>;
impl<'a, const O: u8> PTCO18_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO18_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO18_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO19_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO19_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO19_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO19` writer - Port Clear Output"]
pub type PTCO19_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO19_AW, O>;
impl<'a, const O: u8> PTCO19_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO19_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO19_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO20_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO20_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO20_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO20` writer - Port Clear Output"]
pub type PTCO20_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO20_AW, O>;
impl<'a, const O: u8> PTCO20_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO20_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO20_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO21_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO21_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO21_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO21` writer - Port Clear Output"]
pub type PTCO21_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO21_AW, O>;
impl<'a, const O: u8> PTCO21_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO21_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO21_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO22_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO22_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO22_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO22` writer - Port Clear Output"]
pub type PTCO22_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO22_AW, O>;
impl<'a, const O: u8> PTCO22_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO22_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO22_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO23_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO23_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO23_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO23` writer - Port Clear Output"]
pub type PTCO23_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO23_AW, O>;
impl<'a, const O: u8> PTCO23_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO23_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO23_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO24_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO24_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO24_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO24` writer - Port Clear Output"]
pub type PTCO24_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO24_AW, O>;
impl<'a, const O: u8> PTCO24_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO24_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO24_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO25_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO25_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO25_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO25` writer - Port Clear Output"]
pub type PTCO25_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO25_AW, O>;
impl<'a, const O: u8> PTCO25_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO25_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO25_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO26_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO26_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO26_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO26` writer - Port Clear Output"]
pub type PTCO26_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO26_AW, O>;
impl<'a, const O: u8> PTCO26_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO26_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO26_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO27_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO27_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO27_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO27` writer - Port Clear Output"]
pub type PTCO27_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO27_AW, O>;
impl<'a, const O: u8> PTCO27_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO27_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO27_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO28_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO28_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO28_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO28` writer - Port Clear Output"]
pub type PTCO28_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO28_AW, O>;
impl<'a, const O: u8> PTCO28_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO28_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO28_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO29_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO29_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO29_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO29` writer - Port Clear Output"]
pub type PTCO29_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO29_AW, O>;
impl<'a, const O: u8> PTCO29_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO29_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO29_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO30_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO30_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO30_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO30` writer - Port Clear Output"]
pub type PTCO30_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO30_AW, O>;
impl<'a, const O: u8> PTCO30_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO30_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO30_AW::_1)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PTCO31_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO31_AW> for bool {
    #[inline(always)]
    fn from(variant: PTCO31_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PTCO31` writer - Port Clear Output"]
pub type PTCO31_W<'a, const O: u8> = crate::BitWriter<'a, u32, PCOR_SPEC, PTCO31_AW, O>;
impl<'a, const O: u8> PTCO31_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO31_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO31_AW::_1)
    }
}
impl W {
    #[doc = "Bit 0 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco0(&mut self) -> PTCO0_W<0> {
        PTCO0_W::new(self)
    }
    #[doc = "Bit 1 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco1(&mut self) -> PTCO1_W<1> {
        PTCO1_W::new(self)
    }
    #[doc = "Bit 2 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco2(&mut self) -> PTCO2_W<2> {
        PTCO2_W::new(self)
    }
    #[doc = "Bit 3 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco3(&mut self) -> PTCO3_W<3> {
        PTCO3_W::new(self)
    }
    #[doc = "Bit 4 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco4(&mut self) -> PTCO4_W<4> {
        PTCO4_W::new(self)
    }
    #[doc = "Bit 5 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco5(&mut self) -> PTCO5_W<5> {
        PTCO5_W::new(self)
    }
    #[doc = "Bit 6 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco6(&mut self) -> PTCO6_W<6> {
        PTCO6_W::new(self)
    }
    #[doc = "Bit 7 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco7(&mut self) -> PTCO7_W<7> {
        PTCO7_W::new(self)
    }
    #[doc = "Bit 8 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco8(&mut self) -> PTCO8_W<8> {
        PTCO8_W::new(self)
    }
    #[doc = "Bit 9 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco9(&mut self) -> PTCO9_W<9> {
        PTCO9_W::new(self)
    }
    #[doc = "Bit 10 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco10(&mut self) -> PTCO10_W<10> {
        PTCO10_W::new(self)
    }
    #[doc = "Bit 11 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco11(&mut self) -> PTCO11_W<11> {
        PTCO11_W::new(self)
    }
    #[doc = "Bit 12 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco12(&mut self) -> PTCO12_W<12> {
        PTCO12_W::new(self)
    }
    #[doc = "Bit 13 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco13(&mut self) -> PTCO13_W<13> {
        PTCO13_W::new(self)
    }
    #[doc = "Bit 14 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco14(&mut self) -> PTCO14_W<14> {
        PTCO14_W::new(self)
    }
    #[doc = "Bit 15 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco15(&mut self) -> PTCO15_W<15> {
        PTCO15_W::new(self)
    }
    #[doc = "Bit 16 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco16(&mut self) -> PTCO16_W<16> {
        PTCO16_W::new(self)
    }
    #[doc = "Bit 17 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco17(&mut self) -> PTCO17_W<17> {
        PTCO17_W::new(self)
    }
    #[doc = "Bit 18 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco18(&mut self) -> PTCO18_W<18> {
        PTCO18_W::new(self)
    }
    #[doc = "Bit 19 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco19(&mut self) -> PTCO19_W<19> {
        PTCO19_W::new(self)
    }
    #[doc = "Bit 20 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco20(&mut self) -> PTCO20_W<20> {
        PTCO20_W::new(self)
    }
    #[doc = "Bit 21 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco21(&mut self) -> PTCO21_W<21> {
        PTCO21_W::new(self)
    }
    #[doc = "Bit 22 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco22(&mut self) -> PTCO22_W<22> {
        PTCO22_W::new(self)
    }
    #[doc = "Bit 23 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco23(&mut self) -> PTCO23_W<23> {
        PTCO23_W::new(self)
    }
    #[doc = "Bit 24 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco24(&mut self) -> PTCO24_W<24> {
        PTCO24_W::new(self)
    }
    #[doc = "Bit 25 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco25(&mut self) -> PTCO25_W<25> {
        PTCO25_W::new(self)
    }
    #[doc = "Bit 26 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco26(&mut self) -> PTCO26_W<26> {
        PTCO26_W::new(self)
    }
    #[doc = "Bit 27 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco27(&mut self) -> PTCO27_W<27> {
        PTCO27_W::new(self)
    }
    #[doc = "Bit 28 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco28(&mut self) -> PTCO28_W<28> {
        PTCO28_W::new(self)
    }
    #[doc = "Bit 29 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco29(&mut self) -> PTCO29_W<29> {
        PTCO29_W::new(self)
    }
    #[doc = "Bit 30 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco30(&mut self) -> PTCO30_W<30> {
        PTCO30_W::new(self)
    }
    #[doc = "Bit 31 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco31(&mut self) -> PTCO31_W<31> {
        PTCO31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Clear Output Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcor](index.html) module"]
pub struct PCOR_SPEC;
impl crate::RegisterSpec for PCOR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pcor::W](W) writer structure"]
impl crate::Writable for PCOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCOR to value 0"]
impl crate::Resettable for PCOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
