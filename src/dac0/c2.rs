#[doc = "Register `C2` reader"]
pub struct R(crate::R<C2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C2` writer"]
pub struct W(crate::W<C2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2_SPEC>;
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
impl From<crate::W<C2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DACBFUP` reader - DAC Buffer Upper Limit"]
pub type DACBFUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DACBFUP` writer - DAC Buffer Upper Limit"]
pub type DACBFUP_W<'a, const O: u8> = crate::FieldWriter<'a, u8, C2_SPEC, u8, u8, 4, O>;
#[doc = "Field `DACBFRP` reader - DAC Buffer Read Pointer"]
pub type DACBFRP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DACBFRP` writer - DAC Buffer Read Pointer"]
pub type DACBFRP_W<'a, const O: u8> = crate::FieldWriter<'a, u8, C2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - DAC Buffer Upper Limit"]
    #[inline(always)]
    pub fn dacbfup(&self) -> DACBFUP_R {
        DACBFUP_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:7 - DAC Buffer Read Pointer"]
    #[inline(always)]
    pub fn dacbfrp(&self) -> DACBFRP_R {
        DACBFRP_R::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - DAC Buffer Upper Limit"]
    #[inline(always)]
    #[must_use]
    pub fn dacbfup(&mut self) -> DACBFUP_W<0> {
        DACBFUP_W::new(self)
    }
    #[doc = "Bits 4:7 - DAC Buffer Read Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn dacbfrp(&mut self) -> DACBFRP_W<4> {
        DACBFRP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DAC Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2](index.html) module"]
pub struct C2_SPEC;
impl crate::RegisterSpec for C2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [c2::R](R) reader structure"]
impl crate::Readable for C2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c2::W](W) writer structure"]
impl crate::Writable for C2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets C2 to value 0x0f"]
impl crate::Resettable for C2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f;
}
