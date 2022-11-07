#[doc = "Register `FCFG2` reader"]
pub struct R(crate::R<FCFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAXADDR1` reader - Max address block 1"]
pub type MAXADDR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PFLSH` reader - Program flash only"]
pub type PFLSH_R = crate::BitReader<PFLSH_A>;
#[doc = "Program flash only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFLSH_A {
    #[doc = "0: Device supports FlexNVM"]
    _0 = 0,
    #[doc = "1: Program Flash only, device does not support FlexNVM"]
    _1 = 1,
}
impl From<PFLSH_A> for bool {
    #[inline(always)]
    fn from(variant: PFLSH_A) -> Self {
        variant as u8 != 0
    }
}
impl PFLSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFLSH_A {
        match self.bits {
            false => PFLSH_A::_0,
            true => PFLSH_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFLSH_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFLSH_A::_1
    }
}
#[doc = "Field `MAXADDR0` reader - Max address block 0"]
pub type MAXADDR0_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 16:22 - Max address block 1"]
    #[inline(always)]
    pub fn maxaddr1(&self) -> MAXADDR1_R {
        MAXADDR1_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Program flash only"]
    #[inline(always)]
    pub fn pflsh(&self) -> PFLSH_R {
        PFLSH_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Max address block 0"]
    #[inline(always)]
    pub fn maxaddr0(&self) -> MAXADDR0_R {
        MAXADDR0_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "Flash Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg2](index.html) module"]
pub struct FCFG2_SPEC;
impl crate::RegisterSpec for FCFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcfg2::R](R) reader structure"]
impl crate::Readable for FCFG2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FCFG2 to value 0x7f7f_0000"]
impl crate::Resettable for FCFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f7f_0000;
}
