#[doc = "Register `CLKDIV2` reader"]
pub struct R(crate::R<CLKDIV2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKDIV2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKDIV2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKDIV2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKDIV2` writer"]
pub struct W(crate::W<CLKDIV2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKDIV2_SPEC>;
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
impl From<crate::W<CLKDIV2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKDIV2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBFRAC` reader - USB clock divider fraction"]
pub type USBFRAC_R = crate::BitReader<bool>;
#[doc = "Field `USBFRAC` writer - USB clock divider fraction"]
pub type USBFRAC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLKDIV2_SPEC, bool, O>;
#[doc = "Field `USBDIV` reader - USB clock divider divisor"]
pub type USBDIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBDIV` writer - USB clock divider divisor"]
pub type USBDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CLKDIV2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bit 0 - USB clock divider fraction"]
    #[inline(always)]
    pub fn usbfrac(&self) -> USBFRAC_R {
        USBFRAC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - USB clock divider divisor"]
    #[inline(always)]
    pub fn usbdiv(&self) -> USBDIV_R {
        USBDIV_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USB clock divider fraction"]
    #[inline(always)]
    #[must_use]
    pub fn usbfrac(&mut self) -> USBFRAC_W<0> {
        USBFRAC_W::new(self)
    }
    #[doc = "Bits 1:3 - USB clock divider divisor"]
    #[inline(always)]
    #[must_use]
    pub fn usbdiv(&mut self) -> USBDIV_W<1> {
        USBDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Divider Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv2](index.html) module"]
pub struct CLKDIV2_SPEC;
impl crate::RegisterSpec for CLKDIV2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkdiv2::R](R) reader structure"]
impl crate::Readable for CLKDIV2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkdiv2::W](W) writer structure"]
impl crate::Writable for CLKDIV2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKDIV2 to value 0"]
impl crate::Resettable for CLKDIV2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
