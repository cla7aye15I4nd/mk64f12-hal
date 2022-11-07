#[doc = "Register `ADMAES` reader"]
pub struct R(crate::R<ADMAES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADMAES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADMAES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADMAES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ADMAES` reader - ADMA Error State (When ADMA Error Is Occurred.)"]
pub type ADMAES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADMALME` reader - ADMA Length Mismatch Error"]
pub type ADMALME_R = crate::BitReader<ADMALME_A>;
#[doc = "ADMA Length Mismatch Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADMALME_A {
    #[doc = "0: No error."]
    _0 = 0,
    #[doc = "1: Error."]
    _1 = 1,
}
impl From<ADMALME_A> for bool {
    #[inline(always)]
    fn from(variant: ADMALME_A) -> Self {
        variant as u8 != 0
    }
}
impl ADMALME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADMALME_A {
        match self.bits {
            false => ADMALME_A::_0,
            true => ADMALME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADMALME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADMALME_A::_1
    }
}
#[doc = "Field `ADMADCE` reader - ADMA Descriptor Error"]
pub type ADMADCE_R = crate::BitReader<ADMADCE_A>;
#[doc = "ADMA Descriptor Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADMADCE_A {
    #[doc = "0: No error."]
    _0 = 0,
    #[doc = "1: Error."]
    _1 = 1,
}
impl From<ADMADCE_A> for bool {
    #[inline(always)]
    fn from(variant: ADMADCE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADMADCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADMADCE_A {
        match self.bits {
            false => ADMADCE_A::_0,
            true => ADMADCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADMADCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADMADCE_A::_1
    }
}
impl R {
    #[doc = "Bits 0:1 - ADMA Error State (When ADMA Error Is Occurred.)"]
    #[inline(always)]
    pub fn admaes(&self) -> ADMAES_R {
        ADMAES_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - ADMA Length Mismatch Error"]
    #[inline(always)]
    pub fn admalme(&self) -> ADMALME_R {
        ADMALME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADMA Descriptor Error"]
    #[inline(always)]
    pub fn admadce(&self) -> ADMADCE_R {
        ADMADCE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "ADMA Error Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [admaes](index.html) module"]
pub struct ADMAES_SPEC;
impl crate::RegisterSpec for ADMAES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [admaes::R](R) reader structure"]
impl crate::Readable for ADMAES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADMAES to value 0"]
impl crate::Resettable for ADMAES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
