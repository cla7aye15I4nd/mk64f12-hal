#[doc = "Register `CSAR%s` reader"]
pub struct R(crate::R<CSAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSAR%s` writer"]
pub struct W(crate::W<CSAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSAR_SPEC>;
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
impl From<crate::W<CSAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BA` reader - Base Address"]
pub type BA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BA` writer - Base Address"]
pub type BA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSAR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 16:31 - Base Address"]
    #[inline(always)]
    pub fn ba(&self) -> BA_R {
        BA_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn ba(&mut self) -> BA_W<16> {
        BA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Chip Select Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csar](index.html) module"]
pub struct CSAR_SPEC;
impl crate::RegisterSpec for CSAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csar::R](R) reader structure"]
impl crate::Readable for CSAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csar::W](W) writer structure"]
impl crate::Writable for CSAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSAR%s to value 0"]
impl crate::Resettable for CSAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
