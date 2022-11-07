#[doc = "Register `IS7816` reader"]
pub struct R(crate::R<IS7816_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IS7816_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IS7816_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IS7816_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IS7816` writer"]
pub struct W(crate::W<IS7816_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IS7816_SPEC>;
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
impl From<crate::W<IS7816_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IS7816_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXT` reader - Receive Threshold Exceeded Interrupt"]
pub type RXT_R = crate::BitReader<RXT_A>;
#[doc = "Receive Threshold Exceeded Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXT_A {
    #[doc = "0: The number of consecutive NACKS generated as a result of parity errors and buffer overruns is less than or equal to the value in ET7816\\[RXTHRESHOLD\\]."]
    _0 = 0,
    #[doc = "1: The number of consecutive NACKS generated as a result of parity errors and buffer overruns is greater than the value in ET7816\\[RXTHRESHOLD\\]."]
    _1 = 1,
}
impl From<RXT_A> for bool {
    #[inline(always)]
    fn from(variant: RXT_A) -> Self {
        variant as u8 != 0
    }
}
impl RXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXT_A {
        match self.bits {
            false => RXT_A::_0,
            true => RXT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXT_A::_1
    }
}
#[doc = "Field `RXT` writer - Receive Threshold Exceeded Interrupt"]
pub type RXT_W<'a, const O: u8> = crate::BitWriter<'a, u8, IS7816_SPEC, RXT_A, O>;
impl<'a, const O: u8> RXT_W<'a, O> {
    #[doc = "The number of consecutive NACKS generated as a result of parity errors and buffer overruns is less than or equal to the value in ET7816\\[RXTHRESHOLD\\]."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXT_A::_0)
    }
    #[doc = "The number of consecutive NACKS generated as a result of parity errors and buffer overruns is greater than the value in ET7816\\[RXTHRESHOLD\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXT_A::_1)
    }
}
#[doc = "Field `TXT` reader - Transmit Threshold Exceeded Interrupt"]
pub type TXT_R = crate::BitReader<TXT_A>;
#[doc = "Transmit Threshold Exceeded Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXT_A {
    #[doc = "0: The number of retries and corresponding NACKS does not exceed the value in ET7816\\[TXTHRESHOLD\\]."]
    _0 = 0,
    #[doc = "1: The number of retries and corresponding NACKS exceeds the value in ET7816\\[TXTHRESHOLD\\]."]
    _1 = 1,
}
impl From<TXT_A> for bool {
    #[inline(always)]
    fn from(variant: TXT_A) -> Self {
        variant as u8 != 0
    }
}
impl TXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXT_A {
        match self.bits {
            false => TXT_A::_0,
            true => TXT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXT_A::_1
    }
}
#[doc = "Field `TXT` writer - Transmit Threshold Exceeded Interrupt"]
pub type TXT_W<'a, const O: u8> = crate::BitWriter<'a, u8, IS7816_SPEC, TXT_A, O>;
impl<'a, const O: u8> TXT_W<'a, O> {
    #[doc = "The number of retries and corresponding NACKS does not exceed the value in ET7816\\[TXTHRESHOLD\\]."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXT_A::_0)
    }
    #[doc = "The number of retries and corresponding NACKS exceeds the value in ET7816\\[TXTHRESHOLD\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXT_A::_1)
    }
}
#[doc = "Field `GTV` reader - Guard Timer Violated Interrupt"]
pub type GTV_R = crate::BitReader<GTV_A>;
#[doc = "Guard Timer Violated Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GTV_A {
    #[doc = "0: A guard time (GT, CGT, or BGT) has not been violated."]
    _0 = 0,
    #[doc = "1: A guard time (GT, CGT, or BGT) has been violated."]
    _1 = 1,
}
impl From<GTV_A> for bool {
    #[inline(always)]
    fn from(variant: GTV_A) -> Self {
        variant as u8 != 0
    }
}
impl GTV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GTV_A {
        match self.bits {
            false => GTV_A::_0,
            true => GTV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GTV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GTV_A::_1
    }
}
#[doc = "Field `GTV` writer - Guard Timer Violated Interrupt"]
pub type GTV_W<'a, const O: u8> = crate::BitWriter<'a, u8, IS7816_SPEC, GTV_A, O>;
impl<'a, const O: u8> GTV_W<'a, O> {
    #[doc = "A guard time (GT, CGT, or BGT) has not been violated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GTV_A::_0)
    }
    #[doc = "A guard time (GT, CGT, or BGT) has been violated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GTV_A::_1)
    }
}
#[doc = "Field `INITD` reader - Initial Character Detected Interrupt"]
pub type INITD_R = crate::BitReader<INITD_A>;
#[doc = "Initial Character Detected Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INITD_A {
    #[doc = "0: A valid initial character has not been received."]
    _0 = 0,
    #[doc = "1: A valid initial character has been received."]
    _1 = 1,
}
impl From<INITD_A> for bool {
    #[inline(always)]
    fn from(variant: INITD_A) -> Self {
        variant as u8 != 0
    }
}
impl INITD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INITD_A {
        match self.bits {
            false => INITD_A::_0,
            true => INITD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INITD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INITD_A::_1
    }
}
#[doc = "Field `INITD` writer - Initial Character Detected Interrupt"]
pub type INITD_W<'a, const O: u8> = crate::BitWriter<'a, u8, IS7816_SPEC, INITD_A, O>;
impl<'a, const O: u8> INITD_W<'a, O> {
    #[doc = "A valid initial character has not been received."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INITD_A::_0)
    }
    #[doc = "A valid initial character has been received."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INITD_A::_1)
    }
}
#[doc = "Field `BWT` reader - Block Wait Timer Interrupt"]
pub type BWT_R = crate::BitReader<BWT_A>;
#[doc = "Block Wait Timer Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWT_A {
    #[doc = "0: Block wait time (BWT) has not been violated."]
    _0 = 0,
    #[doc = "1: Block wait time (BWT) has been violated."]
    _1 = 1,
}
impl From<BWT_A> for bool {
    #[inline(always)]
    fn from(variant: BWT_A) -> Self {
        variant as u8 != 0
    }
}
impl BWT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BWT_A {
        match self.bits {
            false => BWT_A::_0,
            true => BWT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BWT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BWT_A::_1
    }
}
#[doc = "Field `BWT` writer - Block Wait Timer Interrupt"]
pub type BWT_W<'a, const O: u8> = crate::BitWriter<'a, u8, IS7816_SPEC, BWT_A, O>;
impl<'a, const O: u8> BWT_W<'a, O> {
    #[doc = "Block wait time (BWT) has not been violated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BWT_A::_0)
    }
    #[doc = "Block wait time (BWT) has been violated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BWT_A::_1)
    }
}
#[doc = "Field `CWT` reader - Character Wait Timer Interrupt"]
pub type CWT_R = crate::BitReader<CWT_A>;
#[doc = "Character Wait Timer Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWT_A {
    #[doc = "0: Character wait time (CWT) has not been violated."]
    _0 = 0,
    #[doc = "1: Character wait time (CWT) has been violated."]
    _1 = 1,
}
impl From<CWT_A> for bool {
    #[inline(always)]
    fn from(variant: CWT_A) -> Self {
        variant as u8 != 0
    }
}
impl CWT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CWT_A {
        match self.bits {
            false => CWT_A::_0,
            true => CWT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CWT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CWT_A::_1
    }
}
#[doc = "Field `CWT` writer - Character Wait Timer Interrupt"]
pub type CWT_W<'a, const O: u8> = crate::BitWriter<'a, u8, IS7816_SPEC, CWT_A, O>;
impl<'a, const O: u8> CWT_W<'a, O> {
    #[doc = "Character wait time (CWT) has not been violated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CWT_A::_0)
    }
    #[doc = "Character wait time (CWT) has been violated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CWT_A::_1)
    }
}
#[doc = "Field `WT` reader - Wait Timer Interrupt"]
pub type WT_R = crate::BitReader<WT_A>;
#[doc = "Wait Timer Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WT_A {
    #[doc = "0: Wait time (WT) has not been violated."]
    _0 = 0,
    #[doc = "1: Wait time (WT) has been violated."]
    _1 = 1,
}
impl From<WT_A> for bool {
    #[inline(always)]
    fn from(variant: WT_A) -> Self {
        variant as u8 != 0
    }
}
impl WT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WT_A {
        match self.bits {
            false => WT_A::_0,
            true => WT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WT_A::_1
    }
}
#[doc = "Field `WT` writer - Wait Timer Interrupt"]
pub type WT_W<'a, const O: u8> = crate::BitWriter<'a, u8, IS7816_SPEC, WT_A, O>;
impl<'a, const O: u8> WT_W<'a, O> {
    #[doc = "Wait time (WT) has not been violated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WT_A::_0)
    }
    #[doc = "Wait time (WT) has been violated."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WT_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receive Threshold Exceeded Interrupt"]
    #[inline(always)]
    pub fn rxt(&self) -> RXT_R {
        RXT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Threshold Exceeded Interrupt"]
    #[inline(always)]
    pub fn txt(&self) -> TXT_R {
        TXT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Guard Timer Violated Interrupt"]
    #[inline(always)]
    pub fn gtv(&self) -> GTV_R {
        GTV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Initial Character Detected Interrupt"]
    #[inline(always)]
    pub fn initd(&self) -> INITD_R {
        INITD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Block Wait Timer Interrupt"]
    #[inline(always)]
    pub fn bwt(&self) -> BWT_R {
        BWT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Character Wait Timer Interrupt"]
    #[inline(always)]
    pub fn cwt(&self) -> CWT_R {
        CWT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wait Timer Interrupt"]
    #[inline(always)]
    pub fn wt(&self) -> WT_R {
        WT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive Threshold Exceeded Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxt(&mut self) -> RXT_W<0> {
        RXT_W::new(self)
    }
    #[doc = "Bit 1 - Transmit Threshold Exceeded Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txt(&mut self) -> TXT_W<1> {
        TXT_W::new(self)
    }
    #[doc = "Bit 2 - Guard Timer Violated Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn gtv(&mut self) -> GTV_W<2> {
        GTV_W::new(self)
    }
    #[doc = "Bit 4 - Initial Character Detected Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn initd(&mut self) -> INITD_W<4> {
        INITD_W::new(self)
    }
    #[doc = "Bit 5 - Block Wait Timer Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn bwt(&mut self) -> BWT_W<5> {
        BWT_W::new(self)
    }
    #[doc = "Bit 6 - Character Wait Timer Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cwt(&mut self) -> CWT_W<6> {
        CWT_W::new(self)
    }
    #[doc = "Bit 7 - Wait Timer Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn wt(&mut self) -> WT_W<7> {
        WT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART 7816 Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [is7816](index.html) module"]
pub struct IS7816_SPEC;
impl crate::RegisterSpec for IS7816_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [is7816::R](R) reader structure"]
impl crate::Readable for IS7816_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [is7816::W](W) writer structure"]
impl crate::Writable for IS7816_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IS7816 to value 0"]
impl crate::Resettable for IS7816_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
