#[doc = "Register `PMPROT` reader"]
pub struct R(crate::R<PMPROT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMPROT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMPROT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMPROT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMPROT` writer"]
pub struct W(crate::W<PMPROT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMPROT_SPEC>;
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
impl From<crate::W<PMPROT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMPROT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVLLS` reader - Allow Very-Low-Leakage Stop Mode"]
pub type AVLLS_R = crate::BitReader<AVLLS_A>;
#[doc = "Allow Very-Low-Leakage Stop Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVLLS_A {
    #[doc = "0: Any VLLSx mode is not allowed"]
    _0 = 0,
    #[doc = "1: Any VLLSx mode is allowed"]
    _1 = 1,
}
impl From<AVLLS_A> for bool {
    #[inline(always)]
    fn from(variant: AVLLS_A) -> Self {
        variant as u8 != 0
    }
}
impl AVLLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVLLS_A {
        match self.bits {
            false => AVLLS_A::_0,
            true => AVLLS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AVLLS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AVLLS_A::_1
    }
}
#[doc = "Field `AVLLS` writer - Allow Very-Low-Leakage Stop Mode"]
pub type AVLLS_W<'a, const O: u8> = crate::BitWriter<'a, u8, PMPROT_SPEC, AVLLS_A, O>;
impl<'a, const O: u8> AVLLS_W<'a, O> {
    #[doc = "Any VLLSx mode is not allowed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVLLS_A::_0)
    }
    #[doc = "Any VLLSx mode is allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVLLS_A::_1)
    }
}
#[doc = "Field `ALLS` reader - Allow Low-Leakage Stop Mode"]
pub type ALLS_R = crate::BitReader<ALLS_A>;
#[doc = "Allow Low-Leakage Stop Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALLS_A {
    #[doc = "0: LLS is not allowed"]
    _0 = 0,
    #[doc = "1: LLS is allowed"]
    _1 = 1,
}
impl From<ALLS_A> for bool {
    #[inline(always)]
    fn from(variant: ALLS_A) -> Self {
        variant as u8 != 0
    }
}
impl ALLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALLS_A {
        match self.bits {
            false => ALLS_A::_0,
            true => ALLS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALLS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALLS_A::_1
    }
}
#[doc = "Field `ALLS` writer - Allow Low-Leakage Stop Mode"]
pub type ALLS_W<'a, const O: u8> = crate::BitWriter<'a, u8, PMPROT_SPEC, ALLS_A, O>;
impl<'a, const O: u8> ALLS_W<'a, O> {
    #[doc = "LLS is not allowed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALLS_A::_0)
    }
    #[doc = "LLS is allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALLS_A::_1)
    }
}
#[doc = "Field `AVLP` reader - Allow Very-Low-Power Modes"]
pub type AVLP_R = crate::BitReader<AVLP_A>;
#[doc = "Allow Very-Low-Power Modes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVLP_A {
    #[doc = "0: VLPR, VLPW, and VLPS are not allowed."]
    _0 = 0,
    #[doc = "1: VLPR, VLPW, and VLPS are allowed."]
    _1 = 1,
}
impl From<AVLP_A> for bool {
    #[inline(always)]
    fn from(variant: AVLP_A) -> Self {
        variant as u8 != 0
    }
}
impl AVLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVLP_A {
        match self.bits {
            false => AVLP_A::_0,
            true => AVLP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AVLP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AVLP_A::_1
    }
}
#[doc = "Field `AVLP` writer - Allow Very-Low-Power Modes"]
pub type AVLP_W<'a, const O: u8> = crate::BitWriter<'a, u8, PMPROT_SPEC, AVLP_A, O>;
impl<'a, const O: u8> AVLP_W<'a, O> {
    #[doc = "VLPR, VLPW, and VLPS are not allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVLP_A::_0)
    }
    #[doc = "VLPR, VLPW, and VLPS are allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVLP_A::_1)
    }
}
impl R {
    #[doc = "Bit 1 - Allow Very-Low-Leakage Stop Mode"]
    #[inline(always)]
    pub fn avlls(&self) -> AVLLS_R {
        AVLLS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Allow Low-Leakage Stop Mode"]
    #[inline(always)]
    pub fn alls(&self) -> ALLS_R {
        ALLS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Allow Very-Low-Power Modes"]
    #[inline(always)]
    pub fn avlp(&self) -> AVLP_R {
        AVLP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Allow Very-Low-Leakage Stop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn avlls(&mut self) -> AVLLS_W<1> {
        AVLLS_W::new(self)
    }
    #[doc = "Bit 3 - Allow Low-Leakage Stop Mode"]
    #[inline(always)]
    #[must_use]
    pub fn alls(&mut self) -> ALLS_W<3> {
        ALLS_W::new(self)
    }
    #[doc = "Bit 5 - Allow Very-Low-Power Modes"]
    #[inline(always)]
    #[must_use]
    pub fn avlp(&mut self) -> AVLP_W<5> {
        AVLP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Mode Protection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmprot](index.html) module"]
pub struct PMPROT_SPEC;
impl crate::RegisterSpec for PMPROT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [pmprot::R](R) reader structure"]
impl crate::Readable for PMPROT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmprot::W](W) writer structure"]
impl crate::Writable for PMPROT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMPROT to value 0"]
impl crate::Resettable for PMPROT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
