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
#[doc = "Field `TCR` reader - Time Compensation Register"]
pub type TCR_R = crate::FieldReader<u8, TCR_A>;
#[doc = "Time Compensation Register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TCR_A {
    #[doc = "128: Time Prescaler Register overflows every 32896 clock cycles."]
    _10000000 = 128,
    #[doc = "255: Time Prescaler Register overflows every 32769 clock cycles."]
    _11111111 = 255,
    #[doc = "0: Time Prescaler Register overflows every 32768 clock cycles."]
    _0 = 0,
    #[doc = "1: Time Prescaler Register overflows every 32767 clock cycles."]
    _1 = 1,
    #[doc = "127: Time Prescaler Register overflows every 32641 clock cycles."]
    _1111111 = 127,
}
impl From<TCR_A> for u8 {
    #[inline(always)]
    fn from(variant: TCR_A) -> Self {
        variant as _
    }
}
impl TCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TCR_A> {
        match self.bits {
            128 => Some(TCR_A::_10000000),
            255 => Some(TCR_A::_11111111),
            0 => Some(TCR_A::_0),
            1 => Some(TCR_A::_1),
            127 => Some(TCR_A::_1111111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_10000000`"]
    #[inline(always)]
    pub fn is_10000000(&self) -> bool {
        *self == TCR_A::_10000000
    }
    #[doc = "Checks if the value of the field is `_11111111`"]
    #[inline(always)]
    pub fn is_11111111(&self) -> bool {
        *self == TCR_A::_11111111
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TCR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TCR_A::_1
    }
    #[doc = "Checks if the value of the field is `_1111111`"]
    #[inline(always)]
    pub fn is_1111111(&self) -> bool {
        *self == TCR_A::_1111111
    }
}
#[doc = "Field `TCR` writer - Time Compensation Register"]
pub type TCR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR_SPEC, u8, TCR_A, 8, O>;
impl<'a, const O: u8> TCR_W<'a, O> {
    #[doc = "Time Prescaler Register overflows every 32896 clock cycles."]
    #[inline(always)]
    pub fn _10000000(self) -> &'a mut W {
        self.variant(TCR_A::_10000000)
    }
    #[doc = "Time Prescaler Register overflows every 32769 clock cycles."]
    #[inline(always)]
    pub fn _11111111(self) -> &'a mut W {
        self.variant(TCR_A::_11111111)
    }
    #[doc = "Time Prescaler Register overflows every 32768 clock cycles."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TCR_A::_0)
    }
    #[doc = "Time Prescaler Register overflows every 32767 clock cycles."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TCR_A::_1)
    }
    #[doc = "Time Prescaler Register overflows every 32641 clock cycles."]
    #[inline(always)]
    pub fn _1111111(self) -> &'a mut W {
        self.variant(TCR_A::_1111111)
    }
}
#[doc = "Field `CIR` reader - Compensation Interval Register"]
pub type CIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CIR` writer - Compensation Interval Register"]
pub type CIR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `TCV` reader - Time Compensation Value"]
pub type TCV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CIC` reader - Compensation Interval Counter"]
pub type CIC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Time Compensation Register"]
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Compensation Interval Register"]
    #[inline(always)]
    pub fn cir(&self) -> CIR_R {
        CIR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Time Compensation Value"]
    #[inline(always)]
    pub fn tcv(&self) -> TCV_R {
        TCV_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Compensation Interval Counter"]
    #[inline(always)]
    pub fn cic(&self) -> CIC_R {
        CIC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Time Compensation Register"]
    #[inline(always)]
    #[must_use]
    pub fn tcr(&mut self) -> TCR_W<0> {
        TCR_W::new(self)
    }
    #[doc = "Bits 8:15 - Compensation Interval Register"]
    #[inline(always)]
    #[must_use]
    pub fn cir(&mut self) -> CIR_W<8> {
        CIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Time Compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr](index.html) module"]
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
