#[doc = "Register `FSTAT` reader"]
pub struct R(crate::R<FSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FSTAT` writer"]
pub struct W(crate::W<FSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FSTAT_SPEC>;
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
impl From<crate::W<FSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MGSTAT0` reader - Memory Controller Command Completion Status Flag"]
pub type MGSTAT0_R = crate::BitReader<bool>;
#[doc = "Field `FPVIOL` reader - Flash Protection Violation Flag"]
pub type FPVIOL_R = crate::BitReader<FPVIOL_A>;
#[doc = "Flash Protection Violation Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPVIOL_A {
    #[doc = "0: No protection violation detected"]
    _0 = 0,
    #[doc = "1: Protection violation detected"]
    _1 = 1,
}
impl From<FPVIOL_A> for bool {
    #[inline(always)]
    fn from(variant: FPVIOL_A) -> Self {
        variant as u8 != 0
    }
}
impl FPVIOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPVIOL_A {
        match self.bits {
            false => FPVIOL_A::_0,
            true => FPVIOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FPVIOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FPVIOL_A::_1
    }
}
#[doc = "Field `FPVIOL` writer - Flash Protection Violation Flag"]
pub type FPVIOL_W<'a, const O: u8> = crate::BitWriter<'a, u8, FSTAT_SPEC, FPVIOL_A, O>;
impl<'a, const O: u8> FPVIOL_W<'a, O> {
    #[doc = "No protection violation detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FPVIOL_A::_0)
    }
    #[doc = "Protection violation detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FPVIOL_A::_1)
    }
}
#[doc = "Field `ACCERR` reader - Flash Access Error Flag"]
pub type ACCERR_R = crate::BitReader<ACCERR_A>;
#[doc = "Flash Access Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACCERR_A {
    #[doc = "0: No access error detected"]
    _0 = 0,
    #[doc = "1: Access error detected"]
    _1 = 1,
}
impl From<ACCERR_A> for bool {
    #[inline(always)]
    fn from(variant: ACCERR_A) -> Self {
        variant as u8 != 0
    }
}
impl ACCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACCERR_A {
        match self.bits {
            false => ACCERR_A::_0,
            true => ACCERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACCERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACCERR_A::_1
    }
}
#[doc = "Field `ACCERR` writer - Flash Access Error Flag"]
pub type ACCERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, FSTAT_SPEC, ACCERR_A, O>;
impl<'a, const O: u8> ACCERR_W<'a, O> {
    #[doc = "No access error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACCERR_A::_0)
    }
    #[doc = "Access error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACCERR_A::_1)
    }
}
#[doc = "Field `RDCOLERR` reader - FTFE Read Collision Error Flag"]
pub type RDCOLERR_R = crate::BitReader<RDCOLERR_A>;
#[doc = "FTFE Read Collision Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDCOLERR_A {
    #[doc = "0: No collision error detected"]
    _0 = 0,
    #[doc = "1: Collision error detected"]
    _1 = 1,
}
impl From<RDCOLERR_A> for bool {
    #[inline(always)]
    fn from(variant: RDCOLERR_A) -> Self {
        variant as u8 != 0
    }
}
impl RDCOLERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDCOLERR_A {
        match self.bits {
            false => RDCOLERR_A::_0,
            true => RDCOLERR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RDCOLERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RDCOLERR_A::_1
    }
}
#[doc = "Field `RDCOLERR` writer - FTFE Read Collision Error Flag"]
pub type RDCOLERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, FSTAT_SPEC, RDCOLERR_A, O>;
impl<'a, const O: u8> RDCOLERR_W<'a, O> {
    #[doc = "No collision error detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RDCOLERR_A::_0)
    }
    #[doc = "Collision error detected"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RDCOLERR_A::_1)
    }
}
#[doc = "Field `CCIF` reader - Command Complete Interrupt Flag"]
pub type CCIF_R = crate::BitReader<CCIF_A>;
#[doc = "Command Complete Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCIF_A {
    #[doc = "0: FTFE command or EEPROM file system operation in progress"]
    _0 = 0,
    #[doc = "1: FTFE command or EEPROM file system operation has completed"]
    _1 = 1,
}
impl From<CCIF_A> for bool {
    #[inline(always)]
    fn from(variant: CCIF_A) -> Self {
        variant as u8 != 0
    }
}
impl CCIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCIF_A {
        match self.bits {
            false => CCIF_A::_0,
            true => CCIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CCIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CCIF_A::_1
    }
}
#[doc = "Field `CCIF` writer - Command Complete Interrupt Flag"]
pub type CCIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, FSTAT_SPEC, CCIF_A, O>;
impl<'a, const O: u8> CCIF_W<'a, O> {
    #[doc = "FTFE command or EEPROM file system operation in progress"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CCIF_A::_0)
    }
    #[doc = "FTFE command or EEPROM file system operation has completed"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CCIF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Memory Controller Command Completion Status Flag"]
    #[inline(always)]
    pub fn mgstat0(&self) -> MGSTAT0_R {
        MGSTAT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Flash Protection Violation Flag"]
    #[inline(always)]
    pub fn fpviol(&self) -> FPVIOL_R {
        FPVIOL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash Access Error Flag"]
    #[inline(always)]
    pub fn accerr(&self) -> ACCERR_R {
        ACCERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FTFE Read Collision Error Flag"]
    #[inline(always)]
    pub fn rdcolerr(&self) -> RDCOLERR_R {
        RDCOLERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ccif(&self) -> CCIF_R {
        CCIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Flash Protection Violation Flag"]
    #[inline(always)]
    #[must_use]
    pub fn fpviol(&mut self) -> FPVIOL_W<4> {
        FPVIOL_W::new(self)
    }
    #[doc = "Bit 5 - Flash Access Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn accerr(&mut self) -> ACCERR_W<5> {
        ACCERR_W::new(self)
    }
    #[doc = "Bit 6 - FTFE Read Collision Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rdcolerr(&mut self) -> RDCOLERR_W<6> {
        RDCOLERR_W::new(self)
    }
    #[doc = "Bit 7 - Command Complete Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ccif(&mut self) -> CCIF_W<7> {
        CCIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fstat](index.html) module"]
pub struct FSTAT_SPEC;
impl crate::RegisterSpec for FSTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [fstat::R](R) reader structure"]
impl crate::Readable for FSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fstat::W](W) writer structure"]
impl crate::Writable for FSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FSTAT to value 0"]
impl crate::Resettable for FSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
