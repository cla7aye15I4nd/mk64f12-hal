#[doc = "Register `DFCR` reader"]
pub struct R(crate::R<DFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFCR` writer"]
pub struct W(crate::W<DFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFCR_SPEC>;
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
impl From<crate::W<DFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS` reader - Clock Source"]
pub type CS_R = crate::BitReader<CS_A>;
#[doc = "Clock Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CS_A {
    #[doc = "0: Digital filters are clocked by the bus clock."]
    _0 = 0,
    #[doc = "1: Digital filters are clocked by the 1 kHz LPO clock."]
    _1 = 1,
}
impl From<CS_A> for bool {
    #[inline(always)]
    fn from(variant: CS_A) -> Self {
        variant as u8 != 0
    }
}
impl CS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS_A {
        match self.bits {
            false => CS_A::_0,
            true => CS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CS_A::_1
    }
}
#[doc = "Field `CS` writer - Clock Source"]
pub type CS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFCR_SPEC, CS_A, O>;
impl<'a, const O: u8> CS_W<'a, O> {
    #[doc = "Digital filters are clocked by the bus clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CS_A::_0)
    }
    #[doc = "Digital filters are clocked by the 1 kHz LPO clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CS_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Source"]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Source"]
    #[inline(always)]
    #[must_use]
    pub fn cs(&mut self) -> CS_W<0> {
        CS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Filter Clock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfcr](index.html) module"]
pub struct DFCR_SPEC;
impl crate::RegisterSpec for DFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfcr::R](R) reader structure"]
impl crate::Readable for DFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfcr::W](W) writer structure"]
impl crate::Writable for DFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFCR to value 0"]
impl crate::Resettable for DFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
