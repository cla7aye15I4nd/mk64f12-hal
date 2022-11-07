#[doc = "Register `EIMR` reader"]
pub struct R(crate::R<EIMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EIMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EIMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EIMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EIMR` writer"]
pub struct W(crate::W<EIMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EIMR_SPEC>;
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
impl From<crate::W<EIMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EIMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS_TIMER` reader - TS_TIMER Interrupt Mask"]
pub type TS_TIMER_R = crate::BitReader<bool>;
#[doc = "Field `TS_TIMER` writer - TS_TIMER Interrupt Mask"]
pub type TS_TIMER_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, bool, O>;
#[doc = "Field `TS_AVAIL` reader - TS_AVAIL Interrupt Mask"]
pub type TS_AVAIL_R = crate::BitReader<bool>;
#[doc = "Field `TS_AVAIL` writer - TS_AVAIL Interrupt Mask"]
pub type TS_AVAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, bool, O>;
#[doc = "Field `WAKEUP` reader - WAKEUP Interrupt Mask"]
pub type WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUP` writer - WAKEUP Interrupt Mask"]
pub type WAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, bool, O>;
#[doc = "Field `PLR` reader - PLR Interrupt Mask"]
pub type PLR_R = crate::BitReader<bool>;
#[doc = "Field `PLR` writer - PLR Interrupt Mask"]
pub type PLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, bool, O>;
#[doc = "Field `UN` reader - UN Interrupt Mask"]
pub type UN_R = crate::BitReader<bool>;
#[doc = "Field `UN` writer - UN Interrupt Mask"]
pub type UN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, bool, O>;
#[doc = "Field `RL` reader - RL Interrupt Mask"]
pub type RL_R = crate::BitReader<bool>;
#[doc = "Field `RL` writer - RL Interrupt Mask"]
pub type RL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, bool, O>;
#[doc = "Field `LC` reader - LC Interrupt Mask"]
pub type LC_R = crate::BitReader<bool>;
#[doc = "Field `LC` writer - LC Interrupt Mask"]
pub type LC_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, bool, O>;
#[doc = "Field `EBERR` reader - EBERR Interrupt Mask"]
pub type EBERR_R = crate::BitReader<bool>;
#[doc = "Field `EBERR` writer - EBERR Interrupt Mask"]
pub type EBERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, bool, O>;
#[doc = "Field `MII` reader - MII Interrupt Mask"]
pub type MII_R = crate::BitReader<bool>;
#[doc = "Field `MII` writer - MII Interrupt Mask"]
pub type MII_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, bool, O>;
#[doc = "Field `RXB` reader - RXB Interrupt Mask"]
pub type RXB_R = crate::BitReader<bool>;
#[doc = "Field `RXB` writer - RXB Interrupt Mask"]
pub type RXB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, bool, O>;
#[doc = "Field `RXF` reader - RXF Interrupt Mask"]
pub type RXF_R = crate::BitReader<bool>;
#[doc = "Field `RXF` writer - RXF Interrupt Mask"]
pub type RXF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, bool, O>;
#[doc = "Field `TXB` reader - TXB Interrupt Mask"]
pub type TXB_R = crate::BitReader<TXB_A>;
#[doc = "TXB Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXB_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    _0 = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    _1 = 1,
}
impl From<TXB_A> for bool {
    #[inline(always)]
    fn from(variant: TXB_A) -> Self {
        variant as u8 != 0
    }
}
impl TXB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXB_A {
        match self.bits {
            false => TXB_A::_0,
            true => TXB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXB_A::_1
    }
}
#[doc = "Field `TXB` writer - TXB Interrupt Mask"]
pub type TXB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, TXB_A, O>;
impl<'a, const O: u8> TXB_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXB_A::_0)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXB_A::_1)
    }
}
#[doc = "Field `TXF` reader - TXF Interrupt Mask"]
pub type TXF_R = crate::BitReader<TXF_A>;
#[doc = "TXF Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXF_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    _0 = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    _1 = 1,
}
impl From<TXF_A> for bool {
    #[inline(always)]
    fn from(variant: TXF_A) -> Self {
        variant as u8 != 0
    }
}
impl TXF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXF_A {
        match self.bits {
            false => TXF_A::_0,
            true => TXF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TXF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TXF_A::_1
    }
}
#[doc = "Field `TXF` writer - TXF Interrupt Mask"]
pub type TXF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, TXF_A, O>;
impl<'a, const O: u8> TXF_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TXF_A::_0)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TXF_A::_1)
    }
}
#[doc = "Field `GRA` reader - GRA Interrupt Mask"]
pub type GRA_R = crate::BitReader<GRA_A>;
#[doc = "GRA Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GRA_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    _0 = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    _1 = 1,
}
impl From<GRA_A> for bool {
    #[inline(always)]
    fn from(variant: GRA_A) -> Self {
        variant as u8 != 0
    }
}
impl GRA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GRA_A {
        match self.bits {
            false => GRA_A::_0,
            true => GRA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == GRA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == GRA_A::_1
    }
}
#[doc = "Field `GRA` writer - GRA Interrupt Mask"]
pub type GRA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, GRA_A, O>;
impl<'a, const O: u8> GRA_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GRA_A::_0)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GRA_A::_1)
    }
}
#[doc = "Field `BABT` reader - BABT Interrupt Mask"]
pub type BABT_R = crate::BitReader<BABT_A>;
#[doc = "BABT Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BABT_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    _0 = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    _1 = 1,
}
impl From<BABT_A> for bool {
    #[inline(always)]
    fn from(variant: BABT_A) -> Self {
        variant as u8 != 0
    }
}
impl BABT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BABT_A {
        match self.bits {
            false => BABT_A::_0,
            true => BABT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BABT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BABT_A::_1
    }
}
#[doc = "Field `BABT` writer - BABT Interrupt Mask"]
pub type BABT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, BABT_A, O>;
impl<'a, const O: u8> BABT_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BABT_A::_0)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BABT_A::_1)
    }
}
#[doc = "Field `BABR` reader - BABR Interrupt Mask"]
pub type BABR_R = crate::BitReader<BABR_A>;
#[doc = "BABR Interrupt Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BABR_A {
    #[doc = "0: The corresponding interrupt source is masked."]
    _0 = 0,
    #[doc = "1: The corresponding interrupt source is not masked."]
    _1 = 1,
}
impl From<BABR_A> for bool {
    #[inline(always)]
    fn from(variant: BABR_A) -> Self {
        variant as u8 != 0
    }
}
impl BABR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BABR_A {
        match self.bits {
            false => BABR_A::_0,
            true => BABR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BABR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BABR_A::_1
    }
}
#[doc = "Field `BABR` writer - BABR Interrupt Mask"]
pub type BABR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIMR_SPEC, BABR_A, O>;
impl<'a, const O: u8> BABR_W<'a, O> {
    #[doc = "The corresponding interrupt source is masked."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BABR_A::_0)
    }
    #[doc = "The corresponding interrupt source is not masked."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BABR_A::_1)
    }
}
impl R {
    #[doc = "Bit 15 - TS_TIMER Interrupt Mask"]
    #[inline(always)]
    pub fn ts_timer(&self) -> TS_TIMER_R {
        TS_TIMER_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TS_AVAIL Interrupt Mask"]
    #[inline(always)]
    pub fn ts_avail(&self) -> TS_AVAIL_R {
        TS_AVAIL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - WAKEUP Interrupt Mask"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PLR Interrupt Mask"]
    #[inline(always)]
    pub fn plr(&self) -> PLR_R {
        PLR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UN Interrupt Mask"]
    #[inline(always)]
    pub fn un(&self) -> UN_R {
        UN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - RL Interrupt Mask"]
    #[inline(always)]
    pub fn rl(&self) -> RL_R {
        RL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LC Interrupt Mask"]
    #[inline(always)]
    pub fn lc(&self) -> LC_R {
        LC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - EBERR Interrupt Mask"]
    #[inline(always)]
    pub fn eberr(&self) -> EBERR_R {
        EBERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MII Interrupt Mask"]
    #[inline(always)]
    pub fn mii(&self) -> MII_R {
        MII_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - RXB Interrupt Mask"]
    #[inline(always)]
    pub fn rxb(&self) -> RXB_R {
        RXB_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - RXF Interrupt Mask"]
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TXB Interrupt Mask"]
    #[inline(always)]
    pub fn txb(&self) -> TXB_R {
        TXB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TXF Interrupt Mask"]
    #[inline(always)]
    pub fn txf(&self) -> TXF_R {
        TXF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - GRA Interrupt Mask"]
    #[inline(always)]
    pub fn gra(&self) -> GRA_R {
        GRA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - BABT Interrupt Mask"]
    #[inline(always)]
    pub fn babt(&self) -> BABT_R {
        BABT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - BABR Interrupt Mask"]
    #[inline(always)]
    pub fn babr(&self) -> BABR_R {
        BABR_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - TS_TIMER Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ts_timer(&mut self) -> TS_TIMER_W<15> {
        TS_TIMER_W::new(self)
    }
    #[doc = "Bit 16 - TS_AVAIL Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn ts_avail(&mut self) -> TS_AVAIL_W<16> {
        TS_AVAIL_W::new(self)
    }
    #[doc = "Bit 17 - WAKEUP Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WAKEUP_W<17> {
        WAKEUP_W::new(self)
    }
    #[doc = "Bit 18 - PLR Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn plr(&mut self) -> PLR_W<18> {
        PLR_W::new(self)
    }
    #[doc = "Bit 19 - UN Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn un(&mut self) -> UN_W<19> {
        UN_W::new(self)
    }
    #[doc = "Bit 20 - RL Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rl(&mut self) -> RL_W<20> {
        RL_W::new(self)
    }
    #[doc = "Bit 21 - LC Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn lc(&mut self) -> LC_W<21> {
        LC_W::new(self)
    }
    #[doc = "Bit 22 - EBERR Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn eberr(&mut self) -> EBERR_W<22> {
        EBERR_W::new(self)
    }
    #[doc = "Bit 23 - MII Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mii(&mut self) -> MII_W<23> {
        MII_W::new(self)
    }
    #[doc = "Bit 24 - RXB Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxb(&mut self) -> RXB_W<24> {
        RXB_W::new(self)
    }
    #[doc = "Bit 25 - RXF Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn rxf(&mut self) -> RXF_W<25> {
        RXF_W::new(self)
    }
    #[doc = "Bit 26 - TXB Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn txb(&mut self) -> TXB_W<26> {
        TXB_W::new(self)
    }
    #[doc = "Bit 27 - TXF Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn txf(&mut self) -> TXF_W<27> {
        TXF_W::new(self)
    }
    #[doc = "Bit 28 - GRA Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn gra(&mut self) -> GRA_W<28> {
        GRA_W::new(self)
    }
    #[doc = "Bit 29 - BABT Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn babt(&mut self) -> BABT_W<29> {
        BABT_W::new(self)
    }
    #[doc = "Bit 30 - BABR Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn babr(&mut self) -> BABR_W<30> {
        BABR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eimr](index.html) module"]
pub struct EIMR_SPEC;
impl crate::RegisterSpec for EIMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eimr::R](R) reader structure"]
impl crate::Readable for EIMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eimr::W](W) writer structure"]
impl crate::Writable for EIMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EIMR to value 0"]
impl crate::Resettable for EIMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
