#[doc = "Register `WF7816` reader"]
pub struct R(crate::R<WF7816_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WF7816_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WF7816_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WF7816_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WF7816` writer"]
pub struct W(crate::W<WF7816_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WF7816_SPEC>;
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
impl From<crate::W<WF7816_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WF7816_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GTFD` reader - FD Multiplier"]
pub type GTFD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GTFD` writer - FD Multiplier"]
pub type GTFD_W<'a, const O: u8> = crate::FieldWriter<'a, u8, WF7816_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - FD Multiplier"]
    #[inline(always)]
    pub fn gtfd(&self) -> GTFD_R {
        GTFD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - FD Multiplier"]
    #[inline(always)]
    #[must_use]
    pub fn gtfd(&mut self) -> GTFD_W<0> {
        GTFD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART 7816 Wait FD Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wf7816](index.html) module"]
pub struct WF7816_SPEC;
impl crate::RegisterSpec for WF7816_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [wf7816::R](R) reader structure"]
impl crate::Readable for WF7816_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wf7816::W](W) writer structure"]
impl crate::Writable for WF7816_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WF7816 to value 0x01"]
impl crate::Resettable for WF7816_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
