#[doc = "Register `TCR` reader"]
pub struct R(crate::R<TCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCR` writer"]
pub struct W(crate::W<TCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCR_SPEC>;
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
impl From<crate::W<TCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GTS` reader - Graceful Transmit Stop"]
pub type GTS_R = crate::BitReader<bool>;
#[doc = "Field `GTS` writer - Graceful Transmit Stop"]
pub type GTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
#[doc = "Field `FDEN` reader - Full-Duplex Enable"]
pub type FDEN_R = crate::BitReader<bool>;
#[doc = "Field `FDEN` writer - Full-Duplex Enable"]
pub type FDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, bool, O>;
#[doc = "Field `TFC_PAUSE` reader - Transmit Frame Control Pause"]
pub type TFC_PAUSE_R = crate::BitReader<TFC_PAUSE_A>;
#[doc = "Transmit Frame Control Pause\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFC_PAUSE_A {
    #[doc = "0: No PAUSE frame transmitted."]
    _0 = 0,
    #[doc = "1: The MAC stops transmission of data frames after the current transmission is complete."]
    _1 = 1,
}
impl From<TFC_PAUSE_A> for bool {
    #[inline(always)]
    fn from(variant: TFC_PAUSE_A) -> Self {
        variant as u8 != 0
    }
}
impl TFC_PAUSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TFC_PAUSE_A {
        match self.bits {
            false => TFC_PAUSE_A::_0,
            true => TFC_PAUSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TFC_PAUSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TFC_PAUSE_A::_1
    }
}
#[doc = "Field `TFC_PAUSE` writer - Transmit Frame Control Pause"]
pub type TFC_PAUSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, TFC_PAUSE_A, O>;
impl<'a, const O: u8> TFC_PAUSE_W<'a, O> {
    #[doc = "No PAUSE frame transmitted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TFC_PAUSE_A::_0)
    }
    #[doc = "The MAC stops transmission of data frames after the current transmission is complete."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TFC_PAUSE_A::_1)
    }
}
#[doc = "Field `RFC_PAUSE` reader - Receive Frame Control Pause"]
pub type RFC_PAUSE_R = crate::BitReader<bool>;
#[doc = "Field `ADDSEL` reader - Source MAC Address Select On Transmit"]
pub type ADDSEL_R = crate::FieldReader<u8, ADDSEL_A>;
#[doc = "Source MAC Address Select On Transmit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADDSEL_A {
    #[doc = "0: Node MAC address programmed on PADDR1/2 registers."]
    _000 = 0,
}
impl From<ADDSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADDSEL_A) -> Self {
        variant as _
    }
}
impl ADDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADDSEL_A> {
        match self.bits {
            0 => Some(ADDSEL_A::_000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == ADDSEL_A::_000
    }
}
#[doc = "Field `ADDSEL` writer - Source MAC Address Select On Transmit"]
pub type ADDSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR_SPEC, u8, ADDSEL_A, 3, O>;
impl<'a, const O: u8> ADDSEL_W<'a, O> {
    #[doc = "Node MAC address programmed on PADDR1/2 registers."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(ADDSEL_A::_000)
    }
}
#[doc = "Field `ADDINS` reader - Set MAC Address On Transmit"]
pub type ADDINS_R = crate::BitReader<ADDINS_A>;
#[doc = "Set MAC Address On Transmit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDINS_A {
    #[doc = "0: The source MAC address is not modified by the MAC."]
    _0 = 0,
    #[doc = "1: The MAC overwrites the source MAC address with the programmed MAC address according to ADDSEL."]
    _1 = 1,
}
impl From<ADDINS_A> for bool {
    #[inline(always)]
    fn from(variant: ADDINS_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDINS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDINS_A {
        match self.bits {
            false => ADDINS_A::_0,
            true => ADDINS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADDINS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADDINS_A::_1
    }
}
#[doc = "Field `ADDINS` writer - Set MAC Address On Transmit"]
pub type ADDINS_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, ADDINS_A, O>;
impl<'a, const O: u8> ADDINS_W<'a, O> {
    #[doc = "The source MAC address is not modified by the MAC."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADDINS_A::_0)
    }
    #[doc = "The MAC overwrites the source MAC address with the programmed MAC address according to ADDSEL."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADDINS_A::_1)
    }
}
#[doc = "Field `CRCFWD` reader - Forward Frame From Application With CRC"]
pub type CRCFWD_R = crate::BitReader<CRCFWD_A>;
#[doc = "Forward Frame From Application With CRC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCFWD_A {
    #[doc = "0: TxBD\\[TC\\]
controls whether the frame has a CRC from the application."]
    _0 = 0,
    #[doc = "1: The transmitter does not append any CRC to transmitted frames, as it is expecting a frame with CRC from the application."]
    _1 = 1,
}
impl From<CRCFWD_A> for bool {
    #[inline(always)]
    fn from(variant: CRCFWD_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCFWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCFWD_A {
        match self.bits {
            false => CRCFWD_A::_0,
            true => CRCFWD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCFWD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCFWD_A::_1
    }
}
#[doc = "Field `CRCFWD` writer - Forward Frame From Application With CRC"]
pub type CRCFWD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR_SPEC, CRCFWD_A, O>;
impl<'a, const O: u8> CRCFWD_W<'a, O> {
    #[doc = "TxBD\\[TC\\]
controls whether the frame has a CRC from the application."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRCFWD_A::_0)
    }
    #[doc = "The transmitter does not append any CRC to transmitted frames, as it is expecting a frame with CRC from the application."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRCFWD_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Graceful Transmit Stop"]
    #[inline(always)]
    pub fn gts(&self) -> GTS_R {
        GTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Full-Duplex Enable"]
    #[inline(always)]
    pub fn fden(&self) -> FDEN_R {
        FDEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Frame Control Pause"]
    #[inline(always)]
    pub fn tfc_pause(&self) -> TFC_PAUSE_R {
        TFC_PAUSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Frame Control Pause"]
    #[inline(always)]
    pub fn rfc_pause(&self) -> RFC_PAUSE_R {
        RFC_PAUSE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Source MAC Address Select On Transmit"]
    #[inline(always)]
    pub fn addsel(&self) -> ADDSEL_R {
        ADDSEL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Set MAC Address On Transmit"]
    #[inline(always)]
    pub fn addins(&self) -> ADDINS_R {
        ADDINS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Forward Frame From Application With CRC"]
    #[inline(always)]
    pub fn crcfwd(&self) -> CRCFWD_R {
        CRCFWD_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Graceful Transmit Stop"]
    #[inline(always)]
    #[must_use]
    pub fn gts(&mut self) -> GTS_W<0> {
        GTS_W::new(self)
    }
    #[doc = "Bit 2 - Full-Duplex Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fden(&mut self) -> FDEN_W<2> {
        FDEN_W::new(self)
    }
    #[doc = "Bit 3 - Transmit Frame Control Pause"]
    #[inline(always)]
    #[must_use]
    pub fn tfc_pause(&mut self) -> TFC_PAUSE_W<3> {
        TFC_PAUSE_W::new(self)
    }
    #[doc = "Bits 5:7 - Source MAC Address Select On Transmit"]
    #[inline(always)]
    #[must_use]
    pub fn addsel(&mut self) -> ADDSEL_W<5> {
        ADDSEL_W::new(self)
    }
    #[doc = "Bit 8 - Set MAC Address On Transmit"]
    #[inline(always)]
    #[must_use]
    pub fn addins(&mut self) -> ADDINS_W<8> {
        ADDINS_W::new(self)
    }
    #[doc = "Bit 9 - Forward Frame From Application With CRC"]
    #[inline(always)]
    #[must_use]
    pub fn crcfwd(&mut self) -> CRCFWD_W<9> {
        CRCFWD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](index.html) module"]
pub struct TCR_SPEC;
impl crate::RegisterSpec for TCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcr::R](R) reader structure"]
impl crate::Readable for TCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcr::W](W) writer structure"]
impl crate::Writable for TCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
