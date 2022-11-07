#[doc = "Register `TOKEN` reader"]
pub struct R(crate::R<TOKEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TOKEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TOKEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TOKEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TOKEN` writer"]
pub struct W(crate::W<TOKEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TOKEN_SPEC>;
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
impl From<crate::W<TOKEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TOKEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOKENENDPT` reader - Holds the Endpoint address for the token command"]
pub type TOKENENDPT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TOKENENDPT` writer - Holds the Endpoint address for the token command"]
pub type TOKENENDPT_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TOKEN_SPEC, u8, u8, 4, O>;
#[doc = "Field `TOKENPID` reader - Contains the token type executed by the USB module."]
pub type TOKENPID_R = crate::FieldReader<u8, TOKENPID_A>;
#[doc = "Contains the token type executed by the USB module.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TOKENPID_A {
    #[doc = "1: OUT Token. USB Module performs an OUT (TX) transaction."]
    _0001 = 1,
    #[doc = "9: IN Token. USB Module performs an In (RX) transaction."]
    _1001 = 9,
    #[doc = "13: SETUP Token. USB Module performs a SETUP (TX) transaction"]
    _1101 = 13,
}
impl From<TOKENPID_A> for u8 {
    #[inline(always)]
    fn from(variant: TOKENPID_A) -> Self {
        variant as _
    }
}
impl TOKENPID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TOKENPID_A> {
        match self.bits {
            1 => Some(TOKENPID_A::_0001),
            9 => Some(TOKENPID_A::_1001),
            13 => Some(TOKENPID_A::_1101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == TOKENPID_A::_0001
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == TOKENPID_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1101`"]
    #[inline(always)]
    pub fn is_1101(&self) -> bool {
        *self == TOKENPID_A::_1101
    }
}
#[doc = "Field `TOKENPID` writer - Contains the token type executed by the USB module."]
pub type TOKENPID_W<'a, const O: u8> = crate::FieldWriter<'a, u8, TOKEN_SPEC, u8, TOKENPID_A, 4, O>;
impl<'a, const O: u8> TOKENPID_W<'a, O> {
    #[doc = "OUT Token. USB Module performs an OUT (TX) transaction."]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(TOKENPID_A::_0001)
    }
    #[doc = "IN Token. USB Module performs an In (RX) transaction."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(TOKENPID_A::_1001)
    }
    #[doc = "SETUP Token. USB Module performs a SETUP (TX) transaction"]
    #[inline(always)]
    pub fn _1101(self) -> &'a mut W {
        self.variant(TOKENPID_A::_1101)
    }
}
impl R {
    #[doc = "Bits 0:3 - Holds the Endpoint address for the token command"]
    #[inline(always)]
    pub fn tokenendpt(&self) -> TOKENENDPT_R {
        TOKENENDPT_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - Contains the token type executed by the USB module."]
    #[inline(always)]
    pub fn tokenpid(&self) -> TOKENPID_R {
        TOKENPID_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Holds the Endpoint address for the token command"]
    #[inline(always)]
    #[must_use]
    pub fn tokenendpt(&mut self) -> TOKENENDPT_W<0> {
        TOKENENDPT_W::new(self)
    }
    #[doc = "Bits 4:7 - Contains the token type executed by the USB module."]
    #[inline(always)]
    #[must_use]
    pub fn tokenpid(&mut self) -> TOKENPID_W<4> {
        TOKENPID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Token register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [token](index.html) module"]
pub struct TOKEN_SPEC;
impl crate::RegisterSpec for TOKEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [token::R](R) reader structure"]
impl crate::Readable for TOKEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [token::W](W) writer structure"]
impl crate::Writable for TOKEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TOKEN to value 0"]
impl crate::Resettable for TOKEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
