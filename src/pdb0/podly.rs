#[doc = "Register `PO%sDLY` reader"]
pub struct R(crate::R<PODLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PODLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PODLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PODLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PO%sDLY` writer"]
pub struct W(crate::W<PODLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PODLY_SPEC>;
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
impl From<crate::W<PODLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PODLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLY2` reader - PDB Pulse-Out Delay 2"]
pub type DLY2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DLY2` writer - PDB Pulse-Out Delay 2"]
pub type DLY2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PODLY_SPEC, u16, u16, 16, O>;
#[doc = "Field `DLY1` reader - PDB Pulse-Out Delay 1"]
pub type DLY1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DLY1` writer - PDB Pulse-Out Delay 1"]
pub type DLY1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PODLY_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - PDB Pulse-Out Delay 2"]
    #[inline(always)]
    pub fn dly2(&self) -> DLY2_R {
        DLY2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - PDB Pulse-Out Delay 1"]
    #[inline(always)]
    pub fn dly1(&self) -> DLY1_R {
        DLY1_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PDB Pulse-Out Delay 2"]
    #[inline(always)]
    #[must_use]
    pub fn dly2(&mut self) -> DLY2_W<0> {
        DLY2_W::new(self)
    }
    #[doc = "Bits 16:31 - PDB Pulse-Out Delay 1"]
    #[inline(always)]
    #[must_use]
    pub fn dly1(&mut self) -> DLY1_W<16> {
        DLY1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pulse-Out n Delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [podly](index.html) module"]
pub struct PODLY_SPEC;
impl crate::RegisterSpec for PODLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [podly::R](R) reader structure"]
impl crate::Readable for PODLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [podly::W](W) writer structure"]
impl crate::Writable for PODLY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PO%sDLY to value 0"]
impl crate::Resettable for PODLY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
