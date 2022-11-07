#[doc = "Register `CAU_AESIC_CASR` writer"]
pub struct W(crate::W<CAU_AESIC_CASR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAU_AESIC_CASR_SPEC>;
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
impl From<crate::W<CAU_AESIC_CASR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAU_AESIC_CASR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IC_AW {
    #[doc = "0: No illegal commands issued"]
    _0 = 0,
    #[doc = "1: Illegal command issued"]
    _1 = 1,
}
impl From<IC_AW> for bool {
    #[inline(always)]
    fn from(variant: IC_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IC` writer - no description available"]
pub type IC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAU_AESIC_CASR_SPEC, IC_AW, O>;
impl<'a, const O: u8> IC_W<'a, O> {
    #[doc = "No illegal commands issued"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IC_AW::_0)
    }
    #[doc = "Illegal command issued"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IC_AW::_1)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DPE_AW {
    #[doc = "0: No error detected"]
    _0 = 0,
    #[doc = "1: DES key parity error detected"]
    _1 = 1,
}
impl From<DPE_AW> for bool {
    #[inline(always)]
    fn from(variant: DPE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DPE` writer - no description available"]
pub type DPE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAU_AESIC_CASR_SPEC, DPE_AW, O>;
impl<'a, const O: u8> DPE_W<'a, O> {
    #[doc = "No error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DPE_AW::_0)
    }
    #[doc = "DES key parity error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DPE_AW::_1)
    }
}
#[doc = "CAU version\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VER_AW {
    #[doc = "1: Initial CAU version"]
    _0001 = 1,
    #[doc = "2: Second version, added support for SHA-256 algorithm.(This is the value on this device)"]
    _0010 = 2,
}
impl From<VER_AW> for u8 {
    #[inline(always)]
    fn from(variant: VER_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `VER` writer - CAU version"]
pub type VER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CAU_AESIC_CASR_SPEC, u8, VER_AW, 4, O>;
impl<'a, const O: u8> VER_W<'a, O> {
    #[doc = "Initial CAU version"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut W {
        self.variant(VER_AW::_0001)
    }
    #[doc = "Second version, added support for SHA-256 algorithm.(This is the value on this device)"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut W {
        self.variant(VER_AW::_0010)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn ic(&mut self) -> IC_W<0> {
        IC_W::new(self)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn dpe(&mut self) -> DPE_W<1> {
        DPE_W::new(self)
    }
    #[doc = "Bits 28:31 - CAU version"]
    #[inline(always)]
    #[must_use]
    pub fn ver(&mut self) -> VER_W<28> {
        VER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register - AES Inverse Column Operation command\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cau_aesic_casr](index.html) module"]
pub struct CAU_AESIC_CASR_SPEC;
impl crate::RegisterSpec for CAU_AESIC_CASR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cau_aesic_casr::W](W) writer structure"]
impl crate::Writable for CAU_AESIC_CASR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAU_AESIC_CASR to value 0x2000_0000"]
impl crate::Resettable for CAU_AESIC_CASR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_0000;
}
