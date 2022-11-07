#[doc = "Register `SOPT1CFG` reader"]
pub struct R(crate::R<SOPT1CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOPT1CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOPT1CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOPT1CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOPT1CFG` writer"]
pub struct W(crate::W<SOPT1CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOPT1CFG_SPEC>;
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
impl From<crate::W<SOPT1CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOPT1CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `URWE` reader - USB voltage regulator enable write enable"]
pub type URWE_R = crate::BitReader<URWE_A>;
#[doc = "USB voltage regulator enable write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum URWE_A {
    #[doc = "0: SOPT1 USBREGEN cannot be written."]
    _0 = 0,
    #[doc = "1: SOPT1 USBREGEN can be written."]
    _1 = 1,
}
impl From<URWE_A> for bool {
    #[inline(always)]
    fn from(variant: URWE_A) -> Self {
        variant as u8 != 0
    }
}
impl URWE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> URWE_A {
        match self.bits {
            false => URWE_A::_0,
            true => URWE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == URWE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == URWE_A::_1
    }
}
#[doc = "Field `URWE` writer - USB voltage regulator enable write enable"]
pub type URWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT1CFG_SPEC, URWE_A, O>;
impl<'a, const O: u8> URWE_W<'a, O> {
    #[doc = "SOPT1 USBREGEN cannot be written."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(URWE_A::_0)
    }
    #[doc = "SOPT1 USBREGEN can be written."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(URWE_A::_1)
    }
}
#[doc = "Field `UVSWE` reader - USB voltage regulator VLP standby write enable"]
pub type UVSWE_R = crate::BitReader<UVSWE_A>;
#[doc = "USB voltage regulator VLP standby write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UVSWE_A {
    #[doc = "0: SOPT1 USBVSTBY cannot be written."]
    _0 = 0,
    #[doc = "1: SOPT1 USBVSTBY can be written."]
    _1 = 1,
}
impl From<UVSWE_A> for bool {
    #[inline(always)]
    fn from(variant: UVSWE_A) -> Self {
        variant as u8 != 0
    }
}
impl UVSWE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UVSWE_A {
        match self.bits {
            false => UVSWE_A::_0,
            true => UVSWE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UVSWE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UVSWE_A::_1
    }
}
#[doc = "Field `UVSWE` writer - USB voltage regulator VLP standby write enable"]
pub type UVSWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT1CFG_SPEC, UVSWE_A, O>;
impl<'a, const O: u8> UVSWE_W<'a, O> {
    #[doc = "SOPT1 USBVSTBY cannot be written."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UVSWE_A::_0)
    }
    #[doc = "SOPT1 USBVSTBY can be written."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UVSWE_A::_1)
    }
}
#[doc = "Field `USSWE` reader - USB voltage regulator stop standby write enable"]
pub type USSWE_R = crate::BitReader<USSWE_A>;
#[doc = "USB voltage regulator stop standby write enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USSWE_A {
    #[doc = "0: SOPT1 USBSSTBY cannot be written."]
    _0 = 0,
    #[doc = "1: SOPT1 USBSSTBY can be written."]
    _1 = 1,
}
impl From<USSWE_A> for bool {
    #[inline(always)]
    fn from(variant: USSWE_A) -> Self {
        variant as u8 != 0
    }
}
impl USSWE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USSWE_A {
        match self.bits {
            false => USSWE_A::_0,
            true => USSWE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USSWE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USSWE_A::_1
    }
}
#[doc = "Field `USSWE` writer - USB voltage regulator stop standby write enable"]
pub type USSWE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT1CFG_SPEC, USSWE_A, O>;
impl<'a, const O: u8> USSWE_W<'a, O> {
    #[doc = "SOPT1 USBSSTBY cannot be written."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USSWE_A::_0)
    }
    #[doc = "SOPT1 USBSSTBY can be written."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USSWE_A::_1)
    }
}
impl R {
    #[doc = "Bit 24 - USB voltage regulator enable write enable"]
    #[inline(always)]
    pub fn urwe(&self) -> URWE_R {
        URWE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - USB voltage regulator VLP standby write enable"]
    #[inline(always)]
    pub fn uvswe(&self) -> UVSWE_R {
        UVSWE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USB voltage regulator stop standby write enable"]
    #[inline(always)]
    pub fn usswe(&self) -> USSWE_R {
        USSWE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - USB voltage regulator enable write enable"]
    #[inline(always)]
    #[must_use]
    pub fn urwe(&mut self) -> URWE_W<24> {
        URWE_W::new(self)
    }
    #[doc = "Bit 25 - USB voltage regulator VLP standby write enable"]
    #[inline(always)]
    #[must_use]
    pub fn uvswe(&mut self) -> UVSWE_W<25> {
        UVSWE_W::new(self)
    }
    #[doc = "Bit 26 - USB voltage regulator stop standby write enable"]
    #[inline(always)]
    #[must_use]
    pub fn usswe(&mut self) -> USSWE_W<26> {
        USSWE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SOPT1 Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopt1cfg](index.html) module"]
pub struct SOPT1CFG_SPEC;
impl crate::RegisterSpec for SOPT1CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sopt1cfg::R](R) reader structure"]
impl crate::Readable for SOPT1CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sopt1cfg::W](W) writer structure"]
impl crate::Writable for SOPT1CFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOPT1CFG to value 0"]
impl crate::Resettable for SOPT1CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
