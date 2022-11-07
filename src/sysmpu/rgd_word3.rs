#[doc = "Register `RGD%s_WORD3` reader"]
pub struct R(crate::R<RGD_WORD3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RGD_WORD3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RGD_WORD3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RGD_WORD3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RGD%s_WORD3` writer"]
pub struct W(crate::W<RGD_WORD3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RGD_WORD3_SPEC>;
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
impl From<crate::W<RGD_WORD3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RGD_WORD3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLD` reader - Valid"]
pub type VLD_R = crate::BitReader<VLD_A>;
#[doc = "Valid\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VLD_A {
    #[doc = "0: Region descriptor is invalid"]
    _0 = 0,
    #[doc = "1: Region descriptor is valid"]
    _1 = 1,
}
impl From<VLD_A> for bool {
    #[inline(always)]
    fn from(variant: VLD_A) -> Self {
        variant as u8 != 0
    }
}
impl VLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLD_A {
        match self.bits {
            false => VLD_A::_0,
            true => VLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VLD_A::_1
    }
}
#[doc = "Field `VLD` writer - Valid"]
pub type VLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RGD_WORD3_SPEC, VLD_A, O>;
impl<'a, const O: u8> VLD_W<'a, O> {
    #[doc = "Region descriptor is invalid"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(VLD_A::_0)
    }
    #[doc = "Region descriptor is valid"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(VLD_A::_1)
    }
}
#[doc = "Field `PIDMASK` reader - Process Identifier Mask"]
pub type PIDMASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIDMASK` writer - Process Identifier Mask"]
pub type PIDMASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGD_WORD3_SPEC, u8, u8, 8, O>;
#[doc = "Field `PID` reader - Process Identifier"]
pub type PID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PID` writer - Process Identifier"]
pub type PID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RGD_WORD3_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    pub fn vld(&self) -> VLD_R {
        VLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - Process Identifier Mask"]
    #[inline(always)]
    pub fn pidmask(&self) -> PIDMASK_R {
        PIDMASK_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Process Identifier"]
    #[inline(always)]
    pub fn pid(&self) -> PID_R {
        PID_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    #[must_use]
    pub fn vld(&mut self) -> VLD_W<0> {
        VLD_W::new(self)
    }
    #[doc = "Bits 16:23 - Process Identifier Mask"]
    #[inline(always)]
    #[must_use]
    pub fn pidmask(&mut self) -> PIDMASK_W<16> {
        PIDMASK_W::new(self)
    }
    #[doc = "Bits 24:31 - Process Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn pid(&mut self) -> PID_W<24> {
        PID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Region Descriptor n, Word 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rgd_word3](index.html) module"]
pub struct RGD_WORD3_SPEC;
impl crate::RegisterSpec for RGD_WORD3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rgd_word3::R](R) reader structure"]
impl crate::Readable for RGD_WORD3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rgd_word3::W](W) writer structure"]
impl crate::Writable for RGD_WORD3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RGD%s_WORD3 to value 0x01"]
impl crate::Resettable for RGD_WORD3_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
