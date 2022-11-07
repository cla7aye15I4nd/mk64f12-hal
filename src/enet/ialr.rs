#[doc = "Register `IALR` reader"]
pub struct R(crate::R<IALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IALR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IALR` writer"]
pub struct W(crate::W<IALR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IALR_SPEC>;
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
impl From<crate::W<IALR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IALR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IADDR2` reader - Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
pub type IADDR2_R = crate::FieldReader<u32, u32>;
#[doc = "Field `IADDR2` writer - Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
pub type IADDR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IALR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    #[inline(always)]
    pub fn iaddr2(&self) -> IADDR2_R {
        IADDR2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Contains the lower 32 bits of the 64-bit hash table used in the address recognition process for receive frames with a unicast address"]
    #[inline(always)]
    #[must_use]
    pub fn iaddr2(&mut self) -> IADDR2_W<0> {
        IADDR2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Descriptor Individual Lower Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ialr](index.html) module"]
pub struct IALR_SPEC;
impl crate::RegisterSpec for IALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ialr::R](R) reader structure"]
impl crate::Readable for IALR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ialr::W](W) writer structure"]
impl crate::Writable for IALR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IALR to value 0"]
impl crate::Resettable for IALR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
