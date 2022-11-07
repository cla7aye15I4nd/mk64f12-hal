#[doc = "Register `RGD%s_WORD1` reader"]
pub struct R(crate::R<RGD_WORD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RGD_WORD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RGD_WORD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RGD_WORD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RGD%s_WORD1` writer"]
pub struct W(crate::W<RGD_WORD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RGD_WORD1_SPEC>;
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
impl From<crate::W<RGD_WORD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RGD_WORD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENDADDR` reader - End Address"]
pub type ENDADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ENDADDR` writer - End Address"]
pub type ENDADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGD_WORD1_SPEC, u32, u32, 27, O>;
impl R {
    #[doc = "Bits 5:31 - End Address"]
    #[inline(always)]
    pub fn endaddr(&self) -> ENDADDR_R {
        ENDADDR_R::new((self.bits >> 5) & 0x07ff_ffff)
    }
}
impl W {
    #[doc = "Bits 5:31 - End Address"]
    #[inline(always)]
    #[must_use]
    pub fn endaddr(&mut self) -> ENDADDR_W<5> {
        ENDADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Region Descriptor n, Word 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgd_word1](index.html) module"]
pub struct RGD_WORD1_SPEC;
impl crate::RegisterSpec for RGD_WORD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rgd_word1::R](R) reader structure"]
impl crate::Readable for RGD_WORD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rgd_word1::W](W) writer structure"]
impl crate::Writable for RGD_WORD1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RGD%s_WORD1 to value 0xffff_ffff"]
impl crate::Resettable for RGD_WORD1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
