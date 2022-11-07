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
#[doc = "Field `AD` reader - Slave Address"]
pub type AD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AD` writer - Slave Address"]
pub type AD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, C2_SPEC, u8, u8, 3, O>;
#[doc = "Field `RMEN` reader - Range Address Matching Enable"]
pub type RMEN_R = crate::BitReader<RMEN_A>;
#[doc = "Range Address Matching Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RMEN_A {
    #[doc = "0: Range mode disabled. No address matching occurs for an address within the range of values of the A1 and RA registers."]
    _0 = 0,
    #[doc = "1: Range mode enabled. Address matching occurs when a slave receives an address within the range of values of the A1 and RA registers."]
    _1 = 1,
}
impl From<RMEN_A> for bool {
    #[inline(always)]
    fn from(variant: RMEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RMEN_A {
        match self.bits {
            false => RMEN_A::_0,
            true => RMEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RMEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RMEN_A::_1
    }
}
#[doc = "Field `RMEN` writer - Range Address Matching Enable"]
pub type RMEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, C2_SPEC, RMEN_A, O>;
impl<'a, const O: u8> RMEN_W<'a, O> {
    #[doc = "Range mode disabled. No address matching occurs for an address within the range of values of the A1 and RA registers."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RMEN_A::_0)
    }
    #[doc = "Range mode enabled. Address matching occurs when a slave receives an address within the range of values of the A1 and RA registers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RMEN_A::_1)
    }
}
#[doc = "Field `SBRC` reader - Slave Baud Rate Control"]
pub type SBRC_R = crate::BitReader<SBRC_A>;
#[doc = "Slave Baud Rate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBRC_A {
    #[doc = "0: The slave baud rate follows the master baud rate and clock stretching may occur"]
    _0 = 0,
    #[doc = "1: Slave baud rate is independent of the master baud rate"]
    _1 = 1,
}
impl From<SBRC_A> for bool {
    #[inline(always)]
    fn from(variant: SBRC_A) -> Self {
        variant as u8 != 0
    }
}
impl SBRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBRC_A {
        match self.bits {
            false => SBRC_A::_0,
            true => SBRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBRC_A::_1
    }
}
#[doc = "Field `SBRC` writer - Slave Baud Rate Control"]
pub type SBRC_W<'a, const O: u8> = crate::BitWriter<'a, u8, C2_SPEC, SBRC_A, O>;
impl<'a, const O: u8> SBRC_W<'a, O> {
    #[doc = "The slave baud rate follows the master baud rate and clock stretching may occur"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SBRC_A::_0)
    }
    #[doc = "Slave baud rate is independent of the master baud rate"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SBRC_A::_1)
    }
}
#[doc = "Field `HDRS` reader - High Drive Select"]
pub type HDRS_R = crate::BitReader<HDRS_A>;
#[doc = "High Drive Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDRS_A {
    #[doc = "0: Normal drive mode"]
    _0 = 0,
    #[doc = "1: High drive mode"]
    _1 = 1,
}
impl From<HDRS_A> for bool {
    #[inline(always)]
    fn from(variant: HDRS_A) -> Self {
        variant as u8 != 0
    }
}
impl HDRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDRS_A {
        match self.bits {
            false => HDRS_A::_0,
            true => HDRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HDRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HDRS_A::_1
    }
}
#[doc = "Field `HDRS` writer - High Drive Select"]
pub type HDRS_W<'a, const O: u8> = crate::BitWriter<'a, u8, C2_SPEC, HDRS_A, O>;
impl<'a, const O: u8> HDRS_W<'a, O> {
    #[doc = "Normal drive mode"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HDRS_A::_0)
    }
    #[doc = "High drive mode"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HDRS_A::_1)
    }
}
#[doc = "Field `ADEXT` reader - Address Extension"]
pub type ADEXT_R = crate::BitReader<ADEXT_A>;
#[doc = "Address Extension\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADEXT_A {
    #[doc = "0: 7-bit address scheme"]
    _0 = 0,
    #[doc = "1: 10-bit address scheme"]
    _1 = 1,
}
impl From<ADEXT_A> for bool {
    #[inline(always)]
    fn from(variant: ADEXT_A) -> Self {
        variant as u8 != 0
    }
}
impl ADEXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADEXT_A {
        match self.bits {
            false => ADEXT_A::_0,
            true => ADEXT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADEXT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADEXT_A::_1
    }
}
#[doc = "Field `ADEXT` writer - Address Extension"]
pub type ADEXT_W<'a, const O: u8> = crate::BitWriter<'a, u8, C2_SPEC, ADEXT_A, O>;
impl<'a, const O: u8> ADEXT_W<'a, O> {
    #[doc = "7-bit address scheme"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADEXT_A::_0)
    }
    #[doc = "10-bit address scheme"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADEXT_A::_1)
    }
}
#[doc = "Field `GCAEN` reader - General Call Address Enable"]
pub type GCAEN_R = crate::BitReader<GCAEN_A>;
#[doc = "General Call Address Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCAEN_A {
    #[doc = "0: Disabled"]
    _0 = 0,
    #[doc = "1: Enabled"]
    _1 = 1,
}
impl From<GCAEN_A> for bool {
    #[inline(always)]
    fn from(variant: GCAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GCAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCAEN_A {
        match self.bits {
            false => GCAEN_A::_0,
            true => GCAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GCAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GCAEN_A::_1
    }
}
#[doc = "Field `GCAEN` writer - General Call Address Enable"]
pub type GCAEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, C2_SPEC, GCAEN_A, O>;
impl<'a, const O: u8> GCAEN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GCAEN_A::_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GCAEN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Slave Address"]
    #[inline(always)]
    pub fn ad(&self) -> AD_R {
        AD_R::new(self.bits & 7)
    }
    #[doc = "Bit 3 - Range Address Matching Enable"]
    #[inline(always)]
    pub fn rmen(&self) -> RMEN_R {
        RMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Baud Rate Control"]
    #[inline(always)]
    pub fn sbrc(&self) -> SBRC_R {
        SBRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - High Drive Select"]
    #[inline(always)]
    pub fn hdrs(&self) -> HDRS_R {
        HDRS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Address Extension"]
    #[inline(always)]
    pub fn adext(&self) -> ADEXT_R {
        ADEXT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - General Call Address Enable"]
    #[inline(always)]
    pub fn gcaen(&self) -> GCAEN_R {
        GCAEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave Address"]
    #[inline(always)]
    #[must_use]
    pub fn ad(&mut self) -> AD_W<0> {
        AD_W::new(self)
    }
    #[doc = "Bit 3 - Range Address Matching Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rmen(&mut self) -> RMEN_W<3> {
        RMEN_W::new(self)
    }
    #[doc = "Bit 4 - Slave Baud Rate Control"]
    #[inline(always)]
    #[must_use]
    pub fn sbrc(&mut self) -> SBRC_W<4> {
        SBRC_W::new(self)
    }
    #[doc = "Bit 5 - High Drive Select"]
    #[inline(always)]
    #[must_use]
    pub fn hdrs(&mut self) -> HDRS_W<5> {
        HDRS_W::new(self)
    }
    #[doc = "Bit 6 - Address Extension"]
    #[inline(always)]
    #[must_use]
    pub fn adext(&mut self) -> ADEXT_W<6> {
        ADEXT_W::new(self)
    }
    #[doc = "Bit 7 - General Call Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcaen(&mut self) -> GCAEN_W<7> {
        GCAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2](index.html) module"]
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
#[doc = "`reset()` method sets C2 to value 0"]
impl crate::Resettable for C2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
