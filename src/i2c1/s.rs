#[doc = "Register `S` reader"]
pub struct R(crate::R<S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S` writer"]
pub struct W(crate::W<S_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S_SPEC>;
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
impl From<crate::W<S_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXAK` reader - Receive Acknowledge"]
pub type RXAK_R = crate::BitReader<RXAK_A>;
#[doc = "Receive Acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXAK_A {
    #[doc = "0: Acknowledge signal was received after the completion of one byte of data transmission on the bus"]
    _0 = 0,
    #[doc = "1: No acknowledge signal detected"]
    _1 = 1,
}
impl From<RXAK_A> for bool {
    #[inline(always)]
    fn from(variant: RXAK_A) -> Self {
        variant as u8 != 0
    }
}
impl RXAK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXAK_A {
        match self.bits {
            false => RXAK_A::_0,
            true => RXAK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RXAK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RXAK_A::_1
    }
}
#[doc = "Field `IICIF` reader - Interrupt Flag"]
pub type IICIF_R = crate::BitReader<IICIF_A>;
#[doc = "Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IICIF_A {
    #[doc = "0: No interrupt pending"]
    _0 = 0,
    #[doc = "1: Interrupt pending"]
    _1 = 1,
}
impl From<IICIF_A> for bool {
    #[inline(always)]
    fn from(variant: IICIF_A) -> Self {
        variant as u8 != 0
    }
}
impl IICIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IICIF_A {
        match self.bits {
            false => IICIF_A::_0,
            true => IICIF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IICIF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IICIF_A::_1
    }
}
#[doc = "Field `IICIF` writer - Interrupt Flag"]
pub type IICIF_W<'a, const O: u8> = crate::BitWriter<'a, u8, S_SPEC, IICIF_A, O>;
impl<'a, const O: u8> IICIF_W<'a, O> {
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IICIF_A::_0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IICIF_A::_1)
    }
}
#[doc = "Field `SRW` reader - Slave Read/Write"]
pub type SRW_R = crate::BitReader<SRW_A>;
#[doc = "Slave Read/Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRW_A {
    #[doc = "0: Slave receive, master writing to slave"]
    _0 = 0,
    #[doc = "1: Slave transmit, master reading from slave"]
    _1 = 1,
}
impl From<SRW_A> for bool {
    #[inline(always)]
    fn from(variant: SRW_A) -> Self {
        variant as u8 != 0
    }
}
impl SRW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRW_A {
        match self.bits {
            false => SRW_A::_0,
            true => SRW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SRW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SRW_A::_1
    }
}
#[doc = "Field `RAM` reader - Range Address Match"]
pub type RAM_R = crate::BitReader<RAM_A>;
#[doc = "Range Address Match\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RAM_A {
    #[doc = "0: Not addressed"]
    _0 = 0,
    #[doc = "1: Addressed as a slave"]
    _1 = 1,
}
impl From<RAM_A> for bool {
    #[inline(always)]
    fn from(variant: RAM_A) -> Self {
        variant as u8 != 0
    }
}
impl RAM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RAM_A {
        match self.bits {
            false => RAM_A::_0,
            true => RAM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RAM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RAM_A::_1
    }
}
#[doc = "Field `RAM` writer - Range Address Match"]
pub type RAM_W<'a, const O: u8> = crate::BitWriter<'a, u8, S_SPEC, RAM_A, O>;
impl<'a, const O: u8> RAM_W<'a, O> {
    #[doc = "Not addressed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RAM_A::_0)
    }
    #[doc = "Addressed as a slave"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RAM_A::_1)
    }
}
#[doc = "Field `ARBL` reader - Arbitration Lost"]
pub type ARBL_R = crate::BitReader<ARBL_A>;
#[doc = "Arbitration Lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARBL_A {
    #[doc = "0: Standard bus operation."]
    _0 = 0,
    #[doc = "1: Loss of arbitration."]
    _1 = 1,
}
impl From<ARBL_A> for bool {
    #[inline(always)]
    fn from(variant: ARBL_A) -> Self {
        variant as u8 != 0
    }
}
impl ARBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARBL_A {
        match self.bits {
            false => ARBL_A::_0,
            true => ARBL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ARBL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ARBL_A::_1
    }
}
#[doc = "Field `ARBL` writer - Arbitration Lost"]
pub type ARBL_W<'a, const O: u8> = crate::BitWriter<'a, u8, S_SPEC, ARBL_A, O>;
impl<'a, const O: u8> ARBL_W<'a, O> {
    #[doc = "Standard bus operation."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ARBL_A::_0)
    }
    #[doc = "Loss of arbitration."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ARBL_A::_1)
    }
}
#[doc = "Field `BUSY` reader - Bus Busy"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "Bus Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: Bus is idle"]
    _0 = 0,
    #[doc = "1: Bus is busy"]
    _1 = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::_0,
            true => BUSY_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BUSY_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BUSY_A::_1
    }
}
#[doc = "Field `IAAS` reader - Addressed As A Slave"]
pub type IAAS_R = crate::BitReader<IAAS_A>;
#[doc = "Addressed As A Slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IAAS_A {
    #[doc = "0: Not addressed"]
    _0 = 0,
    #[doc = "1: Addressed as a slave"]
    _1 = 1,
}
impl From<IAAS_A> for bool {
    #[inline(always)]
    fn from(variant: IAAS_A) -> Self {
        variant as u8 != 0
    }
}
impl IAAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IAAS_A {
        match self.bits {
            false => IAAS_A::_0,
            true => IAAS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IAAS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IAAS_A::_1
    }
}
#[doc = "Field `IAAS` writer - Addressed As A Slave"]
pub type IAAS_W<'a, const O: u8> = crate::BitWriter<'a, u8, S_SPEC, IAAS_A, O>;
impl<'a, const O: u8> IAAS_W<'a, O> {
    #[doc = "Not addressed"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IAAS_A::_0)
    }
    #[doc = "Addressed as a slave"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IAAS_A::_1)
    }
}
#[doc = "Field `TCF` reader - Transfer Complete Flag"]
pub type TCF_R = crate::BitReader<TCF_A>;
#[doc = "Transfer Complete Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCF_A {
    #[doc = "0: Transfer in progress"]
    _0 = 0,
    #[doc = "1: Transfer complete"]
    _1 = 1,
}
impl From<TCF_A> for bool {
    #[inline(always)]
    fn from(variant: TCF_A) -> Self {
        variant as u8 != 0
    }
}
impl TCF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCF_A {
        match self.bits {
            false => TCF_A::_0,
            true => TCF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Receive Acknowledge"]
    #[inline(always)]
    pub fn rxak(&self) -> RXAK_R {
        RXAK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Flag"]
    #[inline(always)]
    pub fn iicif(&self) -> IICIF_R {
        IICIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slave Read/Write"]
    #[inline(always)]
    pub fn srw(&self) -> SRW_R {
        SRW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Range Address Match"]
    #[inline(always)]
    pub fn ram(&self) -> RAM_R {
        RAM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Arbitration Lost"]
    #[inline(always)]
    pub fn arbl(&self) -> ARBL_R {
        ARBL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Addressed As A Slave"]
    #[inline(always)]
    pub fn iaas(&self) -> IAAS_R {
        IAAS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transfer Complete Flag"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn iicif(&mut self) -> IICIF_W<1> {
        IICIF_W::new(self)
    }
    #[doc = "Bit 3 - Range Address Match"]
    #[inline(always)]
    #[must_use]
    pub fn ram(&mut self) -> RAM_W<3> {
        RAM_W::new(self)
    }
    #[doc = "Bit 4 - Arbitration Lost"]
    #[inline(always)]
    #[must_use]
    pub fn arbl(&mut self) -> ARBL_W<4> {
        ARBL_W::new(self)
    }
    #[doc = "Bit 6 - Addressed As A Slave"]
    #[inline(always)]
    #[must_use]
    pub fn iaas(&mut self) -> IAAS_W<6> {
        IAAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s](index.html) module"]
pub struct S_SPEC;
impl crate::RegisterSpec for S_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [s::R](R) reader structure"]
impl crate::Readable for S_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s::W](W) writer structure"]
impl crate::Writable for S_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets S to value 0x80"]
impl crate::Resettable for S_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
