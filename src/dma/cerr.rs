#[doc = "Register `CERR` writer"]
pub struct W(crate::W<CERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CERR_SPEC>;
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
impl From<crate::W<CERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CERR` writer - Clear Error Indicator"]
pub type CERR_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CERR_SPEC, u8, u8, 4, O>;
#[doc = "Clear All Error Indicators\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAEI_AW {
    #[doc = "0: Clear only the ERR bit specified in the CERR field"]
    _0 = 0,
    #[doc = "1: Clear all bits in ERR"]
    _1 = 1,
}
impl From<CAEI_AW> for bool {
    #[inline(always)]
    fn from(variant: CAEI_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAEI` writer - Clear All Error Indicators"]
pub type CAEI_W<'a, const O: u8> = crate::BitWriter<'a, u8, CERR_SPEC, CAEI_AW, O>;
impl<'a, const O: u8> CAEI_W<'a, O> {
    #[doc = "Clear only the ERR bit specified in the CERR field"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAEI_AW::_0)
    }
    #[doc = "Clear all bits in ERR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAEI_AW::_1)
    }
}
#[doc = "No Op enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOP_AW {
    #[doc = "0: Normal operation"]
    _0 = 0,
    #[doc = "1: No operation, ignore the other bits in this register"]
    _1 = 1,
}
impl From<NOP_AW> for bool {
    #[inline(always)]
    fn from(variant: NOP_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOP` writer - No Op enable"]
pub type NOP_W<'a, const O: u8> = crate::BitWriter<'a, u8, CERR_SPEC, NOP_AW, O>;
impl<'a, const O: u8> NOP_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(NOP_AW::_0)
    }
    #[doc = "No operation, ignore the other bits in this register"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(NOP_AW::_1)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clear Error Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn cerr(&mut self) -> CERR_W<0> {
        CERR_W::new(self)
    }
    #[doc = "Bit 6 - Clear All Error Indicators"]
    #[inline(always)]
    #[must_use]
    pub fn caei(&mut self) -> CAEI_W<6> {
        CAEI_W::new(self)
    }
    #[doc = "Bit 7 - No Op enable"]
    #[inline(always)]
    #[must_use]
    pub fn nop(&mut self) -> NOP_W<7> {
        NOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clear Error Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cerr](index.html) module"]
pub struct CERR_SPEC;
impl crate::RegisterSpec for CERR_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [cerr::W](W) writer structure"]
impl crate::Writable for CERR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CERR to value 0"]
impl crate::Resettable for CERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
