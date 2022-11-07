#[doc = "Register `RACC` reader"]
pub struct R(crate::R<RACC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RACC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RACC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RACC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RACC` writer"]
pub struct W(crate::W<RACC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RACC_SPEC>;
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
impl From<crate::W<RACC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RACC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PADREM` reader - Enable Padding Removal For Short IP Frames"]
pub type PADREM_R = crate::BitReader<PADREM_A>;
#[doc = "Enable Padding Removal For Short IP Frames\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PADREM_A {
    #[doc = "0: Padding not removed."]
    _0 = 0,
    #[doc = "1: Any bytes following the IP payload section of the frame are removed from the frame."]
    _1 = 1,
}
impl From<PADREM_A> for bool {
    #[inline(always)]
    fn from(variant: PADREM_A) -> Self {
        variant as u8 != 0
    }
}
impl PADREM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PADREM_A {
        match self.bits {
            false => PADREM_A::_0,
            true => PADREM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PADREM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PADREM_A::_1
    }
}
#[doc = "Field `PADREM` writer - Enable Padding Removal For Short IP Frames"]
pub type PADREM_W<'a, const O: u8> = crate::BitWriter<'a, u32, RACC_SPEC, PADREM_A, O>;
impl<'a, const O: u8> PADREM_W<'a, O> {
    #[doc = "Padding not removed."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PADREM_A::_0)
    }
    #[doc = "Any bytes following the IP payload section of the frame are removed from the frame."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PADREM_A::_1)
    }
}
#[doc = "Field `IPDIS` reader - Enable Discard Of Frames With Wrong IPv4 Header Checksum"]
pub type IPDIS_R = crate::BitReader<IPDIS_A>;
#[doc = "Enable Discard Of Frames With Wrong IPv4 Header Checksum\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPDIS_A {
    #[doc = "0: Frames with wrong IPv4 header checksum are not discarded."]
    _0 = 0,
    #[doc = "1: If an IPv4 frame is received with a mismatching header checksum, the frame is discarded. IPv6 has no header checksum and is not affected by this setting. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared)."]
    _1 = 1,
}
impl From<IPDIS_A> for bool {
    #[inline(always)]
    fn from(variant: IPDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl IPDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPDIS_A {
        match self.bits {
            false => IPDIS_A::_0,
            true => IPDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IPDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IPDIS_A::_1
    }
}
#[doc = "Field `IPDIS` writer - Enable Discard Of Frames With Wrong IPv4 Header Checksum"]
pub type IPDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RACC_SPEC, IPDIS_A, O>;
impl<'a, const O: u8> IPDIS_W<'a, O> {
    #[doc = "Frames with wrong IPv4 header checksum are not discarded."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IPDIS_A::_0)
    }
    #[doc = "If an IPv4 frame is received with a mismatching header checksum, the frame is discarded. IPv6 has no header checksum and is not affected by this setting. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IPDIS_A::_1)
    }
}
#[doc = "Field `PRODIS` reader - Enable Discard Of Frames With Wrong Protocol Checksum"]
pub type PRODIS_R = crate::BitReader<PRODIS_A>;
#[doc = "Enable Discard Of Frames With Wrong Protocol Checksum\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRODIS_A {
    #[doc = "0: Frames with wrong checksum are not discarded."]
    _0 = 0,
    #[doc = "1: If a TCP/IP, UDP/IP, or ICMP/IP frame is received that has a wrong TCP, UDP, or ICMP checksum, the frame is discarded. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared)."]
    _1 = 1,
}
impl From<PRODIS_A> for bool {
    #[inline(always)]
    fn from(variant: PRODIS_A) -> Self {
        variant as u8 != 0
    }
}
impl PRODIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRODIS_A {
        match self.bits {
            false => PRODIS_A::_0,
            true => PRODIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PRODIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PRODIS_A::_1
    }
}
#[doc = "Field `PRODIS` writer - Enable Discard Of Frames With Wrong Protocol Checksum"]
pub type PRODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RACC_SPEC, PRODIS_A, O>;
impl<'a, const O: u8> PRODIS_W<'a, O> {
    #[doc = "Frames with wrong checksum are not discarded."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PRODIS_A::_0)
    }
    #[doc = "If a TCP/IP, UDP/IP, or ICMP/IP frame is received that has a wrong TCP, UDP, or ICMP checksum, the frame is discarded. Discarding is only available when the RX FIFO operates in store and forward mode (RSFL cleared)."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PRODIS_A::_1)
    }
}
#[doc = "Field `LINEDIS` reader - Enable Discard Of Frames With MAC Layer Errors"]
pub type LINEDIS_R = crate::BitReader<LINEDIS_A>;
#[doc = "Enable Discard Of Frames With MAC Layer Errors\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINEDIS_A {
    #[doc = "0: Frames with errors are not discarded."]
    _0 = 0,
    #[doc = "1: Any frame received with a CRC, length, or PHY error is automatically discarded and not forwarded to the user application interface."]
    _1 = 1,
}
impl From<LINEDIS_A> for bool {
    #[inline(always)]
    fn from(variant: LINEDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl LINEDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINEDIS_A {
        match self.bits {
            false => LINEDIS_A::_0,
            true => LINEDIS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LINEDIS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LINEDIS_A::_1
    }
}
#[doc = "Field `LINEDIS` writer - Enable Discard Of Frames With MAC Layer Errors"]
pub type LINEDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RACC_SPEC, LINEDIS_A, O>;
impl<'a, const O: u8> LINEDIS_W<'a, O> {
    #[doc = "Frames with errors are not discarded."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LINEDIS_A::_0)
    }
    #[doc = "Any frame received with a CRC, length, or PHY error is automatically discarded and not forwarded to the user application interface."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LINEDIS_A::_1)
    }
}
#[doc = "Field `SHIFT16` reader - RX FIFO Shift-16"]
pub type SHIFT16_R = crate::BitReader<SHIFT16_A>;
#[doc = "RX FIFO Shift-16\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHIFT16_A {
    #[doc = "0: Disabled."]
    _0 = 0,
    #[doc = "1: Instructs the MAC to write two additional bytes in front of each frame received into the RX FIFO."]
    _1 = 1,
}
impl From<SHIFT16_A> for bool {
    #[inline(always)]
    fn from(variant: SHIFT16_A) -> Self {
        variant as u8 != 0
    }
}
impl SHIFT16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHIFT16_A {
        match self.bits {
            false => SHIFT16_A::_0,
            true => SHIFT16_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHIFT16_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHIFT16_A::_1
    }
}
#[doc = "Field `SHIFT16` writer - RX FIFO Shift-16"]
pub type SHIFT16_W<'a, const O: u8> = crate::BitWriter<'a, u32, RACC_SPEC, SHIFT16_A, O>;
impl<'a, const O: u8> SHIFT16_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHIFT16_A::_0)
    }
    #[doc = "Instructs the MAC to write two additional bytes in front of each frame received into the RX FIFO."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHIFT16_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Padding Removal For Short IP Frames"]
    #[inline(always)]
    pub fn padrem(&self) -> PADREM_R {
        PADREM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Discard Of Frames With Wrong IPv4 Header Checksum"]
    #[inline(always)]
    pub fn ipdis(&self) -> IPDIS_R {
        IPDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Discard Of Frames With Wrong Protocol Checksum"]
    #[inline(always)]
    pub fn prodis(&self) -> PRODIS_R {
        PRODIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Discard Of Frames With MAC Layer Errors"]
    #[inline(always)]
    pub fn linedis(&self) -> LINEDIS_R {
        LINEDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX FIFO Shift-16"]
    #[inline(always)]
    pub fn shift16(&self) -> SHIFT16_R {
        SHIFT16_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Padding Removal For Short IP Frames"]
    #[inline(always)]
    #[must_use]
    pub fn padrem(&mut self) -> PADREM_W<0> {
        PADREM_W::new(self)
    }
    #[doc = "Bit 1 - Enable Discard Of Frames With Wrong IPv4 Header Checksum"]
    #[inline(always)]
    #[must_use]
    pub fn ipdis(&mut self) -> IPDIS_W<1> {
        IPDIS_W::new(self)
    }
    #[doc = "Bit 2 - Enable Discard Of Frames With Wrong Protocol Checksum"]
    #[inline(always)]
    #[must_use]
    pub fn prodis(&mut self) -> PRODIS_W<2> {
        PRODIS_W::new(self)
    }
    #[doc = "Bit 6 - Enable Discard Of Frames With MAC Layer Errors"]
    #[inline(always)]
    #[must_use]
    pub fn linedis(&mut self) -> LINEDIS_W<6> {
        LINEDIS_W::new(self)
    }
    #[doc = "Bit 7 - RX FIFO Shift-16"]
    #[inline(always)]
    #[must_use]
    pub fn shift16(&mut self) -> SHIFT16_W<7> {
        SHIFT16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Accelerator Function Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [racc](index.html) module"]
pub struct RACC_SPEC;
impl crate::RegisterSpec for RACC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [racc::R](R) reader structure"]
impl crate::Readable for RACC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [racc::W](W) writer structure"]
impl crate::Writable for RACC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RACC to value 0"]
impl crate::Resettable for RACC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
