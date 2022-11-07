#[doc = "Register `CTL` reader"]
pub struct R(crate::R<CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTL` writer"]
pub struct W(crate::W<CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTL_SPEC>;
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
impl From<crate::W<CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USBENSOFEN` reader - USB Enable"]
pub type USBENSOFEN_R = crate::BitReader<USBENSOFEN_A>;
#[doc = "USB Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBENSOFEN_A {
    #[doc = "0: Disables the USB Module."]
    _0 = 0,
    #[doc = "1: Enables the USB Module."]
    _1 = 1,
}
impl From<USBENSOFEN_A> for bool {
    #[inline(always)]
    fn from(variant: USBENSOFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl USBENSOFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBENSOFEN_A {
        match self.bits {
            false => USBENSOFEN_A::_0,
            true => USBENSOFEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == USBENSOFEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == USBENSOFEN_A::_1
    }
}
#[doc = "Field `USBENSOFEN` writer - USB Enable"]
pub type USBENSOFEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTL_SPEC, USBENSOFEN_A, O>;
impl<'a, const O: u8> USBENSOFEN_W<'a, O> {
    #[doc = "Disables the USB Module."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(USBENSOFEN_A::_0)
    }
    #[doc = "Enables the USB Module."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(USBENSOFEN_A::_1)
    }
}
#[doc = "Field `ODDRST` reader - Setting this bit to 1 resets all the BDT ODD ping/pong fields to 0, which then specifies the EVEN BDT bank"]
pub type ODDRST_R = crate::BitReader<bool>;
#[doc = "Field `ODDRST` writer - Setting this bit to 1 resets all the BDT ODD ping/pong fields to 0, which then specifies the EVEN BDT bank"]
pub type ODDRST_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTL_SPEC, bool, O>;
#[doc = "Field `RESUME` reader - When set to 1 this bit enables the USB Module to execute resume signaling"]
pub type RESUME_R = crate::BitReader<bool>;
#[doc = "Field `RESUME` writer - When set to 1 this bit enables the USB Module to execute resume signaling"]
pub type RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTL_SPEC, bool, O>;
#[doc = "Field `HOSTMODEEN` reader - When set to 1, this bit enables the USB Module to operate in Host mode"]
pub type HOSTMODEEN_R = crate::BitReader<bool>;
#[doc = "Field `HOSTMODEEN` writer - When set to 1, this bit enables the USB Module to operate in Host mode"]
pub type HOSTMODEEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTL_SPEC, bool, O>;
#[doc = "Field `RESET` reader - Setting this bit enables the USB Module to generate USB reset signaling"]
pub type RESET_R = crate::BitReader<bool>;
#[doc = "Field `RESET` writer - Setting this bit enables the USB Module to generate USB reset signaling"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTL_SPEC, bool, O>;
#[doc = "Field `TXSUSPENDTOKENBUSY` reader - In Host mode, TOKEN_BUSY is set when the USB module is busy executing a USB token"]
pub type TXSUSPENDTOKENBUSY_R = crate::BitReader<bool>;
#[doc = "Field `TXSUSPENDTOKENBUSY` writer - In Host mode, TOKEN_BUSY is set when the USB module is busy executing a USB token"]
pub type TXSUSPENDTOKENBUSY_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTL_SPEC, bool, O>;
#[doc = "Field `SE0` reader - Live USB Single Ended Zero signal"]
pub type SE0_R = crate::BitReader<bool>;
#[doc = "Field `SE0` writer - Live USB Single Ended Zero signal"]
pub type SE0_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTL_SPEC, bool, O>;
#[doc = "Field `JSTATE` reader - Live USB differential receiver JSTATE signal"]
pub type JSTATE_R = crate::BitReader<bool>;
#[doc = "Field `JSTATE` writer - Live USB differential receiver JSTATE signal"]
pub type JSTATE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USB Enable"]
    #[inline(always)]
    pub fn usbensofen(&self) -> USBENSOFEN_R {
        USBENSOFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Setting this bit to 1 resets all the BDT ODD ping/pong fields to 0, which then specifies the EVEN BDT bank"]
    #[inline(always)]
    pub fn oddrst(&self) -> ODDRST_R {
        ODDRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When set to 1 this bit enables the USB Module to execute resume signaling"]
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set to 1, this bit enables the USB Module to operate in Host mode"]
    #[inline(always)]
    pub fn hostmodeen(&self) -> HOSTMODEEN_R {
        HOSTMODEEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Setting this bit enables the USB Module to generate USB reset signaling"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - In Host mode, TOKEN_BUSY is set when the USB module is busy executing a USB token"]
    #[inline(always)]
    pub fn txsuspendtokenbusy(&self) -> TXSUSPENDTOKENBUSY_R {
        TXSUSPENDTOKENBUSY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Live USB Single Ended Zero signal"]
    #[inline(always)]
    pub fn se0(&self) -> SE0_R {
        SE0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Live USB differential receiver JSTATE signal"]
    #[inline(always)]
    pub fn jstate(&self) -> JSTATE_R {
        JSTATE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Enable"]
    #[inline(always)]
    #[must_use]
    pub fn usbensofen(&mut self) -> USBENSOFEN_W<0> {
        USBENSOFEN_W::new(self)
    }
    #[doc = "Bit 1 - Setting this bit to 1 resets all the BDT ODD ping/pong fields to 0, which then specifies the EVEN BDT bank"]
    #[inline(always)]
    #[must_use]
    pub fn oddrst(&mut self) -> ODDRST_W<1> {
        ODDRST_W::new(self)
    }
    #[doc = "Bit 2 - When set to 1 this bit enables the USB Module to execute resume signaling"]
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<2> {
        RESUME_W::new(self)
    }
    #[doc = "Bit 3 - When set to 1, this bit enables the USB Module to operate in Host mode"]
    #[inline(always)]
    #[must_use]
    pub fn hostmodeen(&mut self) -> HOSTMODEEN_W<3> {
        HOSTMODEEN_W::new(self)
    }
    #[doc = "Bit 4 - Setting this bit enables the USB Module to generate USB reset signaling"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<4> {
        RESET_W::new(self)
    }
    #[doc = "Bit 5 - In Host mode, TOKEN_BUSY is set when the USB module is busy executing a USB token"]
    #[inline(always)]
    #[must_use]
    pub fn txsuspendtokenbusy(&mut self) -> TXSUSPENDTOKENBUSY_W<5> {
        TXSUSPENDTOKENBUSY_W::new(self)
    }
    #[doc = "Bit 6 - Live USB Single Ended Zero signal"]
    #[inline(always)]
    #[must_use]
    pub fn se0(&mut self) -> SE0_W<6> {
        SE0_W::new(self)
    }
    #[doc = "Bit 7 - Live USB differential receiver JSTATE signal"]
    #[inline(always)]
    #[must_use]
    pub fn jstate(&mut self) -> JSTATE_W<7> {
        JSTATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctl](index.html) module"]
pub struct CTL_SPEC;
impl crate::RegisterSpec for CTL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctl::R](R) reader structure"]
impl crate::Readable for CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctl::W](W) writer structure"]
impl crate::Writable for CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTL to value 0"]
impl crate::Resettable for CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
