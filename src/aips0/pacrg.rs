#[doc = "Register `PACRG` reader"]
pub struct R(crate::R<PACRG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PACRG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PACRG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PACRG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PACRG` writer"]
pub struct W(crate::W<PACRG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PACRG_SPEC>;
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
impl From<crate::W<PACRG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PACRG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TP7` reader - Trusted Protect"]
pub type TP7_R = crate::BitReader<TP7_A>;
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP7_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP7_A> for bool {
    #[inline(always)]
    fn from(variant: TP7_A) -> Self {
        variant as u8 != 0
    }
}
impl TP7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP7_A {
        match self.bits {
            false => TP7_A::_0,
            true => TP7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TP7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TP7_A::_1
    }
}
#[doc = "Field `TP7` writer - Trusted Protect"]
pub type TP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, TP7_A, O>;
impl<'a, const O: u8> TP7_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP7_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP7_A::_1)
    }
}
#[doc = "Field `WP7` reader - Write Protect"]
pub type WP7_R = crate::BitReader<WP7_A>;
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP7_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP7_A> for bool {
    #[inline(always)]
    fn from(variant: WP7_A) -> Self {
        variant as u8 != 0
    }
}
impl WP7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP7_A {
        match self.bits {
            false => WP7_A::_0,
            true => WP7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP7_A::_1
    }
}
#[doc = "Field `WP7` writer - Write Protect"]
pub type WP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, WP7_A, O>;
impl<'a, const O: u8> WP7_W<'a, O> {
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP7_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP7_A::_1)
    }
}
#[doc = "Field `SP7` reader - Supervisor Protect"]
pub type SP7_R = crate::BitReader<SP7_A>;
#[doc = "Supervisor Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SP7_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP7_A> for bool {
    #[inline(always)]
    fn from(variant: SP7_A) -> Self {
        variant as u8 != 0
    }
}
impl SP7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP7_A {
        match self.bits {
            false => SP7_A::_0,
            true => SP7_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SP7_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SP7_A::_1
    }
}
#[doc = "Field `SP7` writer - Supervisor Protect"]
pub type SP7_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, SP7_A, O>;
impl<'a, const O: u8> SP7_W<'a, O> {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP7_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP7_A::_1)
    }
}
#[doc = "Field `TP6` reader - Trusted Protect"]
pub type TP6_R = crate::BitReader<TP6_A>;
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP6_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP6_A> for bool {
    #[inline(always)]
    fn from(variant: TP6_A) -> Self {
        variant as u8 != 0
    }
}
impl TP6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP6_A {
        match self.bits {
            false => TP6_A::_0,
            true => TP6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TP6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TP6_A::_1
    }
}
#[doc = "Field `TP6` writer - Trusted Protect"]
pub type TP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, TP6_A, O>;
impl<'a, const O: u8> TP6_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP6_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP6_A::_1)
    }
}
#[doc = "Field `WP6` reader - Write Protect"]
pub type WP6_R = crate::BitReader<WP6_A>;
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP6_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP6_A> for bool {
    #[inline(always)]
    fn from(variant: WP6_A) -> Self {
        variant as u8 != 0
    }
}
impl WP6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP6_A {
        match self.bits {
            false => WP6_A::_0,
            true => WP6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP6_A::_1
    }
}
#[doc = "Field `WP6` writer - Write Protect"]
pub type WP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, WP6_A, O>;
impl<'a, const O: u8> WP6_W<'a, O> {
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP6_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP6_A::_1)
    }
}
#[doc = "Field `SP6` reader - Supervisor Protect"]
pub type SP6_R = crate::BitReader<SP6_A>;
#[doc = "Supervisor Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SP6_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP6_A> for bool {
    #[inline(always)]
    fn from(variant: SP6_A) -> Self {
        variant as u8 != 0
    }
}
impl SP6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP6_A {
        match self.bits {
            false => SP6_A::_0,
            true => SP6_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SP6_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SP6_A::_1
    }
}
#[doc = "Field `SP6` writer - Supervisor Protect"]
pub type SP6_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, SP6_A, O>;
impl<'a, const O: u8> SP6_W<'a, O> {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP6_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP6_A::_1)
    }
}
#[doc = "Field `TP5` reader - Trusted Protect"]
pub type TP5_R = crate::BitReader<TP5_A>;
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP5_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP5_A> for bool {
    #[inline(always)]
    fn from(variant: TP5_A) -> Self {
        variant as u8 != 0
    }
}
impl TP5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP5_A {
        match self.bits {
            false => TP5_A::_0,
            true => TP5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TP5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TP5_A::_1
    }
}
#[doc = "Field `TP5` writer - Trusted Protect"]
pub type TP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, TP5_A, O>;
impl<'a, const O: u8> TP5_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP5_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP5_A::_1)
    }
}
#[doc = "Field `WP5` reader - Write Protect"]
pub type WP5_R = crate::BitReader<WP5_A>;
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP5_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP5_A> for bool {
    #[inline(always)]
    fn from(variant: WP5_A) -> Self {
        variant as u8 != 0
    }
}
impl WP5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP5_A {
        match self.bits {
            false => WP5_A::_0,
            true => WP5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP5_A::_1
    }
}
#[doc = "Field `WP5` writer - Write Protect"]
pub type WP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, WP5_A, O>;
impl<'a, const O: u8> WP5_W<'a, O> {
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP5_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP5_A::_1)
    }
}
#[doc = "Field `SP5` reader - Supervisor Protect"]
pub type SP5_R = crate::BitReader<SP5_A>;
#[doc = "Supervisor Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SP5_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP5_A> for bool {
    #[inline(always)]
    fn from(variant: SP5_A) -> Self {
        variant as u8 != 0
    }
}
impl SP5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP5_A {
        match self.bits {
            false => SP5_A::_0,
            true => SP5_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SP5_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SP5_A::_1
    }
}
#[doc = "Field `SP5` writer - Supervisor Protect"]
pub type SP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, SP5_A, O>;
impl<'a, const O: u8> SP5_W<'a, O> {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP5_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP5_A::_1)
    }
}
#[doc = "Field `TP4` reader - Trusted Protect"]
pub type TP4_R = crate::BitReader<TP4_A>;
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP4_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP4_A> for bool {
    #[inline(always)]
    fn from(variant: TP4_A) -> Self {
        variant as u8 != 0
    }
}
impl TP4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP4_A {
        match self.bits {
            false => TP4_A::_0,
            true => TP4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TP4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TP4_A::_1
    }
}
#[doc = "Field `TP4` writer - Trusted Protect"]
pub type TP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, TP4_A, O>;
impl<'a, const O: u8> TP4_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP4_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP4_A::_1)
    }
}
#[doc = "Field `WP4` reader - Write Protect"]
pub type WP4_R = crate::BitReader<WP4_A>;
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP4_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP4_A> for bool {
    #[inline(always)]
    fn from(variant: WP4_A) -> Self {
        variant as u8 != 0
    }
}
impl WP4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP4_A {
        match self.bits {
            false => WP4_A::_0,
            true => WP4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP4_A::_1
    }
}
#[doc = "Field `WP4` writer - Write Protect"]
pub type WP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, WP4_A, O>;
impl<'a, const O: u8> WP4_W<'a, O> {
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP4_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP4_A::_1)
    }
}
#[doc = "Field `SP4` reader - Supervisor Protect"]
pub type SP4_R = crate::BitReader<SP4_A>;
#[doc = "Supervisor Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SP4_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP4_A> for bool {
    #[inline(always)]
    fn from(variant: SP4_A) -> Self {
        variant as u8 != 0
    }
}
impl SP4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP4_A {
        match self.bits {
            false => SP4_A::_0,
            true => SP4_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SP4_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SP4_A::_1
    }
}
#[doc = "Field `SP4` writer - Supervisor Protect"]
pub type SP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, SP4_A, O>;
impl<'a, const O: u8> SP4_W<'a, O> {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP4_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP4_A::_1)
    }
}
#[doc = "Field `TP3` reader - Trusted Protect"]
pub type TP3_R = crate::BitReader<TP3_A>;
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP3_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP3_A> for bool {
    #[inline(always)]
    fn from(variant: TP3_A) -> Self {
        variant as u8 != 0
    }
}
impl TP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP3_A {
        match self.bits {
            false => TP3_A::_0,
            true => TP3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TP3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TP3_A::_1
    }
}
#[doc = "Field `TP3` writer - Trusted Protect"]
pub type TP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, TP3_A, O>;
impl<'a, const O: u8> TP3_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP3_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP3_A::_1)
    }
}
#[doc = "Field `WP3` reader - Write Protect"]
pub type WP3_R = crate::BitReader<WP3_A>;
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP3_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP3_A> for bool {
    #[inline(always)]
    fn from(variant: WP3_A) -> Self {
        variant as u8 != 0
    }
}
impl WP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP3_A {
        match self.bits {
            false => WP3_A::_0,
            true => WP3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP3_A::_1
    }
}
#[doc = "Field `WP3` writer - Write Protect"]
pub type WP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, WP3_A, O>;
impl<'a, const O: u8> WP3_W<'a, O> {
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP3_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP3_A::_1)
    }
}
#[doc = "Field `SP3` reader - Supervisor Protect"]
pub type SP3_R = crate::BitReader<SP3_A>;
#[doc = "Supervisor Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SP3_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP3_A> for bool {
    #[inline(always)]
    fn from(variant: SP3_A) -> Self {
        variant as u8 != 0
    }
}
impl SP3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP3_A {
        match self.bits {
            false => SP3_A::_0,
            true => SP3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SP3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SP3_A::_1
    }
}
#[doc = "Field `SP3` writer - Supervisor Protect"]
pub type SP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, SP3_A, O>;
impl<'a, const O: u8> SP3_W<'a, O> {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP3_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP3_A::_1)
    }
}
#[doc = "Field `TP2` reader - Trusted Protect"]
pub type TP2_R = crate::BitReader<TP2_A>;
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP2_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP2_A> for bool {
    #[inline(always)]
    fn from(variant: TP2_A) -> Self {
        variant as u8 != 0
    }
}
impl TP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP2_A {
        match self.bits {
            false => TP2_A::_0,
            true => TP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TP2_A::_1
    }
}
#[doc = "Field `TP2` writer - Trusted Protect"]
pub type TP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, TP2_A, O>;
impl<'a, const O: u8> TP2_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP2_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP2_A::_1)
    }
}
#[doc = "Field `WP2` reader - Write Protect"]
pub type WP2_R = crate::BitReader<WP2_A>;
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP2_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP2_A> for bool {
    #[inline(always)]
    fn from(variant: WP2_A) -> Self {
        variant as u8 != 0
    }
}
impl WP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP2_A {
        match self.bits {
            false => WP2_A::_0,
            true => WP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP2_A::_1
    }
}
#[doc = "Field `WP2` writer - Write Protect"]
pub type WP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, WP2_A, O>;
impl<'a, const O: u8> WP2_W<'a, O> {
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP2_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP2_A::_1)
    }
}
#[doc = "Field `SP2` reader - Supervisor Protect"]
pub type SP2_R = crate::BitReader<SP2_A>;
#[doc = "Supervisor Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SP2_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP2_A> for bool {
    #[inline(always)]
    fn from(variant: SP2_A) -> Self {
        variant as u8 != 0
    }
}
impl SP2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP2_A {
        match self.bits {
            false => SP2_A::_0,
            true => SP2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SP2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SP2_A::_1
    }
}
#[doc = "Field `SP2` writer - Supervisor Protect"]
pub type SP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, SP2_A, O>;
impl<'a, const O: u8> SP2_W<'a, O> {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP2_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP2_A::_1)
    }
}
#[doc = "Field `TP1` reader - Trusted Protect"]
pub type TP1_R = crate::BitReader<TP1_A>;
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP1_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP1_A> for bool {
    #[inline(always)]
    fn from(variant: TP1_A) -> Self {
        variant as u8 != 0
    }
}
impl TP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP1_A {
        match self.bits {
            false => TP1_A::_0,
            true => TP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TP1_A::_1
    }
}
#[doc = "Field `TP1` writer - Trusted Protect"]
pub type TP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, TP1_A, O>;
impl<'a, const O: u8> TP1_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP1_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP1_A::_1)
    }
}
#[doc = "Field `WP1` reader - Write Protect"]
pub type WP1_R = crate::BitReader<WP1_A>;
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP1_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP1_A> for bool {
    #[inline(always)]
    fn from(variant: WP1_A) -> Self {
        variant as u8 != 0
    }
}
impl WP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP1_A {
        match self.bits {
            false => WP1_A::_0,
            true => WP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP1_A::_1
    }
}
#[doc = "Field `WP1` writer - Write Protect"]
pub type WP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, WP1_A, O>;
impl<'a, const O: u8> WP1_W<'a, O> {
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP1_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP1_A::_1)
    }
}
#[doc = "Field `SP1` reader - Supervisor Protect"]
pub type SP1_R = crate::BitReader<SP1_A>;
#[doc = "Supervisor Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SP1_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP1_A> for bool {
    #[inline(always)]
    fn from(variant: SP1_A) -> Self {
        variant as u8 != 0
    }
}
impl SP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP1_A {
        match self.bits {
            false => SP1_A::_0,
            true => SP1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SP1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SP1_A::_1
    }
}
#[doc = "Field `SP1` writer - Supervisor Protect"]
pub type SP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, SP1_A, O>;
impl<'a, const O: u8> SP1_W<'a, O> {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP1_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP1_A::_1)
    }
}
#[doc = "Field `TP0` reader - Trusted Protect"]
pub type TP0_R = crate::BitReader<TP0_A>;
#[doc = "Trusted Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP0_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    _0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed."]
    _1 = 1,
}
impl From<TP0_A> for bool {
    #[inline(always)]
    fn from(variant: TP0_A) -> Self {
        variant as u8 != 0
    }
}
impl TP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP0_A {
        match self.bits {
            false => TP0_A::_0,
            true => TP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TP0_A::_1
    }
}
#[doc = "Field `TP0` writer - Trusted Protect"]
pub type TP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, TP0_A, O>;
impl<'a, const O: u8> TP0_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TP0_A::_0)
    }
    #[doc = "Accesses from an untrusted master are not allowed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TP0_A::_1)
    }
}
#[doc = "Field `WP0` reader - Write Protect"]
pub type WP0_R = crate::BitReader<WP0_A>;
#[doc = "Write Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WP0_A {
    #[doc = "0: This peripheral allows write accesses."]
    _0 = 0,
    #[doc = "1: This peripheral is write protected."]
    _1 = 1,
}
impl From<WP0_A> for bool {
    #[inline(always)]
    fn from(variant: WP0_A) -> Self {
        variant as u8 != 0
    }
}
impl WP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WP0_A {
        match self.bits {
            false => WP0_A::_0,
            true => WP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WP0_A::_1
    }
}
#[doc = "Field `WP0` writer - Write Protect"]
pub type WP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, WP0_A, O>;
impl<'a, const O: u8> WP0_W<'a, O> {
    #[doc = "This peripheral allows write accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WP0_A::_0)
    }
    #[doc = "This peripheral is write protected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WP0_A::_1)
    }
}
#[doc = "Field `SP0` reader - Supervisor Protect"]
pub type SP0_R = crate::BitReader<SP0_A>;
#[doc = "Supervisor Protect\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SP0_A {
    #[doc = "0: This peripheral does not require supervisor privilege level for accesses."]
    _0 = 0,
    #[doc = "1: This peripheral requires supervisor privilege level for accesses."]
    _1 = 1,
}
impl From<SP0_A> for bool {
    #[inline(always)]
    fn from(variant: SP0_A) -> Self {
        variant as u8 != 0
    }
}
impl SP0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SP0_A {
        match self.bits {
            false => SP0_A::_0,
            true => SP0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SP0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SP0_A::_1
    }
}
#[doc = "Field `SP0` writer - Supervisor Protect"]
pub type SP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRG_SPEC, SP0_A, O>;
impl<'a, const O: u8> SP0_W<'a, O> {
    #[doc = "This peripheral does not require supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SP0_A::_0)
    }
    #[doc = "This peripheral requires supervisor privilege level for accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SP0_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Trusted Protect"]
    #[inline(always)]
    pub fn tp7(&self) -> TP7_R {
        TP7_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Protect"]
    #[inline(always)]
    pub fn wp7(&self) -> WP7_R {
        WP7_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp7(&self) -> SP7_R {
        SP7_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Trusted Protect"]
    #[inline(always)]
    pub fn tp6(&self) -> TP6_R {
        TP6_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write Protect"]
    #[inline(always)]
    pub fn wp6(&self) -> WP6_R {
        WP6_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp6(&self) -> SP6_R {
        SP6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Trusted Protect"]
    #[inline(always)]
    pub fn tp5(&self) -> TP5_R {
        TP5_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write Protect"]
    #[inline(always)]
    pub fn wp5(&self) -> WP5_R {
        WP5_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp5(&self) -> SP5_R {
        SP5_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - Trusted Protect"]
    #[inline(always)]
    pub fn tp4(&self) -> TP4_R {
        TP4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write Protect"]
    #[inline(always)]
    pub fn wp4(&self) -> WP4_R {
        WP4_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp4(&self) -> SP4_R {
        SP4_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Trusted Protect"]
    #[inline(always)]
    pub fn tp3(&self) -> TP3_R {
        TP3_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write Protect"]
    #[inline(always)]
    pub fn wp3(&self) -> WP3_R {
        WP3_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp3(&self) -> SP3_R {
        SP3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Trusted Protect"]
    #[inline(always)]
    pub fn tp2(&self) -> TP2_R {
        TP2_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Write Protect"]
    #[inline(always)]
    pub fn wp2(&self) -> WP2_R {
        WP2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp2(&self) -> SP2_R {
        SP2_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - Trusted Protect"]
    #[inline(always)]
    pub fn tp1(&self) -> TP1_R {
        TP1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write Protect"]
    #[inline(always)]
    pub fn wp1(&self) -> WP1_R {
        WP1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp1(&self) -> SP1_R {
        SP1_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Trusted Protect"]
    #[inline(always)]
    pub fn tp0(&self) -> TP0_R {
        TP0_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Write Protect"]
    #[inline(always)]
    pub fn wp0(&self) -> WP0_R {
        WP0_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Supervisor Protect"]
    #[inline(always)]
    pub fn sp0(&self) -> SP0_R {
        SP0_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Trusted Protect"]
    #[inline(always)]
    #[must_use]
    pub fn tp7(&mut self) -> TP7_W<0> {
        TP7_W::new(self)
    }
    #[doc = "Bit 1 - Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn wp7(&mut self) -> WP7_W<1> {
        WP7_W::new(self)
    }
    #[doc = "Bit 2 - Supervisor Protect"]
    #[inline(always)]
    #[must_use]
    pub fn sp7(&mut self) -> SP7_W<2> {
        SP7_W::new(self)
    }
    #[doc = "Bit 4 - Trusted Protect"]
    #[inline(always)]
    #[must_use]
    pub fn tp6(&mut self) -> TP6_W<4> {
        TP6_W::new(self)
    }
    #[doc = "Bit 5 - Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn wp6(&mut self) -> WP6_W<5> {
        WP6_W::new(self)
    }
    #[doc = "Bit 6 - Supervisor Protect"]
    #[inline(always)]
    #[must_use]
    pub fn sp6(&mut self) -> SP6_W<6> {
        SP6_W::new(self)
    }
    #[doc = "Bit 8 - Trusted Protect"]
    #[inline(always)]
    #[must_use]
    pub fn tp5(&mut self) -> TP5_W<8> {
        TP5_W::new(self)
    }
    #[doc = "Bit 9 - Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn wp5(&mut self) -> WP5_W<9> {
        WP5_W::new(self)
    }
    #[doc = "Bit 10 - Supervisor Protect"]
    #[inline(always)]
    #[must_use]
    pub fn sp5(&mut self) -> SP5_W<10> {
        SP5_W::new(self)
    }
    #[doc = "Bit 12 - Trusted Protect"]
    #[inline(always)]
    #[must_use]
    pub fn tp4(&mut self) -> TP4_W<12> {
        TP4_W::new(self)
    }
    #[doc = "Bit 13 - Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn wp4(&mut self) -> WP4_W<13> {
        WP4_W::new(self)
    }
    #[doc = "Bit 14 - Supervisor Protect"]
    #[inline(always)]
    #[must_use]
    pub fn sp4(&mut self) -> SP4_W<14> {
        SP4_W::new(self)
    }
    #[doc = "Bit 16 - Trusted Protect"]
    #[inline(always)]
    #[must_use]
    pub fn tp3(&mut self) -> TP3_W<16> {
        TP3_W::new(self)
    }
    #[doc = "Bit 17 - Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn wp3(&mut self) -> WP3_W<17> {
        WP3_W::new(self)
    }
    #[doc = "Bit 18 - Supervisor Protect"]
    #[inline(always)]
    #[must_use]
    pub fn sp3(&mut self) -> SP3_W<18> {
        SP3_W::new(self)
    }
    #[doc = "Bit 20 - Trusted Protect"]
    #[inline(always)]
    #[must_use]
    pub fn tp2(&mut self) -> TP2_W<20> {
        TP2_W::new(self)
    }
    #[doc = "Bit 21 - Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn wp2(&mut self) -> WP2_W<21> {
        WP2_W::new(self)
    }
    #[doc = "Bit 22 - Supervisor Protect"]
    #[inline(always)]
    #[must_use]
    pub fn sp2(&mut self) -> SP2_W<22> {
        SP2_W::new(self)
    }
    #[doc = "Bit 24 - Trusted Protect"]
    #[inline(always)]
    #[must_use]
    pub fn tp1(&mut self) -> TP1_W<24> {
        TP1_W::new(self)
    }
    #[doc = "Bit 25 - Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn wp1(&mut self) -> WP1_W<25> {
        WP1_W::new(self)
    }
    #[doc = "Bit 26 - Supervisor Protect"]
    #[inline(always)]
    #[must_use]
    pub fn sp1(&mut self) -> SP1_W<26> {
        SP1_W::new(self)
    }
    #[doc = "Bit 28 - Trusted Protect"]
    #[inline(always)]
    #[must_use]
    pub fn tp0(&mut self) -> TP0_W<28> {
        TP0_W::new(self)
    }
    #[doc = "Bit 29 - Write Protect"]
    #[inline(always)]
    #[must_use]
    pub fn wp0(&mut self) -> WP0_W<29> {
        WP0_W::new(self)
    }
    #[doc = "Bit 30 - Supervisor Protect"]
    #[inline(always)]
    #[must_use]
    pub fn sp0(&mut self) -> SP0_W<30> {
        SP0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pacrg](index.html) module"]
pub struct PACRG_SPEC;
impl crate::RegisterSpec for PACRG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pacrg::R](R) reader structure"]
impl crate::Readable for PACRG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pacrg::W](W) writer structure"]
impl crate::Writable for PACRG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PACRG to value 0x4444_4444"]
impl crate::Resettable for PACRG_SPEC {
    const RESET_VALUE: Self::Ux = 0x4444_4444;
}
