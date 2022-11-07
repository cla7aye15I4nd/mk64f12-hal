#[doc = "Register `OTGISTAT` reader"]
pub struct R(crate::R<OTGISTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTGISTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTGISTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTGISTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTGISTAT` writer"]
pub struct W(crate::W<OTGISTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTGISTAT_SPEC>;
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
impl From<crate::W<OTGISTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTGISTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVBUSCHG` reader - This bit is set when a change in VBUS is detected on an A device."]
pub type AVBUSCHG_R = crate::BitReader<bool>;
#[doc = "Field `AVBUSCHG` writer - This bit is set when a change in VBUS is detected on an A device."]
pub type AVBUSCHG_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGISTAT_SPEC, bool, O>;
#[doc = "Field `B_SESS_CHG` reader - This bit is set when a change in VBUS is detected on a B device."]
pub type B_SESS_CHG_R = crate::BitReader<bool>;
#[doc = "Field `B_SESS_CHG` writer - This bit is set when a change in VBUS is detected on a B device."]
pub type B_SESS_CHG_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGISTAT_SPEC, bool, O>;
#[doc = "Field `SESSVLDCHG` reader - This bit is set when a change in VBUS is detected indicating a session valid or a session no longer valid"]
pub type SESSVLDCHG_R = crate::BitReader<bool>;
#[doc = "Field `SESSVLDCHG` writer - This bit is set when a change in VBUS is detected indicating a session valid or a session no longer valid"]
pub type SESSVLDCHG_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGISTAT_SPEC, bool, O>;
#[doc = "Field `LINE_STATE_CHG` reader - This interrupt is set when the USB line state (CTL\\[SE0\\]
and CTL\\[JSTATE\\]
bits) are stable without change for 1 millisecond, and the value of the line state is different from the last time when the line state was stable"]
pub type LINE_STATE_CHG_R = crate::BitReader<bool>;
#[doc = "Field `LINE_STATE_CHG` writer - This interrupt is set when the USB line state (CTL\\[SE0\\]
and CTL\\[JSTATE\\]
bits) are stable without change for 1 millisecond, and the value of the line state is different from the last time when the line state was stable"]
pub type LINE_STATE_CHG_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGISTAT_SPEC, bool, O>;
#[doc = "Field `ONEMSEC` reader - This bit is set when the 1 millisecond timer expires"]
pub type ONEMSEC_R = crate::BitReader<bool>;
#[doc = "Field `ONEMSEC` writer - This bit is set when the 1 millisecond timer expires"]
pub type ONEMSEC_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGISTAT_SPEC, bool, O>;
#[doc = "Field `IDCHG` reader - This bit is set when a change in the ID Signal from the USB connector is sensed."]
pub type IDCHG_R = crate::BitReader<bool>;
#[doc = "Field `IDCHG` writer - This bit is set when a change in the ID Signal from the USB connector is sensed."]
pub type IDCHG_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGISTAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit is set when a change in VBUS is detected on an A device."]
    #[inline(always)]
    pub fn avbuschg(&self) -> AVBUSCHG_R {
        AVBUSCHG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is set when a change in VBUS is detected on a B device."]
    #[inline(always)]
    pub fn b_sess_chg(&self) -> B_SESS_CHG_R {
        B_SESS_CHG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is set when a change in VBUS is detected indicating a session valid or a session no longer valid"]
    #[inline(always)]
    pub fn sessvldchg(&self) -> SESSVLDCHG_R {
        SESSVLDCHG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - This interrupt is set when the USB line state (CTL\\[SE0\\]
and CTL\\[JSTATE\\]
bits) are stable without change for 1 millisecond, and the value of the line state is different from the last time when the line state was stable"]
    #[inline(always)]
    pub fn line_state_chg(&self) -> LINE_STATE_CHG_R {
        LINE_STATE_CHG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is set when the 1 millisecond timer expires"]
    #[inline(always)]
    pub fn onemsec(&self) -> ONEMSEC_R {
        ONEMSEC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit is set when a change in the ID Signal from the USB connector is sensed."]
    #[inline(always)]
    pub fn idchg(&self) -> IDCHG_R {
        IDCHG_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set when a change in VBUS is detected on an A device."]
    #[inline(always)]
    #[must_use]
    pub fn avbuschg(&mut self) -> AVBUSCHG_W<0> {
        AVBUSCHG_W::new(self)
    }
    #[doc = "Bit 2 - This bit is set when a change in VBUS is detected on a B device."]
    #[inline(always)]
    #[must_use]
    pub fn b_sess_chg(&mut self) -> B_SESS_CHG_W<2> {
        B_SESS_CHG_W::new(self)
    }
    #[doc = "Bit 3 - This bit is set when a change in VBUS is detected indicating a session valid or a session no longer valid"]
    #[inline(always)]
    #[must_use]
    pub fn sessvldchg(&mut self) -> SESSVLDCHG_W<3> {
        SESSVLDCHG_W::new(self)
    }
    #[doc = "Bit 5 - This interrupt is set when the USB line state (CTL\\[SE0\\]
and CTL\\[JSTATE\\]
bits) are stable without change for 1 millisecond, and the value of the line state is different from the last time when the line state was stable"]
    #[inline(always)]
    #[must_use]
    pub fn line_state_chg(&mut self) -> LINE_STATE_CHG_W<5> {
        LINE_STATE_CHG_W::new(self)
    }
    #[doc = "Bit 6 - This bit is set when the 1 millisecond timer expires"]
    #[inline(always)]
    #[must_use]
    pub fn onemsec(&mut self) -> ONEMSEC_W<6> {
        ONEMSEC_W::new(self)
    }
    #[doc = "Bit 7 - This bit is set when a change in the ID Signal from the USB connector is sensed."]
    #[inline(always)]
    #[must_use]
    pub fn idchg(&mut self) -> IDCHG_W<7> {
        IDCHG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otgistat](index.html) module"]
pub struct OTGISTAT_SPEC;
impl crate::RegisterSpec for OTGISTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [otgistat::R](R) reader structure"]
impl crate::Readable for OTGISTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otgistat::W](W) writer structure"]
impl crate::Writable for OTGISTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTGISTAT to value 0"]
impl crate::Resettable for OTGISTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
