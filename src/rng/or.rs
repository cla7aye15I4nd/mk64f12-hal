#[doc = "Register `OR` reader"]
pub struct R(crate::R<OR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RANDOUT` reader - Random Output"]
pub type RANDOUT_R = crate::FieldReader<u32, RANDOUT_A>;
#[doc = "Random Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum RANDOUT_A {
    #[doc = "0: Invalid data (if you read this field when it is 0 and SR\\[OREG_LVL\\]
is 0, RNGA then writes 1 to SR\\[ERRI\\], SR\\[ORU\\], and SR\\[LRS\\]; when the error interrupt is not masked (CR\\[INTM\\]=0), RNGA also asserts an error interrupt request to the interrupt controller)."]
    _0 = 0,
}
impl From<RANDOUT_A> for u32 {
    #[inline(always)]
    fn from(variant: RANDOUT_A) -> Self {
        variant as _
    }
}
impl RANDOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RANDOUT_A> {
        match self.bits {
            0 => Some(RANDOUT_A::_0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RANDOUT_A::_0
    }
}
impl R {
    #[doc = "Bits 0:31 - Random Output"]
    #[inline(always)]
    pub fn randout(&self) -> RANDOUT_R {
        RANDOUT_R::new(self.bits)
    }
}
#[doc = "RNGA Output Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [or](index.html) module"]
pub struct OR_SPEC;
impl crate::RegisterSpec for OR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [or::R](R) reader structure"]
impl crate::Readable for OR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for OR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
