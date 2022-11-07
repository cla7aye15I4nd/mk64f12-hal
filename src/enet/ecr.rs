#[doc = "Register `ECR` reader"]
pub struct R(crate::R<ECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECR` writer"]
pub struct W(crate::W<ECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECR_SPEC>;
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
impl From<crate::W<ECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET` reader - Ethernet MAC Reset"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - Ethernet MAC Reset"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, bool, O>;
#[doc = "Field `ETHEREN` reader - Ethernet Enable"]
pub type ETHEREN_R = crate::BitReader<ETHEREN_A>;
#[doc = "Ethernet Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETHEREN_A {
    #[doc = "0: Reception immediately stops and transmission stops after a bad CRC is appended to any currently transmitted frame."]
    _0 = 0,
    #[doc = "1: MAC is enabled, and reception and transmission are possible."]
    _1 = 1,
}
impl From<ETHEREN_A> for bool {
    #[inline(always)]
    fn from(variant: ETHEREN_A) -> Self {
        variant as u8 != 0
    }
}
impl ETHEREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETHEREN_A {
        match self.bits {
            false => ETHEREN_A::_0,
            true => ETHEREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ETHEREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ETHEREN_A::_1
    }
}
#[doc = "Field `ETHEREN` writer - Ethernet Enable"]
pub type ETHEREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, ETHEREN_A, O>;
impl<'a, const O: u8> ETHEREN_W<'a, O> {
    #[doc = "Reception immediately stops and transmission stops after a bad CRC is appended to any currently transmitted frame."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ETHEREN_A::_0)
    }
    #[doc = "MAC is enabled, and reception and transmission are possible."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ETHEREN_A::_1)
    }
}
#[doc = "Field `MAGICEN` reader - Magic Packet Detection Enable"]
pub type MAGICEN_R = crate::BitReader<MAGICEN_A>;
#[doc = "Magic Packet Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MAGICEN_A {
    #[doc = "0: Magic detection logic disabled."]
    _0 = 0,
    #[doc = "1: The MAC core detects magic packets and asserts EIR\\[WAKEUP\\]
when a frame is detected."]
    _1 = 1,
}
impl From<MAGICEN_A> for bool {
    #[inline(always)]
    fn from(variant: MAGICEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MAGICEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MAGICEN_A {
        match self.bits {
            false => MAGICEN_A::_0,
            true => MAGICEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MAGICEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MAGICEN_A::_1
    }
}
#[doc = "Field `MAGICEN` writer - Magic Packet Detection Enable"]
pub type MAGICEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, MAGICEN_A, O>;
impl<'a, const O: u8> MAGICEN_W<'a, O> {
    #[doc = "Magic detection logic disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MAGICEN_A::_0)
    }
    #[doc = "The MAC core detects magic packets and asserts EIR\\[WAKEUP\\]
when a frame is detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MAGICEN_A::_1)
    }
}
#[doc = "Field `SLEEP` reader - Sleep Mode Enable"]
pub type SLEEP_R = crate::BitReader<SLEEP_A>;
#[doc = "Sleep Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEEP_A {
    #[doc = "0: Normal operating mode."]
    _0 = 0,
    #[doc = "1: Sleep mode."]
    _1 = 1,
}
impl From<SLEEP_A> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_A) -> Self {
        variant as u8 != 0
    }
}
impl SLEEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLEEP_A {
        match self.bits {
            false => SLEEP_A::_0,
            true => SLEEP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLEEP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLEEP_A::_1
    }
}
#[doc = "Field `SLEEP` writer - Sleep Mode Enable"]
pub type SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, SLEEP_A, O>;
impl<'a, const O: u8> SLEEP_W<'a, O> {
    #[doc = "Normal operating mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLEEP_A::_0)
    }
    #[doc = "Sleep mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLEEP_A::_1)
    }
}
#[doc = "Field `EN1588` reader - EN1588 Enable"]
pub type EN1588_R = crate::BitReader<EN1588_A>;
#[doc = "EN1588 Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN1588_A {
    #[doc = "0: Legacy FEC buffer descriptors and functions enabled."]
    _0 = 0,
    #[doc = "1: Enhanced frame time-stamping functions enabled."]
    _1 = 1,
}
impl From<EN1588_A> for bool {
    #[inline(always)]
    fn from(variant: EN1588_A) -> Self {
        variant as u8 != 0
    }
}
impl EN1588_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN1588_A {
        match self.bits {
            false => EN1588_A::_0,
            true => EN1588_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN1588_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN1588_A::_1
    }
}
#[doc = "Field `EN1588` writer - EN1588 Enable"]
pub type EN1588_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, EN1588_A, O>;
impl<'a, const O: u8> EN1588_W<'a, O> {
    #[doc = "Legacy FEC buffer descriptors and functions enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN1588_A::_0)
    }
    #[doc = "Enhanced frame time-stamping functions enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN1588_A::_1)
    }
}
#[doc = "Field `DBGEN` reader - Debug Enable"]
pub type DBGEN_R = crate::BitReader<DBGEN_A>;
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGEN_A {
    #[doc = "0: MAC continues operation in debug mode."]
    _0 = 0,
    #[doc = "1: MAC enters hardware freeze mode when the processor is in debug mode."]
    _1 = 1,
}
impl From<DBGEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGEN_A {
        match self.bits {
            false => DBGEN_A::_0,
            true => DBGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBGEN_A::_1
    }
}
#[doc = "Field `DBGEN` writer - Debug Enable"]
pub type DBGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, DBGEN_A, O>;
impl<'a, const O: u8> DBGEN_W<'a, O> {
    #[doc = "MAC continues operation in debug mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBGEN_A::_0)
    }
    #[doc = "MAC enters hardware freeze mode when the processor is in debug mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBGEN_A::_1)
    }
}
#[doc = "Field `STOPEN` reader - STOPEN Signal Control"]
pub type STOPEN_R = crate::BitReader<bool>;
#[doc = "Field `STOPEN` writer - STOPEN Signal Control"]
pub type STOPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, bool, O>;
#[doc = "Field `DBSWP` reader - Descriptor Byte Swapping Enable"]
pub type DBSWP_R = crate::BitReader<DBSWP_A>;
#[doc = "Descriptor Byte Swapping Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBSWP_A {
    #[doc = "0: The buffer descriptor bytes are not swapped to support big-endian devices."]
    _0 = 0,
    #[doc = "1: The buffer descriptor bytes are swapped to support little-endian devices."]
    _1 = 1,
}
impl From<DBSWP_A> for bool {
    #[inline(always)]
    fn from(variant: DBSWP_A) -> Self {
        variant as u8 != 0
    }
}
impl DBSWP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBSWP_A {
        match self.bits {
            false => DBSWP_A::_0,
            true => DBSWP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBSWP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBSWP_A::_1
    }
}
#[doc = "Field `DBSWP` writer - Descriptor Byte Swapping Enable"]
pub type DBSWP_W<'a, const O: u8> = crate::BitWriter<'a, u32, ECR_SPEC, DBSWP_A, O>;
impl<'a, const O: u8> DBSWP_W<'a, O> {
    #[doc = "The buffer descriptor bytes are not swapped to support big-endian devices."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DBSWP_A::_0)
    }
    #[doc = "The buffer descriptor bytes are swapped to support little-endian devices."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DBSWP_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Ethernet MAC Reset"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ethernet Enable"]
    #[inline(always)]
    pub fn etheren(&self) -> ETHEREN_R {
        ETHEREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Magic Packet Detection Enable"]
    #[inline(always)]
    pub fn magicen(&self) -> MAGICEN_R {
        MAGICEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Sleep Mode Enable"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - EN1588 Enable"]
    #[inline(always)]
    pub fn en1588(&self) -> EN1588_R {
        EN1588_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Debug Enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - STOPEN Signal Control"]
    #[inline(always)]
    pub fn stopen(&self) -> STOPEN_R {
        STOPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Descriptor Byte Swapping Enable"]
    #[inline(always)]
    pub fn dbswp(&self) -> DBSWP_R {
        DBSWP_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ethernet MAC Reset"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<0> {
        RESET_W::new(self)
    }
    #[doc = "Bit 1 - Ethernet Enable"]
    #[inline(always)]
    #[must_use]
    pub fn etheren(&mut self) -> ETHEREN_W<1> {
        ETHEREN_W::new(self)
    }
    #[doc = "Bit 2 - Magic Packet Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn magicen(&mut self) -> MAGICEN_W<2> {
        MAGICEN_W::new(self)
    }
    #[doc = "Bit 3 - Sleep Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<3> {
        SLEEP_W::new(self)
    }
    #[doc = "Bit 4 - EN1588 Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en1588(&mut self) -> EN1588_W<4> {
        EN1588_W::new(self)
    }
    #[doc = "Bit 6 - Debug Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbgen(&mut self) -> DBGEN_W<6> {
        DBGEN_W::new(self)
    }
    #[doc = "Bit 7 - STOPEN Signal Control"]
    #[inline(always)]
    #[must_use]
    pub fn stopen(&mut self) -> STOPEN_W<7> {
        STOPEN_W::new(self)
    }
    #[doc = "Bit 8 - Descriptor Byte Swapping Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbswp(&mut self) -> DBSWP_W<8> {
        DBSWP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Ethernet Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecr](index.html) module"]
pub struct ECR_SPEC;
impl crate::RegisterSpec for ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecr::R](R) reader structure"]
impl crate::Readable for ECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecr::W](W) writer structure"]
impl crate::Writable for ECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECR to value 0xf000_0000"]
impl crate::Resettable for ECR_SPEC {
    const RESET_VALUE: Self::Ux = 0xf000_0000;
}
