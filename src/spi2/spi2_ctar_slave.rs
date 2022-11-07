#[doc = "Register `CTAR_SLAVE` reader"]
pub struct R(crate::R<SPI2_CTAR_SLAVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI2_CTAR_SLAVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI2_CTAR_SLAVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI2_CTAR_SLAVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTAR_SLAVE` writer"]
pub struct W(crate::W<SPI2_CTAR_SLAVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI2_CTAR_SLAVE_SPEC>;
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
impl From<crate::W<SPI2_CTAR_SLAVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI2_CTAR_SLAVE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPHA` reader - Clock Phase"]
pub type CPHA_R = crate::BitReader<CPHA_A>;
#[doc = "Clock Phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA_A {
    #[doc = "0: Data is captured on the leading edge of SCK and changed on the following edge."]
    _0 = 0,
    #[doc = "1: Data is changed on the leading edge of SCK and captured on the following edge."]
    _1 = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
impl CPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::_0,
            true => CPHA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPHA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPHA_A::_1
    }
}
#[doc = "Field `CPHA` writer - Clock Phase"]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2_CTAR_SLAVE_SPEC, CPHA_A, O>;
impl<'a, const O: u8> CPHA_W<'a, O> {
    #[doc = "Data is captured on the leading edge of SCK and changed on the following edge."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPHA_A::_0)
    }
    #[doc = "Data is changed on the leading edge of SCK and captured on the following edge."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPHA_A::_1)
    }
}
#[doc = "Field `CPOL` reader - Clock Polarity"]
pub type CPOL_R = crate::BitReader<CPOL_A>;
#[doc = "Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL_A {
    #[doc = "0: The inactive state value of SCK is low."]
    _0 = 0,
    #[doc = "1: The inactive state value of SCK is high."]
    _1 = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::_0,
            true => CPOL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPOL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPOL_A::_1
    }
}
#[doc = "Field `CPOL` writer - Clock Polarity"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI2_CTAR_SLAVE_SPEC, CPOL_A, O>;
impl<'a, const O: u8> CPOL_W<'a, O> {
    #[doc = "The inactive state value of SCK is low."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPOL_A::_0)
    }
    #[doc = "The inactive state value of SCK is high."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPOL_A::_1)
    }
}
#[doc = "Field `FMSZ` reader - Frame Size"]
pub type FMSZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FMSZ` writer - Frame Size"]
pub type FMSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI2_CTAR_SLAVE_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 25 - Clock Phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Clock Polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - Frame Size"]
    #[inline(always)]
    pub fn fmsz(&self) -> FMSZ_R {
        FMSZ_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 25 - Clock Phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<25> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 26 - Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<26> {
        CPOL_W::new(self)
    }
    #[doc = "Bits 27:31 - Frame Size"]
    #[inline(always)]
    #[must_use]
    pub fn fmsz(&mut self) -> FMSZ_W<27> {
        FMSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock and Transfer Attributes Register (In Slave Mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi2_ctar_slave](index.html) module"]
pub struct SPI2_CTAR_SLAVE_SPEC;
impl crate::RegisterSpec for SPI2_CTAR_SLAVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi2_ctar_slave::R](R) reader structure"]
impl crate::Readable for SPI2_CTAR_SLAVE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi2_ctar_slave::W](W) writer structure"]
impl crate::Writable for SPI2_CTAR_SLAVE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTAR_SLAVE to value 0x7800_0000"]
impl crate::Resettable for SPI2_CTAR_SLAVE_SPEC {
    const RESET_VALUE: Self::Ux = 0x7800_0000;
}
