#[doc = "Register `GALR` reader"]
pub struct R(crate::R<GALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GALR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GALR` writer"]
pub struct W(crate::W<GALR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GALR_SPEC>;
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
impl From<crate::W<GALR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GALR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GADDR2` reader - Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
pub type GADDR2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GADDR2` writer - Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
pub type GADDR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GALR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    #[inline(always)]
    pub fn gaddr2(&self) -> GADDR2_R {
        GADDR2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a multicast address"]
    #[inline(always)]
    #[must_use]
    pub fn gaddr2(&mut self) -> GADDR2_W<0> {
        GADDR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Descriptor Group Lower Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [galr](index.html) module"]
pub struct GALR_SPEC;
impl crate::RegisterSpec for GALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [galr::R](R) reader structure"]
impl crate::Readable for GALR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [galr::W](W) writer structure"]
impl crate::Writable for GALR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GALR to value 0"]
impl crate::Resettable for GALR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
