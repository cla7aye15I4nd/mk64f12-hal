#[doc = "Register `CS12` reader"]
pub struct R(crate::R<CS12_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CS12_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CS12_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CS12_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CS12` writer"]
pub struct W(crate::W<CS12_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CS12_SPEC>;
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
impl From<crate::W<CS12_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CS12_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIME_STAMP` reader - Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
pub type TIME_STAMP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIME_STAMP` writer - Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
pub type TIME_STAMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CS12_SPEC, u16, u16, 16, O>;
#[doc = "Field `DLC` reader - Length of the data to be stored/transmitted."]
pub type DLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLC` writer - Length of the data to be stored/transmitted."]
pub type DLC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CS12_SPEC, u8, u8, 4, O>;
#[doc = "Field `RTR` reader - Remote Transmission Request. One/zero for remote/data frame."]
pub type RTR_R = crate::BitReader<bool>;
#[doc = "Field `RTR` writer - Remote Transmission Request. One/zero for remote/data frame."]
pub type RTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS12_SPEC, bool, O>;
#[doc = "Field `IDE` reader - ID Extended. One/zero for extended/standard format frame."]
pub type IDE_R = crate::BitReader<bool>;
#[doc = "Field `IDE` writer - ID Extended. One/zero for extended/standard format frame."]
pub type IDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS12_SPEC, bool, O>;
#[doc = "Field `SRR` reader - Substitute Remote Request. Contains a fixed recessive bit."]
pub type SRR_R = crate::BitReader<bool>;
#[doc = "Field `SRR` writer - Substitute Remote Request. Contains a fixed recessive bit."]
pub type SRR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS12_SPEC, bool, O>;
#[doc = "Field `CODE` reader - Reserved"]
pub type CODE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CODE` writer - Reserved"]
pub type CODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CS12_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:15 - Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    #[inline(always)]
    pub fn time_stamp(&self) -> TIME_STAMP_R {
        TIME_STAMP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Length of the data to be stored/transmitted."]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Remote Transmission Request. One/zero for remote/data frame."]
    #[inline(always)]
    pub fn rtr(&self) -> RTR_R {
        RTR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ID Extended. One/zero for extended/standard format frame."]
    #[inline(always)]
    pub fn ide(&self) -> IDE_R {
        IDE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Substitute Remote Request. Contains a fixed recessive bit."]
    #[inline(always)]
    pub fn srr(&self) -> SRR_R {
        SRR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Reserved"]
    #[inline(always)]
    pub fn code(&self) -> CODE_R {
        CODE_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Free-Running Counter Time stamp. This 16-bit field is a copy of the Free-Running Timer, captured for Tx and Rx frames at the time when the beginning of the Identifier field appears on the CAN bus."]
    #[inline(always)]
    #[must_use]
    pub fn time_stamp(&mut self) -> TIME_STAMP_W<0> {
        TIME_STAMP_W::new(self)
    }
    #[doc = "Bits 16:19 - Length of the data to be stored/transmitted."]
    #[inline(always)]
    #[must_use]
    pub fn dlc(&mut self) -> DLC_W<16> {
        DLC_W::new(self)
    }
    #[doc = "Bit 20 - Remote Transmission Request. One/zero for remote/data frame."]
    #[inline(always)]
    #[must_use]
    pub fn rtr(&mut self) -> RTR_W<20> {
        RTR_W::new(self)
    }
    #[doc = "Bit 21 - ID Extended. One/zero for extended/standard format frame."]
    #[inline(always)]
    #[must_use]
    pub fn ide(&mut self) -> IDE_W<21> {
        IDE_W::new(self)
    }
    #[doc = "Bit 22 - Substitute Remote Request. Contains a fixed recessive bit."]
    #[inline(always)]
    #[must_use]
    pub fn srr(&mut self) -> SRR_W<22> {
        SRR_W::new(self)
    }
    #[doc = "Bits 24:27 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn code(&mut self) -> CODE_W<24> {
        CODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Buffer 12 CS Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs12](index.html) module"]
pub struct CS12_SPEC;
impl crate::RegisterSpec for CS12_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cs12::R](R) reader structure"]
impl crate::Readable for CS12_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cs12::W](W) writer structure"]
impl crate::Writable for CS12_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CS12 to value 0"]
impl crate::Resettable for CS12_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
