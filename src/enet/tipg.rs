#[doc = "Register `TIPG` reader"]
pub struct R(crate::R<TIPG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIPG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIPG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIPG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIPG` writer"]
pub struct W(crate::W<TIPG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIPG_SPEC>;
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
impl From<crate::W<TIPG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIPG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPG` reader - Transmit Inter-Packet Gap"]
pub type IPG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IPG` writer - Transmit Inter-Packet Gap"]
pub type IPG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIPG_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:4 - Transmit Inter-Packet Gap"]
    #[inline(always)]
    pub fn ipg(&self) -> IPG_R {
        IPG_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Transmit Inter-Packet Gap"]
    #[inline(always)]
    #[must_use]
    pub fn ipg(&mut self) -> IPG_W<0> {
        IPG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Inter-Packet Gap\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tipg](index.html) module"]
pub struct TIPG_SPEC;
impl crate::RegisterSpec for TIPG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tipg::R](R) reader structure"]
impl crate::Readable for TIPG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tipg::W](W) writer structure"]
impl crate::Writable for TIPG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIPG to value 0x0c"]
impl crate::Resettable for TIPG_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c;
}
