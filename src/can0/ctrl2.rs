#[doc = "Register `CTRL2` reader"]
pub struct R(crate::R<CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2` writer"]
pub struct W(crate::W<CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2_SPEC>;
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
impl From<crate::W<CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EACEN` reader - Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
pub type EACEN_R = crate::BitReader<EACEN_A>;
#[doc = "Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EACEN_A {
    #[doc = "0: Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    _0 = 0,
    #[doc = "1: Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    _1 = 1,
}
impl From<EACEN_A> for bool {
    #[inline(always)]
    fn from(variant: EACEN_A) -> Self {
        variant as u8 != 0
    }
}
impl EACEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EACEN_A {
        match self.bits {
            false => EACEN_A::_0,
            true => EACEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EACEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EACEN_A::_1
    }
}
#[doc = "Field `EACEN` writer - Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
pub type EACEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL2_SPEC, EACEN_A, O>;
impl<'a, const O: u8> EACEN_W<'a, O> {
    #[doc = "Rx Mailbox filter's IDE bit is always compared and RTR is never compared despite mask bits."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EACEN_A::_0)
    }
    #[doc = "Enables the comparison of both Rx Mailbox filter's IDE and RTR bit with their corresponding bits within the incoming frame. Mask bits do apply."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EACEN_A::_1)
    }
}
#[doc = "Field `RRS` reader - Remote Request Storing"]
pub type RRS_R = crate::BitReader<RRS_A>;
#[doc = "Remote Request Storing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRS_A {
    #[doc = "0: Remote Response Frame is generated."]
    _0 = 0,
    #[doc = "1: Remote Request Frame is stored."]
    _1 = 1,
}
impl From<RRS_A> for bool {
    #[inline(always)]
    fn from(variant: RRS_A) -> Self {
        variant as u8 != 0
    }
}
impl RRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRS_A {
        match self.bits {
            false => RRS_A::_0,
            true => RRS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RRS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RRS_A::_1
    }
}
#[doc = "Field `RRS` writer - Remote Request Storing"]
pub type RRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL2_SPEC, RRS_A, O>;
impl<'a, const O: u8> RRS_W<'a, O> {
    #[doc = "Remote Response Frame is generated."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RRS_A::_0)
    }
    #[doc = "Remote Request Frame is stored."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RRS_A::_1)
    }
}
#[doc = "Field `MRP` reader - Mailboxes Reception Priority"]
pub type MRP_R = crate::BitReader<MRP_A>;
#[doc = "Mailboxes Reception Priority\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRP_A {
    #[doc = "0: Matching starts from Rx FIFO and continues on Mailboxes."]
    _0 = 0,
    #[doc = "1: Matching starts from Mailboxes and continues on Rx FIFO."]
    _1 = 1,
}
impl From<MRP_A> for bool {
    #[inline(always)]
    fn from(variant: MRP_A) -> Self {
        variant as u8 != 0
    }
}
impl MRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MRP_A {
        match self.bits {
            false => MRP_A::_0,
            true => MRP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MRP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MRP_A::_1
    }
}
#[doc = "Field `MRP` writer - Mailboxes Reception Priority"]
pub type MRP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL2_SPEC, MRP_A, O>;
impl<'a, const O: u8> MRP_W<'a, O> {
    #[doc = "Matching starts from Rx FIFO and continues on Mailboxes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MRP_A::_0)
    }
    #[doc = "Matching starts from Mailboxes and continues on Rx FIFO."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MRP_A::_1)
    }
}
#[doc = "Field `TASD` reader - Tx Arbitration Start Delay"]
pub type TASD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TASD` writer - Tx Arbitration Start Delay"]
pub type TASD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL2_SPEC, u8, u8, 5, O>;
#[doc = "Field `RFFN` reader - Number Of Rx FIFO Filters"]
pub type RFFN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RFFN` writer - Number Of Rx FIFO Filters"]
pub type RFFN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL2_SPEC, u8, u8, 4, O>;
#[doc = "Field `WRMFRZ` reader - Write-Access To Memory In Freeze Mode"]
pub type WRMFRZ_R = crate::BitReader<WRMFRZ_A>;
#[doc = "Write-Access To Memory In Freeze Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WRMFRZ_A {
    #[doc = "0: Maintain the write access restrictions."]
    _0 = 0,
    #[doc = "1: Enable unrestricted write access to FlexCAN memory."]
    _1 = 1,
}
impl From<WRMFRZ_A> for bool {
    #[inline(always)]
    fn from(variant: WRMFRZ_A) -> Self {
        variant as u8 != 0
    }
}
impl WRMFRZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRMFRZ_A {
        match self.bits {
            false => WRMFRZ_A::_0,
            true => WRMFRZ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WRMFRZ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WRMFRZ_A::_1
    }
}
#[doc = "Field `WRMFRZ` writer - Write-Access To Memory In Freeze Mode"]
pub type WRMFRZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL2_SPEC, WRMFRZ_A, O>;
impl<'a, const O: u8> WRMFRZ_W<'a, O> {
    #[doc = "Maintain the write access restrictions."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WRMFRZ_A::_0)
    }
    #[doc = "Enable unrestricted write access to FlexCAN memory."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WRMFRZ_A::_1)
    }
}
impl R {
    #[doc = "Bit 16 - Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
    #[inline(always)]
    pub fn eacen(&self) -> EACEN_R {
        EACEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Remote Request Storing"]
    #[inline(always)]
    pub fn rrs(&self) -> RRS_R {
        RRS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Mailboxes Reception Priority"]
    #[inline(always)]
    pub fn mrp(&self) -> MRP_R {
        MRP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:23 - Tx Arbitration Start Delay"]
    #[inline(always)]
    pub fn tasd(&self) -> TASD_R {
        TASD_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:27 - Number Of Rx FIFO Filters"]
    #[inline(always)]
    pub fn rffn(&self) -> RFFN_R {
        RFFN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Write-Access To Memory In Freeze Mode"]
    #[inline(always)]
    pub fn wrmfrz(&self) -> WRMFRZ_R {
        WRMFRZ_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Entire Frame Arbitration Field Comparison Enable For Rx Mailboxes"]
    #[inline(always)]
    #[must_use]
    pub fn eacen(&mut self) -> EACEN_W<16> {
        EACEN_W::new(self)
    }
    #[doc = "Bit 17 - Remote Request Storing"]
    #[inline(always)]
    #[must_use]
    pub fn rrs(&mut self) -> RRS_W<17> {
        RRS_W::new(self)
    }
    #[doc = "Bit 18 - Mailboxes Reception Priority"]
    #[inline(always)]
    #[must_use]
    pub fn mrp(&mut self) -> MRP_W<18> {
        MRP_W::new(self)
    }
    #[doc = "Bits 19:23 - Tx Arbitration Start Delay"]
    #[inline(always)]
    #[must_use]
    pub fn tasd(&mut self) -> TASD_W<19> {
        TASD_W::new(self)
    }
    #[doc = "Bits 24:27 - Number Of Rx FIFO Filters"]
    #[inline(always)]
    #[must_use]
    pub fn rffn(&mut self) -> RFFN_W<24> {
        RFFN_W::new(self)
    }
    #[doc = "Bit 28 - Write-Access To Memory In Freeze Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wrmfrz(&mut self) -> WRMFRZ_W<28> {
        WRMFRZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](index.html) module"]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl2::R](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL2 to value 0x00b0_0000"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0x00b0_0000;
}
