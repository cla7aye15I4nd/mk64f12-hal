#[doc = "Register `CMPH` reader"]
pub struct R(crate::R<CMPH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPH` writer"]
pub struct W(crate::W<CMPH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPH_SPEC>;
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
impl From<crate::W<CMPH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPAREH` reader - To prevent runaway code from changing this field, software should write to this field after a CPU reset even if the (default) maximum service time is required"]
pub type COMPAREH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPAREH` writer - To prevent runaway code from changing this field, software should write to this field after a CPU reset even if the (default) maximum service time is required"]
pub type COMPAREH_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CMPH_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - To prevent runaway code from changing this field, software should write to this field after a CPU reset even if the (default) maximum service time is required"]
    #[inline(always)]
    pub fn compareh(&self) -> COMPAREH_R {
        COMPAREH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - To prevent runaway code from changing this field, software should write to this field after a CPU reset even if the (default) maximum service time is required"]
    #[inline(always)]
    #[must_use]
    pub fn compareh(&mut self) -> COMPAREH_W<0> {
        COMPAREH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compare High Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmph](index.html) module"]
pub struct CMPH_SPEC;
impl crate::RegisterSpec for CMPH_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cmph::R](R) reader structure"]
impl crate::Readable for CMPH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmph::W](W) writer structure"]
impl crate::Writable for CMPH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPH to value 0xff"]
impl crate::Resettable for CMPH_SPEC {
    const RESET_VALUE: Self::Ux = 0xff;
}
