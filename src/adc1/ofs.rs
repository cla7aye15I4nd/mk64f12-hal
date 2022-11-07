#[doc = "Register `OFS` reader"]
pub struct R(crate::R<OFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFS` writer"]
pub struct W(crate::W<OFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFS_SPEC>;
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
impl From<crate::W<OFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFS` reader - Offset Error Correction Value"]
pub type OFS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OFS` writer - Offset Error Correction Value"]
pub type OFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Offset Error Correction Value"]
    #[inline(always)]
    pub fn ofs(&self) -> OFS_R {
        OFS_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Offset Error Correction Value"]
    #[inline(always)]
    #[must_use]
    pub fn ofs(&mut self) -> OFS_W<0> {
        OFS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Offset Correction Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofs](index.html) module"]
pub struct OFS_SPEC;
impl crate::RegisterSpec for OFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofs::R](R) reader structure"]
impl crate::Readable for OFS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ofs::W](W) writer structure"]
impl crate::Writable for OFS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OFS to value 0x04"]
impl crate::Resettable for OFS_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
