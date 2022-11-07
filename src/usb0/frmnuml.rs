#[doc = "Register `FRMNUML` reader"]
pub struct R(crate::R<FRMNUML_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRMNUML_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRMNUML_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRMNUML_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRMNUML` writer"]
pub struct W(crate::W<FRMNUML_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRMNUML_SPEC>;
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
impl From<crate::W<FRMNUML_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRMNUML_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRM` reader - This 8-bit field and the 3-bit field in the Frame Number Register High are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory"]
pub type FRM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRM` writer - This 8-bit field and the 3-bit field in the Frame Number Register High are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory"]
pub type FRM_W<'a, const O: u8> = crate::FieldWriter<'a, u8, FRMNUML_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - This 8-bit field and the 3-bit field in the Frame Number Register High are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory"]
    #[inline(always)]
    pub fn frm(&self) -> FRM_R {
        FRM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - This 8-bit field and the 3-bit field in the Frame Number Register High are used to compute the address where the current Buffer Descriptor Table (BDT) resides in system memory"]
    #[inline(always)]
    #[must_use]
    pub fn frm(&mut self) -> FRM_W<0> {
        FRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Frame Number register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frmnuml](index.html) module"]
pub struct FRMNUML_SPEC;
impl crate::RegisterSpec for FRMNUML_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [frmnuml::R](R) reader structure"]
impl crate::Readable for FRMNUML_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frmnuml::W](W) writer structure"]
impl crate::Writable for FRMNUML_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRMNUML to value 0"]
impl crate::Resettable for FRMNUML_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
