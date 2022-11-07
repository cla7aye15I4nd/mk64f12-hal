#[doc = "Register `MDR` reader"]
pub struct R(crate::R<MDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MDR` writer"]
pub struct W(crate::W<MDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MDR_SPEC>;
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
impl From<crate::W<MDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVIDE` reader - MCLK Divide"]
pub type DIVIDE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DIVIDE` writer - MCLK Divide"]
pub type DIVIDE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDR_SPEC, u16, u16, 12, O>;
#[doc = "Field `FRACT` reader - MCLK Fraction"]
pub type FRACT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRACT` writer - MCLK Fraction"]
pub type FRACT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MDR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:11 - MCLK Divide"]
    #[inline(always)]
    pub fn divide(&self) -> DIVIDE_R {
        DIVIDE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:19 - MCLK Fraction"]
    #[inline(always)]
    pub fn fract(&self) -> FRACT_R {
        FRACT_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - MCLK Divide"]
    #[inline(always)]
    #[must_use]
    pub fn divide(&mut self) -> DIVIDE_W<0> {
        DIVIDE_W::new(self)
    }
    #[doc = "Bits 12:19 - MCLK Fraction"]
    #[inline(always)]
    #[must_use]
    pub fn fract(&mut self) -> FRACT_W<12> {
        FRACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI MCLK Divide Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mdr](index.html) module"]
pub struct MDR_SPEC;
impl crate::RegisterSpec for MDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mdr::R](R) reader structure"]
impl crate::Readable for MDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mdr::W](W) writer structure"]
impl crate::Writable for MDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MDR to value 0"]
impl crate::Resettable for MDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
