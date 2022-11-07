#[doc = "Register `WML` reader"]
pub struct R(crate::R<WML_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WML_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WML_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WML_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WML` writer"]
pub struct W(crate::W<WML_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WML_SPEC>;
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
impl From<crate::W<WML_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WML_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDWML` reader - Read Watermark Level"]
pub type RDWML_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RDWML` writer - Read Watermark Level"]
pub type RDWML_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WML_SPEC, u8, u8, 8, O>;
#[doc = "Field `WRWML` reader - Write Watermark Level"]
pub type WRWML_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WRWML` writer - Write Watermark Level"]
pub type WRWML_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WML_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Read Watermark Level"]
    #[inline(always)]
    pub fn rdwml(&self) -> RDWML_R {
        RDWML_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Write Watermark Level"]
    #[inline(always)]
    pub fn wrwml(&self) -> WRWML_R {
        WRWML_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read Watermark Level"]
    #[inline(always)]
    #[must_use]
    pub fn rdwml(&mut self) -> RDWML_W<0> {
        RDWML_W::new(self)
    }
    #[doc = "Bits 16:23 - Write Watermark Level"]
    #[inline(always)]
    #[must_use]
    pub fn wrwml(&mut self) -> WRWML_W<16> {
        WRWML_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watermark Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wml](index.html) module"]
pub struct WML_SPEC;
impl crate::RegisterSpec for WML_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wml::R](R) reader structure"]
impl crate::Readable for WML_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wml::W](W) writer structure"]
impl crate::Writable for WML_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WML to value 0x0010_0010"]
impl crate::Resettable for WML_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0010;
}
