#[doc = "Register `TIMER2_BC12` reader"]
pub struct R(crate::R<USBDCD_TIMER2_BC12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBDCD_TIMER2_BC12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBDCD_TIMER2_BC12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBDCD_TIMER2_BC12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER2_BC12` writer"]
pub struct W(crate::W<USBDCD_TIMER2_BC12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBDCD_TIMER2_BC12_SPEC>;
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
impl From<crate::W<USBDCD_TIMER2_BC12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBDCD_TIMER2_BC12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TVDMSRC_ON` reader - Sets the amount of time (in ms) that the module enables the VDM_SRC. Valid values are 0-40ms."]
pub type TVDMSRC_ON_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TVDMSRC_ON` writer - Sets the amount of time (in ms) that the module enables the VDM_SRC. Valid values are 0-40ms."]
pub type TVDMSRC_ON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USBDCD_TIMER2_BC12_SPEC, u16, u16, 10, O>;
#[doc = "Field `TWAIT_AFTER_PRD` reader - Sets the amount of time (in ms) that the module waits after primary detection before start to secondary detection"]
pub type TWAIT_AFTER_PRD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TWAIT_AFTER_PRD` writer - Sets the amount of time (in ms) that the module waits after primary detection before start to secondary detection"]
pub type TWAIT_AFTER_PRD_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USBDCD_TIMER2_BC12_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Sets the amount of time (in ms) that the module enables the VDM_SRC. Valid values are 0-40ms."]
    #[inline(always)]
    pub fn tvdmsrc_on(&self) -> TVDMSRC_ON_R {
        TVDMSRC_ON_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Sets the amount of time (in ms) that the module waits after primary detection before start to secondary detection"]
    #[inline(always)]
    pub fn twait_after_prd(&self) -> TWAIT_AFTER_PRD_R {
        TWAIT_AFTER_PRD_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sets the amount of time (in ms) that the module enables the VDM_SRC. Valid values are 0-40ms."]
    #[inline(always)]
    #[must_use]
    pub fn tvdmsrc_on(&mut self) -> TVDMSRC_ON_W<0> {
        TVDMSRC_ON_W::new(self)
    }
    #[doc = "Bits 16:25 - Sets the amount of time (in ms) that the module waits after primary detection before start to secondary detection"]
    #[inline(always)]
    #[must_use]
    pub fn twait_after_prd(&mut self) -> TWAIT_AFTER_PRD_W<16> {
        TWAIT_AFTER_PRD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMER2_BC12 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbdcd_timer2_bc12](index.html) module"]
pub struct USBDCD_TIMER2_BC12_SPEC;
impl crate::RegisterSpec for USBDCD_TIMER2_BC12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbdcd_timer2_bc12::R](R) reader structure"]
impl crate::Readable for USBDCD_TIMER2_BC12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbdcd_timer2_bc12::W](W) writer structure"]
impl crate::Writable for USBDCD_TIMER2_BC12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER2_BC12 to value 0x0001_0028"]
impl crate::Resettable for USBDCD_TIMER2_BC12_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0028;
}
