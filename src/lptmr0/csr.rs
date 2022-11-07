#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR` writer"]
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEN` reader - Timer Enable"]
pub type TEN_R = crate::BitReader<TEN_A>;
#[doc = "Timer Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEN_A {
    #[doc = "0: LPTMR is disabled and internal logic is reset."]
    _0 = 0,
    #[doc = "1: LPTMR is enabled."]
    _1 = 1,
}
impl From<TEN_A> for bool {
    #[inline(always)]
    fn from(variant: TEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEN_A {
        match self.bits {
            false => TEN_A::_0,
            true => TEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TEN_A::_1
    }
}
#[doc = "Field `TEN` writer - Timer Enable"]
pub type TEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, TEN_A, O>;
impl<'a, const O: u8> TEN_W<'a, O> {
    #[doc = "LPTMR is disabled and internal logic is reset."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TEN_A::_0)
    }
    #[doc = "LPTMR is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TEN_A::_1)
    }
}
#[doc = "Field `TMS` reader - Timer Mode Select"]
pub type TMS_R = crate::BitReader<TMS_A>;
#[doc = "Timer Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TMS_A {
    #[doc = "0: Time Counter mode."]
    _0 = 0,
    #[doc = "1: Pulse Counter mode."]
    _1 = 1,
}
impl From<TMS_A> for bool {
    #[inline(always)]
    fn from(variant: TMS_A) -> Self {
        variant as u8 != 0
    }
}
impl TMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMS_A {
        match self.bits {
            false => TMS_A::_0,
            true => TMS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TMS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TMS_A::_1
    }
}
#[doc = "Field `TMS` writer - Timer Mode Select"]
pub type TMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, TMS_A, O>;
impl<'a, const O: u8> TMS_W<'a, O> {
    #[doc = "Time Counter mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TMS_A::_0)
    }
    #[doc = "Pulse Counter mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TMS_A::_1)
    }
}
#[doc = "Field `TFC` reader - Timer Free-Running Counter"]
pub type TFC_R = crate::BitReader<TFC_A>;
#[doc = "Timer Free-Running Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFC_A {
    #[doc = "0: CNR is reset whenever TCF is set."]
    _0 = 0,
    #[doc = "1: CNR is reset on overflow."]
    _1 = 1,
}
impl From<TFC_A> for bool {
    #[inline(always)]
    fn from(variant: TFC_A) -> Self {
        variant as u8 != 0
    }
}
impl TFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFC_A {
        match self.bits {
            false => TFC_A::_0,
            true => TFC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFC_A::_1
    }
}
#[doc = "Field `TFC` writer - Timer Free-Running Counter"]
pub type TFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, TFC_A, O>;
impl<'a, const O: u8> TFC_W<'a, O> {
    #[doc = "CNR is reset whenever TCF is set."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFC_A::_0)
    }
    #[doc = "CNR is reset on overflow."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFC_A::_1)
    }
}
#[doc = "Field `TPP` reader - Timer Pin Polarity"]
pub type TPP_R = crate::BitReader<TPP_A>;
#[doc = "Timer Pin Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPP_A {
    #[doc = "0: Pulse Counter input source is active-high, and the CNR will increment on the rising-edge."]
    _0 = 0,
    #[doc = "1: Pulse Counter input source is active-low, and the CNR will increment on the falling-edge."]
    _1 = 1,
}
impl From<TPP_A> for bool {
    #[inline(always)]
    fn from(variant: TPP_A) -> Self {
        variant as u8 != 0
    }
}
impl TPP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPP_A {
        match self.bits {
            false => TPP_A::_0,
            true => TPP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TPP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TPP_A::_1
    }
}
#[doc = "Field `TPP` writer - Timer Pin Polarity"]
pub type TPP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, TPP_A, O>;
impl<'a, const O: u8> TPP_W<'a, O> {
    #[doc = "Pulse Counter input source is active-high, and the CNR will increment on the rising-edge."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPP_A::_0)
    }
    #[doc = "Pulse Counter input source is active-low, and the CNR will increment on the falling-edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPP_A::_1)
    }
}
#[doc = "Field `TPS` reader - Timer Pin Select"]
pub type TPS_R = crate::FieldReader<u8, TPS_A>;
#[doc = "Timer Pin Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TPS_A {
    #[doc = "0: Pulse counter input 0 is selected."]
    _00 = 0,
    #[doc = "1: Pulse counter input 1 is selected."]
    _01 = 1,
    #[doc = "2: Pulse counter input 2 is selected."]
    _10 = 2,
    #[doc = "3: Pulse counter input 3 is selected."]
    _11 = 3,
}
impl From<TPS_A> for u8 {
    #[inline(always)]
    fn from(variant: TPS_A) -> Self {
        variant as _
    }
}
impl TPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPS_A {
        match self.bits {
            0 => TPS_A::_00,
            1 => TPS_A::_01,
            2 => TPS_A::_10,
            3 => TPS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == TPS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == TPS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == TPS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == TPS_A::_11
    }
}
#[doc = "Field `TPS` writer - Timer Pin Select"]
pub type TPS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CSR_SPEC, u8, TPS_A, 2, O>;
impl<'a, const O: u8> TPS_W<'a, O> {
    #[doc = "Pulse counter input 0 is selected."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(TPS_A::_00)
    }
    #[doc = "Pulse counter input 1 is selected."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(TPS_A::_01)
    }
    #[doc = "Pulse counter input 2 is selected."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(TPS_A::_10)
    }
    #[doc = "Pulse counter input 3 is selected."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(TPS_A::_11)
    }
}
#[doc = "Field `TIE` reader - Timer Interrupt Enable"]
pub type TIE_R = crate::BitReader<TIE_A>;
#[doc = "Timer Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE_A {
    #[doc = "0: Timer interrupt disabled."]
    _0 = 0,
    #[doc = "1: Timer interrupt enabled."]
    _1 = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::_0,
            true => TIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TIE_A::_1
    }
}
#[doc = "Field `TIE` writer - Timer Interrupt Enable"]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, TIE_A, O>;
impl<'a, const O: u8> TIE_W<'a, O> {
    #[doc = "Timer interrupt disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TIE_A::_0)
    }
    #[doc = "Timer interrupt enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TIE_A::_1)
    }
}
#[doc = "Field `TCF` reader - Timer Compare Flag"]
pub type TCF_R = crate::BitReader<TCF_A>;
#[doc = "Timer Compare Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCF_A {
    #[doc = "0: The value of CNR is not equal to CMR and increments."]
    _0 = 0,
    #[doc = "1: The value of CNR is equal to CMR and increments."]
    _1 = 1,
}
impl From<TCF_A> for bool {
    #[inline(always)]
    fn from(variant: TCF_A) -> Self {
        variant as u8 != 0
    }
}
impl TCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCF_A {
        match self.bits {
            false => TCF_A::_0,
            true => TCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCF_A::_1
    }
}
#[doc = "Field `TCF` writer - Timer Compare Flag"]
pub type TCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, TCF_A, O>;
impl<'a, const O: u8> TCF_W<'a, O> {
    #[doc = "The value of CNR is not equal to CMR and increments."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCF_A::_0)
    }
    #[doc = "The value of CNR is equal to CMR and increments."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Timer Enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer Mode Select"]
    #[inline(always)]
    pub fn tms(&self) -> TMS_R {
        TMS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer Free-Running Counter"]
    #[inline(always)]
    pub fn tfc(&self) -> TFC_R {
        TFC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer Pin Polarity"]
    #[inline(always)]
    pub fn tpp(&self) -> TPP_R {
        TPP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Timer Pin Select"]
    #[inline(always)]
    pub fn tps(&self) -> TPS_R {
        TPS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer Compare Flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TEN_W<0> {
        TEN_W::new(self)
    }
    #[doc = "Bit 1 - Timer Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn tms(&mut self) -> TMS_W<1> {
        TMS_W::new(self)
    }
    #[doc = "Bit 2 - Timer Free-Running Counter"]
    #[inline(always)]
    #[must_use]
    pub fn tfc(&mut self) -> TFC_W<2> {
        TFC_W::new(self)
    }
    #[doc = "Bit 3 - Timer Pin Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn tpp(&mut self) -> TPP_W<3> {
        TPP_W::new(self)
    }
    #[doc = "Bits 4:5 - Timer Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn tps(&mut self) -> TPS_W<4> {
        TPS_W::new(self)
    }
    #[doc = "Bit 6 - Timer Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<6> {
        TIE_W::new(self)
    }
    #[doc = "Bit 7 - Timer Compare Flag"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<7> {
        TCF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Power Timer Control Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr::W](W) writer structure"]
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSR to value 0"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
