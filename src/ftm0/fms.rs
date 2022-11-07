#[doc = "Register `FMS` reader"]
pub struct R(crate::R<FMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FMS` writer"]
pub struct W(crate::W<FMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FMS_SPEC>;
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
impl From<crate::W<FMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FAULTF0` reader - Fault Detection Flag 0"]
pub type FAULTF0_R = crate::BitReader<FAULTF0_A>;
#[doc = "Fault Detection Flag 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAULTF0_A {
    #[doc = "0: No fault condition was detected at the fault input."]
    _0 = 0,
    #[doc = "1: A fault condition was detected at the fault input."]
    _1 = 1,
}
impl From<FAULTF0_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTF0_A) -> Self {
        variant as u8 != 0
    }
}
impl FAULTF0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTF0_A {
        match self.bits {
            false => FAULTF0_A::_0,
            true => FAULTF0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTF0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTF0_A::_1
    }
}
#[doc = "Field `FAULTF0` writer - Fault Detection Flag 0"]
pub type FAULTF0_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMS_SPEC, FAULTF0_A, O>;
impl<'a, const O: u8> FAULTF0_W<'a, O> {
    #[doc = "No fault condition was detected at the fault input."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTF0_A::_0)
    }
    #[doc = "A fault condition was detected at the fault input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTF0_A::_1)
    }
}
#[doc = "Field `FAULTF1` reader - Fault Detection Flag 1"]
pub type FAULTF1_R = crate::BitReader<FAULTF1_A>;
#[doc = "Fault Detection Flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAULTF1_A {
    #[doc = "0: No fault condition was detected at the fault input."]
    _0 = 0,
    #[doc = "1: A fault condition was detected at the fault input."]
    _1 = 1,
}
impl From<FAULTF1_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTF1_A) -> Self {
        variant as u8 != 0
    }
}
impl FAULTF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTF1_A {
        match self.bits {
            false => FAULTF1_A::_0,
            true => FAULTF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTF1_A::_1
    }
}
#[doc = "Field `FAULTF1` writer - Fault Detection Flag 1"]
pub type FAULTF1_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMS_SPEC, FAULTF1_A, O>;
impl<'a, const O: u8> FAULTF1_W<'a, O> {
    #[doc = "No fault condition was detected at the fault input."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTF1_A::_0)
    }
    #[doc = "A fault condition was detected at the fault input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTF1_A::_1)
    }
}
#[doc = "Field `FAULTF2` reader - Fault Detection Flag 2"]
pub type FAULTF2_R = crate::BitReader<FAULTF2_A>;
#[doc = "Fault Detection Flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAULTF2_A {
    #[doc = "0: No fault condition was detected at the fault input."]
    _0 = 0,
    #[doc = "1: A fault condition was detected at the fault input."]
    _1 = 1,
}
impl From<FAULTF2_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTF2_A) -> Self {
        variant as u8 != 0
    }
}
impl FAULTF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTF2_A {
        match self.bits {
            false => FAULTF2_A::_0,
            true => FAULTF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTF2_A::_1
    }
}
#[doc = "Field `FAULTF2` writer - Fault Detection Flag 2"]
pub type FAULTF2_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMS_SPEC, FAULTF2_A, O>;
impl<'a, const O: u8> FAULTF2_W<'a, O> {
    #[doc = "No fault condition was detected at the fault input."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTF2_A::_0)
    }
    #[doc = "A fault condition was detected at the fault input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTF2_A::_1)
    }
}
#[doc = "Field `FAULTF3` reader - Fault Detection Flag 3"]
pub type FAULTF3_R = crate::BitReader<FAULTF3_A>;
#[doc = "Fault Detection Flag 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAULTF3_A {
    #[doc = "0: No fault condition was detected at the fault input."]
    _0 = 0,
    #[doc = "1: A fault condition was detected at the fault input."]
    _1 = 1,
}
impl From<FAULTF3_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTF3_A) -> Self {
        variant as u8 != 0
    }
}
impl FAULTF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTF3_A {
        match self.bits {
            false => FAULTF3_A::_0,
            true => FAULTF3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTF3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTF3_A::_1
    }
}
#[doc = "Field `FAULTF3` writer - Fault Detection Flag 3"]
pub type FAULTF3_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMS_SPEC, FAULTF3_A, O>;
impl<'a, const O: u8> FAULTF3_W<'a, O> {
    #[doc = "No fault condition was detected at the fault input."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTF3_A::_0)
    }
    #[doc = "A fault condition was detected at the fault input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTF3_A::_1)
    }
}
#[doc = "Field `FAULTIN` reader - Fault Inputs"]
pub type FAULTIN_R = crate::BitReader<FAULTIN_A>;
#[doc = "Fault Inputs\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAULTIN_A {
    #[doc = "0: The logic OR of the enabled fault inputs is 0."]
    _0 = 0,
    #[doc = "1: The logic OR of the enabled fault inputs is 1."]
    _1 = 1,
}
impl From<FAULTIN_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTIN_A) -> Self {
        variant as u8 != 0
    }
}
impl FAULTIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTIN_A {
        match self.bits {
            false => FAULTIN_A::_0,
            true => FAULTIN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTIN_A::_1
    }
}
#[doc = "Field `WPEN` reader - Write Protection Enable"]
pub type WPEN_R = crate::BitReader<WPEN_A>;
#[doc = "Write Protection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPEN_A {
    #[doc = "0: Write protection is disabled. Write protected bits can be written."]
    _0 = 0,
    #[doc = "1: Write protection is enabled. Write protected bits cannot be written."]
    _1 = 1,
}
impl From<WPEN_A> for bool {
    #[inline(always)]
    fn from(variant: WPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPEN_A {
        match self.bits {
            false => WPEN_A::_0,
            true => WPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WPEN_A::_1
    }
}
#[doc = "Field `WPEN` writer - Write Protection Enable"]
pub type WPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMS_SPEC, WPEN_A, O>;
impl<'a, const O: u8> WPEN_W<'a, O> {
    #[doc = "Write protection is disabled. Write protected bits can be written."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WPEN_A::_0)
    }
    #[doc = "Write protection is enabled. Write protected bits cannot be written."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WPEN_A::_1)
    }
}
#[doc = "Field `FAULTF` reader - Fault Detection Flag"]
pub type FAULTF_R = crate::BitReader<FAULTF_A>;
#[doc = "Fault Detection Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAULTF_A {
    #[doc = "0: No fault condition was detected."]
    _0 = 0,
    #[doc = "1: A fault condition was detected."]
    _1 = 1,
}
impl From<FAULTF_A> for bool {
    #[inline(always)]
    fn from(variant: FAULTF_A) -> Self {
        variant as u8 != 0
    }
}
impl FAULTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FAULTF_A {
        match self.bits {
            false => FAULTF_A::_0,
            true => FAULTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FAULTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FAULTF_A::_1
    }
}
#[doc = "Field `FAULTF` writer - Fault Detection Flag"]
pub type FAULTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, FMS_SPEC, FAULTF_A, O>;
impl<'a, const O: u8> FAULTF_W<'a, O> {
    #[doc = "No fault condition was detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FAULTF_A::_0)
    }
    #[doc = "A fault condition was detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FAULTF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Fault Detection Flag 0"]
    #[inline(always)]
    pub fn faultf0(&self) -> FAULTF0_R {
        FAULTF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault Detection Flag 1"]
    #[inline(always)]
    pub fn faultf1(&self) -> FAULTF1_R {
        FAULTF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault Detection Flag 2"]
    #[inline(always)]
    pub fn faultf2(&self) -> FAULTF2_R {
        FAULTF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault Detection Flag 3"]
    #[inline(always)]
    pub fn faultf3(&self) -> FAULTF3_R {
        FAULTF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Fault Inputs"]
    #[inline(always)]
    pub fn faultin(&self) -> FAULTIN_R {
        FAULTIN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write Protection Enable"]
    #[inline(always)]
    pub fn wpen(&self) -> WPEN_R {
        WPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fault Detection Flag"]
    #[inline(always)]
    pub fn faultf(&self) -> FAULTF_R {
        FAULTF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault Detection Flag 0"]
    #[inline(always)]
    #[must_use]
    pub fn faultf0(&mut self) -> FAULTF0_W<0> {
        FAULTF0_W::new(self)
    }
    #[doc = "Bit 1 - Fault Detection Flag 1"]
    #[inline(always)]
    #[must_use]
    pub fn faultf1(&mut self) -> FAULTF1_W<1> {
        FAULTF1_W::new(self)
    }
    #[doc = "Bit 2 - Fault Detection Flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn faultf2(&mut self) -> FAULTF2_W<2> {
        FAULTF2_W::new(self)
    }
    #[doc = "Bit 3 - Fault Detection Flag 3"]
    #[inline(always)]
    #[must_use]
    pub fn faultf3(&mut self) -> FAULTF3_W<3> {
        FAULTF3_W::new(self)
    }
    #[doc = "Bit 6 - Write Protection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wpen(&mut self) -> WPEN_W<6> {
        WPEN_W::new(self)
    }
    #[doc = "Bit 7 - Fault Detection Flag"]
    #[inline(always)]
    #[must_use]
    pub fn faultf(&mut self) -> FAULTF_W<7> {
        FAULTF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fault Mode Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fms](index.html) module"]
pub struct FMS_SPEC;
impl crate::RegisterSpec for FMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fms::R](R) reader structure"]
impl crate::Readable for FMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fms::W](W) writer structure"]
impl crate::Writable for FMS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FMS to value 0"]
impl crate::Resettable for FMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
