#[doc = "Register `STAT` reader"]
pub struct R(crate::R<STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ODD` reader - This bit is set if the last buffer descriptor updated was in the odd bank of the BDT."]
pub type ODD_R = crate::BitReader<bool>;
#[doc = "Field `TX` reader - Transmit Indicator"]
pub type TX_R = crate::BitReader<TX_A>;
#[doc = "Transmit Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_A {
    #[doc = "0: The most recent transaction was a receive operation."]
    _0 = 0,
    #[doc = "1: The most recent transaction was a transmit operation."]
    _1 = 1,
}
impl From<TX_A> for bool {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            false => TX_A::_0,
            true => TX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TX_A::_1
    }
}
#[doc = "Field `ENDP` reader - This four-bit field encodes the endpoint address that received or transmitted the previous token"]
pub type ENDP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 2 - This bit is set if the last buffer descriptor updated was in the odd bank of the BDT."]
    #[inline(always)]
    pub fn odd(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Indicator"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - This four-bit field encodes the endpoint address that received or transmitted the previous token"]
    #[inline(always)]
    pub fn endp(&self) -> ENDP_R {
        ENDP_R::new((self.bits >> 4) & 0x0f)
    }
}
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat](index.html) module"]
pub struct STAT_SPEC;
impl crate::RegisterSpec for STAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [stat::R](R) reader structure"]
impl crate::Readable for STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
