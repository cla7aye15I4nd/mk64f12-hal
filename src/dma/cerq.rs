#[doc = "Register `CERQ` writer"]
pub struct W(crate::W<CERQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CERQ_SPEC>;
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
impl From<crate::W<CERQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CERQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CERQ` writer - Clear Enable Request"]
pub type CERQ_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CERQ_SPEC, u8, u8, 4, O>;
#[doc = "Clear All Enable Requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAER_AW {
    #[doc = "0: Clear only the ERQ bit specified in the CERQ field"]
    _0 = 0,
    #[doc = "1: Clear all bits in ERQ"]
    _1 = 1,
}
impl From<CAER_AW> for bool {
    #[inline(always)]
    fn from(variant: CAER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAER` writer - Clear All Enable Requests"]
pub type CAER_W<'a, const O: u8> = crate::BitWriter<'a, u8, CERQ_SPEC, CAER_AW, O>;
impl<'a, const O: u8> CAER_W<'a, O> {
    #[doc = "Clear only the ERQ bit specified in the CERQ field"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CAER_AW::_0)
    }
    #[doc = "Clear all bits in ERQ"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CAER_AW::_1)
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
pub type NOP_W<'a, const O: u8> = crate::BitWriter<'a, u8, CERQ_SPEC, NOP_AW, O>;
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
    #[doc = "Bits 0:3 - Clear Enable Request"]
    #[inline(always)]
    #[must_use]
    pub fn cerq(&mut self) -> CERQ_W<0> {
        CERQ_W::new(self)
    }
    #[doc = "Bit 6 - Clear All Enable Requests"]
    #[inline(always)]
    #[must_use]
    pub fn caer(&mut self) -> CAER_W<6> {
        CAER_W::new(self)
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
#[doc = "Clear Enable Request Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cerq](index.html) module"]
pub struct CERQ_SPEC;
impl crate::RegisterSpec for CERQ_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [cerq::W](W) writer structure"]
impl crate::Writable for CERQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CERQ to value 0"]
impl crate::Resettable for CERQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
