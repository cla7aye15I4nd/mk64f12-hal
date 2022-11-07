#[doc = "Register `ISTAT` reader"]
pub struct R(crate::R<ISTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISTAT` writer"]
pub struct W(crate::W<ISTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISTAT_SPEC>;
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
impl From<crate::W<ISTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBRST` reader - This bit is set when the USB Module has decoded a valid USB reset"]
pub type USBRST_R = crate::BitReader<bool>;
#[doc = "Field `USBRST` writer - This bit is set when the USB Module has decoded a valid USB reset"]
pub type USBRST_W<'a, const O: u8> = crate::BitWriter<'a, u8, ISTAT_SPEC, bool, O>;
#[doc = "Field `ERROR` reader - This bit is set when any of the error conditions within Error Interrupt Status (ERRSTAT) register occur"]
pub type ERROR_R = crate::BitReader<bool>;
#[doc = "Field `ERROR` writer - This bit is set when any of the error conditions within Error Interrupt Status (ERRSTAT) register occur"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u8, ISTAT_SPEC, bool, O>;
#[doc = "Field `SOFTOK` reader - This bit is set when the USB Module receives a Start Of Frame (SOF) token"]
pub type SOFTOK_R = crate::BitReader<bool>;
#[doc = "Field `SOFTOK` writer - This bit is set when the USB Module receives a Start Of Frame (SOF) token"]
pub type SOFTOK_W<'a, const O: u8> = crate::BitWriter<'a, u8, ISTAT_SPEC, bool, O>;
#[doc = "Field `TOKDNE` reader - This bit is set when the current token being processed has completed"]
pub type TOKDNE_R = crate::BitReader<bool>;
#[doc = "Field `TOKDNE` writer - This bit is set when the current token being processed has completed"]
pub type TOKDNE_W<'a, const O: u8> = crate::BitWriter<'a, u8, ISTAT_SPEC, bool, O>;
#[doc = "Field `SLEEP` reader - This bit is set when the USB Module detects a constant idle on the USB bus for 3 ms"]
pub type SLEEP_R = crate::BitReader<bool>;
#[doc = "Field `SLEEP` writer - This bit is set when the USB Module detects a constant idle on the USB bus for 3 ms"]
pub type SLEEP_W<'a, const O: u8> = crate::BitWriter<'a, u8, ISTAT_SPEC, bool, O>;
#[doc = "Field `RESUME` reader - This bit is set when a K-state is observed on the DP/DM signals for 2"]
pub type RESUME_R = crate::BitReader<bool>;
#[doc = "Field `RESUME` writer - This bit is set when a K-state is observed on the DP/DM signals for 2"]
pub type RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u8, ISTAT_SPEC, bool, O>;
#[doc = "Field `ATTACH` reader - Attach Interrupt"]
pub type ATTACH_R = crate::BitReader<bool>;
#[doc = "Field `ATTACH` writer - Attach Interrupt"]
pub type ATTACH_W<'a, const O: u8> = crate::BitWriter<'a, u8, ISTAT_SPEC, bool, O>;
#[doc = "Field `STALL` reader - Stall Interrupt"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `STALL` writer - Stall Interrupt"]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u8, ISTAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit is set when the USB Module has decoded a valid USB reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is set when any of the error conditions within Error Interrupt Status (ERRSTAT) register occur"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is set when the USB Module receives a Start Of Frame (SOF) token"]
    #[inline(always)]
    pub fn softok(&self) -> SOFTOK_R {
        SOFTOK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is set when the current token being processed has completed"]
    #[inline(always)]
    pub fn tokdne(&self) -> TOKDNE_R {
        TOKDNE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit is set when the USB Module detects a constant idle on the USB bus for 3 ms"]
    #[inline(always)]
    pub fn sleep(&self) -> SLEEP_R {
        SLEEP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is set when a K-state is observed on the DP/DM signals for 2"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Attach Interrupt"]
    #[inline(always)]
    pub fn attach(&self) -> ATTACH_R {
        ATTACH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Stall Interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set when the USB Module has decoded a valid USB reset"]
    #[inline(always)]
    #[must_use]
    pub fn usbrst(&mut self) -> USBRST_W<0> {
        USBRST_W::new(self)
    }
    #[doc = "Bit 1 - This bit is set when any of the error conditions within Error Interrupt Status (ERRSTAT) register occur"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<1> {
        ERROR_W::new(self)
    }
    #[doc = "Bit 2 - This bit is set when the USB Module receives a Start Of Frame (SOF) token"]
    #[inline(always)]
    #[must_use]
    pub fn softok(&mut self) -> SOFTOK_W<2> {
        SOFTOK_W::new(self)
    }
    #[doc = "Bit 3 - This bit is set when the current token being processed has completed"]
    #[inline(always)]
    #[must_use]
    pub fn tokdne(&mut self) -> TOKDNE_W<3> {
        TOKDNE_W::new(self)
    }
    #[doc = "Bit 4 - This bit is set when the USB Module detects a constant idle on the USB bus for 3 ms"]
    #[inline(always)]
    #[must_use]
    pub fn sleep(&mut self) -> SLEEP_W<4> {
        SLEEP_W::new(self)
    }
    #[doc = "Bit 5 - This bit is set when a K-state is observed on the DP/DM signals for 2"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<5> {
        RESUME_W::new(self)
    }
    #[doc = "Bit 6 - Attach Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn attach(&mut self) -> ATTACH_W<6> {
        ATTACH_W::new(self)
    }
    #[doc = "Bit 7 - Stall Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<7> {
        STALL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [istat](index.html) module"]
pub struct ISTAT_SPEC;
impl crate::RegisterSpec for ISTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [istat::R](R) reader structure"]
impl crate::Readable for ISTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [istat::W](W) writer structure"]
impl crate::Writable for ISTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISTAT to value 0"]
impl crate::Resettable for ISTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
