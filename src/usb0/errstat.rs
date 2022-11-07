#[doc = "Register `ERRSTAT` reader"]
pub struct R(crate::R<ERRSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERRSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERRSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERRSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERRSTAT` writer"]
pub struct W(crate::W<ERRSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERRSTAT_SPEC>;
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
impl From<crate::W<ERRSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERRSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIDERR` reader - This bit is set when the PID check field fails."]
pub type PIDERR_R = crate::BitReader<bool>;
#[doc = "Field `PIDERR` writer - This bit is set when the PID check field fails."]
pub type PIDERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERRSTAT_SPEC, bool, O>;
#[doc = "Field `CRC5EOF` reader - This error interrupt has two functions"]
pub type CRC5EOF_R = crate::BitReader<bool>;
#[doc = "Field `CRC5EOF` writer - This error interrupt has two functions"]
pub type CRC5EOF_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERRSTAT_SPEC, bool, O>;
#[doc = "Field `CRC16` reader - This bit is set when a data packet is rejected due to a CRC16 error."]
pub type CRC16_R = crate::BitReader<bool>;
#[doc = "Field `CRC16` writer - This bit is set when a data packet is rejected due to a CRC16 error."]
pub type CRC16_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERRSTAT_SPEC, bool, O>;
#[doc = "Field `DFN8` reader - This bit is set if the data field received was not 8 bits in length"]
pub type DFN8_R = crate::BitReader<bool>;
#[doc = "Field `DFN8` writer - This bit is set if the data field received was not 8 bits in length"]
pub type DFN8_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERRSTAT_SPEC, bool, O>;
#[doc = "Field `BTOERR` reader - This bit is set when a bus turnaround timeout error occurs"]
pub type BTOERR_R = crate::BitReader<bool>;
#[doc = "Field `BTOERR` writer - This bit is set when a bus turnaround timeout error occurs"]
pub type BTOERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERRSTAT_SPEC, bool, O>;
#[doc = "Field `DMAERR` reader - This bit is set if the USB Module has requested a DMA access to read a new BDT but has not been given the bus before it needs to receive or transmit data"]
pub type DMAERR_R = crate::BitReader<bool>;
#[doc = "Field `DMAERR` writer - This bit is set if the USB Module has requested a DMA access to read a new BDT but has not been given the bus before it needs to receive or transmit data"]
pub type DMAERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERRSTAT_SPEC, bool, O>;
#[doc = "Field `BTSERR` reader - This bit is set when a bit stuff error is detected"]
pub type BTSERR_R = crate::BitReader<bool>;
#[doc = "Field `BTSERR` writer - This bit is set when a bit stuff error is detected"]
pub type BTSERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, ERRSTAT_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - This bit is set when the PID check field fails."]
    #[inline(always)]
    pub fn piderr(&self) -> PIDERR_R {
        PIDERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This error interrupt has two functions"]
    #[inline(always)]
    pub fn crc5eof(&self) -> CRC5EOF_R {
        CRC5EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is set when a data packet is rejected due to a CRC16 error."]
    #[inline(always)]
    pub fn crc16(&self) -> CRC16_R {
        CRC16_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is set if the data field received was not 8 bits in length"]
    #[inline(always)]
    pub fn dfn8(&self) -> DFN8_R {
        DFN8_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit is set when a bus turnaround timeout error occurs"]
    #[inline(always)]
    pub fn btoerr(&self) -> BTOERR_R {
        BTOERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit is set if the USB Module has requested a DMA access to read a new BDT but has not been given the bus before it needs to receive or transmit data"]
    #[inline(always)]
    pub fn dmaerr(&self) -> DMAERR_R {
        DMAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit is set when a bit stuff error is detected"]
    #[inline(always)]
    pub fn btserr(&self) -> BTSERR_R {
        BTSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is set when the PID check field fails."]
    #[inline(always)]
    #[must_use]
    pub fn piderr(&mut self) -> PIDERR_W<0> {
        PIDERR_W::new(self)
    }
    #[doc = "Bit 1 - This error interrupt has two functions"]
    #[inline(always)]
    #[must_use]
    pub fn crc5eof(&mut self) -> CRC5EOF_W<1> {
        CRC5EOF_W::new(self)
    }
    #[doc = "Bit 2 - This bit is set when a data packet is rejected due to a CRC16 error."]
    #[inline(always)]
    #[must_use]
    pub fn crc16(&mut self) -> CRC16_W<2> {
        CRC16_W::new(self)
    }
    #[doc = "Bit 3 - This bit is set if the data field received was not 8 bits in length"]
    #[inline(always)]
    #[must_use]
    pub fn dfn8(&mut self) -> DFN8_W<3> {
        DFN8_W::new(self)
    }
    #[doc = "Bit 4 - This bit is set when a bus turnaround timeout error occurs"]
    #[inline(always)]
    #[must_use]
    pub fn btoerr(&mut self) -> BTOERR_W<4> {
        BTOERR_W::new(self)
    }
    #[doc = "Bit 5 - This bit is set if the USB Module has requested a DMA access to read a new BDT but has not been given the bus before it needs to receive or transmit data"]
    #[inline(always)]
    #[must_use]
    pub fn dmaerr(&mut self) -> DMAERR_W<5> {
        DMAERR_W::new(self)
    }
    #[doc = "Bit 7 - This bit is set when a bit stuff error is detected"]
    #[inline(always)]
    #[must_use]
    pub fn btserr(&mut self) -> BTSERR_W<7> {
        BTSERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Error Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [errstat](index.html) module"]
pub struct ERRSTAT_SPEC;
impl crate::RegisterSpec for ERRSTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [errstat::R](R) reader structure"]
impl crate::Readable for ERRSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [errstat::W](W) writer structure"]
impl crate::Writable for ERRSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERRSTAT to value 0"]
impl crate::Resettable for ERRSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
