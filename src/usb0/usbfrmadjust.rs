#[doc = "Register `USBFRMADJUST` reader"]
pub struct R(crate::R<USBFRMADJUST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBFRMADJUST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBFRMADJUST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBFRMADJUST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBFRMADJUST` writer"]
pub struct W(crate::W<USBFRMADJUST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBFRMADJUST_SPEC>;
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
impl From<crate::W<USBFRMADJUST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBFRMADJUST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADJ` reader - Frame Adjustment"]
pub type ADJ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADJ` writer - Frame Adjustment"]
pub type ADJ_W<'a, const O: u8> = crate::FieldWriter<'a, u8, USBFRMADJUST_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Frame Adjustment"]
    #[inline(always)]
    pub fn adj(&self) -> ADJ_R {
        ADJ_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn adj(&mut self) -> ADJ_W<0> {
        ADJ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame Adjust Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbfrmadjust](index.html) module"]
pub struct USBFRMADJUST_SPEC;
impl crate::RegisterSpec for USBFRMADJUST_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [usbfrmadjust::R](R) reader structure"]
impl crate::Readable for USBFRMADJUST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbfrmadjust::W](W) writer structure"]
impl crate::Writable for USBFRMADJUST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBFRMADJUST to value 0"]
impl crate::Resettable for USBFRMADJUST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
