#[doc = "Register `TIMER2_BC11` reader"]
pub struct R(crate::R<USBDCD_TIMER2_BC11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBDCD_TIMER2_BC11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBDCD_TIMER2_BC11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBDCD_TIMER2_BC11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER2_BC11` writer"]
pub struct W(crate::W<USBDCD_TIMER2_BC11_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBDCD_TIMER2_BC11_SPEC>;
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
impl From<crate::W<USBDCD_TIMER2_BC11_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBDCD_TIMER2_BC11_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHECK_DM` reader - Time Before Check of D- Line"]
pub type CHECK_DM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CHECK_DM` writer - Time Before Check of D- Line"]
pub type CHECK_DM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USBDCD_TIMER2_BC11_SPEC, u8, u8, 4, O>;
#[doc = "Field `TVDPSRC_CON` reader - Time Period Before Enabling D+ Pullup"]
pub type TVDPSRC_CON_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TVDPSRC_CON` writer - Time Period Before Enabling D+ Pullup"]
pub type TVDPSRC_CON_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USBDCD_TIMER2_BC11_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:3 - Time Before Check of D- Line"]
    #[inline(always)]
    pub fn check_dm(&self) -> CHECK_DM_R {
        CHECK_DM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:25 - Time Period Before Enabling D+ Pullup"]
    #[inline(always)]
    pub fn tvdpsrc_con(&self) -> TVDPSRC_CON_R {
        TVDPSRC_CON_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:3 - Time Before Check of D- Line"]
    #[inline(always)]
    #[must_use]
    pub fn check_dm(&mut self) -> CHECK_DM_W<0> {
        CHECK_DM_W::new(self)
    }
    #[doc = "Bits 16:25 - Time Period Before Enabling D+ Pullup"]
    #[inline(always)]
    #[must_use]
    pub fn tvdpsrc_con(&mut self) -> TVDPSRC_CON_W<16> {
        TVDPSRC_CON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMER2_BC11 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbdcd_timer2_bc11](index.html) module"]
pub struct USBDCD_TIMER2_BC11_SPEC;
impl crate::RegisterSpec for USBDCD_TIMER2_BC11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbdcd_timer2_bc11::R](R) reader structure"]
impl crate::Readable for USBDCD_TIMER2_BC11_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbdcd_timer2_bc11::W](W) writer structure"]
impl crate::Writable for USBDCD_TIMER2_BC11_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER2_BC11 to value 0x0028_0001"]
impl crate::Resettable for USBDCD_TIMER2_BC11_SPEC {
    const RESET_VALUE: Self::Ux = 0x0028_0001;
}
