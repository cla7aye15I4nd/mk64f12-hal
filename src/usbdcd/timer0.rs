#[doc = "Register `TIMER0` reader"]
pub struct R(crate::R<TIMER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER0` writer"]
pub struct W(crate::W<TIMER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER0_SPEC>;
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
impl From<crate::W<TIMER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TUNITCON` reader - Unit Connection Timer Elapse (in ms)"]
pub type TUNITCON_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TSEQ_INIT` reader - Sequence Initiation Time"]
pub type TSEQ_INIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TSEQ_INIT` writer - Sequence Initiation Time"]
pub type TSEQ_INIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMER0_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:11 - Unit Connection Timer Elapse (in ms)"]
    #[inline(always)]
    pub fn tunitcon(&self) -> TUNITCON_R {
        TUNITCON_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:25 - Sequence Initiation Time"]
    #[inline(always)]
    pub fn tseq_init(&self) -> TSEQ_INIT_R {
        TSEQ_INIT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:25 - Sequence Initiation Time"]
    #[inline(always)]
    #[must_use]
    pub fn tseq_init(&mut self) -> TSEQ_INIT_W<16> {
        TSEQ_INIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMER0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0](index.html) module"]
pub struct TIMER0_SPEC;
impl crate::RegisterSpec for TIMER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer0::R](R) reader structure"]
impl crate::Readable for TIMER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer0::W](W) writer structure"]
impl crate::Writable for TIMER0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER0 to value 0x0010_0000"]
impl crate::Resettable for TIMER0_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0000;
}
