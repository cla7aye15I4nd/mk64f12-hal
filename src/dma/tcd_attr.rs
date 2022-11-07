#[doc = "Register `TCD%s_ATTR` reader"]
pub struct R(crate::R<TCD_ATTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCD_ATTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCD_ATTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCD_ATTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD%s_ATTR` writer"]
pub struct W(crate::W<TCD_ATTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCD_ATTR_SPEC>;
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
impl From<crate::W<TCD_ATTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCD_ATTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSIZE` reader - Destination Data Transfer Size"]
pub type DSIZE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DSIZE` writer - Destination Data Transfer Size"]
pub type DSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TCD_ATTR_SPEC, u8, u8, 3, O>;
#[doc = "Field `DMOD` reader - Destination Address Modulo"]
pub type DMOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMOD` writer - Destination Address Modulo"]
pub type DMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TCD_ATTR_SPEC, u8, u8, 5, O>;
#[doc = "Field `SSIZE` reader - Source data transfer size"]
pub type SSIZE_R = crate::FieldReader<u8, SSIZE_A>;
#[doc = "Source data transfer size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SSIZE_A {
    #[doc = "0: 8-bit"]
    _000 = 0,
    #[doc = "1: 16-bit"]
    _001 = 1,
    #[doc = "2: 32-bit"]
    _010 = 2,
    #[doc = "4: 16-byte"]
    _100 = 4,
    #[doc = "5: 32-byte"]
    _101 = 5,
}
impl From<SSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: SSIZE_A) -> Self {
        variant as _
    }
}
impl SSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SSIZE_A> {
        match self.bits {
            0 => Some(SSIZE_A::_000),
            1 => Some(SSIZE_A::_001),
            2 => Some(SSIZE_A::_010),
            4 => Some(SSIZE_A::_100),
            5 => Some(SSIZE_A::_101),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SSIZE_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SSIZE_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SSIZE_A::_010
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SSIZE_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SSIZE_A::_101
    }
}
#[doc = "Field `SSIZE` writer - Source data transfer size"]
pub type SSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TCD_ATTR_SPEC, u8, SSIZE_A, 3, O>;
impl<'a, const O: u8> SSIZE_W<'a, O> {
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(SSIZE_A::_000)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(SSIZE_A::_001)
    }
    #[doc = "32-bit"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(SSIZE_A::_010)
    }
    #[doc = "16-byte"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(SSIZE_A::_100)
    }
    #[doc = "32-byte"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(SSIZE_A::_101)
    }
}
#[doc = "Field `SMOD` reader - Source Address Modulo."]
pub type SMOD_R = crate::FieldReader<u8, SMOD_A>;
#[doc = "Source Address Modulo.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMOD_A {
    #[doc = "0: Source address modulo feature is disabled"]
    _0 = 0,
}
impl From<SMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: SMOD_A) -> Self {
        variant as _
    }
}
impl SMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SMOD_A> {
        match self.bits {
            0 => Some(SMOD_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SMOD_A::_0
    }
}
#[doc = "Field `SMOD` writer - Source Address Modulo."]
pub type SMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TCD_ATTR_SPEC, u8, SMOD_A, 5, O>;
impl<'a, const O: u8> SMOD_W<'a, O> {
    #[doc = "Source address modulo feature is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SMOD_A::_0)
    }
}
impl R {
    #[doc = "Bits 0:2 - Destination Data Transfer Size"]
    #[inline(always)]
    pub fn dsize(&self) -> DSIZE_R {
        DSIZE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - Destination Address Modulo"]
    #[inline(always)]
    pub fn dmod(&self) -> DMOD_R {
        DMOD_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Source data transfer size"]
    #[inline(always)]
    pub fn ssize(&self) -> SSIZE_R {
        SSIZE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - Source Address Modulo."]
    #[inline(always)]
    pub fn smod(&self) -> SMOD_R {
        SMOD_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Destination Data Transfer Size"]
    #[inline(always)]
    #[must_use]
    pub fn dsize(&mut self) -> DSIZE_W<0> {
        DSIZE_W::new(self)
    }
    #[doc = "Bits 3:7 - Destination Address Modulo"]
    #[inline(always)]
    #[must_use]
    pub fn dmod(&mut self) -> DMOD_W<3> {
        DMOD_W::new(self)
    }
    #[doc = "Bits 8:10 - Source data transfer size"]
    #[inline(always)]
    #[must_use]
    pub fn ssize(&mut self) -> SSIZE_W<8> {
        SSIZE_W::new(self)
    }
    #[doc = "Bits 11:15 - Source Address Modulo."]
    #[inline(always)]
    #[must_use]
    pub fn smod(&mut self) -> SMOD_W<11> {
        SMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Transfer Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_attr](index.html) module"]
pub struct TCD_ATTR_SPEC;
impl crate::RegisterSpec for TCD_ATTR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tcd_attr::R](R) reader structure"]
impl crate::Readable for TCD_ATTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcd_attr::W](W) writer structure"]
impl crate::Writable for TCD_ATTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCD%s_ATTR to value 0"]
impl crate::Resettable for TCD_ATTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
