#[doc = "Register `SERQ` writer"]
pub struct W(crate::W<SERQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SERQ_SPEC>;
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
impl From<crate::W<SERQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SERQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SERQ` writer - Set enable request"]
pub type SERQ_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SERQ_SPEC, u8, u8, 4, O>;
#[doc = "Set All Enable Requests\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAER_AW {
    #[doc = "0: Set only the ERQ bit specified in the SERQ field"]
    _0 = 0,
    #[doc = "1: Set all bits in ERQ"]
    _1 = 1,
}
impl From<SAER_AW> for bool {
    #[inline(always)]
    fn from(variant: SAER_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAER` writer - Set All Enable Requests"]
pub type SAER_W<'a, const O: u8> = crate::BitWriter<'a, u8, SERQ_SPEC, SAER_AW, O>;
impl<'a, const O: u8> SAER_W<'a, O> {
    #[doc = "Set only the ERQ bit specified in the SERQ field"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SAER_AW::_0)
    }
    #[doc = "Set all bits in ERQ"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SAER_AW::_1)
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
pub type NOP_W<'a, const O: u8> = crate::BitWriter<'a, u8, SERQ_SPEC, NOP_AW, O>;
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
    #[doc = "Bits 0:3 - Set enable request"]
    #[inline(always)]
    #[must_use]
    pub fn serq(&mut self) -> SERQ_W<0> {
        SERQ_W::new(self)
    }
    #[doc = "Bit 6 - Set All Enable Requests"]
    #[inline(always)]
    #[must_use]
    pub fn saer(&mut self) -> SAER_W<6> {
        SAER_W::new(self)
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
#[doc = "Set Enable Request Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [serq](index.html) module"]
pub struct SERQ_SPEC;
impl crate::RegisterSpec for SERQ_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [serq::W](W) writer structure"]
impl crate::Writable for SERQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SERQ to value 0"]
impl crate::Resettable for SERQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
