#[doc = "Register `ER` writer"]
pub struct W(crate::W<ER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ER_SPEC>;
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
impl From<crate::W<ER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXT_ENT` writer - External Entropy"]
pub type EXT_ENT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ER_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - External Entropy"]
    #[inline(always)]
    #[must_use]
    pub fn ext_ent(&mut self) -> EXT_ENT_W<0> {
        EXT_ENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RNGA Entropy Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [er](index.html) module"]
pub struct ER_SPEC;
impl crate::RegisterSpec for ER_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [er::W](W) writer structure"]
impl crate::Writable for ER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ER to value 0"]
impl crate::Resettable for ER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
