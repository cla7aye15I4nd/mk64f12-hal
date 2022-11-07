#[doc = "Register `DMA` reader"]
pub struct R(crate::R<DMA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA` writer"]
pub struct W(crate::W<DMA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_SPEC>;
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
impl From<crate::W<DMA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA` reader - DMA Enable"]
pub type DMA_R = crate::BitReader<DMA_A>;
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_A {
    #[doc = "0: DMA transfer request and done are disabled."]
    _0 = 0,
    #[doc = "1: DMA transfer request and done are enabled."]
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
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u8, DMA_SPEC, DMA_A, O>;
impl<'a, const O: u8> DMA_W<'a, O> {
    #[doc = "DMA transfer request and done are disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMA_A::_0)
    }
    #[doc = "DMA transfer request and done are enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMA_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<0> {
        DMA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMT Direct Memory Access Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma](index.html) module"]
pub struct DMA_SPEC;
impl crate::RegisterSpec for DMA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [dma::R](R) reader structure"]
impl crate::Readable for DMA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma::W](W) writer structure"]
impl crate::Writable for DMA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA to value 0"]
impl crate::Resettable for DMA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
