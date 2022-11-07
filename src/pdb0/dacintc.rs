#[doc = "Register `DACINTC%s` reader"]
pub struct R(crate::R<DACINTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DACINTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DACINTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DACINTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DACINTC%s` writer"]
pub struct W(crate::W<DACINTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DACINTC_SPEC>;
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
impl From<crate::W<DACINTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DACINTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOE` reader - DAC Interval Trigger Enable"]
pub type TOE_R = crate::BitReader<TOE_A>;
#[doc = "DAC Interval Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOE_A {
    #[doc = "0: DAC interval trigger disabled."]
    _0 = 0,
    #[doc = "1: DAC interval trigger enabled."]
    _1 = 1,
}
impl From<TOE_A> for bool {
    #[inline(always)]
    fn from(variant: TOE_A) -> Self {
        variant as u8 != 0
    }
}
impl TOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOE_A {
        match self.bits {
            false => TOE_A::_0,
            true => TOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOE_A::_1
    }
}
#[doc = "Field `TOE` writer - DAC Interval Trigger Enable"]
pub type TOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACINTC_SPEC, TOE_A, O>;
impl<'a, const O: u8> TOE_W<'a, O> {
    #[doc = "DAC interval trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOE_A::_0)
    }
    #[doc = "DAC interval trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOE_A::_1)
    }
}
#[doc = "Field `EXT` reader - DAC External Trigger Input Enable"]
pub type EXT_R = crate::BitReader<EXT_A>;
#[doc = "DAC External Trigger Input Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXT_A {
    #[doc = "0: DAC external trigger input disabled. DAC interval counter is reset and counting starts when a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0 = 0,
    #[doc = "1: DAC external trigger input enabled. DAC interval counter is bypassed and DAC external trigger input triggers the DAC interval trigger."]
    _1 = 1,
}
impl From<EXT_A> for bool {
    #[inline(always)]
    fn from(variant: EXT_A) -> Self {
        variant as u8 != 0
    }
}
impl EXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXT_A {
        match self.bits {
            false => EXT_A::_0,
            true => EXT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXT_A::_1
    }
}
#[doc = "Field `EXT` writer - DAC External Trigger Input Enable"]
pub type EXT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DACINTC_SPEC, EXT_A, O>;
impl<'a, const O: u8> EXT_W<'a, O> {
    #[doc = "DAC external trigger input disabled. DAC interval counter is reset and counting starts when a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXT_A::_0)
    }
    #[doc = "DAC external trigger input enabled. DAC interval counter is bypassed and DAC external trigger input triggers the DAC interval trigger."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXT_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DAC Interval Trigger Enable"]
    #[inline(always)]
    pub fn toe(&self) -> TOE_R {
        TOE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DAC External Trigger Input Enable"]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC Interval Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn toe(&mut self) -> TOE_W<0> {
        TOE_W::new(self)
    }
    #[doc = "Bit 1 - DAC External Trigger Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ext(&mut self) -> EXT_W<1> {
        EXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Interval Trigger n Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dacintc](index.html) module"]
pub struct DACINTC_SPEC;
impl crate::RegisterSpec for DACINTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dacintc::R](R) reader structure"]
impl crate::Readable for DACINTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dacintc::W](W) writer structure"]
impl crate::Writable for DACINTC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DACINTC%s to value 0"]
impl crate::Resettable for DACINTC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
