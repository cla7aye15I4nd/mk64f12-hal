#[doc = "Register `C1` reader"]
pub struct R(crate::R<C1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1` writer"]
pub struct W(crate::W<C1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1_SPEC>;
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
impl From<crate::W<C1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACBFEN` reader - DAC Buffer Enable"]
pub type DACBFEN_R = crate::BitReader<DACBFEN_A>;
#[doc = "DAC Buffer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DACBFEN_A {
    #[doc = "0: Buffer read pointer is disabled. The converted data is always the first word of the buffer."]
    _0 = 0,
    #[doc = "1: Buffer read pointer is enabled. The converted data is the word that the read pointer points to. It means converted data can be from any word of the buffer."]
    _1 = 1,
}
impl From<DACBFEN_A> for bool {
    #[inline(always)]
    fn from(variant: DACBFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DACBFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBFEN_A {
        match self.bits {
            false => DACBFEN_A::_0,
            true => DACBFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DACBFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DACBFEN_A::_1
    }
}
#[doc = "Field `DACBFEN` writer - DAC Buffer Enable"]
pub type DACBFEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, DACBFEN_A, O>;
impl<'a, const O: u8> DACBFEN_W<'a, O> {
    #[doc = "Buffer read pointer is disabled. The converted data is always the first word of the buffer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DACBFEN_A::_0)
    }
    #[doc = "Buffer read pointer is enabled. The converted data is the word that the read pointer points to. It means converted data can be from any word of the buffer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DACBFEN_A::_1)
    }
}
#[doc = "Field `DACBFMD` reader - DAC Buffer Work Mode Select"]
pub type DACBFMD_R = crate::FieldReader<u8, DACBFMD_A>;
#[doc = "DAC Buffer Work Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DACBFMD_A {
    #[doc = "0: Normal mode"]
    _00 = 0,
    #[doc = "1: Swing mode"]
    _01 = 1,
    #[doc = "2: One-Time Scan mode"]
    _10 = 2,
}
impl From<DACBFMD_A> for u8 {
    #[inline(always)]
    fn from(variant: DACBFMD_A) -> Self {
        variant as _
    }
}
impl DACBFMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DACBFMD_A> {
        match self.bits {
            0 => Some(DACBFMD_A::_00),
            1 => Some(DACBFMD_A::_01),
            2 => Some(DACBFMD_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DACBFMD_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DACBFMD_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DACBFMD_A::_10
    }
}
#[doc = "Field `DACBFMD` writer - DAC Buffer Work Mode Select"]
pub type DACBFMD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, C1_SPEC, u8, DACBFMD_A, 2, O>;
impl<'a, const O: u8> DACBFMD_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DACBFMD_A::_00)
    }
    #[doc = "Swing mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DACBFMD_A::_01)
    }
    #[doc = "One-Time Scan mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DACBFMD_A::_10)
    }
}
#[doc = "Field `DACBFWM` reader - DAC Buffer Watermark Select"]
pub type DACBFWM_R = crate::FieldReader<u8, DACBFWM_A>;
#[doc = "DAC Buffer Watermark Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DACBFWM_A {
    #[doc = "0: 1 word"]
    _00 = 0,
    #[doc = "1: 2 words"]
    _01 = 1,
    #[doc = "2: 3 words"]
    _10 = 2,
    #[doc = "3: 4 words"]
    _11 = 3,
}
impl From<DACBFWM_A> for u8 {
    #[inline(always)]
    fn from(variant: DACBFWM_A) -> Self {
        variant as _
    }
}
impl DACBFWM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DACBFWM_A {
        match self.bits {
            0 => DACBFWM_A::_00,
            1 => DACBFWM_A::_01,
            2 => DACBFWM_A::_10,
            3 => DACBFWM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == DACBFWM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == DACBFWM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == DACBFWM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == DACBFWM_A::_11
    }
}
#[doc = "Field `DACBFWM` writer - DAC Buffer Watermark Select"]
pub type DACBFWM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, C1_SPEC, u8, DACBFWM_A, 2, O>;
impl<'a, const O: u8> DACBFWM_W<'a, O> {
    #[doc = "1 word"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(DACBFWM_A::_00)
    }
    #[doc = "2 words"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(DACBFWM_A::_01)
    }
    #[doc = "3 words"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(DACBFWM_A::_10)
    }
    #[doc = "4 words"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(DACBFWM_A::_11)
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable Select"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "DMA Enable Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: DMA is disabled."]
    _0 = 0,
    #[doc = "1: DMA is enabled. When DMA is enabled, the DMA request will be generated by original interrupts. The interrupts will not be presented on this module at the same time."]
    _1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::_0,
            true => DMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAEN_A::_1
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable Select"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, C1_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEN_A::_0)
    }
    #[doc = "DMA is enabled. When DMA is enabled, the DMA request will be generated by original interrupts. The interrupts will not be presented on this module at the same time."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DAC Buffer Enable"]
    #[inline(always)]
    pub fn dacbfen(&self) -> DACBFEN_R {
        DACBFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - DAC Buffer Work Mode Select"]
    #[inline(always)]
    pub fn dacbfmd(&self) -> DACBFMD_R {
        DACBFMD_R::new((self.bits >> 1) & 3)
    }
    #[doc = "Bits 3:4 - DAC Buffer Watermark Select"]
    #[inline(always)]
    pub fn dacbfwm(&self) -> DACBFWM_R {
        DACBFWM_R::new((self.bits >> 3) & 3)
    }
    #[doc = "Bit 7 - DMA Enable Select"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DAC Buffer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dacbfen(&mut self) -> DACBFEN_W<0> {
        DACBFEN_W::new(self)
    }
    #[doc = "Bits 1:2 - DAC Buffer Work Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn dacbfmd(&mut self) -> DACBFMD_W<1> {
        DACBFMD_W::new(self)
    }
    #[doc = "Bits 3:4 - DAC Buffer Watermark Select"]
    #[inline(always)]
    #[must_use]
    pub fn dacbfwm(&mut self) -> DACBFWM_W<3> {
        DACBFWM_W::new(self)
    }
    #[doc = "Bit 7 - DMA Enable Select"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<7> {
        DMAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1](index.html) module"]
pub struct C1_SPEC;
impl crate::RegisterSpec for C1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c1::R](R) reader structure"]
impl crate::Readable for C1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1::W](W) writer structure"]
impl crate::Writable for C1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C1 to value 0"]
impl crate::Resettable for C1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
