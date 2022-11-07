#[doc = "Register `PUSHR_SLAVE` reader"]
pub struct R(crate::R<SPI1_PUSHR_SLAVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI1_PUSHR_SLAVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI1_PUSHR_SLAVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI1_PUSHR_SLAVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUSHR_SLAVE` writer"]
pub struct W(crate::W<SPI1_PUSHR_SLAVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI1_PUSHR_SLAVE_SPEC>;
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
impl From<crate::W<SPI1_PUSHR_SLAVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI1_PUSHR_SLAVE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDATA` reader - Transmit Data"]
pub type TXDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TXDATA` writer - Transmit Data"]
pub type TXDATA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SPI1_PUSHR_SLAVE_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Transmit Data"]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TXDATA_W<0> {
        TXDATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUSH TX FIFO Register In Slave Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi1_pushr_slave](index.html) module"]
pub struct SPI1_PUSHR_SLAVE_SPEC;
impl crate::RegisterSpec for SPI1_PUSHR_SLAVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi1_pushr_slave::R](R) reader structure"]
impl crate::Readable for SPI1_PUSHR_SLAVE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi1_pushr_slave::W](W) writer structure"]
impl crate::Writable for SPI1_PUSHR_SLAVE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUSHR_SLAVE to value 0"]
impl crate::Resettable for SPI1_PUSHR_SLAVE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
