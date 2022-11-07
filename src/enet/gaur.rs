#[doc = "Register `GAUR` reader"]
pub struct R(crate::R<GAUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GAUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GAUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GAUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GAUR` writer"]
pub struct W(crate::W<GAUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GAUR_SPEC>;
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
impl From<crate::W<GAUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GAUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GADDR1` reader - Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
pub type GADDR1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GADDR1` writer - Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
pub type GADDR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GAUR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    #[inline(always)]
    pub fn gaddr1(&self) -> GADDR1_R {
        GADDR1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Contains the upper 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    #[inline(always)]
    #[must_use]
    pub fn gaddr1(&mut self) -> GADDR1_W<0> {
        GADDR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Descriptor Group Upper Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gaur](index.html) module"]
pub struct GAUR_SPEC;
impl crate::RegisterSpec for GAUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gaur::R](R) reader structure"]
impl crate::Readable for GAUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gaur::W](W) writer structure"]
impl crate::Writable for GAUR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GAUR to value 0"]
impl crate::Resettable for GAUR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
