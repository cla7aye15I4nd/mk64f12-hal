#[doc = "Register `S2` reader"]
pub struct R(crate::R<S2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S2` writer"]
pub struct W(crate::W<S2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S2_SPEC>;
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
impl From<crate::W<S2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAF` reader - Receiver Active Flag"]
pub type RAF_R = crate::BitReader<RAF_A>;
#[doc = "Receiver Active Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAF_A {
    #[doc = "0: UART receiver idle/inactive waiting for a start bit."]
    _0 = 0,
    #[doc = "1: UART receiver active, RxD input not idle."]
    _1 = 1,
}
impl From<RAF_A> for bool {
    #[inline(always)]
    fn from(variant: RAF_A) -> Self {
        variant as u8 != 0
    }
}
impl RAF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAF_A {
        match self.bits {
            false => RAF_A::_0,
            true => RAF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RAF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RAF_A::_1
    }
}
#[doc = "Field `LBKDE` reader - LIN Break Detection Enable"]
pub type LBKDE_R = crate::BitReader<LBKDE_A>;
#[doc = "LIN Break Detection Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBKDE_A {
    #[doc = "0: Break character detection is disabled."]
    _0 = 0,
    #[doc = "1: Break character is detected at length of 11 bit times if C1\\[M\\]
= 0 or 12 bits time if C1\\[M\\]
= 1."]
    _1 = 1,
}
impl From<LBKDE_A> for bool {
    #[inline(always)]
    fn from(variant: LBKDE_A) -> Self {
        variant as u8 != 0
    }
}
impl LBKDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBKDE_A {
        match self.bits {
            false => LBKDE_A::_0,
            true => LBKDE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LBKDE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LBKDE_A::_1
    }
}
#[doc = "Field `LBKDE` writer - LIN Break Detection Enable"]
pub type LBKDE_W<'a, const O: u8> = crate::BitWriter<'a, u8, S2_SPEC, LBKDE_A, O>;
impl<'a, const O: u8> LBKDE_W<'a, O> {
    #[doc = "Break character detection is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDE_A::_0)
    }
    #[doc = "Break character is detected at length of 11 bit times if C1\\[M\\]
= 0 or 12 bits time if C1\\[M\\]
= 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDE_A::_1)
    }
}
#[doc = "Field `BRK13` reader - Break Transmit Character Length"]
pub type BRK13_R = crate::BitReader<BRK13_A>;
#[doc = "Break Transmit Character Length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BRK13_A {
    #[doc = "0: Break character is 10, 11, or 12 bits long."]
    _0 = 0,
    #[doc = "1: Break character is 13 or 14 bits long."]
    _1 = 1,
}
impl From<BRK13_A> for bool {
    #[inline(always)]
    fn from(variant: BRK13_A) -> Self {
        variant as u8 != 0
    }
}
impl BRK13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BRK13_A {
        match self.bits {
            false => BRK13_A::_0,
            true => BRK13_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BRK13_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BRK13_A::_1
    }
}
#[doc = "Field `BRK13` writer - Break Transmit Character Length"]
pub type BRK13_W<'a, const O: u8> = crate::BitWriter<'a, u8, S2_SPEC, BRK13_A, O>;
impl<'a, const O: u8> BRK13_W<'a, O> {
    #[doc = "Break character is 10, 11, or 12 bits long."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BRK13_A::_0)
    }
    #[doc = "Break character is 13 or 14 bits long."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BRK13_A::_1)
    }
}
#[doc = "Field `RWUID` reader - Receive Wakeup Idle Detect"]
pub type RWUID_R = crate::BitReader<RWUID_A>;
#[doc = "Receive Wakeup Idle Detect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWUID_A {
    #[doc = "0: S1\\[IDLE\\]
is not set upon detection of an idle character."]
    _0 = 0,
    #[doc = "1: S1\\[IDLE\\]
is set upon detection of an idle character."]
    _1 = 1,
}
impl From<RWUID_A> for bool {
    #[inline(always)]
    fn from(variant: RWUID_A) -> Self {
        variant as u8 != 0
    }
}
impl RWUID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWUID_A {
        match self.bits {
            false => RWUID_A::_0,
            true => RWUID_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RWUID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RWUID_A::_1
    }
}
#[doc = "Field `RWUID` writer - Receive Wakeup Idle Detect"]
pub type RWUID_W<'a, const O: u8> = crate::BitWriter<'a, u8, S2_SPEC, RWUID_A, O>;
impl<'a, const O: u8> RWUID_W<'a, O> {
    #[doc = "S1\\[IDLE\\]
is not set upon detection of an idle character."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RWUID_A::_0)
    }
    #[doc = "S1\\[IDLE\\]
is set upon detection of an idle character."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RWUID_A::_1)
    }
}
#[doc = "Field `RXINV` reader - Receive Data Inversion"]
pub type RXINV_R = crate::BitReader<RXINV_A>;
#[doc = "Receive Data Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXINV_A {
    #[doc = "0: Receive data is not inverted."]
    _0 = 0,
    #[doc = "1: Receive data is inverted."]
    _1 = 1,
}
impl From<RXINV_A> for bool {
    #[inline(always)]
    fn from(variant: RXINV_A) -> Self {
        variant as u8 != 0
    }
}
impl RXINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXINV_A {
        match self.bits {
            false => RXINV_A::_0,
            true => RXINV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXINV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXINV_A::_1
    }
}
#[doc = "Field `RXINV` writer - Receive Data Inversion"]
pub type RXINV_W<'a, const O: u8> = crate::BitWriter<'a, u8, S2_SPEC, RXINV_A, O>;
impl<'a, const O: u8> RXINV_W<'a, O> {
    #[doc = "Receive data is not inverted."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXINV_A::_0)
    }
    #[doc = "Receive data is inverted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXINV_A::_1)
    }
}
#[doc = "Field `MSBF` reader - Most Significant Bit First"]
pub type MSBF_R = crate::BitReader<MSBF_A>;
#[doc = "Most Significant Bit First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSBF_A {
    #[doc = "0: LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    _0 = 0,
    #[doc = "1: MSB (bit8, bit7 or bit6) is the first bit that is transmitted following the start bit, depending on the setting of C1\\[M\\]
and C1\\[PE\\]. Further, the first bit received after the start bit is identified as bit8, bit7, or bit6, depending on the setting of C1\\[M\\]
and C1\\[PE\\]."]
    _1 = 1,
}
impl From<MSBF_A> for bool {
    #[inline(always)]
    fn from(variant: MSBF_A) -> Self {
        variant as u8 != 0
    }
}
impl MSBF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSBF_A {
        match self.bits {
            false => MSBF_A::_0,
            true => MSBF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MSBF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MSBF_A::_1
    }
}
#[doc = "Field `MSBF` writer - Most Significant Bit First"]
pub type MSBF_W<'a, const O: u8> = crate::BitWriter<'a, u8, S2_SPEC, MSBF_A, O>;
impl<'a, const O: u8> MSBF_W<'a, O> {
    #[doc = "LSB (bit0) is the first bit that is transmitted following the start bit. Further, the first bit received after the start bit is identified as bit0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MSBF_A::_0)
    }
    #[doc = "MSB (bit8, bit7 or bit6) is the first bit that is transmitted following the start bit, depending on the setting of C1\\[M\\]
and C1\\[PE\\]. Further, the first bit received after the start bit is identified as bit8, bit7, or bit6, depending on the setting of C1\\[M\\]
and C1\\[PE\\]."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MSBF_A::_1)
    }
}
#[doc = "Field `RXEDGIF` reader - RxD Pin Active Edge Interrupt Flag"]
pub type RXEDGIF_R = crate::BitReader<RXEDGIF_A>;
#[doc = "RxD Pin Active Edge Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXEDGIF_A {
    #[doc = "0: No active edge on the receive pin has occurred."]
    _0 = 0,
    #[doc = "1: An active edge on the receive pin has occurred."]
    _1 = 1,
}
impl From<RXEDGIF_A> for bool {
    #[inline(always)]
    fn from(variant: RXEDGIF_A) -> Self {
        variant as u8 != 0
    }
}
impl RXEDGIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXEDGIF_A {
        match self.bits {
            false => RXEDGIF_A::_0,
            true => RXEDGIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXEDGIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXEDGIF_A::_1
    }
}
#[doc = "Field `RXEDGIF` writer - RxD Pin Active Edge Interrupt Flag"]
pub type RXEDGIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, S2_SPEC, RXEDGIF_A, O>;
impl<'a, const O: u8> RXEDGIF_W<'a, O> {
    #[doc = "No active edge on the receive pin has occurred."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RXEDGIF_A::_0)
    }
    #[doc = "An active edge on the receive pin has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RXEDGIF_A::_1)
    }
}
#[doc = "Field `LBKDIF` reader - LIN Break Detect Interrupt Flag"]
pub type LBKDIF_R = crate::BitReader<LBKDIF_A>;
#[doc = "LIN Break Detect Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LBKDIF_A {
    #[doc = "0: No LIN break character detected."]
    _0 = 0,
    #[doc = "1: LIN break character detected."]
    _1 = 1,
}
impl From<LBKDIF_A> for bool {
    #[inline(always)]
    fn from(variant: LBKDIF_A) -> Self {
        variant as u8 != 0
    }
}
impl LBKDIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LBKDIF_A {
        match self.bits {
            false => LBKDIF_A::_0,
            true => LBKDIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LBKDIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LBKDIF_A::_1
    }
}
#[doc = "Field `LBKDIF` writer - LIN Break Detect Interrupt Flag"]
pub type LBKDIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, S2_SPEC, LBKDIF_A, O>;
impl<'a, const O: u8> LBKDIF_W<'a, O> {
    #[doc = "No LIN break character detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LBKDIF_A::_0)
    }
    #[doc = "LIN break character detected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LBKDIF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Active Flag"]
    #[inline(always)]
    pub fn raf(&self) -> RAF_R {
        RAF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LIN Break Detection Enable"]
    #[inline(always)]
    pub fn lbkde(&self) -> LBKDE_R {
        LBKDE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Break Transmit Character Length"]
    #[inline(always)]
    pub fn brk13(&self) -> BRK13_R {
        BRK13_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive Wakeup Idle Detect"]
    #[inline(always)]
    pub fn rwuid(&self) -> RWUID_R {
        RWUID_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Data Inversion"]
    #[inline(always)]
    pub fn rxinv(&self) -> RXINV_R {
        RXINV_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Most Significant Bit First"]
    #[inline(always)]
    pub fn msbf(&self) -> MSBF_R {
        MSBF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RxD Pin Active Edge Interrupt Flag"]
    #[inline(always)]
    pub fn rxedgif(&self) -> RXEDGIF_R {
        RXEDGIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LIN Break Detect Interrupt Flag"]
    #[inline(always)]
    pub fn lbkdif(&self) -> LBKDIF_R {
        LBKDIF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - LIN Break Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lbkde(&mut self) -> LBKDE_W<1> {
        LBKDE_W::new(self)
    }
    #[doc = "Bit 2 - Break Transmit Character Length"]
    #[inline(always)]
    #[must_use]
    pub fn brk13(&mut self) -> BRK13_W<2> {
        BRK13_W::new(self)
    }
    #[doc = "Bit 3 - Receive Wakeup Idle Detect"]
    #[inline(always)]
    #[must_use]
    pub fn rwuid(&mut self) -> RWUID_W<3> {
        RWUID_W::new(self)
    }
    #[doc = "Bit 4 - Receive Data Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn rxinv(&mut self) -> RXINV_W<4> {
        RXINV_W::new(self)
    }
    #[doc = "Bit 5 - Most Significant Bit First"]
    #[inline(always)]
    #[must_use]
    pub fn msbf(&mut self) -> MSBF_W<5> {
        MSBF_W::new(self)
    }
    #[doc = "Bit 6 - RxD Pin Active Edge Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rxedgif(&mut self) -> RXEDGIF_W<6> {
        RXEDGIF_W::new(self)
    }
    #[doc = "Bit 7 - LIN Break Detect Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lbkdif(&mut self) -> LBKDIF_W<7> {
        LBKDIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Status Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s2](index.html) module"]
pub struct S2_SPEC;
impl crate::RegisterSpec for S2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [s2::R](R) reader structure"]
impl crate::Readable for S2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s2::W](W) writer structure"]
impl crate::Writable for S2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S2 to value 0"]
impl crate::Resettable for S2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
