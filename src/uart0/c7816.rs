#[doc = "Register `C7816` reader"]
pub struct R(crate::R<C7816_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C7816_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C7816_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C7816_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C7816` writer"]
pub struct W(crate::W<C7816_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C7816_SPEC>;
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
impl From<crate::W<C7816_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C7816_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISO_7816E` reader - ISO-7816 Functionality Enabled"]
pub type ISO_7816E_R = crate::BitReader<ISO_7816E_A>;
#[doc = "ISO-7816 Functionality Enabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ISO_7816E_A {
    #[doc = "0: ISO-7816 functionality is turned off/not enabled."]
    _0 = 0,
    #[doc = "1: ISO-7816 functionality is turned on/enabled."]
    _1 = 1,
}
impl From<ISO_7816E_A> for bool {
    #[inline(always)]
    fn from(variant: ISO_7816E_A) -> Self {
        variant as u8 != 0
    }
}
impl ISO_7816E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISO_7816E_A {
        match self.bits {
            false => ISO_7816E_A::_0,
            true => ISO_7816E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISO_7816E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISO_7816E_A::_1
    }
}
#[doc = "Field `ISO_7816E` writer - ISO-7816 Functionality Enabled"]
pub type ISO_7816E_W<'a, const O: u8> = crate::BitWriter<'a, u8, C7816_SPEC, ISO_7816E_A, O>;
impl<'a, const O: u8> ISO_7816E_W<'a, O> {
    #[doc = "ISO-7816 functionality is turned off/not enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISO_7816E_A::_0)
    }
    #[doc = "ISO-7816 functionality is turned on/enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISO_7816E_A::_1)
    }
}
#[doc = "Field `TTYPE` reader - Transfer Type"]
pub type TTYPE_R = crate::BitReader<TTYPE_A>;
#[doc = "Transfer Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TTYPE_A {
    #[doc = "0: T = 0 per the ISO-7816 specification."]
    _0 = 0,
    #[doc = "1: T = 1 per the ISO-7816 specification."]
    _1 = 1,
}
impl From<TTYPE_A> for bool {
    #[inline(always)]
    fn from(variant: TTYPE_A) -> Self {
        variant as u8 != 0
    }
}
impl TTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TTYPE_A {
        match self.bits {
            false => TTYPE_A::_0,
            true => TTYPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TTYPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TTYPE_A::_1
    }
}
#[doc = "Field `TTYPE` writer - Transfer Type"]
pub type TTYPE_W<'a, const O: u8> = crate::BitWriter<'a, u8, C7816_SPEC, TTYPE_A, O>;
impl<'a, const O: u8> TTYPE_W<'a, O> {
    #[doc = "T = 0 per the ISO-7816 specification."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TTYPE_A::_0)
    }
    #[doc = "T = 1 per the ISO-7816 specification."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TTYPE_A::_1)
    }
}
#[doc = "Field `INIT` reader - Detect Initial Character"]
pub type INIT_R = crate::BitReader<INIT_A>;
#[doc = "Detect Initial Character\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INIT_A {
    #[doc = "0: Normal operating mode. Receiver does not seek to identify initial character."]
    _0 = 0,
    #[doc = "1: Receiver searches for initial character."]
    _1 = 1,
}
impl From<INIT_A> for bool {
    #[inline(always)]
    fn from(variant: INIT_A) -> Self {
        variant as u8 != 0
    }
}
impl INIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INIT_A {
        match self.bits {
            false => INIT_A::_0,
            true => INIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INIT_A::_1
    }
}
#[doc = "Field `INIT` writer - Detect Initial Character"]
pub type INIT_W<'a, const O: u8> = crate::BitWriter<'a, u8, C7816_SPEC, INIT_A, O>;
impl<'a, const O: u8> INIT_W<'a, O> {
    #[doc = "Normal operating mode. Receiver does not seek to identify initial character."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INIT_A::_0)
    }
    #[doc = "Receiver searches for initial character."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INIT_A::_1)
    }
}
#[doc = "Field `ANACK` reader - Generate NACK on Error"]
pub type ANACK_R = crate::BitReader<ANACK_A>;
#[doc = "Generate NACK on Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANACK_A {
    #[doc = "0: No NACK is automatically generated."]
    _0 = 0,
    #[doc = "1: A NACK is automatically generated if a parity error is detected or if an invalid initial character is detected."]
    _1 = 1,
}
impl From<ANACK_A> for bool {
    #[inline(always)]
    fn from(variant: ANACK_A) -> Self {
        variant as u8 != 0
    }
}
impl ANACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANACK_A {
        match self.bits {
            false => ANACK_A::_0,
            true => ANACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ANACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ANACK_A::_1
    }
}
#[doc = "Field `ANACK` writer - Generate NACK on Error"]
pub type ANACK_W<'a, const O: u8> = crate::BitWriter<'a, u8, C7816_SPEC, ANACK_A, O>;
impl<'a, const O: u8> ANACK_W<'a, O> {
    #[doc = "No NACK is automatically generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ANACK_A::_0)
    }
    #[doc = "A NACK is automatically generated if a parity error is detected or if an invalid initial character is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ANACK_A::_1)
    }
}
#[doc = "Field `ONACK` reader - Generate NACK on Overflow"]
pub type ONACK_R = crate::BitReader<ONACK_A>;
#[doc = "Generate NACK on Overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONACK_A {
    #[doc = "0: The received data does not generate a NACK when the receipt of the data results in an overflow event."]
    _0 = 0,
    #[doc = "1: If the receiver buffer overflows, a NACK is automatically sent on a received character."]
    _1 = 1,
}
impl From<ONACK_A> for bool {
    #[inline(always)]
    fn from(variant: ONACK_A) -> Self {
        variant as u8 != 0
    }
}
impl ONACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONACK_A {
        match self.bits {
            false => ONACK_A::_0,
            true => ONACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ONACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ONACK_A::_1
    }
}
#[doc = "Field `ONACK` writer - Generate NACK on Overflow"]
pub type ONACK_W<'a, const O: u8> = crate::BitWriter<'a, u8, C7816_SPEC, ONACK_A, O>;
impl<'a, const O: u8> ONACK_W<'a, O> {
    #[doc = "The received data does not generate a NACK when the receipt of the data results in an overflow event."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ONACK_A::_0)
    }
    #[doc = "If the receiver buffer overflows, a NACK is automatically sent on a received character."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ONACK_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - ISO-7816 Functionality Enabled"]
    #[inline(always)]
    pub fn iso_7816e(&self) -> ISO_7816E_R {
        ISO_7816E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer Type"]
    #[inline(always)]
    pub fn ttype(&self) -> TTYPE_R {
        TTYPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Detect Initial Character"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Generate NACK on Error"]
    #[inline(always)]
    pub fn anack(&self) -> ANACK_R {
        ANACK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Generate NACK on Overflow"]
    #[inline(always)]
    pub fn onack(&self) -> ONACK_R {
        ONACK_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ISO-7816 Functionality Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn iso_7816e(&mut self) -> ISO_7816E_W<0> {
        ISO_7816E_W::new(self)
    }
    #[doc = "Bit 1 - Transfer Type"]
    #[inline(always)]
    #[must_use]
    pub fn ttype(&mut self) -> TTYPE_W<1> {
        TTYPE_W::new(self)
    }
    #[doc = "Bit 2 - Detect Initial Character"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<2> {
        INIT_W::new(self)
    }
    #[doc = "Bit 3 - Generate NACK on Error"]
    #[inline(always)]
    #[must_use]
    pub fn anack(&mut self) -> ANACK_W<3> {
        ANACK_W::new(self)
    }
    #[doc = "Bit 4 - Generate NACK on Overflow"]
    #[inline(always)]
    #[must_use]
    pub fn onack(&mut self) -> ONACK_W<4> {
        ONACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART 7816 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c7816](index.html) module"]
pub struct C7816_SPEC;
impl crate::RegisterSpec for C7816_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c7816::R](R) reader structure"]
impl crate::Readable for C7816_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c7816::W](W) writer structure"]
impl crate::Writable for C7816_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C7816 to value 0"]
impl crate::Resettable for C7816_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
