#[doc = "Register `SMB` reader"]
pub struct R(crate::R<SMB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMB` writer"]
pub struct W(crate::W<SMB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMB_SPEC>;
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
impl From<crate::W<SMB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHTF2IE` reader - SHTF2 Interrupt Enable"]
pub type SHTF2IE_R = crate::BitReader<SHTF2IE_A>;
#[doc = "SHTF2 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHTF2IE_A {
    #[doc = "0: SHTF2 interrupt is disabled"]
    _0 = 0,
    #[doc = "1: SHTF2 interrupt is enabled"]
    _1 = 1,
}
impl From<SHTF2IE_A> for bool {
    #[inline(always)]
    fn from(variant: SHTF2IE_A) -> Self {
        variant as u8 != 0
    }
}
impl SHTF2IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHTF2IE_A {
        match self.bits {
            false => SHTF2IE_A::_0,
            true => SHTF2IE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHTF2IE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHTF2IE_A::_1
    }
}
#[doc = "Field `SHTF2IE` writer - SHTF2 Interrupt Enable"]
pub type SHTF2IE_W<'a, const O: u8> = crate::BitWriter<'a, u8, SMB_SPEC, SHTF2IE_A, O>;
impl<'a, const O: u8> SHTF2IE_W<'a, O> {
    #[doc = "SHTF2 interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHTF2IE_A::_0)
    }
    #[doc = "SHTF2 interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHTF2IE_A::_1)
    }
}
#[doc = "Field `SHTF2` reader - SCL High Timeout Flag 2"]
pub type SHTF2_R = crate::BitReader<SHTF2_A>;
#[doc = "SCL High Timeout Flag 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHTF2_A {
    #[doc = "0: No SCL high and SDA low timeout occurs"]
    _0 = 0,
    #[doc = "1: SCL high and SDA low timeout occurs"]
    _1 = 1,
}
impl From<SHTF2_A> for bool {
    #[inline(always)]
    fn from(variant: SHTF2_A) -> Self {
        variant as u8 != 0
    }
}
impl SHTF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHTF2_A {
        match self.bits {
            false => SHTF2_A::_0,
            true => SHTF2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHTF2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHTF2_A::_1
    }
}
#[doc = "Field `SHTF2` writer - SCL High Timeout Flag 2"]
pub type SHTF2_W<'a, const O: u8> = crate::BitWriter<'a, u8, SMB_SPEC, SHTF2_A, O>;
impl<'a, const O: u8> SHTF2_W<'a, O> {
    #[doc = "No SCL high and SDA low timeout occurs"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SHTF2_A::_0)
    }
    #[doc = "SCL high and SDA low timeout occurs"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SHTF2_A::_1)
    }
}
#[doc = "Field `SHTF1` reader - SCL High Timeout Flag 1"]
pub type SHTF1_R = crate::BitReader<SHTF1_A>;
#[doc = "SCL High Timeout Flag 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SHTF1_A {
    #[doc = "0: No SCL high and SDA high timeout occurs"]
    _0 = 0,
    #[doc = "1: SCL high and SDA high timeout occurs"]
    _1 = 1,
}
impl From<SHTF1_A> for bool {
    #[inline(always)]
    fn from(variant: SHTF1_A) -> Self {
        variant as u8 != 0
    }
}
impl SHTF1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SHTF1_A {
        match self.bits {
            false => SHTF1_A::_0,
            true => SHTF1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SHTF1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SHTF1_A::_1
    }
}
#[doc = "Field `SLTF` reader - SCL Low Timeout Flag"]
pub type SLTF_R = crate::BitReader<SLTF_A>;
#[doc = "SCL Low Timeout Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLTF_A {
    #[doc = "0: No low timeout occurs"]
    _0 = 0,
    #[doc = "1: Low timeout occurs"]
    _1 = 1,
}
impl From<SLTF_A> for bool {
    #[inline(always)]
    fn from(variant: SLTF_A) -> Self {
        variant as u8 != 0
    }
}
impl SLTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLTF_A {
        match self.bits {
            false => SLTF_A::_0,
            true => SLTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLTF_A::_1
    }
}
#[doc = "Field `SLTF` writer - SCL Low Timeout Flag"]
pub type SLTF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SMB_SPEC, SLTF_A, O>;
impl<'a, const O: u8> SLTF_W<'a, O> {
    #[doc = "No low timeout occurs"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLTF_A::_0)
    }
    #[doc = "Low timeout occurs"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLTF_A::_1)
    }
}
#[doc = "Field `TCKSEL` reader - Timeout Counter Clock Select"]
pub type TCKSEL_R = crate::BitReader<TCKSEL_A>;
#[doc = "Timeout Counter Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCKSEL_A {
    #[doc = "0: Timeout counter counts at the frequency of the I2C module clock / 64"]
    _0 = 0,
    #[doc = "1: Timeout counter counts at the frequency of the I2C module clock"]
    _1 = 1,
}
impl From<TCKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TCKSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl TCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCKSEL_A {
        match self.bits {
            false => TCKSEL_A::_0,
            true => TCKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCKSEL_A::_1
    }
}
#[doc = "Field `TCKSEL` writer - Timeout Counter Clock Select"]
pub type TCKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u8, SMB_SPEC, TCKSEL_A, O>;
impl<'a, const O: u8> TCKSEL_W<'a, O> {
    #[doc = "Timeout counter counts at the frequency of the I2C module clock / 64"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCKSEL_A::_0)
    }
    #[doc = "Timeout counter counts at the frequency of the I2C module clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCKSEL_A::_1)
    }
}
#[doc = "Field `SIICAEN` reader - Second I2C Address Enable"]
pub type SIICAEN_R = crate::BitReader<SIICAEN_A>;
#[doc = "Second I2C Address Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIICAEN_A {
    #[doc = "0: I2C address register 2 matching is disabled"]
    _0 = 0,
    #[doc = "1: I2C address register 2 matching is enabled"]
    _1 = 1,
}
impl From<SIICAEN_A> for bool {
    #[inline(always)]
    fn from(variant: SIICAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SIICAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIICAEN_A {
        match self.bits {
            false => SIICAEN_A::_0,
            true => SIICAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SIICAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SIICAEN_A::_1
    }
}
#[doc = "Field `SIICAEN` writer - Second I2C Address Enable"]
pub type SIICAEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SMB_SPEC, SIICAEN_A, O>;
impl<'a, const O: u8> SIICAEN_W<'a, O> {
    #[doc = "I2C address register 2 matching is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SIICAEN_A::_0)
    }
    #[doc = "I2C address register 2 matching is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SIICAEN_A::_1)
    }
}
#[doc = "Field `ALERTEN` reader - SMBus Alert Response Address Enable"]
pub type ALERTEN_R = crate::BitReader<ALERTEN_A>;
#[doc = "SMBus Alert Response Address Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALERTEN_A {
    #[doc = "0: SMBus alert response address matching is disabled"]
    _0 = 0,
    #[doc = "1: SMBus alert response address matching is enabled"]
    _1 = 1,
}
impl From<ALERTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ALERTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ALERTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALERTEN_A {
        match self.bits {
            false => ALERTEN_A::_0,
            true => ALERTEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ALERTEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ALERTEN_A::_1
    }
}
#[doc = "Field `ALERTEN` writer - SMBus Alert Response Address Enable"]
pub type ALERTEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SMB_SPEC, ALERTEN_A, O>;
impl<'a, const O: u8> ALERTEN_W<'a, O> {
    #[doc = "SMBus alert response address matching is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ALERTEN_A::_0)
    }
    #[doc = "SMBus alert response address matching is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ALERTEN_A::_1)
    }
}
#[doc = "Field `FACK` reader - Fast NACK/ACK Enable"]
pub type FACK_R = crate::BitReader<FACK_A>;
#[doc = "Fast NACK/ACK Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FACK_A {
    #[doc = "0: An ACK or NACK is sent on the following receiving data byte"]
    _0 = 0,
    #[doc = "1: Writing 0 to TXAK after receiving a data byte generates an ACK. Writing 1 to TXAK after receiving a data byte generates a NACK."]
    _1 = 1,
}
impl From<FACK_A> for bool {
    #[inline(always)]
    fn from(variant: FACK_A) -> Self {
        variant as u8 != 0
    }
}
impl FACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FACK_A {
        match self.bits {
            false => FACK_A::_0,
            true => FACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FACK_A::_1
    }
}
#[doc = "Field `FACK` writer - Fast NACK/ACK Enable"]
pub type FACK_W<'a, const O: u8> = crate::BitWriter<'a, u8, SMB_SPEC, FACK_A, O>;
impl<'a, const O: u8> FACK_W<'a, O> {
    #[doc = "An ACK or NACK is sent on the following receiving data byte"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FACK_A::_0)
    }
    #[doc = "Writing 0 to TXAK after receiving a data byte generates an ACK. Writing 1 to TXAK after receiving a data byte generates a NACK."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FACK_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - SHTF2 Interrupt Enable"]
    #[inline(always)]
    pub fn shtf2ie(&self) -> SHTF2IE_R {
        SHTF2IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCL High Timeout Flag 2"]
    #[inline(always)]
    pub fn shtf2(&self) -> SHTF2_R {
        SHTF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCL High Timeout Flag 1"]
    #[inline(always)]
    pub fn shtf1(&self) -> SHTF1_R {
        SHTF1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCL Low Timeout Flag"]
    #[inline(always)]
    pub fn sltf(&self) -> SLTF_R {
        SLTF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timeout Counter Clock Select"]
    #[inline(always)]
    pub fn tcksel(&self) -> TCKSEL_R {
        TCKSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Second I2C Address Enable"]
    #[inline(always)]
    pub fn siicaen(&self) -> SIICAEN_R {
        SIICAEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SMBus Alert Response Address Enable"]
    #[inline(always)]
    pub fn alerten(&self) -> ALERTEN_R {
        ALERTEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Fast NACK/ACK Enable"]
    #[inline(always)]
    pub fn fack(&self) -> FACK_R {
        FACK_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SHTF2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn shtf2ie(&mut self) -> SHTF2IE_W<0> {
        SHTF2IE_W::new(self)
    }
    #[doc = "Bit 1 - SCL High Timeout Flag 2"]
    #[inline(always)]
    #[must_use]
    pub fn shtf2(&mut self) -> SHTF2_W<1> {
        SHTF2_W::new(self)
    }
    #[doc = "Bit 3 - SCL Low Timeout Flag"]
    #[inline(always)]
    #[must_use]
    pub fn sltf(&mut self) -> SLTF_W<3> {
        SLTF_W::new(self)
    }
    #[doc = "Bit 4 - Timeout Counter Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn tcksel(&mut self) -> TCKSEL_W<4> {
        TCKSEL_W::new(self)
    }
    #[doc = "Bit 5 - Second I2C Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn siicaen(&mut self) -> SIICAEN_W<5> {
        SIICAEN_W::new(self)
    }
    #[doc = "Bit 6 - SMBus Alert Response Address Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alerten(&mut self) -> ALERTEN_W<6> {
        ALERTEN_W::new(self)
    }
    #[doc = "Bit 7 - Fast NACK/ACK Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fack(&mut self) -> FACK_W<7> {
        FACK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C SMBus Control and Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smb](index.html) module"]
pub struct SMB_SPEC;
impl crate::RegisterSpec for SMB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [smb::R](R) reader structure"]
impl crate::Readable for SMB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smb::W](W) writer structure"]
impl crate::Writable for SMB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMB to value 0"]
impl crate::Resettable for SMB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
