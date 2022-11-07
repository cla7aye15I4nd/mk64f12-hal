#[doc = "Register `RDAR` reader"]
pub struct R(crate::R<RDAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RDAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RDAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RDAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RDAR` writer"]
pub struct W(crate::W<RDAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RDAR_SPEC>;
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
impl From<crate::W<RDAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RDAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RDAR` reader - Receive Descriptor Active"]
pub type RDAR_R = crate::BitReader<bool>;
#[doc = "Field `RDAR` writer - Receive Descriptor Active"]
pub type RDAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, RDAR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 24 - Receive Descriptor Active"]
    #[inline(always)]
    pub fn rdar(&self) -> RDAR_R {
        RDAR_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Receive Descriptor Active"]
    #[inline(always)]
    #[must_use]
    pub fn rdar(&mut self) -> RDAR_W<24> {
        RDAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Descriptor Active Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdar](index.html) module"]
pub struct RDAR_SPEC;
impl crate::RegisterSpec for RDAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rdar::R](R) reader structure"]
impl crate::Readable for RDAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rdar::W](W) writer structure"]
impl crate::Writable for RDAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RDAR to value 0"]
impl crate::Resettable for RDAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
