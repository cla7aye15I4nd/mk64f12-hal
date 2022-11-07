#[doc = "Register `PLAMC` reader"]
pub struct R(crate::R<PLAMC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLAMC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLAMC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLAMC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AMC` reader - Each bit in the AMC field indicates whether there is a corresponding connection to the AXBS master input port."]
pub type AMC_R = crate::FieldReader<u8, AMC_A>;
#[doc = "Each bit in the AMC field indicates whether there is a corresponding connection to the AXBS master input port.\n\nValue on reset: 55"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AMC_A {
    #[doc = "0: A bus master connection to AXBS input port n is absent"]
    _0 = 0,
    #[doc = "1: A bus master connection to AXBS input port n is present"]
    _1 = 1,
}
impl From<AMC_A> for u8 {
    #[inline(always)]
    fn from(variant: AMC_A) -> Self {
        variant as _
    }
}
impl AMC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<AMC_A> {
        match self.bits {
            0 => Some(AMC_A::_0),
            1 => Some(AMC_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AMC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AMC_A::_1
    }
}
impl R {
    #[doc = "Bits 0:7 - Each bit in the AMC field indicates whether there is a corresponding connection to the AXBS master input port."]
    #[inline(always)]
    pub fn amc(&self) -> AMC_R {
        AMC_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Crossbar Switch (AXBS) Master Configuration\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [plamc](index.html) module"]
pub struct PLAMC_SPEC;
impl crate::RegisterSpec for PLAMC_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [plamc::R](R) reader structure"]
impl crate::Readable for PLAMC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PLAMC to value 0x37"]
impl crate::Resettable for PLAMC_SPEC {
    const RESET_VALUE: Self::Ux = 0x37;
}
