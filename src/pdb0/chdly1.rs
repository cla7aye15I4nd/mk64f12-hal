#[doc = "Register `CH%sDLY1` reader"]
pub struct R(crate::R<CHDLY1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHDLY1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHDLY1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHDLY1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%sDLY1` writer"]
pub struct W(crate::W<CHDLY1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHDLY1_SPEC>;
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
impl From<crate::W<CHDLY1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHDLY1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLY` reader - PDB Channel Delay"]
pub type DLY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DLY` writer - PDB Channel Delay"]
pub type DLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHDLY1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - PDB Channel Delay"]
    #[inline(always)]
    pub fn dly(&self) -> DLY_R {
        DLY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PDB Channel Delay"]
    #[inline(always)]
    #[must_use]
    pub fn dly(&mut self) -> DLY_W<0> {
        DLY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Delay 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chdly1](index.html) module"]
pub struct CHDLY1_SPEC;
impl crate::RegisterSpec for CHDLY1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chdly1::R](R) reader structure"]
impl crate::Readable for CHDLY1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chdly1::W](W) writer structure"]
impl crate::Writable for CHDLY1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%sDLY1 to value 0"]
impl crate::Resettable for CHDLY1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
