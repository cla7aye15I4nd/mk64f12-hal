#[doc = "Register `C%sSC` reader"]
pub struct R(crate::R<CSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C%sSC` writer"]
pub struct W(crate::W<CSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSC_SPEC>;
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
impl From<crate::W<CSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA` reader - DMA Enable"]
pub type DMA_R = crate::BitReader<DMA_A>;
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_A {
    #[doc = "0: Disable DMA transfers."]
    _0 = 0,
    #[doc = "1: Enable DMA transfers."]
    _1 = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::_0,
            true => DMA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMA_A::_1
    }
}
#[doc = "Field `DMA` writer - DMA Enable"]
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSC_SPEC, DMA_A, O>;
impl<'a, const O: u8> DMA_W<'a, O> {
    #[doc = "Disable DMA transfers."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMA_A::_0)
    }
    #[doc = "Enable DMA transfers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMA_A::_1)
    }
}
#[doc = "Field `ELSA` reader - Edge or Level Select"]
pub type ELSA_R = crate::BitReader<bool>;
#[doc = "Field `ELSA` writer - Edge or Level Select"]
pub type ELSA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSC_SPEC, bool, O>;
#[doc = "Field `ELSB` reader - Edge or Level Select"]
pub type ELSB_R = crate::BitReader<bool>;
#[doc = "Field `ELSB` writer - Edge or Level Select"]
pub type ELSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSC_SPEC, bool, O>;
#[doc = "Field `MSA` reader - Channel Mode Select"]
pub type MSA_R = crate::BitReader<bool>;
#[doc = "Field `MSA` writer - Channel Mode Select"]
pub type MSA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSC_SPEC, bool, O>;
#[doc = "Field `MSB` reader - Channel Mode Select"]
pub type MSB_R = crate::BitReader<bool>;
#[doc = "Field `MSB` writer - Channel Mode Select"]
pub type MSB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSC_SPEC, bool, O>;
#[doc = "Field `CHIE` reader - Channel Interrupt Enable"]
pub type CHIE_R = crate::BitReader<CHIE_A>;
#[doc = "Channel Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHIE_A {
    #[doc = "0: Disable channel interrupts. Use software polling."]
    _0 = 0,
    #[doc = "1: Enable channel interrupts."]
    _1 = 1,
}
impl From<CHIE_A> for bool {
    #[inline(always)]
    fn from(variant: CHIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CHIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHIE_A {
        match self.bits {
            false => CHIE_A::_0,
            true => CHIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHIE_A::_1
    }
}
#[doc = "Field `CHIE` writer - Channel Interrupt Enable"]
pub type CHIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSC_SPEC, CHIE_A, O>;
impl<'a, const O: u8> CHIE_W<'a, O> {
    #[doc = "Disable channel interrupts. Use software polling."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHIE_A::_0)
    }
    #[doc = "Enable channel interrupts."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHIE_A::_1)
    }
}
#[doc = "Field `CHF` reader - Channel Flag"]
pub type CHF_R = crate::BitReader<CHF_A>;
#[doc = "Channel Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHF_A {
    #[doc = "0: No channel event has occurred."]
    _0 = 0,
    #[doc = "1: A channel event has occurred."]
    _1 = 1,
}
impl From<CHF_A> for bool {
    #[inline(always)]
    fn from(variant: CHF_A) -> Self {
        variant as u8 != 0
    }
}
impl CHF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHF_A {
        match self.bits {
            false => CHF_A::_0,
            true => CHF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CHF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CHF_A::_1
    }
}
#[doc = "Field `CHF` writer - Channel Flag"]
pub type CHF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSC_SPEC, CHF_A, O>;
impl<'a, const O: u8> CHF_W<'a, O> {
    #[doc = "No channel event has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CHF_A::_0)
    }
    #[doc = "A channel event has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CHF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Edge or Level Select"]
    #[inline(always)]
    pub fn elsa(&self) -> ELSA_R {
        ELSA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Edge or Level Select"]
    #[inline(always)]
    pub fn elsb(&self) -> ELSB_R {
        ELSB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel Mode Select"]
    #[inline(always)]
    pub fn msa(&self) -> MSA_R {
        MSA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel Mode Select"]
    #[inline(always)]
    pub fn msb(&self) -> MSB_R {
        MSB_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Interrupt Enable"]
    #[inline(always)]
    pub fn chie(&self) -> CHIE_R {
        CHIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Flag"]
    #[inline(always)]
    pub fn chf(&self) -> CHF_R {
        CHF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<0> {
        DMA_W::new(self)
    }
    #[doc = "Bit 2 - Edge or Level Select"]
    #[inline(always)]
    #[must_use]
    pub fn elsa(&mut self) -> ELSA_W<2> {
        ELSA_W::new(self)
    }
    #[doc = "Bit 3 - Edge or Level Select"]
    #[inline(always)]
    #[must_use]
    pub fn elsb(&mut self) -> ELSB_W<3> {
        ELSB_W::new(self)
    }
    #[doc = "Bit 4 - Channel Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn msa(&mut self) -> MSA_W<4> {
        MSA_W::new(self)
    }
    #[doc = "Bit 5 - Channel Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn msb(&mut self) -> MSB_W<5> {
        MSB_W::new(self)
    }
    #[doc = "Bit 6 - Channel Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn chie(&mut self) -> CHIE_W<6> {
        CHIE_W::new(self)
    }
    #[doc = "Bit 7 - Channel Flag"]
    #[inline(always)]
    #[must_use]
    pub fn chf(&mut self) -> CHF_W<7> {
        CHF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel (n) Status And Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csc](index.html) module"]
pub struct CSC_SPEC;
impl crate::RegisterSpec for CSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csc::R](R) reader structure"]
impl crate::Readable for CSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csc::W](W) writer structure"]
impl crate::Writable for CSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C%sSC to value 0"]
impl crate::Resettable for CSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
