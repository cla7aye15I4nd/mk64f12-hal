#[doc = "Register `WAR` reader"]
pub struct R(crate::R<WAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WAR` writer"]
pub struct W(crate::W<WAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WAR_SPEC>;
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
impl From<crate::W<WAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSRW` reader - Time Seconds Register Write"]
pub type TSRW_R = crate::BitReader<TSRW_A>;
#[doc = "Time Seconds Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSRW_A {
    #[doc = "0: Writes to the Time Seconds Register are ignored."]
    _0 = 0,
    #[doc = "1: Writes to the Time Seconds Register complete as normal."]
    _1 = 1,
}
impl From<TSRW_A> for bool {
    #[inline(always)]
    fn from(variant: TSRW_A) -> Self {
        variant as u8 != 0
    }
}
impl TSRW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSRW_A {
        match self.bits {
            false => TSRW_A::_0,
            true => TSRW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSRW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSRW_A::_1
    }
}
#[doc = "Field `TSRW` writer - Time Seconds Register Write"]
pub type TSRW_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAR_SPEC, TSRW_A, O>;
impl<'a, const O: u8> TSRW_W<'a, O> {
    #[doc = "Writes to the Time Seconds Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSRW_A::_0)
    }
    #[doc = "Writes to the Time Seconds Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSRW_A::_1)
    }
}
#[doc = "Field `TPRW` reader - Time Prescaler Register Write"]
pub type TPRW_R = crate::BitReader<TPRW_A>;
#[doc = "Time Prescaler Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPRW_A {
    #[doc = "0: Writes to the Time Prescaler Register are ignored."]
    _0 = 0,
    #[doc = "1: Writes to the Time Prescaler Register complete as normal."]
    _1 = 1,
}
impl From<TPRW_A> for bool {
    #[inline(always)]
    fn from(variant: TPRW_A) -> Self {
        variant as u8 != 0
    }
}
impl TPRW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPRW_A {
        match self.bits {
            false => TPRW_A::_0,
            true => TPRW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TPRW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TPRW_A::_1
    }
}
#[doc = "Field `TPRW` writer - Time Prescaler Register Write"]
pub type TPRW_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAR_SPEC, TPRW_A, O>;
impl<'a, const O: u8> TPRW_W<'a, O> {
    #[doc = "Writes to the Time Prescaler Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPRW_A::_0)
    }
    #[doc = "Writes to the Time Prescaler Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPRW_A::_1)
    }
}
#[doc = "Field `TARW` reader - Time Alarm Register Write"]
pub type TARW_R = crate::BitReader<TARW_A>;
#[doc = "Time Alarm Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TARW_A {
    #[doc = "0: Writes to the Time Alarm Register are ignored."]
    _0 = 0,
    #[doc = "1: Writes to the Time Alarm Register complete as normal."]
    _1 = 1,
}
impl From<TARW_A> for bool {
    #[inline(always)]
    fn from(variant: TARW_A) -> Self {
        variant as u8 != 0
    }
}
impl TARW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TARW_A {
        match self.bits {
            false => TARW_A::_0,
            true => TARW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TARW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TARW_A::_1
    }
}
#[doc = "Field `TARW` writer - Time Alarm Register Write"]
pub type TARW_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAR_SPEC, TARW_A, O>;
impl<'a, const O: u8> TARW_W<'a, O> {
    #[doc = "Writes to the Time Alarm Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TARW_A::_0)
    }
    #[doc = "Writes to the Time Alarm Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TARW_A::_1)
    }
}
#[doc = "Field `TCRW` reader - Time Compensation Register Write"]
pub type TCRW_R = crate::BitReader<TCRW_A>;
#[doc = "Time Compensation Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCRW_A {
    #[doc = "0: Writes to the Time Compensation Register are ignored."]
    _0 = 0,
    #[doc = "1: Writes to the Time Compensation Register complete as normal."]
    _1 = 1,
}
impl From<TCRW_A> for bool {
    #[inline(always)]
    fn from(variant: TCRW_A) -> Self {
        variant as u8 != 0
    }
}
impl TCRW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCRW_A {
        match self.bits {
            false => TCRW_A::_0,
            true => TCRW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCRW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCRW_A::_1
    }
}
#[doc = "Field `TCRW` writer - Time Compensation Register Write"]
pub type TCRW_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAR_SPEC, TCRW_A, O>;
impl<'a, const O: u8> TCRW_W<'a, O> {
    #[doc = "Writes to the Time Compensation Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCRW_A::_0)
    }
    #[doc = "Writes to the Time Compensation Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCRW_A::_1)
    }
}
#[doc = "Field `CRW` reader - Control Register Write"]
pub type CRW_R = crate::BitReader<CRW_A>;
#[doc = "Control Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRW_A {
    #[doc = "0: Writes to the Control Register are ignored."]
    _0 = 0,
    #[doc = "1: Writes to the Control Register complete as normal."]
    _1 = 1,
}
impl From<CRW_A> for bool {
    #[inline(always)]
    fn from(variant: CRW_A) -> Self {
        variant as u8 != 0
    }
}
impl CRW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRW_A {
        match self.bits {
            false => CRW_A::_0,
            true => CRW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRW_A::_1
    }
}
#[doc = "Field `CRW` writer - Control Register Write"]
pub type CRW_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAR_SPEC, CRW_A, O>;
impl<'a, const O: u8> CRW_W<'a, O> {
    #[doc = "Writes to the Control Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRW_A::_0)
    }
    #[doc = "Writes to the Control Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRW_A::_1)
    }
}
#[doc = "Field `SRW` reader - Status Register Write"]
pub type SRW_R = crate::BitReader<SRW_A>;
#[doc = "Status Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRW_A {
    #[doc = "0: Writes to the Status Register are ignored."]
    _0 = 0,
    #[doc = "1: Writes to the Status Register complete as normal."]
    _1 = 1,
}
impl From<SRW_A> for bool {
    #[inline(always)]
    fn from(variant: SRW_A) -> Self {
        variant as u8 != 0
    }
}
impl SRW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRW_A {
        match self.bits {
            false => SRW_A::_0,
            true => SRW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRW_A::_1
    }
}
#[doc = "Field `SRW` writer - Status Register Write"]
pub type SRW_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAR_SPEC, SRW_A, O>;
impl<'a, const O: u8> SRW_W<'a, O> {
    #[doc = "Writes to the Status Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRW_A::_0)
    }
    #[doc = "Writes to the Status Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRW_A::_1)
    }
}
#[doc = "Field `LRW` reader - Lock Register Write"]
pub type LRW_R = crate::BitReader<LRW_A>;
#[doc = "Lock Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LRW_A {
    #[doc = "0: Writes to the Lock Register are ignored."]
    _0 = 0,
    #[doc = "1: Writes to the Lock Register complete as normal."]
    _1 = 1,
}
impl From<LRW_A> for bool {
    #[inline(always)]
    fn from(variant: LRW_A) -> Self {
        variant as u8 != 0
    }
}
impl LRW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRW_A {
        match self.bits {
            false => LRW_A::_0,
            true => LRW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LRW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LRW_A::_1
    }
}
#[doc = "Field `LRW` writer - Lock Register Write"]
pub type LRW_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAR_SPEC, LRW_A, O>;
impl<'a, const O: u8> LRW_W<'a, O> {
    #[doc = "Writes to the Lock Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LRW_A::_0)
    }
    #[doc = "Writes to the Lock Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LRW_A::_1)
    }
}
#[doc = "Field `IERW` reader - Interrupt Enable Register Write"]
pub type IERW_R = crate::BitReader<IERW_A>;
#[doc = "Interrupt Enable Register Write\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IERW_A {
    #[doc = "0: Writes to the Interupt Enable Register are ignored."]
    _0 = 0,
    #[doc = "1: Writes to the Interrupt Enable Register complete as normal."]
    _1 = 1,
}
impl From<IERW_A> for bool {
    #[inline(always)]
    fn from(variant: IERW_A) -> Self {
        variant as u8 != 0
    }
}
impl IERW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IERW_A {
        match self.bits {
            false => IERW_A::_0,
            true => IERW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IERW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IERW_A::_1
    }
}
#[doc = "Field `IERW` writer - Interrupt Enable Register Write"]
pub type IERW_W<'a, const O: u8> = crate::BitWriter<'a, u32, WAR_SPEC, IERW_A, O>;
impl<'a, const O: u8> IERW_W<'a, O> {
    #[doc = "Writes to the Interupt Enable Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IERW_A::_0)
    }
    #[doc = "Writes to the Interrupt Enable Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IERW_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Time Seconds Register Write"]
    #[inline(always)]
    pub fn tsrw(&self) -> TSRW_R {
        TSRW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time Prescaler Register Write"]
    #[inline(always)]
    pub fn tprw(&self) -> TPRW_R {
        TPRW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Time Alarm Register Write"]
    #[inline(always)]
    pub fn tarw(&self) -> TARW_R {
        TARW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time Compensation Register Write"]
    #[inline(always)]
    pub fn tcrw(&self) -> TCRW_R {
        TCRW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Control Register Write"]
    #[inline(always)]
    pub fn crw(&self) -> CRW_R {
        CRW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status Register Write"]
    #[inline(always)]
    pub fn srw(&self) -> SRW_R {
        SRW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Lock Register Write"]
    #[inline(always)]
    pub fn lrw(&self) -> LRW_R {
        LRW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Enable Register Write"]
    #[inline(always)]
    pub fn ierw(&self) -> IERW_R {
        IERW_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time Seconds Register Write"]
    #[inline(always)]
    #[must_use]
    pub fn tsrw(&mut self) -> TSRW_W<0> {
        TSRW_W::new(self)
    }
    #[doc = "Bit 1 - Time Prescaler Register Write"]
    #[inline(always)]
    #[must_use]
    pub fn tprw(&mut self) -> TPRW_W<1> {
        TPRW_W::new(self)
    }
    #[doc = "Bit 2 - Time Alarm Register Write"]
    #[inline(always)]
    #[must_use]
    pub fn tarw(&mut self) -> TARW_W<2> {
        TARW_W::new(self)
    }
    #[doc = "Bit 3 - Time Compensation Register Write"]
    #[inline(always)]
    #[must_use]
    pub fn tcrw(&mut self) -> TCRW_W<3> {
        TCRW_W::new(self)
    }
    #[doc = "Bit 4 - Control Register Write"]
    #[inline(always)]
    #[must_use]
    pub fn crw(&mut self) -> CRW_W<4> {
        CRW_W::new(self)
    }
    #[doc = "Bit 5 - Status Register Write"]
    #[inline(always)]
    #[must_use]
    pub fn srw(&mut self) -> SRW_W<5> {
        SRW_W::new(self)
    }
    #[doc = "Bit 6 - Lock Register Write"]
    #[inline(always)]
    #[must_use]
    pub fn lrw(&mut self) -> LRW_W<6> {
        LRW_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt Enable Register Write"]
    #[inline(always)]
    #[must_use]
    pub fn ierw(&mut self) -> IERW_W<7> {
        IERW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Write Access Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [war](index.html) module"]
pub struct WAR_SPEC;
impl crate::RegisterSpec for WAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [war::R](R) reader structure"]
impl crate::Readable for WAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [war::W](W) writer structure"]
impl crate::Writable for WAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WAR to value 0xff"]
impl crate::Resettable for WAR_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
