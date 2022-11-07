#[doc = "Register `AIRCR` reader"]
pub struct R(crate::R<AIRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AIRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AIRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AIRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AIRCR` writer"]
pub struct W(crate::W<AIRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AIRCR_SPEC>;
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
impl From<crate::W<AIRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AIRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VECTRESET` writer - no description available"]
pub type VECTRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIRCR_SPEC, bool, O>;
#[doc = "Field `VECTCLRACTIVE` writer - no description available"]
pub type VECTCLRACTIVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIRCR_SPEC, bool, O>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSRESETREQ_AW {
    #[doc = "0: no system reset request"]
    _0 = 0,
    #[doc = "1: asserts a signal to the outer system that requests a reset"]
    _1 = 1,
}
impl From<SYSRESETREQ_AW> for bool {
    #[inline(always)]
    fn from(variant: SYSRESETREQ_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSRESETREQ` writer - no description available"]
pub type SYSRESETREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, AIRCR_SPEC, SYSRESETREQ_AW, O>;
impl<'a, const O: u8> SYSRESETREQ_W<'a, O> {
    #[doc = "no system reset request"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SYSRESETREQ_AW::_0)
    }
    #[doc = "asserts a signal to the outer system that requests a reset"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SYSRESETREQ_AW::_1)
    }
}
#[doc = "Field `PRIGROUP` reader - Interrupt priority grouping field. This field determines the split of group priority from subpriority."]
pub type PRIGROUP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIGROUP` writer - Interrupt priority grouping field. This field determines the split of group priority from subpriority."]
pub type PRIGROUP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AIRCR_SPEC, u8, u8, 3, O>;
#[doc = "Field `ENDIANNESS` reader - no description available"]
pub type ENDIANNESS_R = crate::BitReader<ENDIANNESS_A>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENDIANNESS_A {
    #[doc = "0: Little-endian"]
    _0 = 0,
    #[doc = "1: Big-endian"]
    _1 = 1,
}
impl From<ENDIANNESS_A> for bool {
    #[inline(always)]
    fn from(variant: ENDIANNESS_A) -> Self {
        variant as u8 != 0
    }
}
impl ENDIANNESS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENDIANNESS_A {
        match self.bits {
            false => ENDIANNESS_A::_0,
            true => ENDIANNESS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENDIANNESS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENDIANNESS_A::_1
    }
}
#[doc = "Field `VECTKEY` reader - Register key"]
pub type VECTKEY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VECTKEY` writer - Register key"]
pub type VECTKEY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AIRCR_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 8:10 - Interrupt priority grouping field. This field determines the split of group priority from subpriority."]
    #[inline(always)]
    pub fn prigroup(&self) -> PRIGROUP_R {
        PRIGROUP_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn endianness(&self) -> ENDIANNESS_R {
        ENDIANNESS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline(always)]
    pub fn vectkey(&self) -> VECTKEY_R {
        VECTKEY_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn vectreset(&mut self) -> VECTRESET_W<0> {
        VECTRESET_W::new(self)
    }
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn vectclractive(&mut self) -> VECTCLRACTIVE_W<1> {
        VECTCLRACTIVE_W::new(self)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn sysresetreq(&mut self) -> SYSRESETREQ_W<2> {
        SYSRESETREQ_W::new(self)
    }
    #[doc = "Bits 8:10 - Interrupt priority grouping field. This field determines the split of group priority from subpriority."]
    #[inline(always)]
    #[must_use]
    pub fn prigroup(&mut self) -> PRIGROUP_W<8> {
        PRIGROUP_W::new(self)
    }
    #[doc = "Bits 16:31 - Register key"]
    #[inline(always)]
    #[must_use]
    pub fn vectkey(&mut self) -> VECTKEY_W<16> {
        VECTKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Application Interrupt and Reset Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aircr](index.html) module"]
pub struct AIRCR_SPEC;
impl crate::RegisterSpec for AIRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aircr::R](R) reader structure"]
impl crate::Readable for AIRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aircr::W](W) writer structure"]
impl crate::Writable for AIRCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AIRCR to value 0xfa05_0000"]
impl crate::Resettable for AIRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0xfa05_0000;
}
