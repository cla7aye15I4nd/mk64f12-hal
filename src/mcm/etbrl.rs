#[doc = "Register `ETBRL` reader"]
pub struct R(crate::R<ETBRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ETBRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ETBRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ETBRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ETBRL` writer"]
pub struct W(crate::W<ETBRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ETBRL_SPEC>;
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
impl From<crate::W<ETBRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ETBRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RELOAD` reader - Byte Count Reload Value"]
pub type RELOAD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RELOAD` writer - Byte Count Reload Value"]
pub type RELOAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ETBRL_SPEC, u16, u16, 11, O>;
impl R {
    #[doc = "Bits 0:10 - Byte Count Reload Value"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Byte Count Reload Value"]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<0> {
        RELOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETB Reload register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [etbrl](index.html) module"]
pub struct ETBRL_SPEC;
impl crate::RegisterSpec for ETBRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [etbrl::R](R) reader structure"]
impl crate::Readable for ETBRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [etbrl::W](W) writer structure"]
impl crate::Writable for ETBRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ETBRL to value 0"]
impl crate::Resettable for ETBRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
