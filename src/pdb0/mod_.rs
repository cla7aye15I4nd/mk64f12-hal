#[doc = "Register `MOD` reader"]
pub struct R(crate::R<MOD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MOD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MOD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MOD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MOD` writer"]
pub struct W(crate::W<MOD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MOD_SPEC>;
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
impl From<crate::W<MOD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MOD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOD` reader - PDB Modulus"]
pub type MOD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MOD` writer - PDB Modulus"]
pub type MOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MOD_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - PDB Modulus"]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PDB Modulus"]
    #[inline(always)]
    #[must_use]
    pub fn mod_(&mut self) -> MOD_W<0> {
        MOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Modulus register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mod_](index.html) module"]
pub struct MOD_SPEC;
impl crate::RegisterSpec for MOD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mod_::R](R) reader structure"]
impl crate::Readable for MOD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mod_::W](W) writer structure"]
impl crate::Writable for MOD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MOD to value 0xffff"]
impl crate::Resettable for MOD_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
