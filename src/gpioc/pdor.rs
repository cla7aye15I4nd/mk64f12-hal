#[doc = "Register `PDOR` reader"]
pub struct R(crate::R<PDOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDOR` writer"]
pub struct W(crate::W<PDOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDOR_SPEC>;
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
impl From<crate::W<PDOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDO0` reader - Port Data Output"]
pub type PDO0_R = crate::BitReader<PDO0_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO0_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO0_A> for bool {
    #[inline(always)]
    fn from(variant: PDO0_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO0_A {
        match self.bits {
            false => PDO0_A::_0,
            true => PDO0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO0_A::_1
    }
}
#[doc = "Field `PDO0` writer - Port Data Output"]
pub type PDO0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO0_A, O>;
impl<'a, const O: u8> PDO0_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO0_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO0_A::_1)
    }
}
#[doc = "Field `PDO1` reader - Port Data Output"]
pub type PDO1_R = crate::BitReader<PDO1_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO1_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO1_A> for bool {
    #[inline(always)]
    fn from(variant: PDO1_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO1_A {
        match self.bits {
            false => PDO1_A::_0,
            true => PDO1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO1_A::_1
    }
}
#[doc = "Field `PDO1` writer - Port Data Output"]
pub type PDO1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO1_A, O>;
impl<'a, const O: u8> PDO1_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO1_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO1_A::_1)
    }
}
#[doc = "Field `PDO2` reader - Port Data Output"]
pub type PDO2_R = crate::BitReader<PDO2_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO2_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO2_A> for bool {
    #[inline(always)]
    fn from(variant: PDO2_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO2_A {
        match self.bits {
            false => PDO2_A::_0,
            true => PDO2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO2_A::_1
    }
}
#[doc = "Field `PDO2` writer - Port Data Output"]
pub type PDO2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO2_A, O>;
impl<'a, const O: u8> PDO2_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO2_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO2_A::_1)
    }
}
#[doc = "Field `PDO3` reader - Port Data Output"]
pub type PDO3_R = crate::BitReader<PDO3_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO3_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO3_A> for bool {
    #[inline(always)]
    fn from(variant: PDO3_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO3_A {
        match self.bits {
            false => PDO3_A::_0,
            true => PDO3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO3_A::_1
    }
}
#[doc = "Field `PDO3` writer - Port Data Output"]
pub type PDO3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO3_A, O>;
impl<'a, const O: u8> PDO3_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO3_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO3_A::_1)
    }
}
#[doc = "Field `PDO4` reader - Port Data Output"]
pub type PDO4_R = crate::BitReader<PDO4_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO4_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO4_A> for bool {
    #[inline(always)]
    fn from(variant: PDO4_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO4_A {
        match self.bits {
            false => PDO4_A::_0,
            true => PDO4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO4_A::_1
    }
}
#[doc = "Field `PDO4` writer - Port Data Output"]
pub type PDO4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO4_A, O>;
impl<'a, const O: u8> PDO4_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO4_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO4_A::_1)
    }
}
#[doc = "Field `PDO5` reader - Port Data Output"]
pub type PDO5_R = crate::BitReader<PDO5_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO5_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO5_A> for bool {
    #[inline(always)]
    fn from(variant: PDO5_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO5_A {
        match self.bits {
            false => PDO5_A::_0,
            true => PDO5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO5_A::_1
    }
}
#[doc = "Field `PDO5` writer - Port Data Output"]
pub type PDO5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO5_A, O>;
impl<'a, const O: u8> PDO5_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO5_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO5_A::_1)
    }
}
#[doc = "Field `PDO6` reader - Port Data Output"]
pub type PDO6_R = crate::BitReader<PDO6_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO6_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO6_A> for bool {
    #[inline(always)]
    fn from(variant: PDO6_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO6_A {
        match self.bits {
            false => PDO6_A::_0,
            true => PDO6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO6_A::_1
    }
}
#[doc = "Field `PDO6` writer - Port Data Output"]
pub type PDO6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO6_A, O>;
impl<'a, const O: u8> PDO6_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO6_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO6_A::_1)
    }
}
#[doc = "Field `PDO7` reader - Port Data Output"]
pub type PDO7_R = crate::BitReader<PDO7_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO7_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO7_A> for bool {
    #[inline(always)]
    fn from(variant: PDO7_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO7_A {
        match self.bits {
            false => PDO7_A::_0,
            true => PDO7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO7_A::_1
    }
}
#[doc = "Field `PDO7` writer - Port Data Output"]
pub type PDO7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO7_A, O>;
impl<'a, const O: u8> PDO7_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO7_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO7_A::_1)
    }
}
#[doc = "Field `PDO8` reader - Port Data Output"]
pub type PDO8_R = crate::BitReader<PDO8_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO8_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO8_A> for bool {
    #[inline(always)]
    fn from(variant: PDO8_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO8_A {
        match self.bits {
            false => PDO8_A::_0,
            true => PDO8_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO8_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO8_A::_1
    }
}
#[doc = "Field `PDO8` writer - Port Data Output"]
pub type PDO8_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO8_A, O>;
impl<'a, const O: u8> PDO8_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO8_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO8_A::_1)
    }
}
#[doc = "Field `PDO9` reader - Port Data Output"]
pub type PDO9_R = crate::BitReader<PDO9_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO9_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO9_A> for bool {
    #[inline(always)]
    fn from(variant: PDO9_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO9_A {
        match self.bits {
            false => PDO9_A::_0,
            true => PDO9_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO9_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO9_A::_1
    }
}
#[doc = "Field `PDO9` writer - Port Data Output"]
pub type PDO9_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO9_A, O>;
impl<'a, const O: u8> PDO9_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO9_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO9_A::_1)
    }
}
#[doc = "Field `PDO10` reader - Port Data Output"]
pub type PDO10_R = crate::BitReader<PDO10_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO10_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO10_A> for bool {
    #[inline(always)]
    fn from(variant: PDO10_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO10_A {
        match self.bits {
            false => PDO10_A::_0,
            true => PDO10_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO10_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO10_A::_1
    }
}
#[doc = "Field `PDO10` writer - Port Data Output"]
pub type PDO10_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO10_A, O>;
impl<'a, const O: u8> PDO10_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO10_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO10_A::_1)
    }
}
#[doc = "Field `PDO11` reader - Port Data Output"]
pub type PDO11_R = crate::BitReader<PDO11_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO11_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO11_A> for bool {
    #[inline(always)]
    fn from(variant: PDO11_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO11_A {
        match self.bits {
            false => PDO11_A::_0,
            true => PDO11_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO11_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO11_A::_1
    }
}
#[doc = "Field `PDO11` writer - Port Data Output"]
pub type PDO11_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO11_A, O>;
impl<'a, const O: u8> PDO11_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO11_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO11_A::_1)
    }
}
#[doc = "Field `PDO12` reader - Port Data Output"]
pub type PDO12_R = crate::BitReader<PDO12_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO12_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO12_A> for bool {
    #[inline(always)]
    fn from(variant: PDO12_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO12_A {
        match self.bits {
            false => PDO12_A::_0,
            true => PDO12_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO12_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO12_A::_1
    }
}
#[doc = "Field `PDO12` writer - Port Data Output"]
pub type PDO12_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO12_A, O>;
impl<'a, const O: u8> PDO12_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO12_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO12_A::_1)
    }
}
#[doc = "Field `PDO13` reader - Port Data Output"]
pub type PDO13_R = crate::BitReader<PDO13_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO13_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO13_A> for bool {
    #[inline(always)]
    fn from(variant: PDO13_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO13_A {
        match self.bits {
            false => PDO13_A::_0,
            true => PDO13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO13_A::_1
    }
}
#[doc = "Field `PDO13` writer - Port Data Output"]
pub type PDO13_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO13_A, O>;
impl<'a, const O: u8> PDO13_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO13_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO13_A::_1)
    }
}
#[doc = "Field `PDO14` reader - Port Data Output"]
pub type PDO14_R = crate::BitReader<PDO14_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO14_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO14_A> for bool {
    #[inline(always)]
    fn from(variant: PDO14_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO14_A {
        match self.bits {
            false => PDO14_A::_0,
            true => PDO14_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO14_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO14_A::_1
    }
}
#[doc = "Field `PDO14` writer - Port Data Output"]
pub type PDO14_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO14_A, O>;
impl<'a, const O: u8> PDO14_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO14_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO14_A::_1)
    }
}
#[doc = "Field `PDO15` reader - Port Data Output"]
pub type PDO15_R = crate::BitReader<PDO15_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO15_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO15_A> for bool {
    #[inline(always)]
    fn from(variant: PDO15_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO15_A {
        match self.bits {
            false => PDO15_A::_0,
            true => PDO15_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO15_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO15_A::_1
    }
}
#[doc = "Field `PDO15` writer - Port Data Output"]
pub type PDO15_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO15_A, O>;
impl<'a, const O: u8> PDO15_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO15_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO15_A::_1)
    }
}
#[doc = "Field `PDO16` reader - Port Data Output"]
pub type PDO16_R = crate::BitReader<PDO16_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO16_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO16_A> for bool {
    #[inline(always)]
    fn from(variant: PDO16_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO16_A {
        match self.bits {
            false => PDO16_A::_0,
            true => PDO16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO16_A::_1
    }
}
#[doc = "Field `PDO16` writer - Port Data Output"]
pub type PDO16_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO16_A, O>;
impl<'a, const O: u8> PDO16_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO16_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO16_A::_1)
    }
}
#[doc = "Field `PDO17` reader - Port Data Output"]
pub type PDO17_R = crate::BitReader<PDO17_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO17_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO17_A> for bool {
    #[inline(always)]
    fn from(variant: PDO17_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO17_A {
        match self.bits {
            false => PDO17_A::_0,
            true => PDO17_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO17_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO17_A::_1
    }
}
#[doc = "Field `PDO17` writer - Port Data Output"]
pub type PDO17_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO17_A, O>;
impl<'a, const O: u8> PDO17_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO17_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO17_A::_1)
    }
}
#[doc = "Field `PDO18` reader - Port Data Output"]
pub type PDO18_R = crate::BitReader<PDO18_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO18_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO18_A> for bool {
    #[inline(always)]
    fn from(variant: PDO18_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO18_A {
        match self.bits {
            false => PDO18_A::_0,
            true => PDO18_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO18_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO18_A::_1
    }
}
#[doc = "Field `PDO18` writer - Port Data Output"]
pub type PDO18_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO18_A, O>;
impl<'a, const O: u8> PDO18_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO18_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO18_A::_1)
    }
}
#[doc = "Field `PDO19` reader - Port Data Output"]
pub type PDO19_R = crate::BitReader<PDO19_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO19_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO19_A> for bool {
    #[inline(always)]
    fn from(variant: PDO19_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO19_A {
        match self.bits {
            false => PDO19_A::_0,
            true => PDO19_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO19_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO19_A::_1
    }
}
#[doc = "Field `PDO19` writer - Port Data Output"]
pub type PDO19_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO19_A, O>;
impl<'a, const O: u8> PDO19_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO19_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO19_A::_1)
    }
}
#[doc = "Field `PDO20` reader - Port Data Output"]
pub type PDO20_R = crate::BitReader<PDO20_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO20_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO20_A> for bool {
    #[inline(always)]
    fn from(variant: PDO20_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO20_A {
        match self.bits {
            false => PDO20_A::_0,
            true => PDO20_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO20_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO20_A::_1
    }
}
#[doc = "Field `PDO20` writer - Port Data Output"]
pub type PDO20_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO20_A, O>;
impl<'a, const O: u8> PDO20_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO20_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO20_A::_1)
    }
}
#[doc = "Field `PDO21` reader - Port Data Output"]
pub type PDO21_R = crate::BitReader<PDO21_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO21_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO21_A> for bool {
    #[inline(always)]
    fn from(variant: PDO21_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO21_A {
        match self.bits {
            false => PDO21_A::_0,
            true => PDO21_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO21_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO21_A::_1
    }
}
#[doc = "Field `PDO21` writer - Port Data Output"]
pub type PDO21_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO21_A, O>;
impl<'a, const O: u8> PDO21_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO21_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO21_A::_1)
    }
}
#[doc = "Field `PDO22` reader - Port Data Output"]
pub type PDO22_R = crate::BitReader<PDO22_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO22_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO22_A> for bool {
    #[inline(always)]
    fn from(variant: PDO22_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO22_A {
        match self.bits {
            false => PDO22_A::_0,
            true => PDO22_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO22_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO22_A::_1
    }
}
#[doc = "Field `PDO22` writer - Port Data Output"]
pub type PDO22_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO22_A, O>;
impl<'a, const O: u8> PDO22_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO22_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO22_A::_1)
    }
}
#[doc = "Field `PDO23` reader - Port Data Output"]
pub type PDO23_R = crate::BitReader<PDO23_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO23_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO23_A> for bool {
    #[inline(always)]
    fn from(variant: PDO23_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO23_A {
        match self.bits {
            false => PDO23_A::_0,
            true => PDO23_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO23_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO23_A::_1
    }
}
#[doc = "Field `PDO23` writer - Port Data Output"]
pub type PDO23_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO23_A, O>;
impl<'a, const O: u8> PDO23_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO23_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO23_A::_1)
    }
}
#[doc = "Field `PDO24` reader - Port Data Output"]
pub type PDO24_R = crate::BitReader<PDO24_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO24_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO24_A> for bool {
    #[inline(always)]
    fn from(variant: PDO24_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO24_A {
        match self.bits {
            false => PDO24_A::_0,
            true => PDO24_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO24_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO24_A::_1
    }
}
#[doc = "Field `PDO24` writer - Port Data Output"]
pub type PDO24_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO24_A, O>;
impl<'a, const O: u8> PDO24_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO24_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO24_A::_1)
    }
}
#[doc = "Field `PDO25` reader - Port Data Output"]
pub type PDO25_R = crate::BitReader<PDO25_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO25_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO25_A> for bool {
    #[inline(always)]
    fn from(variant: PDO25_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO25_A {
        match self.bits {
            false => PDO25_A::_0,
            true => PDO25_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO25_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO25_A::_1
    }
}
#[doc = "Field `PDO25` writer - Port Data Output"]
pub type PDO25_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO25_A, O>;
impl<'a, const O: u8> PDO25_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO25_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO25_A::_1)
    }
}
#[doc = "Field `PDO26` reader - Port Data Output"]
pub type PDO26_R = crate::BitReader<PDO26_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO26_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO26_A> for bool {
    #[inline(always)]
    fn from(variant: PDO26_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO26_A {
        match self.bits {
            false => PDO26_A::_0,
            true => PDO26_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO26_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO26_A::_1
    }
}
#[doc = "Field `PDO26` writer - Port Data Output"]
pub type PDO26_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO26_A, O>;
impl<'a, const O: u8> PDO26_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO26_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO26_A::_1)
    }
}
#[doc = "Field `PDO27` reader - Port Data Output"]
pub type PDO27_R = crate::BitReader<PDO27_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO27_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO27_A> for bool {
    #[inline(always)]
    fn from(variant: PDO27_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO27_A {
        match self.bits {
            false => PDO27_A::_0,
            true => PDO27_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO27_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO27_A::_1
    }
}
#[doc = "Field `PDO27` writer - Port Data Output"]
pub type PDO27_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO27_A, O>;
impl<'a, const O: u8> PDO27_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO27_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO27_A::_1)
    }
}
#[doc = "Field `PDO28` reader - Port Data Output"]
pub type PDO28_R = crate::BitReader<PDO28_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO28_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO28_A> for bool {
    #[inline(always)]
    fn from(variant: PDO28_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO28_A {
        match self.bits {
            false => PDO28_A::_0,
            true => PDO28_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO28_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO28_A::_1
    }
}
#[doc = "Field `PDO28` writer - Port Data Output"]
pub type PDO28_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO28_A, O>;
impl<'a, const O: u8> PDO28_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO28_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO28_A::_1)
    }
}
#[doc = "Field `PDO29` reader - Port Data Output"]
pub type PDO29_R = crate::BitReader<PDO29_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO29_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO29_A> for bool {
    #[inline(always)]
    fn from(variant: PDO29_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO29_A {
        match self.bits {
            false => PDO29_A::_0,
            true => PDO29_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO29_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO29_A::_1
    }
}
#[doc = "Field `PDO29` writer - Port Data Output"]
pub type PDO29_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO29_A, O>;
impl<'a, const O: u8> PDO29_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO29_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO29_A::_1)
    }
}
#[doc = "Field `PDO30` reader - Port Data Output"]
pub type PDO30_R = crate::BitReader<PDO30_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO30_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO30_A> for bool {
    #[inline(always)]
    fn from(variant: PDO30_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO30_A {
        match self.bits {
            false => PDO30_A::_0,
            true => PDO30_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO30_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO30_A::_1
    }
}
#[doc = "Field `PDO30` writer - Port Data Output"]
pub type PDO30_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO30_A, O>;
impl<'a, const O: u8> PDO30_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO30_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO30_A::_1)
    }
}
#[doc = "Field `PDO31` reader - Port Data Output"]
pub type PDO31_R = crate::BitReader<PDO31_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDO31_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO31_A> for bool {
    #[inline(always)]
    fn from(variant: PDO31_A) -> Self {
        variant as u8 != 0
    }
}
impl PDO31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDO31_A {
        match self.bits {
            false => PDO31_A::_0,
            true => PDO31_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO31_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO31_A::_1
    }
}
#[doc = "Field `PDO31` writer - Port Data Output"]
pub type PDO31_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDOR_SPEC, PDO31_A, O>;
impl<'a, const O: u8> PDO31_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO31_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO31_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Port Data Output"]
    #[inline(always)]
    pub fn pdo0(&self) -> PDO0_R {
        PDO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port Data Output"]
    #[inline(always)]
    pub fn pdo1(&self) -> PDO1_R {
        PDO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Data Output"]
    #[inline(always)]
    pub fn pdo2(&self) -> PDO2_R {
        PDO2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port Data Output"]
    #[inline(always)]
    pub fn pdo3(&self) -> PDO3_R {
        PDO3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port Data Output"]
    #[inline(always)]
    pub fn pdo4(&self) -> PDO4_R {
        PDO4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port Data Output"]
    #[inline(always)]
    pub fn pdo5(&self) -> PDO5_R {
        PDO5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port Data Output"]
    #[inline(always)]
    pub fn pdo6(&self) -> PDO6_R {
        PDO6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port Data Output"]
    #[inline(always)]
    pub fn pdo7(&self) -> PDO7_R {
        PDO7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port Data Output"]
    #[inline(always)]
    pub fn pdo8(&self) -> PDO8_R {
        PDO8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port Data Output"]
    #[inline(always)]
    pub fn pdo9(&self) -> PDO9_R {
        PDO9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port Data Output"]
    #[inline(always)]
    pub fn pdo10(&self) -> PDO10_R {
        PDO10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port Data Output"]
    #[inline(always)]
    pub fn pdo11(&self) -> PDO11_R {
        PDO11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port Data Output"]
    #[inline(always)]
    pub fn pdo12(&self) -> PDO12_R {
        PDO12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port Data Output"]
    #[inline(always)]
    pub fn pdo13(&self) -> PDO13_R {
        PDO13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port Data Output"]
    #[inline(always)]
    pub fn pdo14(&self) -> PDO14_R {
        PDO14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port Data Output"]
    #[inline(always)]
    pub fn pdo15(&self) -> PDO15_R {
        PDO15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Port Data Output"]
    #[inline(always)]
    pub fn pdo16(&self) -> PDO16_R {
        PDO16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Port Data Output"]
    #[inline(always)]
    pub fn pdo17(&self) -> PDO17_R {
        PDO17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Port Data Output"]
    #[inline(always)]
    pub fn pdo18(&self) -> PDO18_R {
        PDO18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Port Data Output"]
    #[inline(always)]
    pub fn pdo19(&self) -> PDO19_R {
        PDO19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Port Data Output"]
    #[inline(always)]
    pub fn pdo20(&self) -> PDO20_R {
        PDO20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Port Data Output"]
    #[inline(always)]
    pub fn pdo21(&self) -> PDO21_R {
        PDO21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Port Data Output"]
    #[inline(always)]
    pub fn pdo22(&self) -> PDO22_R {
        PDO22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Port Data Output"]
    #[inline(always)]
    pub fn pdo23(&self) -> PDO23_R {
        PDO23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Port Data Output"]
    #[inline(always)]
    pub fn pdo24(&self) -> PDO24_R {
        PDO24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Port Data Output"]
    #[inline(always)]
    pub fn pdo25(&self) -> PDO25_R {
        PDO25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Port Data Output"]
    #[inline(always)]
    pub fn pdo26(&self) -> PDO26_R {
        PDO26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Port Data Output"]
    #[inline(always)]
    pub fn pdo27(&self) -> PDO27_R {
        PDO27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Port Data Output"]
    #[inline(always)]
    pub fn pdo28(&self) -> PDO28_R {
        PDO28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Port Data Output"]
    #[inline(always)]
    pub fn pdo29(&self) -> PDO29_R {
        PDO29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Port Data Output"]
    #[inline(always)]
    pub fn pdo30(&self) -> PDO30_R {
        PDO30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Port Data Output"]
    #[inline(always)]
    pub fn pdo31(&self) -> PDO31_R {
        PDO31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo0(&mut self) -> PDO0_W<0> {
        PDO0_W::new(self)
    }
    #[doc = "Bit 1 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo1(&mut self) -> PDO1_W<1> {
        PDO1_W::new(self)
    }
    #[doc = "Bit 2 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo2(&mut self) -> PDO2_W<2> {
        PDO2_W::new(self)
    }
    #[doc = "Bit 3 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo3(&mut self) -> PDO3_W<3> {
        PDO3_W::new(self)
    }
    #[doc = "Bit 4 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo4(&mut self) -> PDO4_W<4> {
        PDO4_W::new(self)
    }
    #[doc = "Bit 5 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo5(&mut self) -> PDO5_W<5> {
        PDO5_W::new(self)
    }
    #[doc = "Bit 6 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo6(&mut self) -> PDO6_W<6> {
        PDO6_W::new(self)
    }
    #[doc = "Bit 7 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo7(&mut self) -> PDO7_W<7> {
        PDO7_W::new(self)
    }
    #[doc = "Bit 8 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo8(&mut self) -> PDO8_W<8> {
        PDO8_W::new(self)
    }
    #[doc = "Bit 9 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo9(&mut self) -> PDO9_W<9> {
        PDO9_W::new(self)
    }
    #[doc = "Bit 10 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo10(&mut self) -> PDO10_W<10> {
        PDO10_W::new(self)
    }
    #[doc = "Bit 11 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo11(&mut self) -> PDO11_W<11> {
        PDO11_W::new(self)
    }
    #[doc = "Bit 12 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo12(&mut self) -> PDO12_W<12> {
        PDO12_W::new(self)
    }
    #[doc = "Bit 13 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo13(&mut self) -> PDO13_W<13> {
        PDO13_W::new(self)
    }
    #[doc = "Bit 14 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo14(&mut self) -> PDO14_W<14> {
        PDO14_W::new(self)
    }
    #[doc = "Bit 15 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo15(&mut self) -> PDO15_W<15> {
        PDO15_W::new(self)
    }
    #[doc = "Bit 16 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo16(&mut self) -> PDO16_W<16> {
        PDO16_W::new(self)
    }
    #[doc = "Bit 17 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo17(&mut self) -> PDO17_W<17> {
        PDO17_W::new(self)
    }
    #[doc = "Bit 18 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo18(&mut self) -> PDO18_W<18> {
        PDO18_W::new(self)
    }
    #[doc = "Bit 19 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo19(&mut self) -> PDO19_W<19> {
        PDO19_W::new(self)
    }
    #[doc = "Bit 20 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo20(&mut self) -> PDO20_W<20> {
        PDO20_W::new(self)
    }
    #[doc = "Bit 21 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo21(&mut self) -> PDO21_W<21> {
        PDO21_W::new(self)
    }
    #[doc = "Bit 22 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo22(&mut self) -> PDO22_W<22> {
        PDO22_W::new(self)
    }
    #[doc = "Bit 23 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo23(&mut self) -> PDO23_W<23> {
        PDO23_W::new(self)
    }
    #[doc = "Bit 24 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo24(&mut self) -> PDO24_W<24> {
        PDO24_W::new(self)
    }
    #[doc = "Bit 25 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo25(&mut self) -> PDO25_W<25> {
        PDO25_W::new(self)
    }
    #[doc = "Bit 26 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo26(&mut self) -> PDO26_W<26> {
        PDO26_W::new(self)
    }
    #[doc = "Bit 27 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo27(&mut self) -> PDO27_W<27> {
        PDO27_W::new(self)
    }
    #[doc = "Bit 28 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo28(&mut self) -> PDO28_W<28> {
        PDO28_W::new(self)
    }
    #[doc = "Bit 29 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo29(&mut self) -> PDO29_W<29> {
        PDO29_W::new(self)
    }
    #[doc = "Bit 30 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo30(&mut self) -> PDO30_W<30> {
        PDO30_W::new(self)
    }
    #[doc = "Bit 31 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo31(&mut self) -> PDO31_W<31> {
        PDO31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Data Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdor](index.html) module"]
pub struct PDOR_SPEC;
impl crate::RegisterSpec for PDOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdor::R](R) reader structure"]
impl crate::Readable for PDOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdor::W](W) writer structure"]
impl crate::Writable for PDOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDOR to value 0"]
impl crate::Resettable for PDOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
