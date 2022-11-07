#[doc = "Register `ED` reader"]
pub struct R(crate::R<ED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PARITYE` reader - The current received dataword contained in D and C3\\[R8\\]
was received with a parity error."]
pub type PARITYE_R = crate::BitReader<PARITYE_A>;
#[doc = "The current received dataword contained in D and C3\\[R8\\]
was received with a parity error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PARITYE_A {
    #[doc = "0: The dataword was received without a parity error."]
    _0 = 0,
    #[doc = "1: The dataword was received with a parity error."]
    _1 = 1,
}
impl From<PARITYE_A> for bool {
    #[inline(always)]
    fn from(variant: PARITYE_A) -> Self {
        variant as u8 != 0
    }
}
impl PARITYE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARITYE_A {
        match self.bits {
            false => PARITYE_A::_0,
            true => PARITYE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PARITYE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PARITYE_A::_1
    }
}
#[doc = "Field `NOISY` reader - The current received dataword contained in D and C3\\[R8\\]
was received with noise."]
pub type NOISY_R = crate::BitReader<NOISY_A>;
#[doc = "The current received dataword contained in D and C3\\[R8\\]
was received with noise.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOISY_A {
    #[doc = "0: The dataword was received without noise."]
    _0 = 0,
    #[doc = "1: The data was received with noise."]
    _1 = 1,
}
impl From<NOISY_A> for bool {
    #[inline(always)]
    fn from(variant: NOISY_A) -> Self {
        variant as u8 != 0
    }
}
impl NOISY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOISY_A {
        match self.bits {
            false => NOISY_A::_0,
            true => NOISY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NOISY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NOISY_A::_1
    }
}
impl R {
    #[doc = "Bit 6 - The current received dataword contained in D and C3\\[R8\\]
was received with a parity error."]
    #[inline(always)]
    pub fn paritye(&self) -> PARITYE_R {
        PARITYE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The current received dataword contained in D and C3\\[R8\\]
was received with noise."]
    #[inline(always)]
    pub fn noisy(&self) -> NOISY_R {
        NOISY_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "UART Extended Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ed](index.html) module"]
pub struct ED_SPEC;
impl crate::RegisterSpec for ED_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ed::R](R) reader structure"]
impl crate::Readable for ED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ED to value 0"]
impl crate::Resettable for ED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
