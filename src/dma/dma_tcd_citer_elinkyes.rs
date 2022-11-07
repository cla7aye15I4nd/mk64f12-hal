#[doc = "Register `TCD%s_CITER_ELINKYES` reader"]
pub struct R(crate::R<DMA_TCD_CITER_ELINKYES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_TCD_CITER_ELINKYES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_TCD_CITER_ELINKYES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_TCD_CITER_ELINKYES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD%s_CITER_ELINKYES` writer"]
pub struct W(crate::W<DMA_TCD_CITER_ELINKYES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_TCD_CITER_ELINKYES_SPEC>;
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
impl From<crate::W<DMA_TCD_CITER_ELINKYES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_TCD_CITER_ELINKYES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CITER` reader - Current Major Iteration Count"]
pub type CITER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CITER` writer - Current Major Iteration Count"]
pub type CITER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, DMA_TCD_CITER_ELINKYES_SPEC, u16, u16, 9, O>;
#[doc = "Field `LINKCH` reader - Link Channel Number"]
pub type LINKCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LINKCH` writer - Link Channel Number"]
pub type LINKCH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, DMA_TCD_CITER_ELINKYES_SPEC, u8, u8, 4, O>;
#[doc = "Field `ELINK` reader - Enable channel-to-channel linking on minor-loop complete"]
pub type ELINK_R = crate::BitReader<ELINK_A>;
#[doc = "Enable channel-to-channel linking on minor-loop complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELINK_A {
    #[doc = "0: The channel-to-channel linking is disabled"]
    _0 = 0,
    #[doc = "1: The channel-to-channel linking is enabled"]
    _1 = 1,
}
impl From<ELINK_A> for bool {
    #[inline(always)]
    fn from(variant: ELINK_A) -> Self {
        variant as u8 != 0
    }
}
impl ELINK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ELINK_A {
        match self.bits {
            false => ELINK_A::_0,
            true => ELINK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ELINK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ELINK_A::_1
    }
}
#[doc = "Field `ELINK` writer - Enable channel-to-channel linking on minor-loop complete"]
pub type ELINK_W<'a, const O: u8> =
    crate::BitWriter<'a, u16, DMA_TCD_CITER_ELINKYES_SPEC, ELINK_A, O>;
impl<'a, const O: u8> ELINK_W<'a, O> {
    #[doc = "The channel-to-channel linking is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ELINK_A::_0)
    }
    #[doc = "The channel-to-channel linking is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ELINK_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:8 - Current Major Iteration Count"]
    #[inline(always)]
    pub fn citer(&self) -> CITER_R {
        CITER_R::new(self.bits & 0x01ff)
    }
    #[doc = "Bits 9:12 - Link Channel Number"]
    #[inline(always)]
    pub fn linkch(&self) -> LINKCH_R {
        LINKCH_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    pub fn elink(&self) -> ELINK_R {
        ELINK_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Current Major Iteration Count"]
    #[inline(always)]
    #[must_use]
    pub fn citer(&mut self) -> CITER_W<0> {
        CITER_W::new(self)
    }
    #[doc = "Bits 9:12 - Link Channel Number"]
    #[inline(always)]
    #[must_use]
    pub fn linkch(&mut self) -> LINKCH_W<9> {
        LINKCH_W::new(self)
    }
    #[doc = "Bit 15 - Enable channel-to-channel linking on minor-loop complete"]
    #[inline(always)]
    #[must_use]
    pub fn elink(&mut self) -> ELINK_W<15> {
        ELINK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_tcd_citer_elinkyes](index.html) module"]
pub struct DMA_TCD_CITER_ELINKYES_SPEC;
impl crate::RegisterSpec for DMA_TCD_CITER_ELINKYES_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dma_tcd_citer_elinkyes::R](R) reader structure"]
impl crate::Readable for DMA_TCD_CITER_ELINKYES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_tcd_citer_elinkyes::W](W) writer structure"]
impl crate::Writable for DMA_TCD_CITER_ELINKYES_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCD%s_CITER_ELINKYES to value 0"]
impl crate::Resettable for DMA_TCD_CITER_ELINKYES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
