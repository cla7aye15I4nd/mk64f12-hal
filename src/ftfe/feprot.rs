#[doc = "Register `FEPROT` reader"]
pub struct R(crate::R<FEPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FEPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FEPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FEPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FEPROT` writer"]
pub struct W(crate::W<FEPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEPROT_SPEC>;
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
impl From<crate::W<FEPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPROT` reader - EEPROM Region Protect"]
pub type EPROT_R = crate::FieldReader<u8, EPROT_A>;
#[doc = "EEPROM Region Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EPROT_A {
    #[doc = "0: For devices with program flash only: Reserved For devices with FlexNVM: EEPROM region is protected"]
    _0 = 0,
    #[doc = "1: For devices with program flash only: Reserved For devices with FlexNVM: EEPROM region is not protected"]
    _1 = 1,
}
impl From<EPROT_A> for u8 {
    #[inline(always)]
    fn from(variant: EPROT_A) -> Self {
        variant as _
    }
}
impl EPROT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EPROT_A> {
        match self.bits {
            0 => Some(EPROT_A::_0),
            1 => Some(EPROT_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EPROT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EPROT_A::_1
    }
}
#[doc = "Field `EPROT` writer - EEPROM Region Protect"]
pub type EPROT_W<'a, const O: u8> = crate::FieldWriter<'a, u8, FEPROT_SPEC, u8, EPROT_A, 8, O>;
impl<'a, const O: u8> EPROT_W<'a, O> {
    #[doc = "For devices with program flash only: Reserved For devices with FlexNVM: EEPROM region is protected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EPROT_A::_0)
    }
    #[doc = "For devices with program flash only: Reserved For devices with FlexNVM: EEPROM region is not protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EPROT_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - EEPROM Region Protect"]
    #[inline(always)]
    pub fn eprot(&self) -> EPROT_R {
        EPROT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - EEPROM Region Protect"]
    #[inline(always)]
    #[must_use]
    pub fn eprot(&mut self) -> EPROT_W<0> {
        EPROT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EEPROM Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [feprot](index.html) module"]
pub struct FEPROT_SPEC;
impl crate::RegisterSpec for FEPROT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [feprot::R](R) reader structure"]
impl crate::Readable for FEPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [feprot::W](W) writer structure"]
impl crate::Writable for FEPROT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FEPROT to value 0"]
impl crate::Resettable for FEPROT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
