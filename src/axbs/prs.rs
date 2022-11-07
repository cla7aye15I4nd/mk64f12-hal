#[doc = "Register `PRS%s` reader"]
pub struct R(crate::R<PRS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRS%s` writer"]
pub struct W(crate::W<PRS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRS_SPEC>;
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
impl From<crate::W<PRS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M0` reader - Master 0 Priority. Sets the arbitration priority for this port on the associated slave port."]
pub type M0_R = crate::FieldReader<u8, M0_A>;
#[doc = "Master 0 Priority. Sets the arbitration priority for this port on the associated slave port.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum M0_A {
    #[doc = "0: This master has level 1, or highest, priority when accessing the slave port."]
    _000 = 0,
    #[doc = "1: This master has level 2 priority when accessing the slave port."]
    _001 = 1,
    #[doc = "2: This master has level 3 priority when accessing the slave port."]
    _010 = 2,
    #[doc = "3: This master has level 4 priority when accessing the slave port."]
    _011 = 3,
    #[doc = "4: This master has level 5 priority when accessing the slave port."]
    _100 = 4,
    #[doc = "5: This master has level 6 priority when accessing the slave port."]
    _101 = 5,
    #[doc = "6: This master has level 7 priority when accessing the slave port."]
    _110 = 6,
    #[doc = "7: This master has level 8, or lowest, priority when accessing the slave port."]
    _111 = 7,
}
impl From<M0_A> for u8 {
    #[inline(always)]
    fn from(variant: M0_A) -> Self {
        variant as _
    }
}
impl M0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M0_A {
        match self.bits {
            0 => M0_A::_000,
            1 => M0_A::_001,
            2 => M0_A::_010,
            3 => M0_A::_011,
            4 => M0_A::_100,
            5 => M0_A::_101,
            6 => M0_A::_110,
            7 => M0_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == M0_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == M0_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == M0_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == M0_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == M0_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == M0_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == M0_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == M0_A::_111
    }
}
#[doc = "Field `M0` writer - Master 0 Priority. Sets the arbitration priority for this port on the associated slave port."]
pub type M0_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PRS_SPEC, u8, M0_A, 3, O>;
impl<'a, const O: u8> M0_W<'a, O> {
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(M0_A::_000)
    }
    #[doc = "This master has level 2 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(M0_A::_001)
    }
    #[doc = "This master has level 3 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(M0_A::_010)
    }
    #[doc = "This master has level 4 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(M0_A::_011)
    }
    #[doc = "This master has level 5 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(M0_A::_100)
    }
    #[doc = "This master has level 6 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(M0_A::_101)
    }
    #[doc = "This master has level 7 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(M0_A::_110)
    }
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(M0_A::_111)
    }
}
#[doc = "Field `M1` reader - Master 1 Priority. Sets the arbitration priority for this port on the associated slave port."]
pub type M1_R = crate::FieldReader<u8, M1_A>;
#[doc = "Master 1 Priority. Sets the arbitration priority for this port on the associated slave port.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum M1_A {
    #[doc = "0: This master has level 1, or highest, priority when accessing the slave port."]
    _000 = 0,
    #[doc = "1: This master has level 2 priority when accessing the slave port."]
    _001 = 1,
    #[doc = "2: This master has level 3 priority when accessing the slave port."]
    _010 = 2,
    #[doc = "3: This master has level 4 priority when accessing the slave port."]
    _011 = 3,
    #[doc = "4: This master has level 5 priority when accessing the slave port."]
    _100 = 4,
    #[doc = "5: This master has level 6 priority when accessing the slave port."]
    _101 = 5,
    #[doc = "6: This master has level 7 priority when accessing the slave port."]
    _110 = 6,
    #[doc = "7: This master has level 8, or lowest, priority when accessing the slave port."]
    _111 = 7,
}
impl From<M1_A> for u8 {
    #[inline(always)]
    fn from(variant: M1_A) -> Self {
        variant as _
    }
}
impl M1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M1_A {
        match self.bits {
            0 => M1_A::_000,
            1 => M1_A::_001,
            2 => M1_A::_010,
            3 => M1_A::_011,
            4 => M1_A::_100,
            5 => M1_A::_101,
            6 => M1_A::_110,
            7 => M1_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == M1_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == M1_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == M1_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == M1_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == M1_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == M1_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == M1_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == M1_A::_111
    }
}
#[doc = "Field `M1` writer - Master 1 Priority. Sets the arbitration priority for this port on the associated slave port."]
pub type M1_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PRS_SPEC, u8, M1_A, 3, O>;
impl<'a, const O: u8> M1_W<'a, O> {
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(M1_A::_000)
    }
    #[doc = "This master has level 2 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(M1_A::_001)
    }
    #[doc = "This master has level 3 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(M1_A::_010)
    }
    #[doc = "This master has level 4 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(M1_A::_011)
    }
    #[doc = "This master has level 5 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(M1_A::_100)
    }
    #[doc = "This master has level 6 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(M1_A::_101)
    }
    #[doc = "This master has level 7 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(M1_A::_110)
    }
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(M1_A::_111)
    }
}
#[doc = "Field `M2` reader - Master 2 Priority. Sets the arbitration priority for this port on the associated slave port."]
pub type M2_R = crate::FieldReader<u8, M2_A>;
#[doc = "Master 2 Priority. Sets the arbitration priority for this port on the associated slave port.\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum M2_A {
    #[doc = "0: This master has level 1, or highest, priority when accessing the slave port."]
    _000 = 0,
    #[doc = "1: This master has level 2 priority when accessing the slave port."]
    _001 = 1,
    #[doc = "2: This master has level 3 priority when accessing the slave port."]
    _010 = 2,
    #[doc = "3: This master has level 4 priority when accessing the slave port."]
    _011 = 3,
    #[doc = "4: This master has level 5 priority when accessing the slave port."]
    _100 = 4,
    #[doc = "5: This master has level 6 priority when accessing the slave port."]
    _101 = 5,
    #[doc = "6: This master has level 7 priority when accessing the slave port."]
    _110 = 6,
    #[doc = "7: This master has level 8, or lowest, priority when accessing the slave port."]
    _111 = 7,
}
impl From<M2_A> for u8 {
    #[inline(always)]
    fn from(variant: M2_A) -> Self {
        variant as _
    }
}
impl M2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M2_A {
        match self.bits {
            0 => M2_A::_000,
            1 => M2_A::_001,
            2 => M2_A::_010,
            3 => M2_A::_011,
            4 => M2_A::_100,
            5 => M2_A::_101,
            6 => M2_A::_110,
            7 => M2_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == M2_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == M2_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == M2_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == M2_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == M2_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == M2_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == M2_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == M2_A::_111
    }
}
#[doc = "Field `M2` writer - Master 2 Priority. Sets the arbitration priority for this port on the associated slave port."]
pub type M2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PRS_SPEC, u8, M2_A, 3, O>;
impl<'a, const O: u8> M2_W<'a, O> {
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(M2_A::_000)
    }
    #[doc = "This master has level 2 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(M2_A::_001)
    }
    #[doc = "This master has level 3 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(M2_A::_010)
    }
    #[doc = "This master has level 4 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(M2_A::_011)
    }
    #[doc = "This master has level 5 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(M2_A::_100)
    }
    #[doc = "This master has level 6 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(M2_A::_101)
    }
    #[doc = "This master has level 7 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(M2_A::_110)
    }
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(M2_A::_111)
    }
}
#[doc = "Field `M3` reader - Master 3 Priority. Sets the arbitration priority for this port on the associated slave port."]
pub type M3_R = crate::FieldReader<u8, M3_A>;
#[doc = "Master 3 Priority. Sets the arbitration priority for this port on the associated slave port.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum M3_A {
    #[doc = "0: This master has level 1, or highest, priority when accessing the slave port."]
    _000 = 0,
    #[doc = "1: This master has level 2 priority when accessing the slave port."]
    _001 = 1,
    #[doc = "2: This master has level 3 priority when accessing the slave port."]
    _010 = 2,
    #[doc = "3: This master has level 4 priority when accessing the slave port."]
    _011 = 3,
    #[doc = "4: This master has level 5 priority when accessing the slave port."]
    _100 = 4,
    #[doc = "5: This master has level 6 priority when accessing the slave port."]
    _101 = 5,
    #[doc = "6: This master has level 7 priority when accessing the slave port."]
    _110 = 6,
    #[doc = "7: This master has level 8, or lowest, priority when accessing the slave port."]
    _111 = 7,
}
impl From<M3_A> for u8 {
    #[inline(always)]
    fn from(variant: M3_A) -> Self {
        variant as _
    }
}
impl M3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M3_A {
        match self.bits {
            0 => M3_A::_000,
            1 => M3_A::_001,
            2 => M3_A::_010,
            3 => M3_A::_011,
            4 => M3_A::_100,
            5 => M3_A::_101,
            6 => M3_A::_110,
            7 => M3_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == M3_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == M3_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == M3_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == M3_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == M3_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == M3_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == M3_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == M3_A::_111
    }
}
#[doc = "Field `M3` writer - Master 3 Priority. Sets the arbitration priority for this port on the associated slave port."]
pub type M3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PRS_SPEC, u8, M3_A, 3, O>;
impl<'a, const O: u8> M3_W<'a, O> {
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(M3_A::_000)
    }
    #[doc = "This master has level 2 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(M3_A::_001)
    }
    #[doc = "This master has level 3 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(M3_A::_010)
    }
    #[doc = "This master has level 4 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(M3_A::_011)
    }
    #[doc = "This master has level 5 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(M3_A::_100)
    }
    #[doc = "This master has level 6 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(M3_A::_101)
    }
    #[doc = "This master has level 7 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(M3_A::_110)
    }
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(M3_A::_111)
    }
}
#[doc = "Field `M4` reader - Master 4 Priority. Sets the arbitration priority for this port on the associated slave port."]
pub type M4_R = crate::FieldReader<u8, M4_A>;
#[doc = "Master 4 Priority. Sets the arbitration priority for this port on the associated slave port.\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum M4_A {
    #[doc = "0: This master has level 1, or highest, priority when accessing the slave port."]
    _000 = 0,
    #[doc = "1: This master has level 2 priority when accessing the slave port."]
    _001 = 1,
    #[doc = "2: This master has level 3 priority when accessing the slave port."]
    _010 = 2,
    #[doc = "3: This master has level 4 priority when accessing the slave port."]
    _011 = 3,
    #[doc = "4: This master has level 5 priority when accessing the slave port."]
    _100 = 4,
    #[doc = "5: This master has level 6 priority when accessing the slave port."]
    _101 = 5,
    #[doc = "6: This master has level 7 priority when accessing the slave port."]
    _110 = 6,
    #[doc = "7: This master has level 8, or lowest, priority when accessing the slave port."]
    _111 = 7,
}
impl From<M4_A> for u8 {
    #[inline(always)]
    fn from(variant: M4_A) -> Self {
        variant as _
    }
}
impl M4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M4_A {
        match self.bits {
            0 => M4_A::_000,
            1 => M4_A::_001,
            2 => M4_A::_010,
            3 => M4_A::_011,
            4 => M4_A::_100,
            5 => M4_A::_101,
            6 => M4_A::_110,
            7 => M4_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == M4_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == M4_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == M4_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == M4_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == M4_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == M4_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == M4_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == M4_A::_111
    }
}
#[doc = "Field `M4` writer - Master 4 Priority. Sets the arbitration priority for this port on the associated slave port."]
pub type M4_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PRS_SPEC, u8, M4_A, 3, O>;
impl<'a, const O: u8> M4_W<'a, O> {
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(M4_A::_000)
    }
    #[doc = "This master has level 2 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(M4_A::_001)
    }
    #[doc = "This master has level 3 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(M4_A::_010)
    }
    #[doc = "This master has level 4 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(M4_A::_011)
    }
    #[doc = "This master has level 5 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(M4_A::_100)
    }
    #[doc = "This master has level 6 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(M4_A::_101)
    }
    #[doc = "This master has level 7 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(M4_A::_110)
    }
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(M4_A::_111)
    }
}
#[doc = "Field `M5` reader - Master 5 Priority. Sets the arbitration priority for this port on the associated slave port."]
pub type M5_R = crate::FieldReader<u8, M5_A>;
#[doc = "Master 5 Priority. Sets the arbitration priority for this port on the associated slave port.\n\nValue on reset: 5"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum M5_A {
    #[doc = "0: This master has level 1, or highest, priority when accessing the slave port."]
    _000 = 0,
    #[doc = "1: This master has level 2 priority when accessing the slave port."]
    _001 = 1,
    #[doc = "2: This master has level 3 priority when accessing the slave port."]
    _010 = 2,
    #[doc = "3: This master has level 4 priority when accessing the slave port."]
    _011 = 3,
    #[doc = "4: This master has level 5 priority when accessing the slave port."]
    _100 = 4,
    #[doc = "5: This master has level 6 priority when accessing the slave port."]
    _101 = 5,
    #[doc = "6: This master has level 7 priority when accessing the slave port."]
    _110 = 6,
    #[doc = "7: This master has level 8, or lowest, priority when accessing the slave port."]
    _111 = 7,
}
impl From<M5_A> for u8 {
    #[inline(always)]
    fn from(variant: M5_A) -> Self {
        variant as _
    }
}
impl M5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M5_A {
        match self.bits {
            0 => M5_A::_000,
            1 => M5_A::_001,
            2 => M5_A::_010,
            3 => M5_A::_011,
            4 => M5_A::_100,
            5 => M5_A::_101,
            6 => M5_A::_110,
            7 => M5_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == M5_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == M5_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == M5_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == M5_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == M5_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == M5_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == M5_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == M5_A::_111
    }
}
#[doc = "Field `M5` writer - Master 5 Priority. Sets the arbitration priority for this port on the associated slave port."]
pub type M5_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PRS_SPEC, u8, M5_A, 3, O>;
impl<'a, const O: u8> M5_W<'a, O> {
    #[doc = "This master has level 1, or highest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(M5_A::_000)
    }
    #[doc = "This master has level 2 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(M5_A::_001)
    }
    #[doc = "This master has level 3 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(M5_A::_010)
    }
    #[doc = "This master has level 4 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(M5_A::_011)
    }
    #[doc = "This master has level 5 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(M5_A::_100)
    }
    #[doc = "This master has level 6 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(M5_A::_101)
    }
    #[doc = "This master has level 7 priority when accessing the slave port."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(M5_A::_110)
    }
    #[doc = "This master has level 8, or lowest, priority when accessing the slave port."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(M5_A::_111)
    }
}
impl R {
    #[doc = "Bits 0:2 - Master 0 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    pub fn m0(&self) -> M0_R {
        M0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Master 1 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    pub fn m1(&self) -> M1_R {
        M1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Master 2 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    pub fn m2(&self) -> M2_R {
        M2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Master 3 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    pub fn m3(&self) -> M3_R {
        M3_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Master 4 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    pub fn m4(&self) -> M4_R {
        M4_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Master 5 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    pub fn m5(&self) -> M5_R {
        M5_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Master 0 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    #[must_use]
    pub fn m0(&mut self) -> M0_W<0> {
        M0_W::new(self)
    }
    #[doc = "Bits 4:6 - Master 1 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    #[must_use]
    pub fn m1(&mut self) -> M1_W<4> {
        M1_W::new(self)
    }
    #[doc = "Bits 8:10 - Master 2 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    #[must_use]
    pub fn m2(&mut self) -> M2_W<8> {
        M2_W::new(self)
    }
    #[doc = "Bits 12:14 - Master 3 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    #[must_use]
    pub fn m3(&mut self) -> M3_W<12> {
        M3_W::new(self)
    }
    #[doc = "Bits 16:18 - Master 4 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    #[must_use]
    pub fn m4(&mut self) -> M4_W<16> {
        M4_W::new(self)
    }
    #[doc = "Bits 20:22 - Master 5 Priority. Sets the arbitration priority for this port on the associated slave port."]
    #[inline(always)]
    #[must_use]
    pub fn m5(&mut self) -> M5_W<20> {
        M5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Priority Registers Slave\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prs](index.html) module"]
pub struct PRS_SPEC;
impl crate::RegisterSpec for PRS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prs::R](R) reader structure"]
impl crate::Readable for PRS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prs::W](W) writer structure"]
impl crate::Writable for PRS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRS%s to value 0x0054_3210"]
impl crate::Resettable for PRS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0054_3210;
}
