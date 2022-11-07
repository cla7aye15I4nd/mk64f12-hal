#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIF` reader - Time Invalid Flag"]
pub type TIF_R = crate::BitReader<TIF_A>;
#[doc = "Time Invalid Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIF_A {
    #[doc = "0: Time is valid."]
    _0 = 0,
    #[doc = "1: Time is invalid and time counter is read as zero."]
    _1 = 1,
}
impl From<TIF_A> for bool {
    #[inline(always)]
    fn from(variant: TIF_A) -> Self {
        variant as u8 != 0
    }
}
impl TIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIF_A {
        match self.bits {
            false => TIF_A::_0,
            true => TIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIF_A::_1
    }
}
#[doc = "Field `TOF` reader - Time Overflow Flag"]
pub type TOF_R = crate::BitReader<TOF_A>;
#[doc = "Time Overflow Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOF_A {
    #[doc = "0: Time overflow has not occurred."]
    _0 = 0,
    #[doc = "1: Time overflow has occurred and time counter is read as zero."]
    _1 = 1,
}
impl From<TOF_A> for bool {
    #[inline(always)]
    fn from(variant: TOF_A) -> Self {
        variant as u8 != 0
    }
}
impl TOF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOF_A {
        match self.bits {
            false => TOF_A::_0,
            true => TOF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOF_A::_1
    }
}
#[doc = "Field `TAF` reader - Time Alarm Flag"]
pub type TAF_R = crate::BitReader<TAF_A>;
#[doc = "Time Alarm Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAF_A {
    #[doc = "0: Time alarm has not occurred."]
    _0 = 0,
    #[doc = "1: Time alarm has occurred."]
    _1 = 1,
}
impl From<TAF_A> for bool {
    #[inline(always)]
    fn from(variant: TAF_A) -> Self {
        variant as u8 != 0
    }
}
impl TAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TAF_A {
        match self.bits {
            false => TAF_A::_0,
            true => TAF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TAF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TAF_A::_1
    }
}
#[doc = "Field `TCE` reader - Time Counter Enable"]
pub type TCE_R = crate::BitReader<TCE_A>;
#[doc = "Time Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCE_A {
    #[doc = "0: Time counter is disabled."]
    _0 = 0,
    #[doc = "1: Time counter is enabled."]
    _1 = 1,
}
impl From<TCE_A> for bool {
    #[inline(always)]
    fn from(variant: TCE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCE_A {
        match self.bits {
            false => TCE_A::_0,
            true => TCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCE_A::_1
    }
}
#[doc = "Field `TCE` writer - Time Counter Enable"]
pub type TCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, TCE_A, O>;
impl<'a, const O: u8> TCE_W<'a, O> {
    #[doc = "Time counter is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCE_A::_0)
    }
    #[doc = "Time counter is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Time Invalid Flag"]
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time Overflow Flag"]
    #[inline(always)]
    pub fn tof(&self) -> TOF_R {
        TOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Time Alarm Flag"]
    #[inline(always)]
    pub fn taf(&self) -> TAF_R {
        TAF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Time Counter Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Time Counter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tce(&mut self) -> TCE_W<4> {
        TCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0x01"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
