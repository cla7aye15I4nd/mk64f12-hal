#[doc = "Register `FPROT%s` reader"]
pub struct R(crate::R<FPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FPROT%s` writer"]
pub struct W(crate::W<FPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FPROT_SPEC>;
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
impl From<crate::W<FPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PROT` reader - Program Flash Region Protect"]
pub type PROT_R = crate::FieldReader<u8, PROT_A>;
#[doc = "Program Flash Region Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PROT_A {
    #[doc = "0: Program flash region is protected."]
    _0 = 0,
    #[doc = "1: Program flash region is not protected"]
    _1 = 1,
}
impl From<PROT_A> for u8 {
    #[inline(always)]
    fn from(variant: PROT_A) -> Self {
        variant as _
    }
}
impl PROT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PROT_A> {
        match self.bits {
            0 => Some(PROT_A::_0),
            1 => Some(PROT_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PROT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PROT_A::_1
    }
}
#[doc = "Field `PROT` writer - Program Flash Region Protect"]
pub type PROT_W<'a, const O: u8> = crate::FieldWriter<'a, u8, FPROT_SPEC, u8, PROT_A, 8, O>;
impl<'a, const O: u8> PROT_W<'a, O> {
    #[doc = "Program flash region is protected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PROT_A::_0)
    }
    #[doc = "Program flash region is not protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PROT_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Program Flash Region Protect"]
    #[inline(always)]
    pub fn prot(&self) -> PROT_R {
        PROT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Program Flash Region Protect"]
    #[inline(always)]
    #[must_use]
    pub fn prot(&mut self) -> PROT_W<0> {
        PROT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Program Flash Protection Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fprot](index.html) module"]
pub struct FPROT_SPEC;
impl crate::RegisterSpec for FPROT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fprot::R](R) reader structure"]
impl crate::Readable for FPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fprot::W](W) writer structure"]
impl crate::Writable for FPROT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FPROT%s to value 0"]
impl crate::Resettable for FPROT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
