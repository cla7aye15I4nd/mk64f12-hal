#[doc = "Register `TDAR` reader"]
pub struct R(crate::R<TDAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TDAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TDAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TDAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TDAR` writer"]
pub struct W(crate::W<TDAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TDAR_SPEC>;
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
impl From<crate::W<TDAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TDAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDAR` reader - Transmit Descriptor Active"]
pub type TDAR_R = crate::BitReader<bool>;
#[doc = "Field `TDAR` writer - Transmit Descriptor Active"]
pub type TDAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, TDAR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 24 - Transmit Descriptor Active"]
    #[inline(always)]
    pub fn tdar(&self) -> TDAR_R {
        TDAR_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Transmit Descriptor Active"]
    #[inline(always)]
    #[must_use]
    pub fn tdar(&mut self) -> TDAR_W<24> {
        TDAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Descriptor Active Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tdar](index.html) module"]
pub struct TDAR_SPEC;
impl crate::RegisterSpec for TDAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tdar::R](R) reader structure"]
impl crate::Readable for TDAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tdar::W](W) writer structure"]
impl crate::Writable for TDAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TDAR to value 0"]
impl crate::Resettable for TDAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
