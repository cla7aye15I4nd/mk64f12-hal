#[doc = "Register `ERREN` reader"]
pub struct R(crate::R<ERREN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERREN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERREN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERREN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERREN` writer"]
pub struct W(crate::W<ERREN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERREN_SPEC>;
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
impl From<crate::W<ERREN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERREN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIDERREN` reader - PIDERR Interrupt Enable"]
pub type PIDERREN_R = crate::BitReader<PIDERREN_A>;
#[doc = "PIDERR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIDERREN_A {
    #[doc = "0: Disables the PIDERR interrupt."]
    _0 = 0,
    #[doc = "1: Enters the PIDERR interrupt."]
    _1 = 1,
}
impl From<PIDERREN_A> for bool {
    #[inline(always)]
    fn from(variant: PIDERREN_A) -> Self {
        variant as u8 != 0
    }
}
impl PIDERREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIDERREN_A {
        match self.bits {
            false => PIDERREN_A::_0,
            true => PIDERREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIDERREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIDERREN_A::_1
    }
}
#[doc = "Field `PIDERREN` writer - PIDERR Interrupt Enable"]
pub type PIDERREN_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERREN_SPEC, PIDERREN_A, O>;
impl<'a, const O: u8> PIDERREN_W<'a, O> {
    #[doc = "Disables the PIDERR interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIDERREN_A::_0)
    }
    #[doc = "Enters the PIDERR interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIDERREN_A::_1)
    }
}
#[doc = "Field `CRC5EOFEN` reader - CRC5/EOF Interrupt Enable"]
pub type CRC5EOFEN_R = crate::BitReader<CRC5EOFEN_A>;
#[doc = "CRC5/EOF Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRC5EOFEN_A {
    #[doc = "0: Disables the CRC5/EOF interrupt."]
    _0 = 0,
    #[doc = "1: Enables the CRC5/EOF interrupt."]
    _1 = 1,
}
impl From<CRC5EOFEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRC5EOFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRC5EOFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC5EOFEN_A {
        match self.bits {
            false => CRC5EOFEN_A::_0,
            true => CRC5EOFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRC5EOFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRC5EOFEN_A::_1
    }
}
#[doc = "Field `CRC5EOFEN` writer - CRC5/EOF Interrupt Enable"]
pub type CRC5EOFEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERREN_SPEC, CRC5EOFEN_A, O>;
impl<'a, const O: u8> CRC5EOFEN_W<'a, O> {
    #[doc = "Disables the CRC5/EOF interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC5EOFEN_A::_0)
    }
    #[doc = "Enables the CRC5/EOF interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC5EOFEN_A::_1)
    }
}
#[doc = "Field `CRC16EN` reader - CRC16 Interrupt Enable"]
pub type CRC16EN_R = crate::BitReader<CRC16EN_A>;
#[doc = "CRC16 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRC16EN_A {
    #[doc = "0: Disables the CRC16 interrupt."]
    _0 = 0,
    #[doc = "1: Enables the CRC16 interrupt."]
    _1 = 1,
}
impl From<CRC16EN_A> for bool {
    #[inline(always)]
    fn from(variant: CRC16EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRC16EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC16EN_A {
        match self.bits {
            false => CRC16EN_A::_0,
            true => CRC16EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRC16EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRC16EN_A::_1
    }
}
#[doc = "Field `CRC16EN` writer - CRC16 Interrupt Enable"]
pub type CRC16EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERREN_SPEC, CRC16EN_A, O>;
impl<'a, const O: u8> CRC16EN_W<'a, O> {
    #[doc = "Disables the CRC16 interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC16EN_A::_0)
    }
    #[doc = "Enables the CRC16 interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC16EN_A::_1)
    }
}
#[doc = "Field `DFN8EN` reader - DFN8 Interrupt Enable"]
pub type DFN8EN_R = crate::BitReader<DFN8EN_A>;
#[doc = "DFN8 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DFN8EN_A {
    #[doc = "0: Disables the DFN8 interrupt."]
    _0 = 0,
    #[doc = "1: Enables the DFN8 interrupt."]
    _1 = 1,
}
impl From<DFN8EN_A> for bool {
    #[inline(always)]
    fn from(variant: DFN8EN_A) -> Self {
        variant as u8 != 0
    }
}
impl DFN8EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFN8EN_A {
        match self.bits {
            false => DFN8EN_A::_0,
            true => DFN8EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFN8EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFN8EN_A::_1
    }
}
#[doc = "Field `DFN8EN` writer - DFN8 Interrupt Enable"]
pub type DFN8EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERREN_SPEC, DFN8EN_A, O>;
impl<'a, const O: u8> DFN8EN_W<'a, O> {
    #[doc = "Disables the DFN8 interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFN8EN_A::_0)
    }
    #[doc = "Enables the DFN8 interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFN8EN_A::_1)
    }
}
#[doc = "Field `BTOERREN` reader - BTOERR Interrupt Enable"]
pub type BTOERREN_R = crate::BitReader<BTOERREN_A>;
#[doc = "BTOERR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BTOERREN_A {
    #[doc = "0: Disables the BTOERR interrupt."]
    _0 = 0,
    #[doc = "1: Enables the BTOERR interrupt."]
    _1 = 1,
}
impl From<BTOERREN_A> for bool {
    #[inline(always)]
    fn from(variant: BTOERREN_A) -> Self {
        variant as u8 != 0
    }
}
impl BTOERREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTOERREN_A {
        match self.bits {
            false => BTOERREN_A::_0,
            true => BTOERREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BTOERREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BTOERREN_A::_1
    }
}
#[doc = "Field `BTOERREN` writer - BTOERR Interrupt Enable"]
pub type BTOERREN_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERREN_SPEC, BTOERREN_A, O>;
impl<'a, const O: u8> BTOERREN_W<'a, O> {
    #[doc = "Disables the BTOERR interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BTOERREN_A::_0)
    }
    #[doc = "Enables the BTOERR interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BTOERREN_A::_1)
    }
}
#[doc = "Field `DMAERREN` reader - DMAERR Interrupt Enable"]
pub type DMAERREN_R = crate::BitReader<DMAERREN_A>;
#[doc = "DMAERR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAERREN_A {
    #[doc = "0: Disables the DMAERR interrupt."]
    _0 = 0,
    #[doc = "1: Enables the DMAERR interrupt."]
    _1 = 1,
}
impl From<DMAERREN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAERREN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAERREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAERREN_A {
        match self.bits {
            false => DMAERREN_A::_0,
            true => DMAERREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAERREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAERREN_A::_1
    }
}
#[doc = "Field `DMAERREN` writer - DMAERR Interrupt Enable"]
pub type DMAERREN_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERREN_SPEC, DMAERREN_A, O>;
impl<'a, const O: u8> DMAERREN_W<'a, O> {
    #[doc = "Disables the DMAERR interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAERREN_A::_0)
    }
    #[doc = "Enables the DMAERR interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAERREN_A::_1)
    }
}
#[doc = "Field `BTSERREN` reader - BTSERR Interrupt Enable"]
pub type BTSERREN_R = crate::BitReader<BTSERREN_A>;
#[doc = "BTSERR Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BTSERREN_A {
    #[doc = "0: Disables the BTSERR interrupt."]
    _0 = 0,
    #[doc = "1: Enables the BTSERR interrupt."]
    _1 = 1,
}
impl From<BTSERREN_A> for bool {
    #[inline(always)]
    fn from(variant: BTSERREN_A) -> Self {
        variant as u8 != 0
    }
}
impl BTSERREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BTSERREN_A {
        match self.bits {
            false => BTSERREN_A::_0,
            true => BTSERREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BTSERREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BTSERREN_A::_1
    }
}
#[doc = "Field `BTSERREN` writer - BTSERR Interrupt Enable"]
pub type BTSERREN_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERREN_SPEC, BTSERREN_A, O>;
impl<'a, const O: u8> BTSERREN_W<'a, O> {
    #[doc = "Disables the BTSERR interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BTSERREN_A::_0)
    }
    #[doc = "Enables the BTSERR interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BTSERREN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - PIDERR Interrupt Enable"]
    #[inline(always)]
    pub fn piderren(&self) -> PIDERREN_R {
        PIDERREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CRC5/EOF Interrupt Enable"]
    #[inline(always)]
    pub fn crc5eofen(&self) -> CRC5EOFEN_R {
        CRC5EOFEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CRC16 Interrupt Enable"]
    #[inline(always)]
    pub fn crc16en(&self) -> CRC16EN_R {
        CRC16EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DFN8 Interrupt Enable"]
    #[inline(always)]
    pub fn dfn8en(&self) -> DFN8EN_R {
        DFN8EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BTOERR Interrupt Enable"]
    #[inline(always)]
    pub fn btoerren(&self) -> BTOERREN_R {
        BTOERREN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMAERR Interrupt Enable"]
    #[inline(always)]
    pub fn dmaerren(&self) -> DMAERREN_R {
        DMAERREN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - BTSERR Interrupt Enable"]
    #[inline(always)]
    pub fn btserren(&self) -> BTSERREN_R {
        BTSERREN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PIDERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn piderren(&mut self) -> PIDERREN_W<0> {
        PIDERREN_W::new(self)
    }
    #[doc = "Bit 1 - CRC5/EOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crc5eofen(&mut self) -> CRC5EOFEN_W<1> {
        CRC5EOFEN_W::new(self)
    }
    #[doc = "Bit 2 - CRC16 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn crc16en(&mut self) -> CRC16EN_W<2> {
        CRC16EN_W::new(self)
    }
    #[doc = "Bit 3 - DFN8 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfn8en(&mut self) -> DFN8EN_W<3> {
        DFN8EN_W::new(self)
    }
    #[doc = "Bit 4 - BTOERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn btoerren(&mut self) -> BTOERREN_W<4> {
        BTOERREN_W::new(self)
    }
    #[doc = "Bit 5 - DMAERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaerren(&mut self) -> DMAERREN_W<5> {
        DMAERREN_W::new(self)
    }
    #[doc = "Bit 7 - BTSERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn btserren(&mut self) -> BTSERREN_W<7> {
        BTSERREN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erren](index.html) module"]
pub struct ERREN_SPEC;
impl crate::RegisterSpec for ERREN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [erren::R](R) reader structure"]
impl crate::Readable for ERREN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [erren::W](W) writer structure"]
impl crate::Writable for ERREN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERREN to value 0"]
impl crate::Resettable for ERREN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
