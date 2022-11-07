#[doc = "Register `CDNE` writer"]
pub struct W(crate::W<CDNE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDNE_SPEC>;
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
impl From<crate::W<CDNE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDNE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDNE` writer - Clear DONE Bit"]
pub type CDNE_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CDNE_SPEC, u8, u8, 4, O>;
#[doc = "Clears All DONE Bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CADN_AW {
    #[doc = "0: Clears only the TCDn_CSR\\[DONE\\]
bit specified in the CDNE field"]
    _0 = 0,
    #[doc = "1: Clears all bits in TCDn_CSR\\[DONE\\]"]
    _1 = 1,
}
impl From<CADN_AW> for bool {
    #[inline(always)]
    fn from(variant: CADN_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CADN` writer - Clears All DONE Bits"]
pub type CADN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CDNE_SPEC, CADN_AW, O>;
impl<'a, const O: u8> CADN_W<'a, O> {
    #[doc = "Clears only the TCDn_CSR\\[DONE\\]
bit specified in the CDNE field"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CADN_AW::_0)
    }
    #[doc = "Clears all bits in TCDn_CSR\\[DONE\\]"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CADN_AW::_1)
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
pub type NOP_W<'a, const O: u8> = crate::BitWriter<'a, u8, CDNE_SPEC, NOP_AW, O>;
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
    #[doc = "Bits 0:3 - Clear DONE Bit"]
    #[inline(always)]
    #[must_use]
    pub fn cdne(&mut self) -> CDNE_W<0> {
        CDNE_W::new(self)
    }
    #[doc = "Bit 6 - Clears All DONE Bits"]
    #[inline(always)]
    #[must_use]
    pub fn cadn(&mut self) -> CADN_W<6> {
        CADN_W::new(self)
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
#[doc = "Clear DONE Status Bit Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdne](index.html) module"]
pub struct CDNE_SPEC;
impl crate::RegisterSpec for CDNE_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [cdne::W](W) writer structure"]
impl crate::Writable for CDNE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDNE to value 0"]
impl crate::Resettable for CDNE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
