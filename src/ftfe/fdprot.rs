#[doc = "Register `FDPROT` reader"]
pub struct R(crate::R<FDPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FDPROT` writer"]
pub struct W(crate::W<FDPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FDPROT_SPEC>;
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
impl From<crate::W<FDPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FDPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DPROT` reader - Data Flash Region Protect"]
pub type DPROT_R = crate::FieldReader<u8, DPROT_A>;
#[doc = "Data Flash Region Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DPROT_A {
    #[doc = "0: Data Flash region is protected"]
    _0 = 0,
    #[doc = "1: Data Flash region is not protected"]
    _1 = 1,
}
impl From<DPROT_A> for u8 {
    #[inline(always)]
    fn from(variant: DPROT_A) -> Self {
        variant as _
    }
}
impl DPROT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DPROT_A> {
        match self.bits {
            0 => Some(DPROT_A::_0),
            1 => Some(DPROT_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DPROT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DPROT_A::_1
    }
}
#[doc = "Field `DPROT` writer - Data Flash Region Protect"]
pub type DPROT_W<'a, const O: u8> = crate::FieldWriter<'a, u8, FDPROT_SPEC, u8, DPROT_A, 8, O>;
impl<'a, const O: u8> DPROT_W<'a, O> {
    #[doc = "Data Flash region is protected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPROT_A::_0)
    }
    #[doc = "Data Flash region is not protected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPROT_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - Data Flash Region Protect"]
    #[inline(always)]
    pub fn dprot(&self) -> DPROT_R {
        DPROT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Flash Region Protect"]
    #[inline(always)]
    #[must_use]
    pub fn dprot(&mut self) -> DPROT_W<0> {
        DPROT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data Flash Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdprot](index.html) module"]
pub struct FDPROT_SPEC;
impl crate::RegisterSpec for FDPROT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fdprot::R](R) reader structure"]
impl crate::Readable for FDPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fdprot::W](W) writer structure"]
impl crate::Writable for FDPROT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FDPROT to value 0"]
impl crate::Resettable for FDPROT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
