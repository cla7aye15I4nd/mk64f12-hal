#[doc = "Register `MMFR` reader"]
pub struct R(crate::R<MMFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MMFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MMFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MMFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MMFR` writer"]
pub struct W(crate::W<MMFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MMFR_SPEC>;
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
impl From<crate::W<MMFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MMFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Management Frame Data"]
pub type DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA` writer - Management Frame Data"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMFR_SPEC, u16, u16, 16, O>;
#[doc = "Field `TA` reader - Turn Around"]
pub type TA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TA` writer - Turn Around"]
pub type TA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMFR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RA` reader - Register Address"]
pub type RA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RA` writer - Register Address"]
pub type RA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMFR_SPEC, u8, u8, 5, O>;
#[doc = "Field `PA` reader - PHY Address"]
pub type PA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PA` writer - PHY Address"]
pub type PA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMFR_SPEC, u8, u8, 5, O>;
#[doc = "Field `OP` reader - Operation Code"]
pub type OP_R = crate::FieldReader<u8, OP_A>;
#[doc = "Operation Code\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OP_A {
    #[doc = "0: Write frame operation, but not MII compliant."]
    _00 = 0,
    #[doc = "1: Write frame operation for a valid MII management frame."]
    _01 = 1,
    #[doc = "2: Read frame operation for a valid MII management frame."]
    _10 = 2,
    #[doc = "3: Read frame operation, but not MII compliant."]
    _11 = 3,
}
impl From<OP_A> for u8 {
    #[inline(always)]
    fn from(variant: OP_A) -> Self {
        variant as _
    }
}
impl OP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OP_A {
        match self.bits {
            0 => OP_A::_00,
            1 => OP_A::_01,
            2 => OP_A::_10,
            3 => OP_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OP_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OP_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OP_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OP_A::_11
    }
}
#[doc = "Field `OP` writer - Operation Code"]
pub type OP_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MMFR_SPEC, u8, OP_A, 2, O>;
impl<'a, const O: u8> OP_W<'a, O> {
    #[doc = "Write frame operation, but not MII compliant."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OP_A::_00)
    }
    #[doc = "Write frame operation for a valid MII management frame."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OP_A::_01)
    }
    #[doc = "Read frame operation for a valid MII management frame."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OP_A::_10)
    }
    #[doc = "Read frame operation, but not MII compliant."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OP_A::_11)
    }
}
#[doc = "Field `ST` reader - Start Of Frame Delimiter"]
pub type ST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ST` writer - Start Of Frame Delimiter"]
pub type ST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MMFR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:15 - Management Frame Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Turn Around"]
    #[inline(always)]
    pub fn ta(&self) -> TA_R {
        TA_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline(always)]
    pub fn ra(&self) -> RA_R {
        RA_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline(always)]
    pub fn pa(&self) -> PA_R {
        PA_R::new(((self.bits >> 23) & 0x1f) as u8)
    }
    #[doc = "Bits 28:29 - Operation Code"]
    #[inline(always)]
    pub fn op(&self) -> OP_R {
        OP_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Start Of Frame Delimiter"]
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Management Frame Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Bits 16:17 - Turn Around"]
    #[inline(always)]
    #[must_use]
    pub fn ta(&mut self) -> TA_W<16> {
        TA_W::new(self)
    }
    #[doc = "Bits 18:22 - Register Address"]
    #[inline(always)]
    #[must_use]
    pub fn ra(&mut self) -> RA_W<18> {
        RA_W::new(self)
    }
    #[doc = "Bits 23:27 - PHY Address"]
    #[inline(always)]
    #[must_use]
    pub fn pa(&mut self) -> PA_W<23> {
        PA_W::new(self)
    }
    #[doc = "Bits 28:29 - Operation Code"]
    #[inline(always)]
    #[must_use]
    pub fn op(&mut self) -> OP_W<28> {
        OP_W::new(self)
    }
    #[doc = "Bits 30:31 - Start Of Frame Delimiter"]
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<30> {
        ST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MII Management Frame Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mmfr](index.html) module"]
pub struct MMFR_SPEC;
impl crate::RegisterSpec for MMFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mmfr::R](R) reader structure"]
impl crate::Readable for MMFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mmfr::W](W) writer structure"]
impl crate::Writable for MMFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MMFR to value 0"]
impl crate::Resettable for MMFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
