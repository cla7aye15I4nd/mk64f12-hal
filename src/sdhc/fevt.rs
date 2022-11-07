#[doc = "Register `FEVT` writer"]
pub struct W(crate::W<FEVT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FEVT_SPEC>;
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
impl From<crate::W<FEVT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FEVT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AC12NE` writer - Force Event Auto Command 12 Not Executed"]
pub type AC12NE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEVT_SPEC, bool, O>;
#[doc = "Field `AC12TOE` writer - Force Event Auto Command 12 Time Out Error"]
pub type AC12TOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEVT_SPEC, bool, O>;
#[doc = "Field `AC12CE` writer - Force Event Auto Command 12 CRC Error"]
pub type AC12CE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEVT_SPEC, bool, O>;
#[doc = "Field `AC12EBE` writer - Force Event Auto Command 12 End Bit Error"]
pub type AC12EBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEVT_SPEC, bool, O>;
#[doc = "Field `AC12IE` writer - Force Event Auto Command 12 Index Error"]
pub type AC12IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEVT_SPEC, bool, O>;
#[doc = "Field `CNIBAC12E` writer - Force Event Command Not Executed By Auto Command 12 Error"]
pub type CNIBAC12E_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEVT_SPEC, bool, O>;
#[doc = "Field `CTOE` writer - Force Event Command Time Out Error"]
pub type CTOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEVT_SPEC, bool, O>;
#[doc = "Field `CCE` writer - Force Event Command CRC Error"]
pub type CCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEVT_SPEC, bool, O>;
#[doc = "Field `CEBE` writer - Force Event Command End Bit Error"]
pub type CEBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEVT_SPEC, bool, O>;
#[doc = "Field `CIE` writer - Force Event Command Index Error"]
pub type CIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEVT_SPEC, bool, O>;
#[doc = "Field `DTOE` writer - Force Event Data Time Out Error"]
pub type DTOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEVT_SPEC, bool, O>;
#[doc = "Field `DCE` writer - Force Event Data CRC Error"]
pub type DCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEVT_SPEC, bool, O>;
#[doc = "Field `DEBE` writer - Force Event Data End Bit Error"]
pub type DEBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEVT_SPEC, bool, O>;
#[doc = "Field `AC12E` writer - Force Event Auto Command 12 Error"]
pub type AC12E_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEVT_SPEC, bool, O>;
#[doc = "Field `DMAE` writer - Force Event DMA Error"]
pub type DMAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEVT_SPEC, bool, O>;
#[doc = "Field `CINT` writer - Force Event Card Interrupt"]
pub type CINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, FEVT_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Force Event Auto Command 12 Not Executed"]
    #[inline(always)]
    #[must_use]
    pub fn ac12ne(&mut self) -> AC12NE_W<0> {
        AC12NE_W::new(self)
    }
    #[doc = "Bit 1 - Force Event Auto Command 12 Time Out Error"]
    #[inline(always)]
    #[must_use]
    pub fn ac12toe(&mut self) -> AC12TOE_W<1> {
        AC12TOE_W::new(self)
    }
    #[doc = "Bit 2 - Force Event Auto Command 12 CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn ac12ce(&mut self) -> AC12CE_W<2> {
        AC12CE_W::new(self)
    }
    #[doc = "Bit 3 - Force Event Auto Command 12 End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn ac12ebe(&mut self) -> AC12EBE_W<3> {
        AC12EBE_W::new(self)
    }
    #[doc = "Bit 4 - Force Event Auto Command 12 Index Error"]
    #[inline(always)]
    #[must_use]
    pub fn ac12ie(&mut self) -> AC12IE_W<4> {
        AC12IE_W::new(self)
    }
    #[doc = "Bit 7 - Force Event Command Not Executed By Auto Command 12 Error"]
    #[inline(always)]
    #[must_use]
    pub fn cnibac12e(&mut self) -> CNIBAC12E_W<7> {
        CNIBAC12E_W::new(self)
    }
    #[doc = "Bit 16 - Force Event Command Time Out Error"]
    #[inline(always)]
    #[must_use]
    pub fn ctoe(&mut self) -> CTOE_W<16> {
        CTOE_W::new(self)
    }
    #[doc = "Bit 17 - Force Event Command CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn cce(&mut self) -> CCE_W<17> {
        CCE_W::new(self)
    }
    #[doc = "Bit 18 - Force Event Command End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn cebe(&mut self) -> CEBE_W<18> {
        CEBE_W::new(self)
    }
    #[doc = "Bit 19 - Force Event Command Index Error"]
    #[inline(always)]
    #[must_use]
    pub fn cie(&mut self) -> CIE_W<19> {
        CIE_W::new(self)
    }
    #[doc = "Bit 20 - Force Event Data Time Out Error"]
    #[inline(always)]
    #[must_use]
    pub fn dtoe(&mut self) -> DTOE_W<20> {
        DTOE_W::new(self)
    }
    #[doc = "Bit 21 - Force Event Data CRC Error"]
    #[inline(always)]
    #[must_use]
    pub fn dce(&mut self) -> DCE_W<21> {
        DCE_W::new(self)
    }
    #[doc = "Bit 22 - Force Event Data End Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn debe(&mut self) -> DEBE_W<22> {
        DEBE_W::new(self)
    }
    #[doc = "Bit 24 - Force Event Auto Command 12 Error"]
    #[inline(always)]
    #[must_use]
    pub fn ac12e(&mut self) -> AC12E_W<24> {
        AC12E_W::new(self)
    }
    #[doc = "Bit 28 - Force Event DMA Error"]
    #[inline(always)]
    #[must_use]
    pub fn dmae(&mut self) -> DMAE_W<28> {
        DMAE_W::new(self)
    }
    #[doc = "Bit 31 - Force Event Card Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cint(&mut self) -> CINT_W<31> {
        CINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Force Event register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fevt](index.html) module"]
pub struct FEVT_SPEC;
impl crate::RegisterSpec for FEVT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [fevt::W](W) writer structure"]
impl crate::Writable for FEVT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FEVT to value 0"]
impl crate::Resettable for FEVT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
