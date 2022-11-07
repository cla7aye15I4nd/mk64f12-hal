#[doc = "Register `TCD%s_CSR` reader"]
pub struct R(crate::R<TCD_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCD_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCD_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCD_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD%s_CSR` writer"]
pub struct W(crate::W<TCD_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCD_CSR_SPEC>;
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
impl From<crate::W<TCD_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCD_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `START` reader - Channel Start"]
pub type START_R = crate::BitReader<START_A>;
#[doc = "Channel Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    #[doc = "0: The channel is not explicitly started"]
    _0 = 0,
    #[doc = "1: The channel is explicitly started via a software initiated service request"]
    _1 = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::_0,
            true => START_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == START_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == START_A::_1
    }
}
#[doc = "Field `START` writer - Channel Start"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u16, TCD_CSR_SPEC, START_A, O>;
impl<'a, const O: u8> START_W<'a, O> {
    #[doc = "The channel is not explicitly started"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(START_A::_0)
    }
    #[doc = "The channel is explicitly started via a software initiated service request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(START_A::_1)
    }
}
#[doc = "Field `INTMAJOR` reader - Enable an interrupt when major iteration count completes"]
pub type INTMAJOR_R = crate::BitReader<INTMAJOR_A>;
#[doc = "Enable an interrupt when major iteration count completes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTMAJOR_A {
    #[doc = "0: The end-of-major loop interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The end-of-major loop interrupt is enabled"]
    _1 = 1,
}
impl From<INTMAJOR_A> for bool {
    #[inline(always)]
    fn from(variant: INTMAJOR_A) -> Self {
        variant as u8 != 0
    }
}
impl INTMAJOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTMAJOR_A {
        match self.bits {
            false => INTMAJOR_A::_0,
            true => INTMAJOR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INTMAJOR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INTMAJOR_A::_1
    }
}
#[doc = "Field `INTMAJOR` writer - Enable an interrupt when major iteration count completes"]
pub type INTMAJOR_W<'a, const O: u8> = crate::BitWriter<'a, u16, TCD_CSR_SPEC, INTMAJOR_A, O>;
impl<'a, const O: u8> INTMAJOR_W<'a, O> {
    #[doc = "The end-of-major loop interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTMAJOR_A::_0)
    }
    #[doc = "The end-of-major loop interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTMAJOR_A::_1)
    }
}
#[doc = "Field `INTHALF` reader - Enable an interrupt when major counter is half complete."]
pub type INTHALF_R = crate::BitReader<INTHALF_A>;
#[doc = "Enable an interrupt when major counter is half complete.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INTHALF_A {
    #[doc = "0: The half-point interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The half-point interrupt is enabled"]
    _1 = 1,
}
impl From<INTHALF_A> for bool {
    #[inline(always)]
    fn from(variant: INTHALF_A) -> Self {
        variant as u8 != 0
    }
}
impl INTHALF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTHALF_A {
        match self.bits {
            false => INTHALF_A::_0,
            true => INTHALF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INTHALF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INTHALF_A::_1
    }
}
#[doc = "Field `INTHALF` writer - Enable an interrupt when major counter is half complete."]
pub type INTHALF_W<'a, const O: u8> = crate::BitWriter<'a, u16, TCD_CSR_SPEC, INTHALF_A, O>;
impl<'a, const O: u8> INTHALF_W<'a, O> {
    #[doc = "The half-point interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INTHALF_A::_0)
    }
    #[doc = "The half-point interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INTHALF_A::_1)
    }
}
#[doc = "Field `DREQ` reader - Disable Request"]
pub type DREQ_R = crate::BitReader<DREQ_A>;
#[doc = "Disable Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DREQ_A {
    #[doc = "0: The channel's ERQ bit is not affected"]
    _0 = 0,
    #[doc = "1: The channel's ERQ bit is cleared when the major loop is complete"]
    _1 = 1,
}
impl From<DREQ_A> for bool {
    #[inline(always)]
    fn from(variant: DREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl DREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DREQ_A {
        match self.bits {
            false => DREQ_A::_0,
            true => DREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DREQ_A::_1
    }
}
#[doc = "Field `DREQ` writer - Disable Request"]
pub type DREQ_W<'a, const O: u8> = crate::BitWriter<'a, u16, TCD_CSR_SPEC, DREQ_A, O>;
impl<'a, const O: u8> DREQ_W<'a, O> {
    #[doc = "The channel's ERQ bit is not affected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DREQ_A::_0)
    }
    #[doc = "The channel's ERQ bit is cleared when the major loop is complete"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DREQ_A::_1)
    }
}
#[doc = "Field `ESG` reader - Enable Scatter/Gather Processing"]
pub type ESG_R = crate::BitReader<ESG_A>;
#[doc = "Enable Scatter/Gather Processing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ESG_A {
    #[doc = "0: The current channel's TCD is normal format."]
    _0 = 0,
    #[doc = "1: The current channel's TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution."]
    _1 = 1,
}
impl From<ESG_A> for bool {
    #[inline(always)]
    fn from(variant: ESG_A) -> Self {
        variant as u8 != 0
    }
}
impl ESG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ESG_A {
        match self.bits {
            false => ESG_A::_0,
            true => ESG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ESG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ESG_A::_1
    }
}
#[doc = "Field `ESG` writer - Enable Scatter/Gather Processing"]
pub type ESG_W<'a, const O: u8> = crate::BitWriter<'a, u16, TCD_CSR_SPEC, ESG_A, O>;
impl<'a, const O: u8> ESG_W<'a, O> {
    #[doc = "The current channel's TCD is normal format."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ESG_A::_0)
    }
    #[doc = "The current channel's TCD specifies a scatter gather format. The DLASTSGA field provides a memory pointer to the next TCD to be loaded into this channel after the major loop completes its execution."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ESG_A::_1)
    }
}
#[doc = "Field `MAJORELINK` reader - Enable channel-to-channel linking on major loop complete"]
pub type MAJORELINK_R = crate::BitReader<MAJORELINK_A>;
#[doc = "Enable channel-to-channel linking on major loop complete\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAJORELINK_A {
    #[doc = "0: The channel-to-channel linking is disabled"]
    _0 = 0,
    #[doc = "1: The channel-to-channel linking is enabled"]
    _1 = 1,
}
impl From<MAJORELINK_A> for bool {
    #[inline(always)]
    fn from(variant: MAJORELINK_A) -> Self {
        variant as u8 != 0
    }
}
impl MAJORELINK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAJORELINK_A {
        match self.bits {
            false => MAJORELINK_A::_0,
            true => MAJORELINK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MAJORELINK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MAJORELINK_A::_1
    }
}
#[doc = "Field `MAJORELINK` writer - Enable channel-to-channel linking on major loop complete"]
pub type MAJORELINK_W<'a, const O: u8> = crate::BitWriter<'a, u16, TCD_CSR_SPEC, MAJORELINK_A, O>;
impl<'a, const O: u8> MAJORELINK_W<'a, O> {
    #[doc = "The channel-to-channel linking is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAJORELINK_A::_0)
    }
    #[doc = "The channel-to-channel linking is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAJORELINK_A::_1)
    }
}
#[doc = "Field `ACTIVE` reader - Channel Active"]
pub type ACTIVE_R = crate::BitReader<bool>;
#[doc = "Field `ACTIVE` writer - Channel Active"]
pub type ACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TCD_CSR_SPEC, bool, O>;
#[doc = "Field `DONE` reader - Channel Done"]
pub type DONE_R = crate::BitReader<bool>;
#[doc = "Field `DONE` writer - Channel Done"]
pub type DONE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TCD_CSR_SPEC, bool, O>;
#[doc = "Field `MAJORLINKCH` reader - Link Channel Number"]
pub type MAJORLINKCH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MAJORLINKCH` writer - Link Channel Number"]
pub type MAJORLINKCH_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TCD_CSR_SPEC, u8, u8, 4, O>;
#[doc = "Field `BWC` reader - Bandwidth Control"]
pub type BWC_R = crate::FieldReader<u8, BWC_A>;
#[doc = "Bandwidth Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BWC_A {
    #[doc = "0: No eDMA engine stalls"]
    _00 = 0,
    #[doc = "2: eDMA engine stalls for 4 cycles after each r/w"]
    _10 = 2,
    #[doc = "3: eDMA engine stalls for 8 cycles after each r/w"]
    _11 = 3,
}
impl From<BWC_A> for u8 {
    #[inline(always)]
    fn from(variant: BWC_A) -> Self {
        variant as _
    }
}
impl BWC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BWC_A> {
        match self.bits {
            0 => Some(BWC_A::_00),
            2 => Some(BWC_A::_10),
            3 => Some(BWC_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == BWC_A::_00
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BWC_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == BWC_A::_11
    }
}
#[doc = "Field `BWC` writer - Bandwidth Control"]
pub type BWC_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TCD_CSR_SPEC, u8, BWC_A, 2, O>;
impl<'a, const O: u8> BWC_W<'a, O> {
    #[doc = "No eDMA engine stalls"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(BWC_A::_00)
    }
    #[doc = "eDMA engine stalls for 4 cycles after each r/w"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(BWC_A::_10)
    }
    #[doc = "eDMA engine stalls for 8 cycles after each r/w"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(BWC_A::_11)
    }
}
impl R {
    #[doc = "Bit 0 - Channel Start"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable an interrupt when major iteration count completes"]
    #[inline(always)]
    pub fn intmajor(&self) -> INTMAJOR_R {
        INTMAJOR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    pub fn inthalf(&self) -> INTHALF_R {
        INTHALF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable Request"]
    #[inline(always)]
    pub fn dreq(&self) -> DREQ_R {
        DREQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Scatter/Gather Processing"]
    #[inline(always)]
    pub fn esg(&self) -> ESG_R {
        ESG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    pub fn majorelink(&self) -> MAJORELINK_R {
        MAJORELINK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel Active"]
    #[inline(always)]
    pub fn active(&self) -> ACTIVE_R {
        ACTIVE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel Done"]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Link Channel Number"]
    #[inline(always)]
    pub fn majorlinkch(&self) -> MAJORLINKCH_R {
        MAJORLINKCH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 14:15 - Bandwidth Control"]
    #[inline(always)]
    pub fn bwc(&self) -> BWC_R {
        BWC_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Channel Start"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<0> {
        START_W::new(self)
    }
    #[doc = "Bit 1 - Enable an interrupt when major iteration count completes"]
    #[inline(always)]
    #[must_use]
    pub fn intmajor(&mut self) -> INTMAJOR_W<1> {
        INTMAJOR_W::new(self)
    }
    #[doc = "Bit 2 - Enable an interrupt when major counter is half complete."]
    #[inline(always)]
    #[must_use]
    pub fn inthalf(&mut self) -> INTHALF_W<2> {
        INTHALF_W::new(self)
    }
    #[doc = "Bit 3 - Disable Request"]
    #[inline(always)]
    #[must_use]
    pub fn dreq(&mut self) -> DREQ_W<3> {
        DREQ_W::new(self)
    }
    #[doc = "Bit 4 - Enable Scatter/Gather Processing"]
    #[inline(always)]
    #[must_use]
    pub fn esg(&mut self) -> ESG_W<4> {
        ESG_W::new(self)
    }
    #[doc = "Bit 5 - Enable channel-to-channel linking on major loop complete"]
    #[inline(always)]
    #[must_use]
    pub fn majorelink(&mut self) -> MAJORELINK_W<5> {
        MAJORELINK_W::new(self)
    }
    #[doc = "Bit 6 - Channel Active"]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ACTIVE_W<6> {
        ACTIVE_W::new(self)
    }
    #[doc = "Bit 7 - Channel Done"]
    #[inline(always)]
    #[must_use]
    pub fn done(&mut self) -> DONE_W<7> {
        DONE_W::new(self)
    }
    #[doc = "Bits 8:11 - Link Channel Number"]
    #[inline(always)]
    #[must_use]
    pub fn majorlinkch(&mut self) -> MAJORLINKCH_W<8> {
        MAJORLINKCH_W::new(self)
    }
    #[doc = "Bits 14:15 - Bandwidth Control"]
    #[inline(always)]
    #[must_use]
    pub fn bwc(&mut self) -> BWC_W<14> {
        BWC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd_csr](index.html) module"]
pub struct TCD_CSR_SPEC;
impl crate::RegisterSpec for TCD_CSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tcd_csr::R](R) reader structure"]
impl crate::Readable for TCD_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcd_csr::W](W) writer structure"]
impl crate::Writable for TCD_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCD%s_CSR to value 0"]
impl crate::Resettable for TCD_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
