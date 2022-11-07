#[doc = "Register `TIMER1` reader"]
pub struct R(crate::R<TIMER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER1` writer"]
pub struct W(crate::W<TIMER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1_SPEC>;
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
impl From<crate::W<TIMER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TVDPSRC_ON` reader - Time Period Comparator Enabled"]
pub type TVDPSRC_ON_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TVDPSRC_ON` writer - Time Period Comparator Enabled"]
pub type TVDPSRC_ON_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMER1_SPEC, u16, u16, 10, O>;
#[doc = "Field `TDCD_DBNC` reader - Time Period to Debounce D+ Signal"]
pub type TDCD_DBNC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TDCD_DBNC` writer - Time Period to Debounce D+ Signal"]
pub type TDCD_DBNC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMER1_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Time Period Comparator Enabled"]
    #[inline(always)]
    pub fn tvdpsrc_on(&self) -> TVDPSRC_ON_R {
        TVDPSRC_ON_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Time Period to Debounce D+ Signal"]
    #[inline(always)]
    pub fn tdcd_dbnc(&self) -> TDCD_DBNC_R {
        TDCD_DBNC_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Time Period Comparator Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn tvdpsrc_on(&mut self) -> TVDPSRC_ON_W<0> {
        TVDPSRC_ON_W::new(self)
    }
    #[doc = "Bits 16:25 - Time Period to Debounce D+ Signal"]
    #[inline(always)]
    #[must_use]
    pub fn tdcd_dbnc(&mut self) -> TDCD_DBNC_W<16> {
        TDCD_DBNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMER1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1](index.html) module"]
pub struct TIMER1_SPEC;
impl crate::RegisterSpec for TIMER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1::R](R) reader structure"]
impl crate::Readable for TIMER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1::W](W) writer structure"]
impl crate::Writable for TIMER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER1 to value 0x000a_0028"]
impl crate::Resettable for TIMER1_SPEC {
    const RESET_VALUE: Self::Ux = 0x000a_0028;
}
