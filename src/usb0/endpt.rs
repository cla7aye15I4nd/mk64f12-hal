#[doc = "Register `ENDPT%s` reader"]
pub struct R(crate::R<ENDPT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENDPT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENDPT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENDPT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENDPT%s` writer"]
pub struct W(crate::W<ENDPT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENDPT_SPEC>;
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
impl From<crate::W<ENDPT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENDPT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EPHSHK` reader - When set this bit enables an endpoint to perform handshaking during a transaction to this endpoint"]
pub type EPHSHK_R = crate::BitReader<bool>;
#[doc = "Field `EPHSHK` writer - When set this bit enables an endpoint to perform handshaking during a transaction to this endpoint"]
pub type EPHSHK_W<'a, const O: u8> = crate::BitWriter<'a, u8, ENDPT_SPEC, bool, O>;
#[doc = "Field `EPSTALL` reader - When set this bit indicates that the endpoint is called"]
pub type EPSTALL_R = crate::BitReader<bool>;
#[doc = "Field `EPSTALL` writer - When set this bit indicates that the endpoint is called"]
pub type EPSTALL_W<'a, const O: u8> = crate::BitWriter<'a, u8, ENDPT_SPEC, bool, O>;
#[doc = "Field `EPTXEN` reader - This bit, when set, enables the endpoint for TX transfers."]
pub type EPTXEN_R = crate::BitReader<bool>;
#[doc = "Field `EPTXEN` writer - This bit, when set, enables the endpoint for TX transfers."]
pub type EPTXEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, ENDPT_SPEC, bool, O>;
#[doc = "Field `EPRXEN` reader - This bit, when set, enables the endpoint for RX transfers."]
pub type EPRXEN_R = crate::BitReader<bool>;
#[doc = "Field `EPRXEN` writer - This bit, when set, enables the endpoint for RX transfers."]
pub type EPRXEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, ENDPT_SPEC, bool, O>;
#[doc = "Field `EPCTLDIS` reader - This bit, when set, disables control (SETUP) transfers"]
pub type EPCTLDIS_R = crate::BitReader<bool>;
#[doc = "Field `EPCTLDIS` writer - This bit, when set, disables control (SETUP) transfers"]
pub type EPCTLDIS_W<'a, const O: u8> = crate::BitWriter<'a, u8, ENDPT_SPEC, bool, O>;
#[doc = "Field `RETRYDIS` reader - This is a Host mode only bit and is present in the control register for endpoint 0 (ENDPT0) only"]
pub type RETRYDIS_R = crate::BitReader<bool>;
#[doc = "Field `RETRYDIS` writer - This is a Host mode only bit and is present in the control register for endpoint 0 (ENDPT0) only"]
pub type RETRYDIS_W<'a, const O: u8> = crate::BitWriter<'a, u8, ENDPT_SPEC, bool, O>;
#[doc = "Field `HOSTWOHUB` reader - This is a Host mode only field and is present in the control register for endpoint 0 (ENDPT0) only"]
pub type HOSTWOHUB_R = crate::BitReader<bool>;
#[doc = "Field `HOSTWOHUB` writer - This is a Host mode only field and is present in the control register for endpoint 0 (ENDPT0) only"]
pub type HOSTWOHUB_W<'a, const O: u8> = crate::BitWriter<'a, u8, ENDPT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - When set this bit enables an endpoint to perform handshaking during a transaction to this endpoint"]
    #[inline(always)]
    pub fn ephshk(&self) -> EPHSHK_R {
        EPHSHK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set this bit indicates that the endpoint is called"]
    #[inline(always)]
    pub fn epstall(&self) -> EPSTALL_R {
        EPSTALL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit, when set, enables the endpoint for TX transfers."]
    #[inline(always)]
    pub fn eptxen(&self) -> EPTXEN_R {
        EPTXEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit, when set, enables the endpoint for RX transfers."]
    #[inline(always)]
    pub fn eprxen(&self) -> EPRXEN_R {
        EPRXEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit, when set, disables control (SETUP) transfers"]
    #[inline(always)]
    pub fn epctldis(&self) -> EPCTLDIS_R {
        EPCTLDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - This is a Host mode only bit and is present in the control register for endpoint 0 (ENDPT0) only"]
    #[inline(always)]
    pub fn retrydis(&self) -> RETRYDIS_R {
        RETRYDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is a Host mode only field and is present in the control register for endpoint 0 (ENDPT0) only"]
    #[inline(always)]
    pub fn hostwohub(&self) -> HOSTWOHUB_R {
        HOSTWOHUB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When set this bit enables an endpoint to perform handshaking during a transaction to this endpoint"]
    #[inline(always)]
    #[must_use]
    pub fn ephshk(&mut self) -> EPHSHK_W<0> {
        EPHSHK_W::new(self)
    }
    #[doc = "Bit 1 - When set this bit indicates that the endpoint is called"]
    #[inline(always)]
    #[must_use]
    pub fn epstall(&mut self) -> EPSTALL_W<1> {
        EPSTALL_W::new(self)
    }
    #[doc = "Bit 2 - This bit, when set, enables the endpoint for TX transfers."]
    #[inline(always)]
    #[must_use]
    pub fn eptxen(&mut self) -> EPTXEN_W<2> {
        EPTXEN_W::new(self)
    }
    #[doc = "Bit 3 - This bit, when set, enables the endpoint for RX transfers."]
    #[inline(always)]
    #[must_use]
    pub fn eprxen(&mut self) -> EPRXEN_W<3> {
        EPRXEN_W::new(self)
    }
    #[doc = "Bit 4 - This bit, when set, disables control (SETUP) transfers"]
    #[inline(always)]
    #[must_use]
    pub fn epctldis(&mut self) -> EPCTLDIS_W<4> {
        EPCTLDIS_W::new(self)
    }
    #[doc = "Bit 6 - This is a Host mode only bit and is present in the control register for endpoint 0 (ENDPT0) only"]
    #[inline(always)]
    #[must_use]
    pub fn retrydis(&mut self) -> RETRYDIS_W<6> {
        RETRYDIS_W::new(self)
    }
    #[doc = "Bit 7 - This is a Host mode only field and is present in the control register for endpoint 0 (ENDPT0) only"]
    #[inline(always)]
    #[must_use]
    pub fn hostwohub(&mut self) -> HOSTWOHUB_W<7> {
        HOSTWOHUB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [endpt](index.html) module"]
pub struct ENDPT_SPEC;
impl crate::RegisterSpec for ENDPT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [endpt::R](R) reader structure"]
impl crate::Readable for ENDPT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [endpt::W](W) writer structure"]
impl crate::Writable for ENDPT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ENDPT%s to value 0"]
impl crate::Resettable for ENDPT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
