#[doc = "Register `MA1` reader"]
pub struct R(crate::R<MA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MA1` writer"]
pub struct W(crate::W<MA1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MA1_SPEC>;
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
impl From<crate::W<MA1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MA1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MA` reader - Match Address"]
pub type MA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MA` writer - Match Address"]
pub type MA_W<'a, const O: u8> = crate::FieldWriter<'a, u8, MA1_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Match Address"]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Match Address"]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<0> {
        MA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Match Address Registers 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ma1](index.html) module"]
pub struct MA1_SPEC;
impl crate::RegisterSpec for MA1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ma1::R](R) reader structure"]
impl crate::Readable for MA1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ma1::W](W) writer structure"]
impl crate::Writable for MA1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MA1 to value 0"]
impl crate::Resettable for MA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
