#[doc = "Register `CFG1` reader"]
pub struct R(crate::R<CFG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG1` writer"]
pub struct W(crate::W<CFG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG1_SPEC>;
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
impl From<crate::W<CFG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADICLK` reader - Input Clock Select"]
pub type ADICLK_R = crate::FieldReader<u8, ADICLK_A>;
#[doc = "Input Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADICLK_A {
    #[doc = "0: Bus clock"]
    _00 = 0,
    #[doc = "1: Alternate clock 2 (ALTCLK2)"]
    _01 = 1,
    #[doc = "2: Alternate clock (ALTCLK)"]
    _10 = 2,
    #[doc = "3: Asynchronous clock (ADACK)"]
    _11 = 3,
}
impl From<ADICLK_A> for u8 {
    #[inline(always)]
    fn from(variant: ADICLK_A) -> Self {
        variant as _
    }
}
impl ADICLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADICLK_A {
        match self.bits {
            0 => ADICLK_A::_00,
            1 => ADICLK_A::_01,
            2 => ADICLK_A::_10,
            3 => ADICLK_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ADICLK_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ADICLK_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ADICLK_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ADICLK_A::_11
    }
}
#[doc = "Field `ADICLK` writer - Input Clock Select"]
pub type ADICLK_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, ADICLK_A, 2, O>;
impl<'a, const O: u8> ADICLK_W<'a, O> {
    #[doc = "Bus clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADICLK_A::_00)
    }
    #[doc = "Alternate clock 2 (ALTCLK2)"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADICLK_A::_01)
    }
    #[doc = "Alternate clock (ALTCLK)"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADICLK_A::_10)
    }
    #[doc = "Asynchronous clock (ADACK)"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADICLK_A::_11)
    }
}
#[doc = "Field `MODE` reader - Conversion mode selection"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Conversion mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: When DIFF=0:It is single-ended 8-bit conversion; when DIFF=1, it is differential 9-bit conversion with 2's complement output."]
    _00 = 0,
    #[doc = "1: When DIFF=0:It is single-ended 12-bit conversion ; when DIFF=1, it is differential 13-bit conversion with 2's complement output."]
    _01 = 1,
    #[doc = "2: When DIFF=0:It is single-ended 10-bit conversion. ; when DIFF=1, it is differential 11-bit conversion with 2's complement output"]
    _10 = 2,
    #[doc = "3: When DIFF=0:It is single-ended 16-bit conversion..; when DIFF=1, it is differential 16-bit conversion with 2's complement output"]
    _11 = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::_00,
            1 => MODE_A::_01,
            2 => MODE_A::_10,
            3 => MODE_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MODE_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MODE_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MODE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MODE_A::_11
    }
}
#[doc = "Field `MODE` writer - Conversion mode selection"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "When DIFF=0:It is single-ended 8-bit conversion; when DIFF=1, it is differential 9-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(MODE_A::_00)
    }
    #[doc = "When DIFF=0:It is single-ended 12-bit conversion ; when DIFF=1, it is differential 13-bit conversion with 2's complement output."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(MODE_A::_01)
    }
    #[doc = "When DIFF=0:It is single-ended 10-bit conversion. ; when DIFF=1, it is differential 11-bit conversion with 2's complement output"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(MODE_A::_10)
    }
    #[doc = "When DIFF=0:It is single-ended 16-bit conversion..; when DIFF=1, it is differential 16-bit conversion with 2's complement output"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(MODE_A::_11)
    }
}
#[doc = "Field `ADLSMP` reader - Sample Time Configuration"]
pub type ADLSMP_R = crate::BitReader<ADLSMP_A>;
#[doc = "Sample Time Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADLSMP_A {
    #[doc = "0: Short sample time."]
    _0 = 0,
    #[doc = "1: Long sample time."]
    _1 = 1,
}
impl From<ADLSMP_A> for bool {
    #[inline(always)]
    fn from(variant: ADLSMP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADLSMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADLSMP_A {
        match self.bits {
            false => ADLSMP_A::_0,
            true => ADLSMP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADLSMP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADLSMP_A::_1
    }
}
#[doc = "Field `ADLSMP` writer - Sample Time Configuration"]
pub type ADLSMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, ADLSMP_A, O>;
impl<'a, const O: u8> ADLSMP_W<'a, O> {
    #[doc = "Short sample time."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADLSMP_A::_0)
    }
    #[doc = "Long sample time."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADLSMP_A::_1)
    }
}
#[doc = "Field `ADIV` reader - Clock Divide Select"]
pub type ADIV_R = crate::FieldReader<u8, ADIV_A>;
#[doc = "Clock Divide Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADIV_A {
    #[doc = "0: The divide ratio is 1 and the clock rate is input clock."]
    _00 = 0,
    #[doc = "1: The divide ratio is 2 and the clock rate is (input clock)/2."]
    _01 = 1,
    #[doc = "2: The divide ratio is 4 and the clock rate is (input clock)/4."]
    _10 = 2,
    #[doc = "3: The divide ratio is 8 and the clock rate is (input clock)/8."]
    _11 = 3,
}
impl From<ADIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADIV_A) -> Self {
        variant as _
    }
}
impl ADIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADIV_A {
        match self.bits {
            0 => ADIV_A::_00,
            1 => ADIV_A::_01,
            2 => ADIV_A::_10,
            3 => ADIV_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ADIV_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ADIV_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ADIV_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ADIV_A::_11
    }
}
#[doc = "Field `ADIV` writer - Clock Divide Select"]
pub type ADIV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CFG1_SPEC, u8, ADIV_A, 2, O>;
impl<'a, const O: u8> ADIV_W<'a, O> {
    #[doc = "The divide ratio is 1 and the clock rate is input clock."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ADIV_A::_00)
    }
    #[doc = "The divide ratio is 2 and the clock rate is (input clock)/2."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ADIV_A::_01)
    }
    #[doc = "The divide ratio is 4 and the clock rate is (input clock)/4."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ADIV_A::_10)
    }
    #[doc = "The divide ratio is 8 and the clock rate is (input clock)/8."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ADIV_A::_11)
    }
}
#[doc = "Field `ADLPC` reader - Low-Power Configuration"]
pub type ADLPC_R = crate::BitReader<ADLPC_A>;
#[doc = "Low-Power Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADLPC_A {
    #[doc = "0: Normal power configuration."]
    _0 = 0,
    #[doc = "1: Low-power configuration. The power is reduced at the expense of maximum clock speed."]
    _1 = 1,
}
impl From<ADLPC_A> for bool {
    #[inline(always)]
    fn from(variant: ADLPC_A) -> Self {
        variant as u8 != 0
    }
}
impl ADLPC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADLPC_A {
        match self.bits {
            false => ADLPC_A::_0,
            true => ADLPC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADLPC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADLPC_A::_1
    }
}
#[doc = "Field `ADLPC` writer - Low-Power Configuration"]
pub type ADLPC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CFG1_SPEC, ADLPC_A, O>;
impl<'a, const O: u8> ADLPC_W<'a, O> {
    #[doc = "Normal power configuration."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADLPC_A::_0)
    }
    #[doc = "Low-power configuration. The power is reduced at the expense of maximum clock speed."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADLPC_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Input Clock Select"]
    #[inline(always)]
    pub fn adiclk(&self) -> ADICLK_R {
        ADICLK_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Conversion mode selection"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Sample Time Configuration"]
    #[inline(always)]
    pub fn adlsmp(&self) -> ADLSMP_R {
        ADLSMP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Clock Divide Select"]
    #[inline(always)]
    pub fn adiv(&self) -> ADIV_R {
        ADIV_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Low-Power Configuration"]
    #[inline(always)]
    pub fn adlpc(&self) -> ADLPC_R {
        ADLPC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Input Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn adiclk(&mut self) -> ADICLK_W<0> {
        ADICLK_W::new(self)
    }
    #[doc = "Bits 2:3 - Conversion mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<2> {
        MODE_W::new(self)
    }
    #[doc = "Bit 4 - Sample Time Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn adlsmp(&mut self) -> ADLSMP_W<4> {
        ADLSMP_W::new(self)
    }
    #[doc = "Bits 5:6 - Clock Divide Select"]
    #[inline(always)]
    #[must_use]
    pub fn adiv(&mut self) -> ADIV_W<5> {
        ADIV_W::new(self)
    }
    #[doc = "Bit 7 - Low-Power Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn adlpc(&mut self) -> ADLPC_W<7> {
        ADLPC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Configuration Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfg1](index.html) module"]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg1::R](R) reader structure"]
impl crate::Readable for CFG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg1::W](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFG1 to value 0"]
impl crate::Resettable for CFG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
