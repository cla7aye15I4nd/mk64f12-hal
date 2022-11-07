#[doc = "Register `DSADDR` reader"]
pub struct R(crate::R<DSADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DSADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DSADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DSADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DSADDR` writer"]
pub struct W(crate::W<DSADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DSADDR_SPEC>;
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
impl From<crate::W<DSADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DSADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DSADDR` reader - DMA System Address"]
pub type DSADDR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DSADDR` writer - DMA System Address"]
pub type DSADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DSADDR_SPEC, u32, u32, 30, O>;
impl R {
    #[doc = "Bits 2:31 - DMA System Address"]
    #[inline(always)]
    pub fn dsaddr(&self) -> DSADDR_R {
        DSADDR_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - DMA System Address"]
    #[inline(always)]
    #[must_use]
    pub fn dsaddr(&mut self) -> DSADDR_W<2> {
        DSADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA System Address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dsaddr](index.html) module"]
pub struct DSADDR_SPEC;
impl crate::RegisterSpec for DSADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dsaddr::R](R) reader structure"]
impl crate::Readable for DSADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dsaddr::W](W) writer structure"]
impl crate::Writable for DSADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DSADDR to value 0"]
impl crate::Resettable for DSADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
