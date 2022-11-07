#[doc = "Register `EIR` reader"]
pub struct R(crate::R<EIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EIR` writer"]
pub struct W(crate::W<EIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EIR_SPEC>;
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
impl From<crate::W<EIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TS_TIMER` reader - Timestamp Timer"]
pub type TS_TIMER_R = crate::BitReader<bool>;
#[doc = "Field `TS_TIMER` writer - Timestamp Timer"]
pub type TS_TIMER_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, bool, O>;
#[doc = "Field `TS_AVAIL` reader - Transmit Timestamp Available"]
pub type TS_AVAIL_R = crate::BitReader<bool>;
#[doc = "Field `TS_AVAIL` writer - Transmit Timestamp Available"]
pub type TS_AVAIL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, bool, O>;
#[doc = "Field `WAKEUP` reader - Node Wakeup Request Indication"]
pub type WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUP` writer - Node Wakeup Request Indication"]
pub type WAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, bool, O>;
#[doc = "Field `PLR` reader - Payload Receive Error"]
pub type PLR_R = crate::BitReader<bool>;
#[doc = "Field `PLR` writer - Payload Receive Error"]
pub type PLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, bool, O>;
#[doc = "Field `UN` reader - Transmit FIFO Underrun"]
pub type UN_R = crate::BitReader<bool>;
#[doc = "Field `UN` writer - Transmit FIFO Underrun"]
pub type UN_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, bool, O>;
#[doc = "Field `RL` reader - Collision Retry Limit"]
pub type RL_R = crate::BitReader<bool>;
#[doc = "Field `RL` writer - Collision Retry Limit"]
pub type RL_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, bool, O>;
#[doc = "Field `LC` reader - Late Collision"]
pub type LC_R = crate::BitReader<bool>;
#[doc = "Field `LC` writer - Late Collision"]
pub type LC_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, bool, O>;
#[doc = "Field `EBERR` reader - Ethernet Bus Error"]
pub type EBERR_R = crate::BitReader<bool>;
#[doc = "Field `EBERR` writer - Ethernet Bus Error"]
pub type EBERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, bool, O>;
#[doc = "Field `MII` reader - MII Interrupt."]
pub type MII_R = crate::BitReader<bool>;
#[doc = "Field `MII` writer - MII Interrupt."]
pub type MII_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, bool, O>;
#[doc = "Field `RXB` reader - Receive Buffer Interrupt"]
pub type RXB_R = crate::BitReader<bool>;
#[doc = "Field `RXB` writer - Receive Buffer Interrupt"]
pub type RXB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, bool, O>;
#[doc = "Field `RXF` reader - Receive Frame Interrupt"]
pub type RXF_R = crate::BitReader<bool>;
#[doc = "Field `RXF` writer - Receive Frame Interrupt"]
pub type RXF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, bool, O>;
#[doc = "Field `TXB` reader - Transmit Buffer Interrupt"]
pub type TXB_R = crate::BitReader<bool>;
#[doc = "Field `TXB` writer - Transmit Buffer Interrupt"]
pub type TXB_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, bool, O>;
#[doc = "Field `TXF` reader - Transmit Frame Interrupt"]
pub type TXF_R = crate::BitReader<bool>;
#[doc = "Field `TXF` writer - Transmit Frame Interrupt"]
pub type TXF_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, bool, O>;
#[doc = "Field `GRA` reader - Graceful Stop Complete"]
pub type GRA_R = crate::BitReader<bool>;
#[doc = "Field `GRA` writer - Graceful Stop Complete"]
pub type GRA_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, bool, O>;
#[doc = "Field `BABT` reader - Babbling Transmit Error"]
pub type BABT_R = crate::BitReader<bool>;
#[doc = "Field `BABT` writer - Babbling Transmit Error"]
pub type BABT_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, bool, O>;
#[doc = "Field `BABR` reader - Babbling Receive Error"]
pub type BABR_R = crate::BitReader<bool>;
#[doc = "Field `BABR` writer - Babbling Receive Error"]
pub type BABR_W<'a, const O: u8> = crate::BitWriter<'a, u32, EIR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 15 - Timestamp Timer"]
    #[inline(always)]
    pub fn ts_timer(&self) -> TS_TIMER_R {
        TS_TIMER_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transmit Timestamp Available"]
    #[inline(always)]
    pub fn ts_avail(&self) -> TS_AVAIL_R {
        TS_AVAIL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Node Wakeup Request Indication"]
    #[inline(always)]
    pub fn wakeup(&self) -> WAKEUP_R {
        WAKEUP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Payload Receive Error"]
    #[inline(always)]
    pub fn plr(&self) -> PLR_R {
        PLR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmit FIFO Underrun"]
    #[inline(always)]
    pub fn un(&self) -> UN_R {
        UN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Collision Retry Limit"]
    #[inline(always)]
    pub fn rl(&self) -> RL_R {
        RL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Late Collision"]
    #[inline(always)]
    pub fn lc(&self) -> LC_R {
        LC_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Ethernet Bus Error"]
    #[inline(always)]
    pub fn eberr(&self) -> EBERR_R {
        EBERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MII Interrupt."]
    #[inline(always)]
    pub fn mii(&self) -> MII_R {
        MII_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Receive Buffer Interrupt"]
    #[inline(always)]
    pub fn rxb(&self) -> RXB_R {
        RXB_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Receive Frame Interrupt"]
    #[inline(always)]
    pub fn rxf(&self) -> RXF_R {
        RXF_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Transmit Buffer Interrupt"]
    #[inline(always)]
    pub fn txb(&self) -> TXB_R {
        TXB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit Frame Interrupt"]
    #[inline(always)]
    pub fn txf(&self) -> TXF_R {
        TXF_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Graceful Stop Complete"]
    #[inline(always)]
    pub fn gra(&self) -> GRA_R {
        GRA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Babbling Transmit Error"]
    #[inline(always)]
    pub fn babt(&self) -> BABT_R {
        BABT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Babbling Receive Error"]
    #[inline(always)]
    pub fn babr(&self) -> BABR_R {
        BABR_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - Timestamp Timer"]
    #[inline(always)]
    #[must_use]
    pub fn ts_timer(&mut self) -> TS_TIMER_W<15> {
        TS_TIMER_W::new(self)
    }
    #[doc = "Bit 16 - Transmit Timestamp Available"]
    #[inline(always)]
    #[must_use]
    pub fn ts_avail(&mut self) -> TS_AVAIL_W<16> {
        TS_AVAIL_W::new(self)
    }
    #[doc = "Bit 17 - Node Wakeup Request Indication"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup(&mut self) -> WAKEUP_W<17> {
        WAKEUP_W::new(self)
    }
    #[doc = "Bit 18 - Payload Receive Error"]
    #[inline(always)]
    #[must_use]
    pub fn plr(&mut self) -> PLR_W<18> {
        PLR_W::new(self)
    }
    #[doc = "Bit 19 - Transmit FIFO Underrun"]
    #[inline(always)]
    #[must_use]
    pub fn un(&mut self) -> UN_W<19> {
        UN_W::new(self)
    }
    #[doc = "Bit 20 - Collision Retry Limit"]
    #[inline(always)]
    #[must_use]
    pub fn rl(&mut self) -> RL_W<20> {
        RL_W::new(self)
    }
    #[doc = "Bit 21 - Late Collision"]
    #[inline(always)]
    #[must_use]
    pub fn lc(&mut self) -> LC_W<21> {
        LC_W::new(self)
    }
    #[doc = "Bit 22 - Ethernet Bus Error"]
    #[inline(always)]
    #[must_use]
    pub fn eberr(&mut self) -> EBERR_W<22> {
        EBERR_W::new(self)
    }
    #[doc = "Bit 23 - MII Interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn mii(&mut self) -> MII_W<23> {
        MII_W::new(self)
    }
    #[doc = "Bit 24 - Receive Buffer Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxb(&mut self) -> RXB_W<24> {
        RXB_W::new(self)
    }
    #[doc = "Bit 25 - Receive Frame Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rxf(&mut self) -> RXF_W<25> {
        RXF_W::new(self)
    }
    #[doc = "Bit 26 - Transmit Buffer Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txb(&mut self) -> TXB_W<26> {
        TXB_W::new(self)
    }
    #[doc = "Bit 27 - Transmit Frame Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn txf(&mut self) -> TXF_W<27> {
        TXF_W::new(self)
    }
    #[doc = "Bit 28 - Graceful Stop Complete"]
    #[inline(always)]
    #[must_use]
    pub fn gra(&mut self) -> GRA_W<28> {
        GRA_W::new(self)
    }
    #[doc = "Bit 29 - Babbling Transmit Error"]
    #[inline(always)]
    #[must_use]
    pub fn babt(&mut self) -> BABT_W<29> {
        BABT_W::new(self)
    }
    #[doc = "Bit 30 - Babbling Receive Error"]
    #[inline(always)]
    #[must_use]
    pub fn babr(&mut self) -> BABR_W<30> {
        BABR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Event Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eir](index.html) module"]
pub struct EIR_SPEC;
impl crate::RegisterSpec for EIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eir::R](R) reader structure"]
impl crate::Readable for EIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eir::W](W) writer structure"]
impl crate::Writable for EIR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EIR to value 0"]
impl crate::Resettable for EIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
