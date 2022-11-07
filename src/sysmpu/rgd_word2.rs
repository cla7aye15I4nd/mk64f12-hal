#[doc = "Register `RGD%s_WORD2` reader"]
pub struct R(crate::R<RGD_WORD2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RGD_WORD2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RGD_WORD2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RGD_WORD2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RGD%s_WORD2` writer"]
pub struct W(crate::W<RGD_WORD2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RGD_WORD2_SPEC>;
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
impl From<crate::W<RGD_WORD2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RGD_WORD2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M0UM` reader - Bus Master 0 User Mode Access Control"]
pub type M0UM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M0UM` writer - Bus Master 0 User Mode Access Control"]
pub type M0UM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGD_WORD2_SPEC, u8, u8, 3, O>;
#[doc = "Field `M0SM` reader - Bus Master 0 Supervisor Mode Access Control"]
pub type M0SM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M0SM` writer - Bus Master 0 Supervisor Mode Access Control"]
pub type M0SM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGD_WORD2_SPEC, u8, u8, 2, O>;
#[doc = "Field `M0PE` reader - Bus Master 0 Process Identifier enable"]
pub type M0PE_R = crate::BitReader<bool>;
#[doc = "Field `M0PE` writer - Bus Master 0 Process Identifier enable"]
pub type M0PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGD_WORD2_SPEC, bool, O>;
#[doc = "Field `M1UM` reader - Bus Master 1 User Mode Access Control"]
pub type M1UM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M1UM` writer - Bus Master 1 User Mode Access Control"]
pub type M1UM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGD_WORD2_SPEC, u8, u8, 3, O>;
#[doc = "Field `M1SM` reader - Bus Master 1 Supervisor Mode Access Control"]
pub type M1SM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M1SM` writer - Bus Master 1 Supervisor Mode Access Control"]
pub type M1SM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGD_WORD2_SPEC, u8, u8, 2, O>;
#[doc = "Field `M1PE` reader - Bus Master 1 Process Identifier enable"]
pub type M1PE_R = crate::BitReader<bool>;
#[doc = "Field `M1PE` writer - Bus Master 1 Process Identifier enable"]
pub type M1PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGD_WORD2_SPEC, bool, O>;
#[doc = "Field `M2UM` reader - Bus Master 2 User Mode Access control"]
pub type M2UM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M2UM` writer - Bus Master 2 User Mode Access control"]
pub type M2UM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGD_WORD2_SPEC, u8, u8, 3, O>;
#[doc = "Field `M2SM` reader - Bus Master 2 Supervisor Mode Access Control"]
pub type M2SM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `M2SM` writer - Bus Master 2 Supervisor Mode Access Control"]
pub type M2SM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGD_WORD2_SPEC, u8, u8, 2, O>;
#[doc = "Field `M2PE` reader - Bus Master 2 Process Identifier Enable"]
pub type M2PE_R = crate::BitReader<bool>;
#[doc = "Field `M2PE` writer - Bus Master 2 Process Identifier Enable"]
pub type M2PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGD_WORD2_SPEC, bool, O>;
#[doc = "Field `M3UM` reader - Bus Master 3 User Mode Access Control"]
pub type M3UM_R = crate::FieldReader<u8, M3UM_A>;
#[doc = "Bus Master 3 User Mode Access Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum M3UM_A {
    #[doc = "0: An attempted access of that mode may be terminated with an access error (if not allowed by another descriptor) and the access not performed."]
    _0 = 0,
    #[doc = "1: Allows the given access type to occur"]
    _1 = 1,
}
impl From<M3UM_A> for u8 {
    #[inline(always)]
    fn from(variant: M3UM_A) -> Self {
        variant as _
    }
}
impl M3UM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<M3UM_A> {
        match self.bits {
            0 => Some(M3UM_A::_0),
            1 => Some(M3UM_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M3UM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M3UM_A::_1
    }
}
#[doc = "Field `M3UM` writer - Bus Master 3 User Mode Access Control"]
pub type M3UM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGD_WORD2_SPEC, u8, M3UM_A, 3, O>;
impl<'a, const O: u8> M3UM_W<'a, O> {
    #[doc = "An attempted access of that mode may be terminated with an access error (if not allowed by another descriptor) and the access not performed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M3UM_A::_0)
    }
    #[doc = "Allows the given access type to occur"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M3UM_A::_1)
    }
}
#[doc = "Field `M3SM` reader - Bus Master 3 Supervisor Mode Access Control"]
pub type M3SM_R = crate::FieldReader<u8, M3SM_A>;
#[doc = "Bus Master 3 Supervisor Mode Access Control\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum M3SM_A {
    #[doc = "0: r/w/x; read, write and execute allowed"]
    _00 = 0,
    #[doc = "1: r/x; read and execute allowed, but no write"]
    _01 = 1,
    #[doc = "2: r/w; read and write allowed, but no execute"]
    _10 = 2,
    #[doc = "3: Same as User mode defined in M3UM"]
    _11 = 3,
}
impl From<M3SM_A> for u8 {
    #[inline(always)]
    fn from(variant: M3SM_A) -> Self {
        variant as _
    }
}
impl M3SM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M3SM_A {
        match self.bits {
            0 => M3SM_A::_00,
            1 => M3SM_A::_01,
            2 => M3SM_A::_10,
            3 => M3SM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == M3SM_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == M3SM_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == M3SM_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == M3SM_A::_11
    }
}
#[doc = "Field `M3SM` writer - Bus Master 3 Supervisor Mode Access Control"]
pub type M3SM_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, RGD_WORD2_SPEC, u8, M3SM_A, 2, O>;
impl<'a, const O: u8> M3SM_W<'a, O> {
    #[doc = "r/w/x; read, write and execute allowed"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(M3SM_A::_00)
    }
    #[doc = "r/x; read and execute allowed, but no write"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(M3SM_A::_01)
    }
    #[doc = "r/w; read and write allowed, but no execute"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(M3SM_A::_10)
    }
    #[doc = "Same as User mode defined in M3UM"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(M3SM_A::_11)
    }
}
#[doc = "Field `M3PE` reader - Bus Master 3 Process Identifier Enable"]
pub type M3PE_R = crate::BitReader<M3PE_A>;
#[doc = "Bus Master 3 Process Identifier Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M3PE_A {
    #[doc = "0: Do not include the process identifier in the evaluation"]
    _0 = 0,
    #[doc = "1: Include the process identifier and mask (RGDn_WORD3) in the region hit evaluation"]
    _1 = 1,
}
impl From<M3PE_A> for bool {
    #[inline(always)]
    fn from(variant: M3PE_A) -> Self {
        variant as u8 != 0
    }
}
impl M3PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M3PE_A {
        match self.bits {
            false => M3PE_A::_0,
            true => M3PE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M3PE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M3PE_A::_1
    }
}
#[doc = "Field `M3PE` writer - Bus Master 3 Process Identifier Enable"]
pub type M3PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGD_WORD2_SPEC, M3PE_A, O>;
impl<'a, const O: u8> M3PE_W<'a, O> {
    #[doc = "Do not include the process identifier in the evaluation"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M3PE_A::_0)
    }
    #[doc = "Include the process identifier and mask (RGDn_WORD3) in the region hit evaluation"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M3PE_A::_1)
    }
}
#[doc = "Field `M4WE` reader - Bus Master 4 Write Enable"]
pub type M4WE_R = crate::BitReader<M4WE_A>;
#[doc = "Bus Master 4 Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M4WE_A {
    #[doc = "0: Bus master 4 writes terminate with an access error and the write is not performed"]
    _0 = 0,
    #[doc = "1: Bus master 4 writes allowed"]
    _1 = 1,
}
impl From<M4WE_A> for bool {
    #[inline(always)]
    fn from(variant: M4WE_A) -> Self {
        variant as u8 != 0
    }
}
impl M4WE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M4WE_A {
        match self.bits {
            false => M4WE_A::_0,
            true => M4WE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M4WE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M4WE_A::_1
    }
}
#[doc = "Field `M4WE` writer - Bus Master 4 Write Enable"]
pub type M4WE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGD_WORD2_SPEC, M4WE_A, O>;
impl<'a, const O: u8> M4WE_W<'a, O> {
    #[doc = "Bus master 4 writes terminate with an access error and the write is not performed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M4WE_A::_0)
    }
    #[doc = "Bus master 4 writes allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M4WE_A::_1)
    }
}
#[doc = "Field `M4RE` reader - Bus Master 4 Read Enable"]
pub type M4RE_R = crate::BitReader<M4RE_A>;
#[doc = "Bus Master 4 Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M4RE_A {
    #[doc = "0: Bus master 4 reads terminate with an access error and the read is not performed"]
    _0 = 0,
    #[doc = "1: Bus master 4 reads allowed"]
    _1 = 1,
}
impl From<M4RE_A> for bool {
    #[inline(always)]
    fn from(variant: M4RE_A) -> Self {
        variant as u8 != 0
    }
}
impl M4RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M4RE_A {
        match self.bits {
            false => M4RE_A::_0,
            true => M4RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M4RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M4RE_A::_1
    }
}
#[doc = "Field `M4RE` writer - Bus Master 4 Read Enable"]
pub type M4RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGD_WORD2_SPEC, M4RE_A, O>;
impl<'a, const O: u8> M4RE_W<'a, O> {
    #[doc = "Bus master 4 reads terminate with an access error and the read is not performed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M4RE_A::_0)
    }
    #[doc = "Bus master 4 reads allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M4RE_A::_1)
    }
}
#[doc = "Field `M5WE` reader - Bus Master 5 Write Enable"]
pub type M5WE_R = crate::BitReader<M5WE_A>;
#[doc = "Bus Master 5 Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M5WE_A {
    #[doc = "0: Bus master 5 writes terminate with an access error and the write is not performed"]
    _0 = 0,
    #[doc = "1: Bus master 5 writes allowed"]
    _1 = 1,
}
impl From<M5WE_A> for bool {
    #[inline(always)]
    fn from(variant: M5WE_A) -> Self {
        variant as u8 != 0
    }
}
impl M5WE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M5WE_A {
        match self.bits {
            false => M5WE_A::_0,
            true => M5WE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M5WE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M5WE_A::_1
    }
}
#[doc = "Field `M5WE` writer - Bus Master 5 Write Enable"]
pub type M5WE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGD_WORD2_SPEC, M5WE_A, O>;
impl<'a, const O: u8> M5WE_W<'a, O> {
    #[doc = "Bus master 5 writes terminate with an access error and the write is not performed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M5WE_A::_0)
    }
    #[doc = "Bus master 5 writes allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M5WE_A::_1)
    }
}
#[doc = "Field `M5RE` reader - Bus Master 5 Read Enable"]
pub type M5RE_R = crate::BitReader<M5RE_A>;
#[doc = "Bus Master 5 Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M5RE_A {
    #[doc = "0: Bus master 5 reads terminate with an access error and the read is not performed"]
    _0 = 0,
    #[doc = "1: Bus master 5 reads allowed"]
    _1 = 1,
}
impl From<M5RE_A> for bool {
    #[inline(always)]
    fn from(variant: M5RE_A) -> Self {
        variant as u8 != 0
    }
}
impl M5RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M5RE_A {
        match self.bits {
            false => M5RE_A::_0,
            true => M5RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M5RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M5RE_A::_1
    }
}
#[doc = "Field `M5RE` writer - Bus Master 5 Read Enable"]
pub type M5RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGD_WORD2_SPEC, M5RE_A, O>;
impl<'a, const O: u8> M5RE_W<'a, O> {
    #[doc = "Bus master 5 reads terminate with an access error and the read is not performed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M5RE_A::_0)
    }
    #[doc = "Bus master 5 reads allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M5RE_A::_1)
    }
}
#[doc = "Field `M6WE` reader - Bus Master 6 Write Enable"]
pub type M6WE_R = crate::BitReader<M6WE_A>;
#[doc = "Bus Master 6 Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M6WE_A {
    #[doc = "0: Bus master 6 writes terminate with an access error and the write is not performed"]
    _0 = 0,
    #[doc = "1: Bus master 6 writes allowed"]
    _1 = 1,
}
impl From<M6WE_A> for bool {
    #[inline(always)]
    fn from(variant: M6WE_A) -> Self {
        variant as u8 != 0
    }
}
impl M6WE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M6WE_A {
        match self.bits {
            false => M6WE_A::_0,
            true => M6WE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M6WE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M6WE_A::_1
    }
}
#[doc = "Field `M6WE` writer - Bus Master 6 Write Enable"]
pub type M6WE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGD_WORD2_SPEC, M6WE_A, O>;
impl<'a, const O: u8> M6WE_W<'a, O> {
    #[doc = "Bus master 6 writes terminate with an access error and the write is not performed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M6WE_A::_0)
    }
    #[doc = "Bus master 6 writes allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M6WE_A::_1)
    }
}
#[doc = "Field `M6RE` reader - Bus Master 6 Read Enable"]
pub type M6RE_R = crate::BitReader<M6RE_A>;
#[doc = "Bus Master 6 Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M6RE_A {
    #[doc = "0: Bus master 6 reads terminate with an access error and the read is not performed"]
    _0 = 0,
    #[doc = "1: Bus master 6 reads allowed"]
    _1 = 1,
}
impl From<M6RE_A> for bool {
    #[inline(always)]
    fn from(variant: M6RE_A) -> Self {
        variant as u8 != 0
    }
}
impl M6RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M6RE_A {
        match self.bits {
            false => M6RE_A::_0,
            true => M6RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M6RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M6RE_A::_1
    }
}
#[doc = "Field `M6RE` writer - Bus Master 6 Read Enable"]
pub type M6RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGD_WORD2_SPEC, M6RE_A, O>;
impl<'a, const O: u8> M6RE_W<'a, O> {
    #[doc = "Bus master 6 reads terminate with an access error and the read is not performed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M6RE_A::_0)
    }
    #[doc = "Bus master 6 reads allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M6RE_A::_1)
    }
}
#[doc = "Field `M7WE` reader - Bus Master 7 Write Enable"]
pub type M7WE_R = crate::BitReader<M7WE_A>;
#[doc = "Bus Master 7 Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M7WE_A {
    #[doc = "0: Bus master 7 writes terminate with an access error and the write is not performed"]
    _0 = 0,
    #[doc = "1: Bus master 7 writes allowed"]
    _1 = 1,
}
impl From<M7WE_A> for bool {
    #[inline(always)]
    fn from(variant: M7WE_A) -> Self {
        variant as u8 != 0
    }
}
impl M7WE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M7WE_A {
        match self.bits {
            false => M7WE_A::_0,
            true => M7WE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M7WE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M7WE_A::_1
    }
}
#[doc = "Field `M7WE` writer - Bus Master 7 Write Enable"]
pub type M7WE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGD_WORD2_SPEC, M7WE_A, O>;
impl<'a, const O: u8> M7WE_W<'a, O> {
    #[doc = "Bus master 7 writes terminate with an access error and the write is not performed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M7WE_A::_0)
    }
    #[doc = "Bus master 7 writes allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M7WE_A::_1)
    }
}
#[doc = "Field `M7RE` reader - Bus Master 7 Read Enable"]
pub type M7RE_R = crate::BitReader<M7RE_A>;
#[doc = "Bus Master 7 Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M7RE_A {
    #[doc = "0: Bus master 7 reads terminate with an access error and the read is not performed"]
    _0 = 0,
    #[doc = "1: Bus master 7 reads allowed"]
    _1 = 1,
}
impl From<M7RE_A> for bool {
    #[inline(always)]
    fn from(variant: M7RE_A) -> Self {
        variant as u8 != 0
    }
}
impl M7RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M7RE_A {
        match self.bits {
            false => M7RE_A::_0,
            true => M7RE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M7RE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M7RE_A::_1
    }
}
#[doc = "Field `M7RE` writer - Bus Master 7 Read Enable"]
pub type M7RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGD_WORD2_SPEC, M7RE_A, O>;
impl<'a, const O: u8> M7RE_W<'a, O> {
    #[doc = "Bus master 7 reads terminate with an access error and the read is not performed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M7RE_A::_0)
    }
    #[doc = "Bus master 7 reads allowed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M7RE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - Bus Master 0 User Mode Access Control"]
    #[inline(always)]
    pub fn m0um(&self) -> M0UM_R {
        M0UM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Bus Master 0 Supervisor Mode Access Control"]
    #[inline(always)]
    pub fn m0sm(&self) -> M0SM_R {
        M0SM_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Bus Master 0 Process Identifier enable"]
    #[inline(always)]
    pub fn m0pe(&self) -> M0PE_R {
        M0PE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - Bus Master 1 User Mode Access Control"]
    #[inline(always)]
    pub fn m1um(&self) -> M1UM_R {
        M1UM_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:10 - Bus Master 1 Supervisor Mode Access Control"]
    #[inline(always)]
    pub fn m1sm(&self) -> M1SM_R {
        M1SM_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Bus Master 1 Process Identifier enable"]
    #[inline(always)]
    pub fn m1pe(&self) -> M1PE_R {
        M1PE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Bus Master 2 User Mode Access control"]
    #[inline(always)]
    pub fn m2um(&self) -> M2UM_R {
        M2UM_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:16 - Bus Master 2 Supervisor Mode Access Control"]
    #[inline(always)]
    pub fn m2sm(&self) -> M2SM_R {
        M2SM_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - Bus Master 2 Process Identifier Enable"]
    #[inline(always)]
    pub fn m2pe(&self) -> M2PE_R {
        M2PE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Bus Master 3 User Mode Access Control"]
    #[inline(always)]
    pub fn m3um(&self) -> M3UM_R {
        M3UM_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:22 - Bus Master 3 Supervisor Mode Access Control"]
    #[inline(always)]
    pub fn m3sm(&self) -> M3SM_R {
        M3SM_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Bus Master 3 Process Identifier Enable"]
    #[inline(always)]
    pub fn m3pe(&self) -> M3PE_R {
        M3PE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Bus Master 4 Write Enable"]
    #[inline(always)]
    pub fn m4we(&self) -> M4WE_R {
        M4WE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus Master 4 Read Enable"]
    #[inline(always)]
    pub fn m4re(&self) -> M4RE_R {
        M4RE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bus Master 5 Write Enable"]
    #[inline(always)]
    pub fn m5we(&self) -> M5WE_R {
        M5WE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bus Master 5 Read Enable"]
    #[inline(always)]
    pub fn m5re(&self) -> M5RE_R {
        M5RE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Bus Master 6 Write Enable"]
    #[inline(always)]
    pub fn m6we(&self) -> M6WE_R {
        M6WE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Bus Master 6 Read Enable"]
    #[inline(always)]
    pub fn m6re(&self) -> M6RE_R {
        M6RE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Bus Master 7 Write Enable"]
    #[inline(always)]
    pub fn m7we(&self) -> M7WE_R {
        M7WE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Bus Master 7 Read Enable"]
    #[inline(always)]
    pub fn m7re(&self) -> M7RE_R {
        M7RE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Bus Master 0 User Mode Access Control"]
    #[inline(always)]
    #[must_use]
    pub fn m0um(&mut self) -> M0UM_W<0> {
        M0UM_W::new(self)
    }
    #[doc = "Bits 3:4 - Bus Master 0 Supervisor Mode Access Control"]
    #[inline(always)]
    #[must_use]
    pub fn m0sm(&mut self) -> M0SM_W<3> {
        M0SM_W::new(self)
    }
    #[doc = "Bit 5 - Bus Master 0 Process Identifier enable"]
    #[inline(always)]
    #[must_use]
    pub fn m0pe(&mut self) -> M0PE_W<5> {
        M0PE_W::new(self)
    }
    #[doc = "Bits 6:8 - Bus Master 1 User Mode Access Control"]
    #[inline(always)]
    #[must_use]
    pub fn m1um(&mut self) -> M1UM_W<6> {
        M1UM_W::new(self)
    }
    #[doc = "Bits 9:10 - Bus Master 1 Supervisor Mode Access Control"]
    #[inline(always)]
    #[must_use]
    pub fn m1sm(&mut self) -> M1SM_W<9> {
        M1SM_W::new(self)
    }
    #[doc = "Bit 11 - Bus Master 1 Process Identifier enable"]
    #[inline(always)]
    #[must_use]
    pub fn m1pe(&mut self) -> M1PE_W<11> {
        M1PE_W::new(self)
    }
    #[doc = "Bits 12:14 - Bus Master 2 User Mode Access control"]
    #[inline(always)]
    #[must_use]
    pub fn m2um(&mut self) -> M2UM_W<12> {
        M2UM_W::new(self)
    }
    #[doc = "Bits 15:16 - Bus Master 2 Supervisor Mode Access Control"]
    #[inline(always)]
    #[must_use]
    pub fn m2sm(&mut self) -> M2SM_W<15> {
        M2SM_W::new(self)
    }
    #[doc = "Bit 17 - Bus Master 2 Process Identifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn m2pe(&mut self) -> M2PE_W<17> {
        M2PE_W::new(self)
    }
    #[doc = "Bits 18:20 - Bus Master 3 User Mode Access Control"]
    #[inline(always)]
    #[must_use]
    pub fn m3um(&mut self) -> M3UM_W<18> {
        M3UM_W::new(self)
    }
    #[doc = "Bits 21:22 - Bus Master 3 Supervisor Mode Access Control"]
    #[inline(always)]
    #[must_use]
    pub fn m3sm(&mut self) -> M3SM_W<21> {
        M3SM_W::new(self)
    }
    #[doc = "Bit 23 - Bus Master 3 Process Identifier Enable"]
    #[inline(always)]
    #[must_use]
    pub fn m3pe(&mut self) -> M3PE_W<23> {
        M3PE_W::new(self)
    }
    #[doc = "Bit 24 - Bus Master 4 Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn m4we(&mut self) -> M4WE_W<24> {
        M4WE_W::new(self)
    }
    #[doc = "Bit 25 - Bus Master 4 Read Enable"]
    #[inline(always)]
    #[must_use]
    pub fn m4re(&mut self) -> M4RE_W<25> {
        M4RE_W::new(self)
    }
    #[doc = "Bit 26 - Bus Master 5 Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn m5we(&mut self) -> M5WE_W<26> {
        M5WE_W::new(self)
    }
    #[doc = "Bit 27 - Bus Master 5 Read Enable"]
    #[inline(always)]
    #[must_use]
    pub fn m5re(&mut self) -> M5RE_W<27> {
        M5RE_W::new(self)
    }
    #[doc = "Bit 28 - Bus Master 6 Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn m6we(&mut self) -> M6WE_W<28> {
        M6WE_W::new(self)
    }
    #[doc = "Bit 29 - Bus Master 6 Read Enable"]
    #[inline(always)]
    #[must_use]
    pub fn m6re(&mut self) -> M6RE_W<29> {
        M6RE_W::new(self)
    }
    #[doc = "Bit 30 - Bus Master 7 Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn m7we(&mut self) -> M7WE_W<30> {
        M7WE_W::new(self)
    }
    #[doc = "Bit 31 - Bus Master 7 Read Enable"]
    #[inline(always)]
    #[must_use]
    pub fn m7re(&mut self) -> M7RE_W<31> {
        M7RE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Region Descriptor n, Word 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgd_word2](index.html) module"]
pub struct RGD_WORD2_SPEC;
impl crate::RegisterSpec for RGD_WORD2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rgd_word2::R](R) reader structure"]
impl crate::Readable for RGD_WORD2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rgd_word2::W](W) writer structure"]
impl crate::Writable for RGD_WORD2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RGD%s_WORD2 to value 0x0061_f7df"]
impl crate::Resettable for RGD_WORD2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0061_f7df;
}
