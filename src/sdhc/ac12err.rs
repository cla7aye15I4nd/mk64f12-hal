#[doc = "Register `AC12ERR` reader"]
pub struct R(crate::R<AC12ERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AC12ERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AC12ERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AC12ERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `AC12NE` reader - Auto CMD12 Not Executed"]
pub type AC12NE_R = crate::BitReader<AC12NE_A>;
#[doc = "Auto CMD12 Not Executed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AC12NE_A {
    #[doc = "0: Executed."]
    _0 = 0,
    #[doc = "1: Not executed."]
    _1 = 1,
}
impl From<AC12NE_A> for bool {
    #[inline(always)]
    fn from(variant: AC12NE_A) -> Self {
        variant as u8 != 0
    }
}
impl AC12NE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12NE_A {
        match self.bits {
            false => AC12NE_A::_0,
            true => AC12NE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AC12NE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AC12NE_A::_1
    }
}
#[doc = "Field `AC12TOE` reader - Auto CMD12 Timeout Error"]
pub type AC12TOE_R = crate::BitReader<AC12TOE_A>;
#[doc = "Auto CMD12 Timeout Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AC12TOE_A {
    #[doc = "0: No error."]
    _0 = 0,
    #[doc = "1: Time out."]
    _1 = 1,
}
impl From<AC12TOE_A> for bool {
    #[inline(always)]
    fn from(variant: AC12TOE_A) -> Self {
        variant as u8 != 0
    }
}
impl AC12TOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12TOE_A {
        match self.bits {
            false => AC12TOE_A::_0,
            true => AC12TOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AC12TOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AC12TOE_A::_1
    }
}
#[doc = "Field `AC12EBE` reader - Auto CMD12 End Bit Error"]
pub type AC12EBE_R = crate::BitReader<AC12EBE_A>;
#[doc = "Auto CMD12 End Bit Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AC12EBE_A {
    #[doc = "0: No error."]
    _0 = 0,
    #[doc = "1: End bit error generated."]
    _1 = 1,
}
impl From<AC12EBE_A> for bool {
    #[inline(always)]
    fn from(variant: AC12EBE_A) -> Self {
        variant as u8 != 0
    }
}
impl AC12EBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12EBE_A {
        match self.bits {
            false => AC12EBE_A::_0,
            true => AC12EBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AC12EBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AC12EBE_A::_1
    }
}
#[doc = "Field `AC12CE` reader - Auto CMD12 CRC Error"]
pub type AC12CE_R = crate::BitReader<AC12CE_A>;
#[doc = "Auto CMD12 CRC Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AC12CE_A {
    #[doc = "0: No CRC error."]
    _0 = 0,
    #[doc = "1: CRC error met in Auto CMD12 response."]
    _1 = 1,
}
impl From<AC12CE_A> for bool {
    #[inline(always)]
    fn from(variant: AC12CE_A) -> Self {
        variant as u8 != 0
    }
}
impl AC12CE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12CE_A {
        match self.bits {
            false => AC12CE_A::_0,
            true => AC12CE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AC12CE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AC12CE_A::_1
    }
}
#[doc = "Field `AC12IE` reader - Auto CMD12 Index Error"]
pub type AC12IE_R = crate::BitReader<AC12IE_A>;
#[doc = "Auto CMD12 Index Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AC12IE_A {
    #[doc = "0: No error."]
    _0 = 0,
    #[doc = "1: Error, the CMD index in response is not CMD12."]
    _1 = 1,
}
impl From<AC12IE_A> for bool {
    #[inline(always)]
    fn from(variant: AC12IE_A) -> Self {
        variant as u8 != 0
    }
}
impl AC12IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12IE_A {
        match self.bits {
            false => AC12IE_A::_0,
            true => AC12IE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AC12IE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AC12IE_A::_1
    }
}
#[doc = "Field `CNIBAC12E` reader - Command Not Issued By Auto CMD12 Error"]
pub type CNIBAC12E_R = crate::BitReader<CNIBAC12E_A>;
#[doc = "Command Not Issued By Auto CMD12 Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNIBAC12E_A {
    #[doc = "0: No error."]
    _0 = 0,
    #[doc = "1: Not issued."]
    _1 = 1,
}
impl From<CNIBAC12E_A> for bool {
    #[inline(always)]
    fn from(variant: CNIBAC12E_A) -> Self {
        variant as u8 != 0
    }
}
impl CNIBAC12E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNIBAC12E_A {
        match self.bits {
            false => CNIBAC12E_A::_0,
            true => CNIBAC12E_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CNIBAC12E_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CNIBAC12E_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Auto CMD12 Not Executed"]
    #[inline(always)]
    pub fn ac12ne(&self) -> AC12NE_R {
        AC12NE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto CMD12 Timeout Error"]
    #[inline(always)]
    pub fn ac12toe(&self) -> AC12TOE_R {
        AC12TOE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto CMD12 End Bit Error"]
    #[inline(always)]
    pub fn ac12ebe(&self) -> AC12EBE_R {
        AC12EBE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auto CMD12 CRC Error"]
    #[inline(always)]
    pub fn ac12ce(&self) -> AC12CE_R {
        AC12CE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Auto CMD12 Index Error"]
    #[inline(always)]
    pub fn ac12ie(&self) -> AC12IE_R {
        AC12IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Command Not Issued By Auto CMD12 Error"]
    #[inline(always)]
    pub fn cnibac12e(&self) -> CNIBAC12E_R {
        CNIBAC12E_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Auto CMD12 Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ac12err](index.html) module"]
pub struct AC12ERR_SPEC;
impl crate::RegisterSpec for AC12ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ac12err::R](R) reader structure"]
impl crate::Readable for AC12ERR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AC12ERR to value 0"]
impl crate::Resettable for AC12ERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
