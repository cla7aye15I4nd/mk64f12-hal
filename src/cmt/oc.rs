#[doc = "Register `OC` reader"]
pub struct R(crate::R<OC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OC` writer"]
pub struct W(crate::W<OC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OC_SPEC>;
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
impl From<crate::W<OC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IROPEN` reader - IRO Pin Enable"]
pub type IROPEN_R = crate::BitReader<IROPEN_A>;
#[doc = "IRO Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IROPEN_A {
    #[doc = "0: The IRO signal is disabled."]
    _0 = 0,
    #[doc = "1: The IRO signal is enabled as output."]
    _1 = 1,
}
impl From<IROPEN_A> for bool {
    #[inline(always)]
    fn from(variant: IROPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IROPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IROPEN_A {
        match self.bits {
            false => IROPEN_A::_0,
            true => IROPEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IROPEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IROPEN_A::_1
    }
}
#[doc = "Field `IROPEN` writer - IRO Pin Enable"]
pub type IROPEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, OC_SPEC, IROPEN_A, O>;
impl<'a, const O: u8> IROPEN_W<'a, O> {
    #[doc = "The IRO signal is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IROPEN_A::_0)
    }
    #[doc = "The IRO signal is enabled as output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IROPEN_A::_1)
    }
}
#[doc = "Field `CMTPOL` reader - CMT Output Polarity"]
pub type CMTPOL_R = crate::BitReader<CMTPOL_A>;
#[doc = "CMT Output Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMTPOL_A {
    #[doc = "0: The IRO signal is active-low."]
    _0 = 0,
    #[doc = "1: The IRO signal is active-high."]
    _1 = 1,
}
impl From<CMTPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CMTPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CMTPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMTPOL_A {
        match self.bits {
            false => CMTPOL_A::_0,
            true => CMTPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CMTPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CMTPOL_A::_1
    }
}
#[doc = "Field `CMTPOL` writer - CMT Output Polarity"]
pub type CMTPOL_W<'a, const O: u8> = crate::BitWriter<'a, u8, OC_SPEC, CMTPOL_A, O>;
impl<'a, const O: u8> CMTPOL_W<'a, O> {
    #[doc = "The IRO signal is active-low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CMTPOL_A::_0)
    }
    #[doc = "The IRO signal is active-high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CMTPOL_A::_1)
    }
}
#[doc = "Field `IROL` reader - IRO Latch Control"]
pub type IROL_R = crate::BitReader<bool>;
#[doc = "Field `IROL` writer - IRO Latch Control"]
pub type IROL_W<'a, const O: u8> = crate::BitWriter<'a, u8, OC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 5 - IRO Pin Enable"]
    #[inline(always)]
    pub fn iropen(&self) -> IROPEN_R {
        IROPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMT Output Polarity"]
    #[inline(always)]
    pub fn cmtpol(&self) -> CMTPOL_R {
        CMTPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - IRO Latch Control"]
    #[inline(always)]
    pub fn irol(&self) -> IROL_R {
        IROL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - IRO Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iropen(&mut self) -> IROPEN_W<5> {
        IROPEN_W::new(self)
    }
    #[doc = "Bit 6 - CMT Output Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cmtpol(&mut self) -> CMTPOL_W<6> {
        CMTPOL_W::new(self)
    }
    #[doc = "Bit 7 - IRO Latch Control"]
    #[inline(always)]
    #[must_use]
    pub fn irol(&mut self) -> IROL_W<7> {
        IROL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMT Output Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oc](index.html) module"]
pub struct OC_SPEC;
impl crate::RegisterSpec for OC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [oc::R](R) reader structure"]
impl crate::Readable for OC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [oc::W](W) writer structure"]
impl crate::Writable for OC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OC to value 0"]
impl crate::Resettable for OC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
