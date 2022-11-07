#[doc = "Register `PFAPR` reader"]
pub struct R(crate::R<PFAPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PFAPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PFAPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PFAPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PFAPR` writer"]
pub struct W(crate::W<PFAPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PFAPR_SPEC>;
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
impl From<crate::W<PFAPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PFAPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M0AP` reader - Master 0 Access Protection"]
pub type M0AP_R = crate::FieldReader<u8, M0AP_A>;
#[doc = "Master 0 Access Protection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum M0AP_A {
    #[doc = "0: No access may be performed by this master"]
    _00 = 0,
    #[doc = "1: Only read accesses may be performed by this master"]
    _01 = 1,
    #[doc = "2: Only write accesses may be performed by this master"]
    _10 = 2,
    #[doc = "3: Both read and write accesses may be performed by this master"]
    _11 = 3,
}
impl From<M0AP_A> for u8 {
    #[inline(always)]
    fn from(variant: M0AP_A) -> Self {
        variant as _
    }
}
impl M0AP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M0AP_A {
        match self.bits {
            0 => M0AP_A::_00,
            1 => M0AP_A::_01,
            2 => M0AP_A::_10,
            3 => M0AP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == M0AP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == M0AP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == M0AP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == M0AP_A::_11
    }
}
#[doc = "Field `M0AP` writer - Master 0 Access Protection"]
pub type M0AP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PFAPR_SPEC, u8, M0AP_A, 2, O>;
impl<'a, const O: u8> M0AP_W<'a, O> {
    #[doc = "No access may be performed by this master"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(M0AP_A::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(M0AP_A::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(M0AP_A::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(M0AP_A::_11)
    }
}
#[doc = "Field `M1AP` reader - Master 1 Access Protection"]
pub type M1AP_R = crate::FieldReader<u8, M1AP_A>;
#[doc = "Master 1 Access Protection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum M1AP_A {
    #[doc = "0: No access may be performed by this master"]
    _00 = 0,
    #[doc = "1: Only read accesses may be performed by this master"]
    _01 = 1,
    #[doc = "2: Only write accesses may be performed by this master"]
    _10 = 2,
    #[doc = "3: Both read and write accesses may be performed by this master"]
    _11 = 3,
}
impl From<M1AP_A> for u8 {
    #[inline(always)]
    fn from(variant: M1AP_A) -> Self {
        variant as _
    }
}
impl M1AP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M1AP_A {
        match self.bits {
            0 => M1AP_A::_00,
            1 => M1AP_A::_01,
            2 => M1AP_A::_10,
            3 => M1AP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == M1AP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == M1AP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == M1AP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == M1AP_A::_11
    }
}
#[doc = "Field `M1AP` writer - Master 1 Access Protection"]
pub type M1AP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PFAPR_SPEC, u8, M1AP_A, 2, O>;
impl<'a, const O: u8> M1AP_W<'a, O> {
    #[doc = "No access may be performed by this master"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(M1AP_A::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(M1AP_A::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(M1AP_A::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(M1AP_A::_11)
    }
}
#[doc = "Field `M2AP` reader - Master 2 Access Protection"]
pub type M2AP_R = crate::FieldReader<u8, M2AP_A>;
#[doc = "Master 2 Access Protection\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum M2AP_A {
    #[doc = "0: No access may be performed by this master"]
    _00 = 0,
    #[doc = "1: Only read accesses may be performed by this master"]
    _01 = 1,
    #[doc = "2: Only write accesses may be performed by this master"]
    _10 = 2,
    #[doc = "3: Both read and write accesses may be performed by this master"]
    _11 = 3,
}
impl From<M2AP_A> for u8 {
    #[inline(always)]
    fn from(variant: M2AP_A) -> Self {
        variant as _
    }
}
impl M2AP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M2AP_A {
        match self.bits {
            0 => M2AP_A::_00,
            1 => M2AP_A::_01,
            2 => M2AP_A::_10,
            3 => M2AP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == M2AP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == M2AP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == M2AP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == M2AP_A::_11
    }
}
#[doc = "Field `M2AP` writer - Master 2 Access Protection"]
pub type M2AP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PFAPR_SPEC, u8, M2AP_A, 2, O>;
impl<'a, const O: u8> M2AP_W<'a, O> {
    #[doc = "No access may be performed by this master"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(M2AP_A::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(M2AP_A::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(M2AP_A::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(M2AP_A::_11)
    }
}
#[doc = "Field `M3AP` reader - Master 3 Access Protection"]
pub type M3AP_R = crate::FieldReader<u8, M3AP_A>;
#[doc = "Master 3 Access Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum M3AP_A {
    #[doc = "0: No access may be performed by this master"]
    _00 = 0,
    #[doc = "1: Only read accesses may be performed by this master"]
    _01 = 1,
    #[doc = "2: Only write accesses may be performed by this master"]
    _10 = 2,
    #[doc = "3: Both read and write accesses may be performed by this master"]
    _11 = 3,
}
impl From<M3AP_A> for u8 {
    #[inline(always)]
    fn from(variant: M3AP_A) -> Self {
        variant as _
    }
}
impl M3AP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M3AP_A {
        match self.bits {
            0 => M3AP_A::_00,
            1 => M3AP_A::_01,
            2 => M3AP_A::_10,
            3 => M3AP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == M3AP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == M3AP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == M3AP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == M3AP_A::_11
    }
}
#[doc = "Field `M3AP` writer - Master 3 Access Protection"]
pub type M3AP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PFAPR_SPEC, u8, M3AP_A, 2, O>;
impl<'a, const O: u8> M3AP_W<'a, O> {
    #[doc = "No access may be performed by this master"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(M3AP_A::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(M3AP_A::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(M3AP_A::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(M3AP_A::_11)
    }
}
#[doc = "Field `M4AP` reader - Master 4 Access Protection"]
pub type M4AP_R = crate::FieldReader<u8, M4AP_A>;
#[doc = "Master 4 Access Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum M4AP_A {
    #[doc = "0: No access may be performed by this master"]
    _00 = 0,
    #[doc = "1: Only read accesses may be performed by this master"]
    _01 = 1,
    #[doc = "2: Only write accesses may be performed by this master"]
    _10 = 2,
    #[doc = "3: Both read and write accesses may be performed by this master"]
    _11 = 3,
}
impl From<M4AP_A> for u8 {
    #[inline(always)]
    fn from(variant: M4AP_A) -> Self {
        variant as _
    }
}
impl M4AP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M4AP_A {
        match self.bits {
            0 => M4AP_A::_00,
            1 => M4AP_A::_01,
            2 => M4AP_A::_10,
            3 => M4AP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == M4AP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == M4AP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == M4AP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == M4AP_A::_11
    }
}
#[doc = "Field `M4AP` writer - Master 4 Access Protection"]
pub type M4AP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PFAPR_SPEC, u8, M4AP_A, 2, O>;
impl<'a, const O: u8> M4AP_W<'a, O> {
    #[doc = "No access may be performed by this master"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(M4AP_A::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(M4AP_A::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(M4AP_A::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(M4AP_A::_11)
    }
}
#[doc = "Field `M5AP` reader - Master 5 Access Protection"]
pub type M5AP_R = crate::FieldReader<u8, M5AP_A>;
#[doc = "Master 5 Access Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum M5AP_A {
    #[doc = "0: No access may be performed by this master"]
    _00 = 0,
    #[doc = "1: Only read accesses may be performed by this master"]
    _01 = 1,
    #[doc = "2: Only write accesses may be performed by this master"]
    _10 = 2,
    #[doc = "3: Both read and write accesses may be performed by this master"]
    _11 = 3,
}
impl From<M5AP_A> for u8 {
    #[inline(always)]
    fn from(variant: M5AP_A) -> Self {
        variant as _
    }
}
impl M5AP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M5AP_A {
        match self.bits {
            0 => M5AP_A::_00,
            1 => M5AP_A::_01,
            2 => M5AP_A::_10,
            3 => M5AP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == M5AP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == M5AP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == M5AP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == M5AP_A::_11
    }
}
#[doc = "Field `M5AP` writer - Master 5 Access Protection"]
pub type M5AP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PFAPR_SPEC, u8, M5AP_A, 2, O>;
impl<'a, const O: u8> M5AP_W<'a, O> {
    #[doc = "No access may be performed by this master"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(M5AP_A::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(M5AP_A::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(M5AP_A::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(M5AP_A::_11)
    }
}
#[doc = "Field `M6AP` reader - Master 6 Access Protection"]
pub type M6AP_R = crate::FieldReader<u8, M6AP_A>;
#[doc = "Master 6 Access Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum M6AP_A {
    #[doc = "0: No access may be performed by this master"]
    _00 = 0,
    #[doc = "1: Only read accesses may be performed by this master"]
    _01 = 1,
    #[doc = "2: Only write accesses may be performed by this master"]
    _10 = 2,
    #[doc = "3: Both read and write accesses may be performed by this master"]
    _11 = 3,
}
impl From<M6AP_A> for u8 {
    #[inline(always)]
    fn from(variant: M6AP_A) -> Self {
        variant as _
    }
}
impl M6AP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M6AP_A {
        match self.bits {
            0 => M6AP_A::_00,
            1 => M6AP_A::_01,
            2 => M6AP_A::_10,
            3 => M6AP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == M6AP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == M6AP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == M6AP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == M6AP_A::_11
    }
}
#[doc = "Field `M6AP` writer - Master 6 Access Protection"]
pub type M6AP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PFAPR_SPEC, u8, M6AP_A, 2, O>;
impl<'a, const O: u8> M6AP_W<'a, O> {
    #[doc = "No access may be performed by this master"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(M6AP_A::_00)
    }
    #[doc = "Only read accesses may be performed by this master"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(M6AP_A::_01)
    }
    #[doc = "Only write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(M6AP_A::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(M6AP_A::_11)
    }
}
#[doc = "Field `M7AP` reader - Master 7 Access Protection"]
pub type M7AP_R = crate::FieldReader<u8, M7AP_A>;
#[doc = "Master 7 Access Protection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum M7AP_A {
    #[doc = "0: No access may be performed by this master."]
    _00 = 0,
    #[doc = "1: Only read accesses may be performed by this master."]
    _01 = 1,
    #[doc = "2: Only write accesses may be performed by this master."]
    _10 = 2,
    #[doc = "3: Both read and write accesses may be performed by this master."]
    _11 = 3,
}
impl From<M7AP_A> for u8 {
    #[inline(always)]
    fn from(variant: M7AP_A) -> Self {
        variant as _
    }
}
impl M7AP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M7AP_A {
        match self.bits {
            0 => M7AP_A::_00,
            1 => M7AP_A::_01,
            2 => M7AP_A::_10,
            3 => M7AP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == M7AP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == M7AP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == M7AP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == M7AP_A::_11
    }
}
#[doc = "Field `M7AP` writer - Master 7 Access Protection"]
pub type M7AP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, PFAPR_SPEC, u8, M7AP_A, 2, O>;
impl<'a, const O: u8> M7AP_W<'a, O> {
    #[doc = "No access may be performed by this master."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(M7AP_A::_00)
    }
    #[doc = "Only read accesses may be performed by this master."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(M7AP_A::_01)
    }
    #[doc = "Only write accesses may be performed by this master."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(M7AP_A::_10)
    }
    #[doc = "Both read and write accesses may be performed by this master."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(M7AP_A::_11)
    }
}
#[doc = "Field `M0PFD` reader - Master 0 Prefetch Disable"]
pub type M0PFD_R = crate::BitReader<M0PFD_A>;
#[doc = "Master 0 Prefetch Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M0PFD_A {
    #[doc = "0: Prefetching for this master is enabled."]
    _0 = 0,
    #[doc = "1: Prefetching for this master is disabled."]
    _1 = 1,
}
impl From<M0PFD_A> for bool {
    #[inline(always)]
    fn from(variant: M0PFD_A) -> Self {
        variant as u8 != 0
    }
}
impl M0PFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M0PFD_A {
        match self.bits {
            false => M0PFD_A::_0,
            true => M0PFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M0PFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M0PFD_A::_1
    }
}
#[doc = "Field `M0PFD` writer - Master 0 Prefetch Disable"]
pub type M0PFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFAPR_SPEC, M0PFD_A, O>;
impl<'a, const O: u8> M0PFD_W<'a, O> {
    #[doc = "Prefetching for this master is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M0PFD_A::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M0PFD_A::_1)
    }
}
#[doc = "Field `M1PFD` reader - Master 1 Prefetch Disable"]
pub type M1PFD_R = crate::BitReader<M1PFD_A>;
#[doc = "Master 1 Prefetch Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M1PFD_A {
    #[doc = "0: Prefetching for this master is enabled."]
    _0 = 0,
    #[doc = "1: Prefetching for this master is disabled."]
    _1 = 1,
}
impl From<M1PFD_A> for bool {
    #[inline(always)]
    fn from(variant: M1PFD_A) -> Self {
        variant as u8 != 0
    }
}
impl M1PFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M1PFD_A {
        match self.bits {
            false => M1PFD_A::_0,
            true => M1PFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M1PFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M1PFD_A::_1
    }
}
#[doc = "Field `M1PFD` writer - Master 1 Prefetch Disable"]
pub type M1PFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFAPR_SPEC, M1PFD_A, O>;
impl<'a, const O: u8> M1PFD_W<'a, O> {
    #[doc = "Prefetching for this master is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M1PFD_A::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M1PFD_A::_1)
    }
}
#[doc = "Field `M2PFD` reader - Master 2 Prefetch Disable"]
pub type M2PFD_R = crate::BitReader<M2PFD_A>;
#[doc = "Master 2 Prefetch Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M2PFD_A {
    #[doc = "0: Prefetching for this master is enabled."]
    _0 = 0,
    #[doc = "1: Prefetching for this master is disabled."]
    _1 = 1,
}
impl From<M2PFD_A> for bool {
    #[inline(always)]
    fn from(variant: M2PFD_A) -> Self {
        variant as u8 != 0
    }
}
impl M2PFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M2PFD_A {
        match self.bits {
            false => M2PFD_A::_0,
            true => M2PFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M2PFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M2PFD_A::_1
    }
}
#[doc = "Field `M2PFD` writer - Master 2 Prefetch Disable"]
pub type M2PFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFAPR_SPEC, M2PFD_A, O>;
impl<'a, const O: u8> M2PFD_W<'a, O> {
    #[doc = "Prefetching for this master is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M2PFD_A::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M2PFD_A::_1)
    }
}
#[doc = "Field `M3PFD` reader - Master 3 Prefetch Disable"]
pub type M3PFD_R = crate::BitReader<M3PFD_A>;
#[doc = "Master 3 Prefetch Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M3PFD_A {
    #[doc = "0: Prefetching for this master is enabled."]
    _0 = 0,
    #[doc = "1: Prefetching for this master is disabled."]
    _1 = 1,
}
impl From<M3PFD_A> for bool {
    #[inline(always)]
    fn from(variant: M3PFD_A) -> Self {
        variant as u8 != 0
    }
}
impl M3PFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M3PFD_A {
        match self.bits {
            false => M3PFD_A::_0,
            true => M3PFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M3PFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M3PFD_A::_1
    }
}
#[doc = "Field `M3PFD` writer - Master 3 Prefetch Disable"]
pub type M3PFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFAPR_SPEC, M3PFD_A, O>;
impl<'a, const O: u8> M3PFD_W<'a, O> {
    #[doc = "Prefetching for this master is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M3PFD_A::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M3PFD_A::_1)
    }
}
#[doc = "Field `M4PFD` reader - Master 4 Prefetch Disable"]
pub type M4PFD_R = crate::BitReader<M4PFD_A>;
#[doc = "Master 4 Prefetch Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M4PFD_A {
    #[doc = "0: Prefetching for this master is enabled."]
    _0 = 0,
    #[doc = "1: Prefetching for this master is disabled."]
    _1 = 1,
}
impl From<M4PFD_A> for bool {
    #[inline(always)]
    fn from(variant: M4PFD_A) -> Self {
        variant as u8 != 0
    }
}
impl M4PFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M4PFD_A {
        match self.bits {
            false => M4PFD_A::_0,
            true => M4PFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M4PFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M4PFD_A::_1
    }
}
#[doc = "Field `M4PFD` writer - Master 4 Prefetch Disable"]
pub type M4PFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFAPR_SPEC, M4PFD_A, O>;
impl<'a, const O: u8> M4PFD_W<'a, O> {
    #[doc = "Prefetching for this master is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M4PFD_A::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M4PFD_A::_1)
    }
}
#[doc = "Field `M5PFD` reader - Master 5 Prefetch Disable"]
pub type M5PFD_R = crate::BitReader<M5PFD_A>;
#[doc = "Master 5 Prefetch Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M5PFD_A {
    #[doc = "0: Prefetching for this master is enabled."]
    _0 = 0,
    #[doc = "1: Prefetching for this master is disabled."]
    _1 = 1,
}
impl From<M5PFD_A> for bool {
    #[inline(always)]
    fn from(variant: M5PFD_A) -> Self {
        variant as u8 != 0
    }
}
impl M5PFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M5PFD_A {
        match self.bits {
            false => M5PFD_A::_0,
            true => M5PFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M5PFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M5PFD_A::_1
    }
}
#[doc = "Field `M5PFD` writer - Master 5 Prefetch Disable"]
pub type M5PFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFAPR_SPEC, M5PFD_A, O>;
impl<'a, const O: u8> M5PFD_W<'a, O> {
    #[doc = "Prefetching for this master is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M5PFD_A::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M5PFD_A::_1)
    }
}
#[doc = "Field `M6PFD` reader - Master 6 Prefetch Disable"]
pub type M6PFD_R = crate::BitReader<M6PFD_A>;
#[doc = "Master 6 Prefetch Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M6PFD_A {
    #[doc = "0: Prefetching for this master is enabled."]
    _0 = 0,
    #[doc = "1: Prefetching for this master is disabled."]
    _1 = 1,
}
impl From<M6PFD_A> for bool {
    #[inline(always)]
    fn from(variant: M6PFD_A) -> Self {
        variant as u8 != 0
    }
}
impl M6PFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M6PFD_A {
        match self.bits {
            false => M6PFD_A::_0,
            true => M6PFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M6PFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M6PFD_A::_1
    }
}
#[doc = "Field `M6PFD` writer - Master 6 Prefetch Disable"]
pub type M6PFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFAPR_SPEC, M6PFD_A, O>;
impl<'a, const O: u8> M6PFD_W<'a, O> {
    #[doc = "Prefetching for this master is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M6PFD_A::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M6PFD_A::_1)
    }
}
#[doc = "Field `M7PFD` reader - Master 7 Prefetch Disable"]
pub type M7PFD_R = crate::BitReader<M7PFD_A>;
#[doc = "Master 7 Prefetch Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M7PFD_A {
    #[doc = "0: Prefetching for this master is enabled."]
    _0 = 0,
    #[doc = "1: Prefetching for this master is disabled."]
    _1 = 1,
}
impl From<M7PFD_A> for bool {
    #[inline(always)]
    fn from(variant: M7PFD_A) -> Self {
        variant as u8 != 0
    }
}
impl M7PFD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M7PFD_A {
        match self.bits {
            false => M7PFD_A::_0,
            true => M7PFD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M7PFD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M7PFD_A::_1
    }
}
#[doc = "Field `M7PFD` writer - Master 7 Prefetch Disable"]
pub type M7PFD_W<'a, const O: u8> = crate::BitWriter<'a, u32, PFAPR_SPEC, M7PFD_A, O>;
impl<'a, const O: u8> M7PFD_W<'a, O> {
    #[doc = "Prefetching for this master is enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M7PFD_A::_0)
    }
    #[doc = "Prefetching for this master is disabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M7PFD_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Master 0 Access Protection"]
    #[inline(always)]
    pub fn m0ap(&self) -> M0AP_R {
        M0AP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Master 1 Access Protection"]
    #[inline(always)]
    pub fn m1ap(&self) -> M1AP_R {
        M1AP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Master 2 Access Protection"]
    #[inline(always)]
    pub fn m2ap(&self) -> M2AP_R {
        M2AP_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Master 3 Access Protection"]
    #[inline(always)]
    pub fn m3ap(&self) -> M3AP_R {
        M3AP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Master 4 Access Protection"]
    #[inline(always)]
    pub fn m4ap(&self) -> M4AP_R {
        M4AP_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Master 5 Access Protection"]
    #[inline(always)]
    pub fn m5ap(&self) -> M5AP_R {
        M5AP_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Master 6 Access Protection"]
    #[inline(always)]
    pub fn m6ap(&self) -> M6AP_R {
        M6AP_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Master 7 Access Protection"]
    #[inline(always)]
    pub fn m7ap(&self) -> M7AP_R {
        M7AP_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Master 0 Prefetch Disable"]
    #[inline(always)]
    pub fn m0pfd(&self) -> M0PFD_R {
        M0PFD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Master 1 Prefetch Disable"]
    #[inline(always)]
    pub fn m1pfd(&self) -> M1PFD_R {
        M1PFD_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Master 2 Prefetch Disable"]
    #[inline(always)]
    pub fn m2pfd(&self) -> M2PFD_R {
        M2PFD_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Master 3 Prefetch Disable"]
    #[inline(always)]
    pub fn m3pfd(&self) -> M3PFD_R {
        M3PFD_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Master 4 Prefetch Disable"]
    #[inline(always)]
    pub fn m4pfd(&self) -> M4PFD_R {
        M4PFD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Master 5 Prefetch Disable"]
    #[inline(always)]
    pub fn m5pfd(&self) -> M5PFD_R {
        M5PFD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Master 6 Prefetch Disable"]
    #[inline(always)]
    pub fn m6pfd(&self) -> M6PFD_R {
        M6PFD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Master 7 Prefetch Disable"]
    #[inline(always)]
    pub fn m7pfd(&self) -> M7PFD_R {
        M7PFD_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Master 0 Access Protection"]
    #[inline(always)]
    #[must_use]
    pub fn m0ap(&mut self) -> M0AP_W<0> {
        M0AP_W::new(self)
    }
    #[doc = "Bits 2:3 - Master 1 Access Protection"]
    #[inline(always)]
    #[must_use]
    pub fn m1ap(&mut self) -> M1AP_W<2> {
        M1AP_W::new(self)
    }
    #[doc = "Bits 4:5 - Master 2 Access Protection"]
    #[inline(always)]
    #[must_use]
    pub fn m2ap(&mut self) -> M2AP_W<4> {
        M2AP_W::new(self)
    }
    #[doc = "Bits 6:7 - Master 3 Access Protection"]
    #[inline(always)]
    #[must_use]
    pub fn m3ap(&mut self) -> M3AP_W<6> {
        M3AP_W::new(self)
    }
    #[doc = "Bits 8:9 - Master 4 Access Protection"]
    #[inline(always)]
    #[must_use]
    pub fn m4ap(&mut self) -> M4AP_W<8> {
        M4AP_W::new(self)
    }
    #[doc = "Bits 10:11 - Master 5 Access Protection"]
    #[inline(always)]
    #[must_use]
    pub fn m5ap(&mut self) -> M5AP_W<10> {
        M5AP_W::new(self)
    }
    #[doc = "Bits 12:13 - Master 6 Access Protection"]
    #[inline(always)]
    #[must_use]
    pub fn m6ap(&mut self) -> M6AP_W<12> {
        M6AP_W::new(self)
    }
    #[doc = "Bits 14:15 - Master 7 Access Protection"]
    #[inline(always)]
    #[must_use]
    pub fn m7ap(&mut self) -> M7AP_W<14> {
        M7AP_W::new(self)
    }
    #[doc = "Bit 16 - Master 0 Prefetch Disable"]
    #[inline(always)]
    #[must_use]
    pub fn m0pfd(&mut self) -> M0PFD_W<16> {
        M0PFD_W::new(self)
    }
    #[doc = "Bit 17 - Master 1 Prefetch Disable"]
    #[inline(always)]
    #[must_use]
    pub fn m1pfd(&mut self) -> M1PFD_W<17> {
        M1PFD_W::new(self)
    }
    #[doc = "Bit 18 - Master 2 Prefetch Disable"]
    #[inline(always)]
    #[must_use]
    pub fn m2pfd(&mut self) -> M2PFD_W<18> {
        M2PFD_W::new(self)
    }
    #[doc = "Bit 19 - Master 3 Prefetch Disable"]
    #[inline(always)]
    #[must_use]
    pub fn m3pfd(&mut self) -> M3PFD_W<19> {
        M3PFD_W::new(self)
    }
    #[doc = "Bit 20 - Master 4 Prefetch Disable"]
    #[inline(always)]
    #[must_use]
    pub fn m4pfd(&mut self) -> M4PFD_W<20> {
        M4PFD_W::new(self)
    }
    #[doc = "Bit 21 - Master 5 Prefetch Disable"]
    #[inline(always)]
    #[must_use]
    pub fn m5pfd(&mut self) -> M5PFD_W<21> {
        M5PFD_W::new(self)
    }
    #[doc = "Bit 22 - Master 6 Prefetch Disable"]
    #[inline(always)]
    #[must_use]
    pub fn m6pfd(&mut self) -> M6PFD_W<22> {
        M6PFD_W::new(self)
    }
    #[doc = "Bit 23 - Master 7 Prefetch Disable"]
    #[inline(always)]
    #[must_use]
    pub fn m7pfd(&mut self) -> M7PFD_W<23> {
        M7PFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Access Protection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pfapr](index.html) module"]
pub struct PFAPR_SPEC;
impl crate::RegisterSpec for PFAPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pfapr::R](R) reader structure"]
impl crate::Readable for PFAPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pfapr::W](W) writer structure"]
impl crate::Writable for PFAPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PFAPR to value 0x00f8_003f"]
impl crate::Resettable for PFAPR_SPEC {
    const RESET_VALUE: Self::Ux = 0x00f8_003f;
}
