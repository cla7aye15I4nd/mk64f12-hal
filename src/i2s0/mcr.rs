#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MICS` reader - MCLK Input Clock Select"]
pub type MICS_R = crate::FieldReader<u8, MICS_A>;
#[doc = "MCLK Input Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MICS_A {
    #[doc = "0: MCLK divider input clock 0 selected."]
    _00 = 0,
    #[doc = "1: MCLK divider input clock 1 selected."]
    _01 = 1,
    #[doc = "2: MCLK divider input clock 2 selected."]
    _10 = 2,
    #[doc = "3: MCLK divider input clock 3 selected."]
    _11 = 3,
}
impl From<MICS_A> for u8 {
    #[inline(always)]
    fn from(variant: MICS_A) -> Self {
        variant as _
    }
}
impl MICS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MICS_A {
        match self.bits {
            0 => MICS_A::_00,
            1 => MICS_A::_01,
            2 => MICS_A::_10,
            3 => MICS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MICS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MICS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MICS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MICS_A::_11
    }
}
#[doc = "Field `MICS` writer - MCLK Input Clock Select"]
pub type MICS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MCR_SPEC, u8, MICS_A, 2, O>;
impl<'a, const O: u8> MICS_W<'a, O> {
    #[doc = "MCLK divider input clock 0 selected."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MICS_A::_00)
    }
    #[doc = "MCLK divider input clock 1 selected."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MICS_A::_01)
    }
    #[doc = "MCLK divider input clock 2 selected."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MICS_A::_10)
    }
    #[doc = "MCLK divider input clock 3 selected."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(MICS_A::_11)
    }
}
#[doc = "Field `MOE` reader - MCLK Output Enable"]
pub type MOE_R = crate::BitReader<MOE_A>;
#[doc = "MCLK Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOE_A {
    #[doc = "0: MCLK signal pin is configured as an input that bypasses the MCLK divider."]
    _0 = 0,
    #[doc = "1: MCLK signal pin is configured as an output from the MCLK divider and the MCLK divider is enabled."]
    _1 = 1,
}
impl From<MOE_A> for bool {
    #[inline(always)]
    fn from(variant: MOE_A) -> Self {
        variant as u8 != 0
    }
}
impl MOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOE_A {
        match self.bits {
            false => MOE_A::_0,
            true => MOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MOE_A::_1
    }
}
#[doc = "Field `MOE` writer - MCLK Output Enable"]
pub type MOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MOE_A, O>;
impl<'a, const O: u8> MOE_W<'a, O> {
    #[doc = "MCLK signal pin is configured as an input that bypasses the MCLK divider."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MOE_A::_0)
    }
    #[doc = "MCLK signal pin is configured as an output from the MCLK divider and the MCLK divider is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MOE_A::_1)
    }
}
#[doc = "Field `DUF` reader - Divider Update Flag"]
pub type DUF_R = crate::BitReader<DUF_A>;
#[doc = "Divider Update Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DUF_A {
    #[doc = "0: MCLK divider ratio is not being updated currently."]
    _0 = 0,
    #[doc = "1: MCLK divider ratio is updating on-the-fly. Further updates to the MCLK divider ratio are blocked while this flag remains set."]
    _1 = 1,
}
impl From<DUF_A> for bool {
    #[inline(always)]
    fn from(variant: DUF_A) -> Self {
        variant as u8 != 0
    }
}
impl DUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DUF_A {
        match self.bits {
            false => DUF_A::_0,
            true => DUF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DUF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DUF_A::_1
    }
}
impl R {
    #[doc = "Bits 24:25 - MCLK Input Clock Select"]
    #[inline(always)]
    pub fn mics(&self) -> MICS_R {
        MICS_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 30 - MCLK Output Enable"]
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Divider Update Flag"]
    #[inline(always)]
    pub fn duf(&self) -> DUF_R {
        DUF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:25 - MCLK Input Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn mics(&mut self) -> MICS_W<24> {
        MICS_W::new(self)
    }
    #[doc = "Bit 30 - MCLK Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn moe(&mut self) -> MOE_W<30> {
        MOE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI MCLK Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
