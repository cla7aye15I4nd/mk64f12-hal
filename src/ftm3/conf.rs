#[doc = "Register `CONF` reader"]
pub struct R(crate::R<CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF` writer"]
pub struct W(crate::W<CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF_SPEC>;
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
impl From<crate::W<CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NUMTOF` reader - TOF Frequency"]
pub type NUMTOF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUMTOF` writer - TOF Frequency"]
pub type NUMTOF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONF_SPEC, u8, u8, 5, O>;
#[doc = "Field `BDMMODE` reader - BDM Mode"]
pub type BDMMODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BDMMODE` writer - BDM Mode"]
pub type BDMMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CONF_SPEC, u8, u8, 2, O>;
#[doc = "Field `GTBEEN` reader - Global Time Base Enable"]
pub type GTBEEN_R = crate::BitReader<GTBEEN_A>;
#[doc = "Global Time Base Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GTBEEN_A {
    #[doc = "0: Use of an external global time base is disabled."]
    _0 = 0,
    #[doc = "1: Use of an external global time base is enabled."]
    _1 = 1,
}
impl From<GTBEEN_A> for bool {
    #[inline(always)]
    fn from(variant: GTBEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GTBEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GTBEEN_A {
        match self.bits {
            false => GTBEEN_A::_0,
            true => GTBEEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GTBEEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GTBEEN_A::_1
    }
}
#[doc = "Field `GTBEEN` writer - Global Time Base Enable"]
pub type GTBEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF_SPEC, GTBEEN_A, O>;
impl<'a, const O: u8> GTBEEN_W<'a, O> {
    #[doc = "Use of an external global time base is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GTBEEN_A::_0)
    }
    #[doc = "Use of an external global time base is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GTBEEN_A::_1)
    }
}
#[doc = "Field `GTBEOUT` reader - Global Time Base Output"]
pub type GTBEOUT_R = crate::BitReader<GTBEOUT_A>;
#[doc = "Global Time Base Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GTBEOUT_A {
    #[doc = "0: A global time base signal generation is disabled."]
    _0 = 0,
    #[doc = "1: A global time base signal generation is enabled."]
    _1 = 1,
}
impl From<GTBEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: GTBEOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl GTBEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GTBEOUT_A {
        match self.bits {
            false => GTBEOUT_A::_0,
            true => GTBEOUT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GTBEOUT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GTBEOUT_A::_1
    }
}
#[doc = "Field `GTBEOUT` writer - Global Time Base Output"]
pub type GTBEOUT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CONF_SPEC, GTBEOUT_A, O>;
impl<'a, const O: u8> GTBEOUT_W<'a, O> {
    #[doc = "A global time base signal generation is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GTBEOUT_A::_0)
    }
    #[doc = "A global time base signal generation is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GTBEOUT_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - TOF Frequency"]
    #[inline(always)]
    pub fn numtof(&self) -> NUMTOF_R {
        NUMTOF_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:7 - BDM Mode"]
    #[inline(always)]
    pub fn bdmmode(&self) -> BDMMODE_R {
        BDMMODE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 9 - Global Time Base Enable"]
    #[inline(always)]
    pub fn gtbeen(&self) -> GTBEEN_R {
        GTBEEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Global Time Base Output"]
    #[inline(always)]
    pub fn gtbeout(&self) -> GTBEOUT_R {
        GTBEOUT_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - TOF Frequency"]
    #[inline(always)]
    #[must_use]
    pub fn numtof(&mut self) -> NUMTOF_W<0> {
        NUMTOF_W::new(self)
    }
    #[doc = "Bits 6:7 - BDM Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bdmmode(&mut self) -> BDMMODE_W<6> {
        BDMMODE_W::new(self)
    }
    #[doc = "Bit 9 - Global Time Base Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gtbeen(&mut self) -> GTBEEN_W<9> {
        GTBEEN_W::new(self)
    }
    #[doc = "Bit 10 - Global Time Base Output"]
    #[inline(always)]
    #[must_use]
    pub fn gtbeout(&mut self) -> GTBEOUT_W<10> {
        GTBEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf](index.html) module"]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf::R](R) reader structure"]
impl crate::Readable for CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf::W](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
