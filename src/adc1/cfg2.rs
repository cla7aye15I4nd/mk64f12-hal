#[doc = "Register `CFG2` reader"]
pub struct R(crate::R<CFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG2` writer"]
pub struct W(crate::W<CFG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG2_SPEC>;
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
impl From<crate::W<CFG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADLSTS` reader - Long Sample Time Select"]
pub type ADLSTS_R = crate::FieldReader<u8, ADLSTS_A>;
#[doc = "Long Sample Time Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADLSTS_A {
    #[doc = "0: Default longest sample time; 20 extra ADCK cycles; 24 ADCK cycles total."]
    _00 = 0,
    #[doc = "1: 12 extra ADCK cycles; 16 ADCK cycles total sample time."]
    _01 = 1,
    #[doc = "2: 6 extra ADCK cycles; 10 ADCK cycles total sample time."]
    _10 = 2,
    #[doc = "3: 2 extra ADCK cycles; 6 ADCK cycles total sample time."]
    _11 = 3,
}
impl From<ADLSTS_A> for u8 {
    #[inline(always)]
    fn from(variant: ADLSTS_A) -> Self {
        variant as _
    }
}
impl ADLSTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADLSTS_A {
        match self.bits {
            0 => ADLSTS_A::_00,
            1 => ADLSTS_A::_01,
            2 => ADLSTS_A::_10,
            3 => ADLSTS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ADLSTS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ADLSTS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ADLSTS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ADLSTS_A::_11
    }
}
#[doc = "Field `ADLSTS` writer - Long Sample Time Select"]
pub type ADLSTS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG2_SPEC, u8, ADLSTS_A, 2, O>;
impl<'a, const O: u8> ADLSTS_W<'a, O> {
    #[doc = "Default longest sample time; 20 extra ADCK cycles; 24 ADCK cycles total."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADLSTS_A::_00)
    }
    #[doc = "12 extra ADCK cycles; 16 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADLSTS_A::_01)
    }
    #[doc = "6 extra ADCK cycles; 10 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADLSTS_A::_10)
    }
    #[doc = "2 extra ADCK cycles; 6 ADCK cycles total sample time."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADLSTS_A::_11)
    }
}
#[doc = "Field `ADHSC` reader - High-Speed Configuration"]
pub type ADHSC_R = crate::BitReader<ADHSC_A>;
#[doc = "High-Speed Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADHSC_A {
    #[doc = "0: Normal conversion sequence selected."]
    _0 = 0,
    #[doc = "1: High-speed conversion sequence selected with 2 additional ADCK cycles to total conversion time."]
    _1 = 1,
}
impl From<ADHSC_A> for bool {
    #[inline(always)]
    fn from(variant: ADHSC_A) -> Self {
        variant as u8 != 0
    }
}
impl ADHSC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADHSC_A {
        match self.bits {
            false => ADHSC_A::_0,
            true => ADHSC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADHSC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADHSC_A::_1
    }
}
#[doc = "Field `ADHSC` writer - High-Speed Configuration"]
pub type ADHSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, ADHSC_A, O>;
impl<'a, const O: u8> ADHSC_W<'a, O> {
    #[doc = "Normal conversion sequence selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADHSC_A::_0)
    }
    #[doc = "High-speed conversion sequence selected with 2 additional ADCK cycles to total conversion time."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADHSC_A::_1)
    }
}
#[doc = "Field `ADACKEN` reader - Asynchronous Clock Output Enable"]
pub type ADACKEN_R = crate::BitReader<ADACKEN_A>;
#[doc = "Asynchronous Clock Output Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADACKEN_A {
    #[doc = "0: Asynchronous clock output disabled; Asynchronous clock is enabled only if selected by ADICLK and a conversion is active."]
    _0 = 0,
    #[doc = "1: Asynchronous clock and clock output is enabled regardless of the state of the ADC."]
    _1 = 1,
}
impl From<ADACKEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADACKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADACKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADACKEN_A {
        match self.bits {
            false => ADACKEN_A::_0,
            true => ADACKEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADACKEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADACKEN_A::_1
    }
}
#[doc = "Field `ADACKEN` writer - Asynchronous Clock Output Enable"]
pub type ADACKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, ADACKEN_A, O>;
impl<'a, const O: u8> ADACKEN_W<'a, O> {
    #[doc = "Asynchronous clock output disabled; Asynchronous clock is enabled only if selected by ADICLK and a conversion is active."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADACKEN_A::_0)
    }
    #[doc = "Asynchronous clock and clock output is enabled regardless of the state of the ADC."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADACKEN_A::_1)
    }
}
#[doc = "Field `MUXSEL` reader - ADC Mux Select"]
pub type MUXSEL_R = crate::BitReader<MUXSEL_A>;
#[doc = "ADC Mux Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUXSEL_A {
    #[doc = "0: ADxxa channels are selected."]
    _0 = 0,
    #[doc = "1: ADxxb channels are selected."]
    _1 = 1,
}
impl From<MUXSEL_A> for bool {
    #[inline(always)]
    fn from(variant: MUXSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl MUXSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUXSEL_A {
        match self.bits {
            false => MUXSEL_A::_0,
            true => MUXSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MUXSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MUXSEL_A::_1
    }
}
#[doc = "Field `MUXSEL` writer - ADC Mux Select"]
pub type MUXSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG2_SPEC, MUXSEL_A, O>;
impl<'a, const O: u8> MUXSEL_W<'a, O> {
    #[doc = "ADxxa channels are selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MUXSEL_A::_0)
    }
    #[doc = "ADxxb channels are selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MUXSEL_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Long Sample Time Select"]
    #[inline(always)]
    pub fn adlsts(&self) -> ADLSTS_R {
        ADLSTS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - High-Speed Configuration"]
    #[inline(always)]
    pub fn adhsc(&self) -> ADHSC_R {
        ADHSC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Asynchronous Clock Output Enable"]
    #[inline(always)]
    pub fn adacken(&self) -> ADACKEN_R {
        ADACKEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC Mux Select"]
    #[inline(always)]
    pub fn muxsel(&self) -> MUXSEL_R {
        MUXSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Long Sample Time Select"]
    #[inline(always)]
    #[must_use]
    pub fn adlsts(&mut self) -> ADLSTS_W<0> {
        ADLSTS_W::new(self)
    }
    #[doc = "Bit 2 - High-Speed Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn adhsc(&mut self) -> ADHSC_W<2> {
        ADHSC_W::new(self)
    }
    #[doc = "Bit 3 - Asynchronous Clock Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adacken(&mut self) -> ADACKEN_W<3> {
        ADACKEN_W::new(self)
    }
    #[doc = "Bit 4 - ADC Mux Select"]
    #[inline(always)]
    #[must_use]
    pub fn muxsel(&mut self) -> MUXSEL_W<4> {
        MUXSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg2](index.html) module"]
pub struct CFG2_SPEC;
impl crate::RegisterSpec for CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg2::R](R) reader structure"]
impl crate::Readable for CFG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg2::W](W) writer structure"]
impl crate::Writable for CFG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
