#[doc = "Register `CRS%s` reader"]
pub struct R(crate::R<CRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRS%s` writer"]
pub struct W(crate::W<CRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRS_SPEC>;
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
impl From<crate::W<CRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PARK` reader - Park"]
pub type PARK_R = crate::FieldReader<u8, PARK_A>;
#[doc = "Park\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PARK_A {
    #[doc = "0: Park on master port M0"]
    _000 = 0,
    #[doc = "1: Park on master port M1"]
    _001 = 1,
    #[doc = "2: Park on master port M2"]
    _010 = 2,
    #[doc = "3: Park on master port M3"]
    _011 = 3,
    #[doc = "4: Park on master port M4"]
    _100 = 4,
    #[doc = "5: Park on master port M5"]
    _101 = 5,
    #[doc = "6: Park on master port M6"]
    _110 = 6,
    #[doc = "7: Park on master port M7"]
    _111 = 7,
}
impl From<PARK_A> for u8 {
    #[inline(always)]
    fn from(variant: PARK_A) -> Self {
        variant as _
    }
}
impl PARK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PARK_A {
        match self.bits {
            0 => PARK_A::_000,
            1 => PARK_A::_001,
            2 => PARK_A::_010,
            3 => PARK_A::_011,
            4 => PARK_A::_100,
            5 => PARK_A::_101,
            6 => PARK_A::_110,
            7 => PARK_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == PARK_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == PARK_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == PARK_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == PARK_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == PARK_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == PARK_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == PARK_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == PARK_A::_111
    }
}
#[doc = "Field `PARK` writer - Park"]
pub type PARK_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CRS_SPEC, u8, PARK_A, 3, O>;
impl<'a, const O: u8> PARK_W<'a, O> {
    #[doc = "Park on master port M0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(PARK_A::_000)
    }
    #[doc = "Park on master port M1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(PARK_A::_001)
    }
    #[doc = "Park on master port M2"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(PARK_A::_010)
    }
    #[doc = "Park on master port M3"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(PARK_A::_011)
    }
    #[doc = "Park on master port M4"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(PARK_A::_100)
    }
    #[doc = "Park on master port M5"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(PARK_A::_101)
    }
    #[doc = "Park on master port M6"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(PARK_A::_110)
    }
    #[doc = "Park on master port M7"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(PARK_A::_111)
    }
}
#[doc = "Field `PCTL` reader - Parking Control"]
pub type PCTL_R = crate::FieldReader<u8, PCTL_A>;
#[doc = "Parking Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCTL_A {
    #[doc = "0: When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK field"]
    _00 = 0,
    #[doc = "1: When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port"]
    _01 = 1,
    #[doc = "2: When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state"]
    _10 = 2,
}
impl From<PCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: PCTL_A) -> Self {
        variant as _
    }
}
impl PCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PCTL_A> {
        match self.bits {
            0 => Some(PCTL_A::_00),
            1 => Some(PCTL_A::_01),
            2 => Some(PCTL_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PCTL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PCTL_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PCTL_A::_10
    }
}
#[doc = "Field `PCTL` writer - Parking Control"]
pub type PCTL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRS_SPEC, u8, PCTL_A, 2, O>;
impl<'a, const O: u8> PCTL_W<'a, O> {
    #[doc = "When no master makes a request, the arbiter parks the slave port on the master port defined by the PARK field"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PCTL_A::_00)
    }
    #[doc = "When no master makes a request, the arbiter parks the slave port on the last master to be in control of the slave port"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PCTL_A::_01)
    }
    #[doc = "When no master makes a request, the slave port is not parked on a master and the arbiter drives all outputs to a constant safe state"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PCTL_A::_10)
    }
}
#[doc = "Field `ARB` reader - Arbitration Mode"]
pub type ARB_R = crate::FieldReader<u8, ARB_A>;
#[doc = "Arbitration Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARB_A {
    #[doc = "0: Fixed priority"]
    _00 = 0,
    #[doc = "1: Round-robin, or rotating, priority"]
    _01 = 1,
}
impl From<ARB_A> for u8 {
    #[inline(always)]
    fn from(variant: ARB_A) -> Self {
        variant as _
    }
}
impl ARB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ARB_A> {
        match self.bits {
            0 => Some(ARB_A::_00),
            1 => Some(ARB_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ARB_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ARB_A::_01
    }
}
#[doc = "Field `ARB` writer - Arbitration Mode"]
pub type ARB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRS_SPEC, u8, ARB_A, 2, O>;
impl<'a, const O: u8> ARB_W<'a, O> {
    #[doc = "Fixed priority"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ARB_A::_00)
    }
    #[doc = "Round-robin, or rotating, priority"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ARB_A::_01)
    }
}
#[doc = "Field `HLP` reader - Halt Low Priority"]
pub type HLP_R = crate::BitReader<HLP_A>;
#[doc = "Halt Low Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HLP_A {
    #[doc = "0: The low power mode request has the highest priority for arbitration on this slave port"]
    _0 = 0,
    #[doc = "1: The low power mode request has the lowest initial priority for arbitration on this slave port"]
    _1 = 1,
}
impl From<HLP_A> for bool {
    #[inline(always)]
    fn from(variant: HLP_A) -> Self {
        variant as u8 != 0
    }
}
impl HLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HLP_A {
        match self.bits {
            false => HLP_A::_0,
            true => HLP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == HLP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == HLP_A::_1
    }
}
#[doc = "Field `HLP` writer - Halt Low Priority"]
pub type HLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRS_SPEC, HLP_A, O>;
impl<'a, const O: u8> HLP_W<'a, O> {
    #[doc = "The low power mode request has the highest priority for arbitration on this slave port"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(HLP_A::_0)
    }
    #[doc = "The low power mode request has the lowest initial priority for arbitration on this slave port"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(HLP_A::_1)
    }
}
#[doc = "Field `RO` reader - Read Only"]
pub type RO_R = crate::BitReader<RO_A>;
#[doc = "Read Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RO_A {
    #[doc = "0: The slave port's registers are writeable"]
    _0 = 0,
    #[doc = "1: The slave port's registers are read-only and cannot be written. Attempted writes have no effect on the registers and result in a bus error response."]
    _1 = 1,
}
impl From<RO_A> for bool {
    #[inline(always)]
    fn from(variant: RO_A) -> Self {
        variant as u8 != 0
    }
}
impl RO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RO_A {
        match self.bits {
            false => RO_A::_0,
            true => RO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RO_A::_1
    }
}
#[doc = "Field `RO` writer - Read Only"]
pub type RO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CRS_SPEC, RO_A, O>;
impl<'a, const O: u8> RO_W<'a, O> {
    #[doc = "The slave port's registers are writeable"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RO_A::_0)
    }
    #[doc = "The slave port's registers are read-only and cannot be written. Attempted writes have no effect on the registers and result in a bus error response."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RO_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Park"]
    #[inline(always)]
    pub fn park(&self) -> PARK_R {
        PARK_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - Parking Control"]
    #[inline(always)]
    pub fn pctl(&self) -> PCTL_R {
        PCTL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Arbitration Mode"]
    #[inline(always)]
    pub fn arb(&self) -> ARB_R {
        ARB_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 30 - Halt Low Priority"]
    #[inline(always)]
    pub fn hlp(&self) -> HLP_R {
        HLP_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Read Only"]
    #[inline(always)]
    pub fn ro(&self) -> RO_R {
        RO_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Park"]
    #[inline(always)]
    #[must_use]
    pub fn park(&mut self) -> PARK_W<0> {
        PARK_W::new(self)
    }
    #[doc = "Bits 4:5 - Parking Control"]
    #[inline(always)]
    #[must_use]
    pub fn pctl(&mut self) -> PCTL_W<4> {
        PCTL_W::new(self)
    }
    #[doc = "Bits 8:9 - Arbitration Mode"]
    #[inline(always)]
    #[must_use]
    pub fn arb(&mut self) -> ARB_W<8> {
        ARB_W::new(self)
    }
    #[doc = "Bit 30 - Halt Low Priority"]
    #[inline(always)]
    #[must_use]
    pub fn hlp(&mut self) -> HLP_W<30> {
        HLP_W::new(self)
    }
    #[doc = "Bit 31 - Read Only"]
    #[inline(always)]
    #[must_use]
    pub fn ro(&mut self) -> RO_W<31> {
        RO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crs](index.html) module"]
pub struct CRS_SPEC;
impl crate::RegisterSpec for CRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crs::R](R) reader structure"]
impl crate::Readable for CRS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crs::W](W) writer structure"]
impl crate::Writable for CRS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CRS%s to value 0"]
impl crate::Resettable for CRS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
