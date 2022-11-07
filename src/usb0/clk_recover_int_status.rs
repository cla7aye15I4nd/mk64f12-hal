#[doc = "Register `CLK_RECOVER_INT_STATUS` reader"]
pub struct R(crate::R<CLK_RECOVER_INT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_RECOVER_INT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_RECOVER_INT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_RECOVER_INT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_RECOVER_INT_STATUS` writer"]
pub struct W(crate::W<CLK_RECOVER_INT_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_RECOVER_INT_STATUS_SPEC>;
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
impl From<crate::W<CLK_RECOVER_INT_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_RECOVER_INT_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVF_ERROR` reader - Indicates that the USB clock recovery algorithm has detected that the frequency trim adjustment needed for the IRC48M output clock is outside the available TRIM_FINE adjustment range for the IRC48M module"]
pub type OVF_ERROR_R = crate::BitReader<OVF_ERROR_A>;
#[doc = "Indicates that the USB clock recovery algorithm has detected that the frequency trim adjustment needed for the IRC48M output clock is outside the available TRIM_FINE adjustment range for the IRC48M module\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVF_ERROR_A {
    #[doc = "0: No interrupt is reported"]
    _0 = 0,
    #[doc = "1: Unmasked interrupt has been generated"]
    _1 = 1,
}
impl From<OVF_ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: OVF_ERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl OVF_ERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVF_ERROR_A {
        match self.bits {
            false => OVF_ERROR_A::_0,
            true => OVF_ERROR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVF_ERROR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVF_ERROR_A::_1
    }
}
#[doc = "Field `OVF_ERROR` writer - Indicates that the USB clock recovery algorithm has detected that the frequency trim adjustment needed for the IRC48M output clock is outside the available TRIM_FINE adjustment range for the IRC48M module"]
pub type OVF_ERROR_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, CLK_RECOVER_INT_STATUS_SPEC, OVF_ERROR_A, O>;
impl<'a, const O: u8> OVF_ERROR_W<'a, O> {
    #[doc = "No interrupt is reported"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OVF_ERROR_A::_0)
    }
    #[doc = "Unmasked interrupt has been generated"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OVF_ERROR_A::_1)
    }
}
impl R {
    #[doc = "Bit 4 - Indicates that the USB clock recovery algorithm has detected that the frequency trim adjustment needed for the IRC48M output clock is outside the available TRIM_FINE adjustment range for the IRC48M module"]
    #[inline(always)]
    pub fn ovf_error(&self) -> OVF_ERROR_R {
        OVF_ERROR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Indicates that the USB clock recovery algorithm has detected that the frequency trim adjustment needed for the IRC48M output clock is outside the available TRIM_FINE adjustment range for the IRC48M module"]
    #[inline(always)]
    #[must_use]
    pub fn ovf_error(&mut self) -> OVF_ERROR_W<4> {
        OVF_ERROR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock recovery separated interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_recover_int_status](index.html) module"]
pub struct CLK_RECOVER_INT_STATUS_SPEC;
impl crate::RegisterSpec for CLK_RECOVER_INT_STATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [clk_recover_int_status::R](R) reader structure"]
impl crate::Readable for CLK_RECOVER_INT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_recover_int_status::W](W) writer structure"]
impl crate::Writable for CLK_RECOVER_INT_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLK_RECOVER_INT_STATUS to value 0"]
impl crate::Resettable for CLK_RECOVER_INT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
