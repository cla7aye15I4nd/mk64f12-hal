#[doc = "Register `ECR` reader"]
pub struct R(crate::R<ECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECR` writer"]
pub struct W(crate::W<ECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECR_SPEC>;
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
impl From<crate::W<ECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXERRCNT` reader - Transmit Error Counter"]
pub type TXERRCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXERRCNT` writer - Transmit Error Counter"]
pub type TXERRCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECR_SPEC, u8, u8, 8, O>;
#[doc = "Field `RXERRCNT` reader - Receive Error Counter"]
pub type RXERRCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXERRCNT` writer - Receive Error Counter"]
pub type RXERRCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ECR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    pub fn txerrcnt(&self) -> TXERRCNT_R {
        TXERRCNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Receive Error Counter"]
    #[inline(always)]
    pub fn rxerrcnt(&self) -> RXERRCNT_R {
        RXERRCNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Error Counter"]
    #[inline(always)]
    #[must_use]
    pub fn txerrcnt(&mut self) -> TXERRCNT_W<0> {
        TXERRCNT_W::new(self)
    }
    #[doc = "Bits 8:15 - Receive Error Counter"]
    #[inline(always)]
    #[must_use]
    pub fn rxerrcnt(&mut self) -> RXERRCNT_W<8> {
        RXERRCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Counter\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecr](index.html) module"]
pub struct ECR_SPEC;
impl crate::RegisterSpec for ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecr::R](R) reader structure"]
impl crate::Readable for ECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecr::W](W) writer structure"]
impl crate::Writable for ECR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for ECR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
