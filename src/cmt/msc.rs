#[doc = "Register `MSC` reader"]
pub struct R(crate::R<MSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSC` writer"]
pub struct W(crate::W<MSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSC_SPEC>;
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
impl From<crate::W<MSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCGEN` reader - Modulator and Carrier Generator Enable"]
pub type MCGEN_R = crate::BitReader<MCGEN_A>;
#[doc = "Modulator and Carrier Generator Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MCGEN_A {
    #[doc = "0: Modulator and carrier generator disabled"]
    _0 = 0,
    #[doc = "1: Modulator and carrier generator enabled"]
    _1 = 1,
}
impl From<MCGEN_A> for bool {
    #[inline(always)]
    fn from(variant: MCGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MCGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCGEN_A {
        match self.bits {
            false => MCGEN_A::_0,
            true => MCGEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MCGEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MCGEN_A::_1
    }
}
#[doc = "Field `MCGEN` writer - Modulator and Carrier Generator Enable"]
pub type MCGEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, MSC_SPEC, MCGEN_A, O>;
impl<'a, const O: u8> MCGEN_W<'a, O> {
    #[doc = "Modulator and carrier generator disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MCGEN_A::_0)
    }
    #[doc = "Modulator and carrier generator enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MCGEN_A::_1)
    }
}
#[doc = "Field `EOCIE` reader - End of Cycle Interrupt Enable"]
pub type EOCIE_R = crate::BitReader<EOCIE_A>;
#[doc = "End of Cycle Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCIE_A {
    #[doc = "0: CPU interrupt is disabled."]
    _0 = 0,
    #[doc = "1: CPU interrupt is enabled."]
    _1 = 1,
}
impl From<EOCIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCIE_A {
        match self.bits {
            false => EOCIE_A::_0,
            true => EOCIE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOCIE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOCIE_A::_1
    }
}
#[doc = "Field `EOCIE` writer - End of Cycle Interrupt Enable"]
pub type EOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u8, MSC_SPEC, EOCIE_A, O>;
impl<'a, const O: u8> EOCIE_W<'a, O> {
    #[doc = "CPU interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOCIE_A::_0)
    }
    #[doc = "CPU interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOCIE_A::_1)
    }
}
#[doc = "Field `FSK` reader - FSK Mode Select"]
pub type FSK_R = crate::BitReader<FSK_A>;
#[doc = "FSK Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSK_A {
    #[doc = "0: The CMT operates in Time or Baseband mode."]
    _0 = 0,
    #[doc = "1: The CMT operates in FSK mode."]
    _1 = 1,
}
impl From<FSK_A> for bool {
    #[inline(always)]
    fn from(variant: FSK_A) -> Self {
        variant as u8 != 0
    }
}
impl FSK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSK_A {
        match self.bits {
            false => FSK_A::_0,
            true => FSK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FSK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FSK_A::_1
    }
}
#[doc = "Field `FSK` writer - FSK Mode Select"]
pub type FSK_W<'a, const O: u8> = crate::BitWriter<'a, u8, MSC_SPEC, FSK_A, O>;
impl<'a, const O: u8> FSK_W<'a, O> {
    #[doc = "The CMT operates in Time or Baseband mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSK_A::_0)
    }
    #[doc = "The CMT operates in FSK mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSK_A::_1)
    }
}
#[doc = "Field `BASE` reader - Baseband Enable"]
pub type BASE_R = crate::BitReader<BASE_A>;
#[doc = "Baseband Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BASE_A {
    #[doc = "0: Baseband mode is disabled."]
    _0 = 0,
    #[doc = "1: Baseband mode is enabled."]
    _1 = 1,
}
impl From<BASE_A> for bool {
    #[inline(always)]
    fn from(variant: BASE_A) -> Self {
        variant as u8 != 0
    }
}
impl BASE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BASE_A {
        match self.bits {
            false => BASE_A::_0,
            true => BASE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BASE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BASE_A::_1
    }
}
#[doc = "Field `BASE` writer - Baseband Enable"]
pub type BASE_W<'a, const O: u8> = crate::BitWriter<'a, u8, MSC_SPEC, BASE_A, O>;
impl<'a, const O: u8> BASE_W<'a, O> {
    #[doc = "Baseband mode is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BASE_A::_0)
    }
    #[doc = "Baseband mode is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BASE_A::_1)
    }
}
#[doc = "Field `EXSPC` reader - Extended Space Enable"]
pub type EXSPC_R = crate::BitReader<EXSPC_A>;
#[doc = "Extended Space Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXSPC_A {
    #[doc = "0: Extended space is disabled."]
    _0 = 0,
    #[doc = "1: Extended space is enabled."]
    _1 = 1,
}
impl From<EXSPC_A> for bool {
    #[inline(always)]
    fn from(variant: EXSPC_A) -> Self {
        variant as u8 != 0
    }
}
impl EXSPC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXSPC_A {
        match self.bits {
            false => EXSPC_A::_0,
            true => EXSPC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXSPC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXSPC_A::_1
    }
}
#[doc = "Field `EXSPC` writer - Extended Space Enable"]
pub type EXSPC_W<'a, const O: u8> = crate::BitWriter<'a, u8, MSC_SPEC, EXSPC_A, O>;
impl<'a, const O: u8> EXSPC_W<'a, O> {
    #[doc = "Extended space is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXSPC_A::_0)
    }
    #[doc = "Extended space is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXSPC_A::_1)
    }
}
#[doc = "Field `CMTDIV` reader - CMT Clock Divide Prescaler"]
pub type CMTDIV_R = crate::FieldReader<u8, CMTDIV_A>;
#[doc = "CMT Clock Divide Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMTDIV_A {
    #[doc = "0: IF * 1"]
    _00 = 0,
    #[doc = "1: IF * 2"]
    _01 = 1,
    #[doc = "2: IF * 4"]
    _10 = 2,
    #[doc = "3: IF * 8"]
    _11 = 3,
}
impl From<CMTDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CMTDIV_A) -> Self {
        variant as _
    }
}
impl CMTDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMTDIV_A {
        match self.bits {
            0 => CMTDIV_A::_00,
            1 => CMTDIV_A::_01,
            2 => CMTDIV_A::_10,
            3 => CMTDIV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CMTDIV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CMTDIV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CMTDIV_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == CMTDIV_A::_11
    }
}
#[doc = "Field `CMTDIV` writer - CMT Clock Divide Prescaler"]
pub type CMTDIV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, MSC_SPEC, u8, CMTDIV_A, 2, O>;
impl<'a, const O: u8> CMTDIV_W<'a, O> {
    #[doc = "IF * 1"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(CMTDIV_A::_00)
    }
    #[doc = "IF * 2"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(CMTDIV_A::_01)
    }
    #[doc = "IF * 4"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(CMTDIV_A::_10)
    }
    #[doc = "IF * 8"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(CMTDIV_A::_11)
    }
}
#[doc = "Field `EOCF` reader - End Of Cycle Status Flag"]
pub type EOCF_R = crate::BitReader<EOCF_A>;
#[doc = "End Of Cycle Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCF_A {
    #[doc = "0: End of modulation cycle has not occurred since the flag last cleared."]
    _0 = 0,
    #[doc = "1: End of modulator cycle has occurred."]
    _1 = 1,
}
impl From<EOCF_A> for bool {
    #[inline(always)]
    fn from(variant: EOCF_A) -> Self {
        variant as u8 != 0
    }
}
impl EOCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCF_A {
        match self.bits {
            false => EOCF_A::_0,
            true => EOCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOCF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Modulator and Carrier Generator Enable"]
    #[inline(always)]
    pub fn mcgen(&self) -> MCGEN_R {
        MCGEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of Cycle Interrupt Enable"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FSK Mode Select"]
    #[inline(always)]
    pub fn fsk(&self) -> FSK_R {
        FSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Baseband Enable"]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Extended Space Enable"]
    #[inline(always)]
    pub fn exspc(&self) -> EXSPC_R {
        EXSPC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - CMT Clock Divide Prescaler"]
    #[inline(always)]
    pub fn cmtdiv(&self) -> CMTDIV_R {
        CMTDIV_R::new((self.bits >> 5) & 3)
    }
    #[doc = "Bit 7 - End Of Cycle Status Flag"]
    #[inline(always)]
    pub fn eocf(&self) -> EOCF_R {
        EOCF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Modulator and Carrier Generator Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcgen(&mut self) -> MCGEN_W<0> {
        MCGEN_W::new(self)
    }
    #[doc = "Bit 1 - End of Cycle Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<1> {
        EOCIE_W::new(self)
    }
    #[doc = "Bit 2 - FSK Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn fsk(&mut self) -> FSK_W<2> {
        FSK_W::new(self)
    }
    #[doc = "Bit 3 - Baseband Enable"]
    #[inline(always)]
    #[must_use]
    pub fn base(&mut self) -> BASE_W<3> {
        BASE_W::new(self)
    }
    #[doc = "Bit 4 - Extended Space Enable"]
    #[inline(always)]
    #[must_use]
    pub fn exspc(&mut self) -> EXSPC_W<4> {
        EXSPC_W::new(self)
    }
    #[doc = "Bits 5:6 - CMT Clock Divide Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn cmtdiv(&mut self) -> CMTDIV_W<5> {
        CMTDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMT Modulator Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msc](index.html) module"]
pub struct MSC_SPEC;
impl crate::RegisterSpec for MSC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [msc::R](R) reader structure"]
impl crate::Readable for MSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msc::W](W) writer structure"]
impl crate::Writable for MSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSC to value 0"]
impl crate::Resettable for MSC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
