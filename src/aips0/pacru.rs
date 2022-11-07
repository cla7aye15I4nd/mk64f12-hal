#[doc = "Register `PACRU` reader"]
pub struct R(crate::R<PACRU_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PACRU_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PACRU_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PACRU_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PACRU` writer"]
pub struct W(crate::W<PACRU_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PACRU_SPEC>;
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
impl From<crate::W<PACRU_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PACRU_SPEC>) -> Self {
        W(writer)
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
pub type TP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRU_SPEC, TP1_A, O>;
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
pub type WP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRU_SPEC, WP1_A, O>;
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
pub type SP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRU_SPEC, SP1_A, O>;
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
pub type TP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRU_SPEC, TP0_A, O>;
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
pub type WP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRU_SPEC, WP0_A, O>;
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
pub type SP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PACRU_SPEC, SP0_A, O>;
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
#[doc = "Peripheral Access Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pacru](index.html) module"]
pub struct PACRU_SPEC;
impl crate::RegisterSpec for PACRU_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pacru::R](R) reader structure"]
impl crate::Readable for PACRU_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pacru::W](W) writer structure"]
impl crate::Writable for PACRU_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PACRU to value 0x4400_0000"]
impl crate::Resettable for PACRU_SPEC {
    const RESET_VALUE: Self::Ux = 0x4400_0000;
}
