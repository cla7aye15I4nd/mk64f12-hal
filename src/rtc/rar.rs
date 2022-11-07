#[doc = "Register `RAR` reader"]
pub struct R(crate::R<RAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RAR` writer"]
pub struct W(crate::W<RAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RAR_SPEC>;
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
impl From<crate::W<RAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSRR` reader - Time Seconds Register Read"]
pub type TSRR_R = crate::BitReader<TSRR_A>;
#[doc = "Time Seconds Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSRR_A {
    #[doc = "0: Reads to the Time Seconds Register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the Time Seconds Register complete as normal."]
    _1 = 1,
}
impl From<TSRR_A> for bool {
    #[inline(always)]
    fn from(variant: TSRR_A) -> Self {
        variant as u8 != 0
    }
}
impl TSRR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSRR_A {
        match self.bits {
            false => TSRR_A::_0,
            true => TSRR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TSRR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TSRR_A::_1
    }
}
#[doc = "Field `TSRR` writer - Time Seconds Register Read"]
pub type TSRR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAR_SPEC, TSRR_A, O>;
impl<'a, const O: u8> TSRR_W<'a, O> {
    #[doc = "Reads to the Time Seconds Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TSRR_A::_0)
    }
    #[doc = "Reads to the Time Seconds Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TSRR_A::_1)
    }
}
#[doc = "Field `TPRR` reader - Time Prescaler Register Read"]
pub type TPRR_R = crate::BitReader<TPRR_A>;
#[doc = "Time Prescaler Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TPRR_A {
    #[doc = "0: Reads to the Time Pprescaler Register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the Time Prescaler Register complete as normal."]
    _1 = 1,
}
impl From<TPRR_A> for bool {
    #[inline(always)]
    fn from(variant: TPRR_A) -> Self {
        variant as u8 != 0
    }
}
impl TPRR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TPRR_A {
        match self.bits {
            false => TPRR_A::_0,
            true => TPRR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TPRR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TPRR_A::_1
    }
}
#[doc = "Field `TPRR` writer - Time Prescaler Register Read"]
pub type TPRR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAR_SPEC, TPRR_A, O>;
impl<'a, const O: u8> TPRR_W<'a, O> {
    #[doc = "Reads to the Time Pprescaler Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TPRR_A::_0)
    }
    #[doc = "Reads to the Time Prescaler Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TPRR_A::_1)
    }
}
#[doc = "Field `TARR` reader - Time Alarm Register Read"]
pub type TARR_R = crate::BitReader<TARR_A>;
#[doc = "Time Alarm Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TARR_A {
    #[doc = "0: Reads to the Time Alarm Register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the Time Alarm Register complete as normal."]
    _1 = 1,
}
impl From<TARR_A> for bool {
    #[inline(always)]
    fn from(variant: TARR_A) -> Self {
        variant as u8 != 0
    }
}
impl TARR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TARR_A {
        match self.bits {
            false => TARR_A::_0,
            true => TARR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TARR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TARR_A::_1
    }
}
#[doc = "Field `TARR` writer - Time Alarm Register Read"]
pub type TARR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAR_SPEC, TARR_A, O>;
impl<'a, const O: u8> TARR_W<'a, O> {
    #[doc = "Reads to the Time Alarm Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TARR_A::_0)
    }
    #[doc = "Reads to the Time Alarm Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TARR_A::_1)
    }
}
#[doc = "Field `TCRR` reader - Time Compensation Register Read"]
pub type TCRR_R = crate::BitReader<TCRR_A>;
#[doc = "Time Compensation Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCRR_A {
    #[doc = "0: Reads to the Time Compensation Register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the Time Compensation Register complete as normal."]
    _1 = 1,
}
impl From<TCRR_A> for bool {
    #[inline(always)]
    fn from(variant: TCRR_A) -> Self {
        variant as u8 != 0
    }
}
impl TCRR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCRR_A {
        match self.bits {
            false => TCRR_A::_0,
            true => TCRR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCRR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCRR_A::_1
    }
}
#[doc = "Field `TCRR` writer - Time Compensation Register Read"]
pub type TCRR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAR_SPEC, TCRR_A, O>;
impl<'a, const O: u8> TCRR_W<'a, O> {
    #[doc = "Reads to the Time Compensation Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCRR_A::_0)
    }
    #[doc = "Reads to the Time Compensation Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCRR_A::_1)
    }
}
#[doc = "Field `CRR` reader - Control Register Read"]
pub type CRR_R = crate::BitReader<CRR_A>;
#[doc = "Control Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRR_A {
    #[doc = "0: Reads to the Control Register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the Control Register complete as normal."]
    _1 = 1,
}
impl From<CRR_A> for bool {
    #[inline(always)]
    fn from(variant: CRR_A) -> Self {
        variant as u8 != 0
    }
}
impl CRR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRR_A {
        match self.bits {
            false => CRR_A::_0,
            true => CRR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRR_A::_1
    }
}
#[doc = "Field `CRR` writer - Control Register Read"]
pub type CRR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAR_SPEC, CRR_A, O>;
impl<'a, const O: u8> CRR_W<'a, O> {
    #[doc = "Reads to the Control Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRR_A::_0)
    }
    #[doc = "Reads to the Control Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRR_A::_1)
    }
}
#[doc = "Field `SRR` reader - Status Register Read"]
pub type SRR_R = crate::BitReader<SRR_A>;
#[doc = "Status Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRR_A {
    #[doc = "0: Reads to the Status Register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the Status Register complete as normal."]
    _1 = 1,
}
impl From<SRR_A> for bool {
    #[inline(always)]
    fn from(variant: SRR_A) -> Self {
        variant as u8 != 0
    }
}
impl SRR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRR_A {
        match self.bits {
            false => SRR_A::_0,
            true => SRR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRR_A::_1
    }
}
#[doc = "Field `SRR` writer - Status Register Read"]
pub type SRR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAR_SPEC, SRR_A, O>;
impl<'a, const O: u8> SRR_W<'a, O> {
    #[doc = "Reads to the Status Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SRR_A::_0)
    }
    #[doc = "Reads to the Status Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SRR_A::_1)
    }
}
#[doc = "Field `LRR` reader - Lock Register Read"]
pub type LRR_R = crate::BitReader<LRR_A>;
#[doc = "Lock Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LRR_A {
    #[doc = "0: Reads to the Lock Register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the Lock Register complete as normal."]
    _1 = 1,
}
impl From<LRR_A> for bool {
    #[inline(always)]
    fn from(variant: LRR_A) -> Self {
        variant as u8 != 0
    }
}
impl LRR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LRR_A {
        match self.bits {
            false => LRR_A::_0,
            true => LRR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LRR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LRR_A::_1
    }
}
#[doc = "Field `LRR` writer - Lock Register Read"]
pub type LRR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAR_SPEC, LRR_A, O>;
impl<'a, const O: u8> LRR_W<'a, O> {
    #[doc = "Reads to the Lock Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LRR_A::_0)
    }
    #[doc = "Reads to the Lock Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LRR_A::_1)
    }
}
#[doc = "Field `IERR` reader - Interrupt Enable Register Read"]
pub type IERR_R = crate::BitReader<IERR_A>;
#[doc = "Interrupt Enable Register Read\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IERR_A {
    #[doc = "0: Reads to the Interrupt Enable Register are ignored."]
    _0 = 0,
    #[doc = "1: Reads to the Interrupt Enable Register complete as normal."]
    _1 = 1,
}
impl From<IERR_A> for bool {
    #[inline(always)]
    fn from(variant: IERR_A) -> Self {
        variant as u8 != 0
    }
}
impl IERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IERR_A {
        match self.bits {
            false => IERR_A::_0,
            true => IERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IERR_A::_1
    }
}
#[doc = "Field `IERR` writer - Interrupt Enable Register Read"]
pub type IERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RAR_SPEC, IERR_A, O>;
impl<'a, const O: u8> IERR_W<'a, O> {
    #[doc = "Reads to the Interrupt Enable Register are ignored."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IERR_A::_0)
    }
    #[doc = "Reads to the Interrupt Enable Register complete as normal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IERR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Time Seconds Register Read"]
    #[inline(always)]
    pub fn tsrr(&self) -> TSRR_R {
        TSRR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Time Prescaler Register Read"]
    #[inline(always)]
    pub fn tprr(&self) -> TPRR_R {
        TPRR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Time Alarm Register Read"]
    #[inline(always)]
    pub fn tarr(&self) -> TARR_R {
        TARR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Time Compensation Register Read"]
    #[inline(always)]
    pub fn tcrr(&self) -> TCRR_R {
        TCRR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Control Register Read"]
    #[inline(always)]
    pub fn crr(&self) -> CRR_R {
        CRR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status Register Read"]
    #[inline(always)]
    pub fn srr(&self) -> SRR_R {
        SRR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Lock Register Read"]
    #[inline(always)]
    pub fn lrr(&self) -> LRR_R {
        LRR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Enable Register Read"]
    #[inline(always)]
    pub fn ierr(&self) -> IERR_R {
        IERR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Time Seconds Register Read"]
    #[inline(always)]
    #[must_use]
    pub fn tsrr(&mut self) -> TSRR_W<0> {
        TSRR_W::new(self)
    }
    #[doc = "Bit 1 - Time Prescaler Register Read"]
    #[inline(always)]
    #[must_use]
    pub fn tprr(&mut self) -> TPRR_W<1> {
        TPRR_W::new(self)
    }
    #[doc = "Bit 2 - Time Alarm Register Read"]
    #[inline(always)]
    #[must_use]
    pub fn tarr(&mut self) -> TARR_W<2> {
        TARR_W::new(self)
    }
    #[doc = "Bit 3 - Time Compensation Register Read"]
    #[inline(always)]
    #[must_use]
    pub fn tcrr(&mut self) -> TCRR_W<3> {
        TCRR_W::new(self)
    }
    #[doc = "Bit 4 - Control Register Read"]
    #[inline(always)]
    #[must_use]
    pub fn crr(&mut self) -> CRR_W<4> {
        CRR_W::new(self)
    }
    #[doc = "Bit 5 - Status Register Read"]
    #[inline(always)]
    #[must_use]
    pub fn srr(&mut self) -> SRR_W<5> {
        SRR_W::new(self)
    }
    #[doc = "Bit 6 - Lock Register Read"]
    #[inline(always)]
    #[must_use]
    pub fn lrr(&mut self) -> LRR_W<6> {
        LRR_W::new(self)
    }
    #[doc = "Bit 7 - Interrupt Enable Register Read"]
    #[inline(always)]
    #[must_use]
    pub fn ierr(&mut self) -> IERR_W<7> {
        IERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Read Access Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rar](index.html) module"]
pub struct RAR_SPEC;
impl crate::RegisterSpec for RAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rar::R](R) reader structure"]
impl crate::Readable for RAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rar::W](W) writer structure"]
impl crate::Writable for RAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RAR to value 0xff"]
impl crate::Resettable for RAR_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
