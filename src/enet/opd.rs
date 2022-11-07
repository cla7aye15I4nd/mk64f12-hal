#[doc = "Register `OPD` reader"]
pub struct R(crate::R<OPD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPD` writer"]
pub struct W(crate::W<OPD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPD_SPEC>;
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
impl From<crate::W<OPD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAUSE_DUR` reader - Pause Duration"]
pub type PAUSE_DUR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PAUSE_DUR` writer - Pause Duration"]
pub type PAUSE_DUR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPD_SPEC, u16, u16, 16, O>;
#[doc = "Field `OPCODE` reader - Opcode Field In PAUSE Frames"]
pub type OPCODE_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Pause Duration"]
    #[inline(always)]
    pub fn pause_dur(&self) -> PAUSE_DUR_R {
        PAUSE_DUR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Opcode Field In PAUSE Frames"]
    #[inline(always)]
    pub fn opcode(&self) -> OPCODE_R {
        OPCODE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pause Duration"]
    #[inline(always)]
    #[must_use]
    pub fn pause_dur(&mut self) -> PAUSE_DUR_W<0> {
        PAUSE_DUR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Opcode/Pause Duration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opd](index.html) module"]
pub struct OPD_SPEC;
impl crate::RegisterSpec for OPD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opd::R](R) reader structure"]
impl crate::Readable for OPD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opd::W](W) writer structure"]
impl crate::Writable for OPD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPD to value 0x0001_0000"]
impl crate::Resettable for OPD_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
