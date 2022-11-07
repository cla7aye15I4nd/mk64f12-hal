#[doc = "Register `ETBCC` reader"]
pub struct R(crate::R<ETBCC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETBCC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETBCC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETBCC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETBCC` writer"]
pub struct W(crate::W<ETBCC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETBCC_SPEC>;
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
impl From<crate::W<ETBCC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETBCC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTEN` reader - Counter Enable"]
pub type CNTEN_R = crate::BitReader<CNTEN_A>;
#[doc = "Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNTEN_A {
    #[doc = "0: ETB counter disabled"]
    _0 = 0,
    #[doc = "1: ETB counter enabled"]
    _1 = 1,
}
impl From<CNTEN_A> for bool {
    #[inline(always)]
    fn from(variant: CNTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CNTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTEN_A {
        match self.bits {
            false => CNTEN_A::_0,
            true => CNTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CNTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CNTEN_A::_1
    }
}
#[doc = "Field `CNTEN` writer - Counter Enable"]
pub type CNTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETBCC_SPEC, CNTEN_A, O>;
impl<'a, const O: u8> CNTEN_W<'a, O> {
    #[doc = "ETB counter disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CNTEN_A::_0)
    }
    #[doc = "ETB counter enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CNTEN_A::_1)
    }
}
#[doc = "Field `RSPT` reader - Response Type"]
pub type RSPT_R = crate::FieldReader<u8, RSPT_A>;
#[doc = "Response Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RSPT_A {
    #[doc = "0: No response when the ETB count expires"]
    _00 = 0,
    #[doc = "1: Generate a normal interrupt when the ETB count expires"]
    _01 = 1,
    #[doc = "2: Generate an NMI when the ETB count expires"]
    _10 = 2,
    #[doc = "3: Generate a debug halt when the ETB count expires"]
    _11 = 3,
}
impl From<RSPT_A> for u8 {
    #[inline(always)]
    fn from(variant: RSPT_A) -> Self {
        variant as _
    }
}
impl RSPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSPT_A {
        match self.bits {
            0 => RSPT_A::_00,
            1 => RSPT_A::_01,
            2 => RSPT_A::_10,
            3 => RSPT_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RSPT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RSPT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RSPT_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RSPT_A::_11
    }
}
#[doc = "Field `RSPT` writer - Response Type"]
pub type RSPT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, ETBCC_SPEC, u8, RSPT_A, 2, O>;
impl<'a, const O: u8> RSPT_W<'a, O> {
    #[doc = "No response when the ETB count expires"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RSPT_A::_00)
    }
    #[doc = "Generate a normal interrupt when the ETB count expires"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RSPT_A::_01)
    }
    #[doc = "Generate an NMI when the ETB count expires"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RSPT_A::_10)
    }
    #[doc = "Generate a debug halt when the ETB count expires"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RSPT_A::_11)
    }
}
#[doc = "Field `RLRQ` reader - Reload Request"]
pub type RLRQ_R = crate::BitReader<RLRQ_A>;
#[doc = "Reload Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RLRQ_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Clears pending debug halt, NMI, or IRQ interrupt requests"]
    _1 = 1,
}
impl From<RLRQ_A> for bool {
    #[inline(always)]
    fn from(variant: RLRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl RLRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RLRQ_A {
        match self.bits {
            false => RLRQ_A::_0,
            true => RLRQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RLRQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RLRQ_A::_1
    }
}
#[doc = "Field `RLRQ` writer - Reload Request"]
pub type RLRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETBCC_SPEC, RLRQ_A, O>;
impl<'a, const O: u8> RLRQ_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RLRQ_A::_0)
    }
    #[doc = "Clears pending debug halt, NMI, or IRQ interrupt requests"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RLRQ_A::_1)
    }
}
#[doc = "Field `ETDIS` reader - ETM-To-TPIU Disable"]
pub type ETDIS_R = crate::BitReader<ETDIS_A>;
#[doc = "ETM-To-TPIU Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETDIS_A {
    #[doc = "0: ETM-to-TPIU trace path enabled"]
    _0 = 0,
    #[doc = "1: ETM-to-TPIU trace path disabled"]
    _1 = 1,
}
impl From<ETDIS_A> for bool {
    #[inline(always)]
    fn from(variant: ETDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl ETDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETDIS_A {
        match self.bits {
            false => ETDIS_A::_0,
            true => ETDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ETDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ETDIS_A::_1
    }
}
#[doc = "Field `ETDIS` writer - ETM-To-TPIU Disable"]
pub type ETDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETBCC_SPEC, ETDIS_A, O>;
impl<'a, const O: u8> ETDIS_W<'a, O> {
    #[doc = "ETM-to-TPIU trace path enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ETDIS_A::_0)
    }
    #[doc = "ETM-to-TPIU trace path disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ETDIS_A::_1)
    }
}
#[doc = "Field `ITDIS` reader - ITM-To-TPIU Disable"]
pub type ITDIS_R = crate::BitReader<ITDIS_A>;
#[doc = "ITM-To-TPIU Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITDIS_A {
    #[doc = "0: ITM-to-TPIU trace path enabled"]
    _0 = 0,
    #[doc = "1: ITM-to-TPIU trace path disabled"]
    _1 = 1,
}
impl From<ITDIS_A> for bool {
    #[inline(always)]
    fn from(variant: ITDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl ITDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ITDIS_A {
        match self.bits {
            false => ITDIS_A::_0,
            true => ITDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ITDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ITDIS_A::_1
    }
}
#[doc = "Field `ITDIS` writer - ITM-To-TPIU Disable"]
pub type ITDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, ETBCC_SPEC, ITDIS_A, O>;
impl<'a, const O: u8> ITDIS_W<'a, O> {
    #[doc = "ITM-to-TPIU trace path enabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ITDIS_A::_0)
    }
    #[doc = "ITM-to-TPIU trace path disabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ITDIS_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Counter Enable"]
    #[inline(always)]
    pub fn cnten(&self) -> CNTEN_R {
        CNTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Response Type"]
    #[inline(always)]
    pub fn rspt(&self) -> RSPT_R {
        RSPT_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Reload Request"]
    #[inline(always)]
    pub fn rlrq(&self) -> RLRQ_R {
        RLRQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ETM-To-TPIU Disable"]
    #[inline(always)]
    pub fn etdis(&self) -> ETDIS_R {
        ETDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ITM-To-TPIU Disable"]
    #[inline(always)]
    pub fn itdis(&self) -> ITDIS_R {
        ITDIS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cnten(&mut self) -> CNTEN_W<0> {
        CNTEN_W::new(self)
    }
    #[doc = "Bits 1:2 - Response Type"]
    #[inline(always)]
    #[must_use]
    pub fn rspt(&mut self) -> RSPT_W<1> {
        RSPT_W::new(self)
    }
    #[doc = "Bit 3 - Reload Request"]
    #[inline(always)]
    #[must_use]
    pub fn rlrq(&mut self) -> RLRQ_W<3> {
        RLRQ_W::new(self)
    }
    #[doc = "Bit 4 - ETM-To-TPIU Disable"]
    #[inline(always)]
    #[must_use]
    pub fn etdis(&mut self) -> ETDIS_W<4> {
        ETDIS_W::new(self)
    }
    #[doc = "Bit 5 - ITM-To-TPIU Disable"]
    #[inline(always)]
    #[must_use]
    pub fn itdis(&mut self) -> ITDIS_W<5> {
        ITDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETB Counter Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etbcc](index.html) module"]
pub struct ETBCC_SPEC;
impl crate::RegisterSpec for ETBCC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etbcc::R](R) reader structure"]
impl crate::Readable for ETBCC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etbcc::W](W) writer structure"]
impl crate::Writable for ETBCC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETBCC to value 0"]
impl crate::Resettable for ETBCC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
