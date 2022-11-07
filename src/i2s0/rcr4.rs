#[doc = "Register `RCR4` reader"]
pub struct R(crate::R<RCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCR4` writer"]
pub struct W(crate::W<RCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCR4_SPEC>;
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
impl From<crate::W<RCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSD` reader - Frame Sync Direction"]
pub type FSD_R = crate::BitReader<FSD_A>;
#[doc = "Frame Sync Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSD_A {
    #[doc = "0: Frame Sync is generated externally in Slave mode."]
    _0 = 0,
    #[doc = "1: Frame Sync is generated internally in Master mode."]
    _1 = 1,
}
impl From<FSD_A> for bool {
    #[inline(always)]
    fn from(variant: FSD_A) -> Self {
        variant as u8 != 0
    }
}
impl FSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSD_A {
        match self.bits {
            false => FSD_A::_0,
            true => FSD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FSD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FSD_A::_1
    }
}
#[doc = "Field `FSD` writer - Frame Sync Direction"]
pub type FSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR4_SPEC, FSD_A, O>;
impl<'a, const O: u8> FSD_W<'a, O> {
    #[doc = "Frame Sync is generated externally in Slave mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSD_A::_0)
    }
    #[doc = "Frame Sync is generated internally in Master mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSD_A::_1)
    }
}
#[doc = "Field `FSP` reader - Frame Sync Polarity"]
pub type FSP_R = crate::BitReader<FSP_A>;
#[doc = "Frame Sync Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSP_A {
    #[doc = "0: Frame sync is active high."]
    _0 = 0,
    #[doc = "1: Frame sync is active low."]
    _1 = 1,
}
impl From<FSP_A> for bool {
    #[inline(always)]
    fn from(variant: FSP_A) -> Self {
        variant as u8 != 0
    }
}
impl FSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSP_A {
        match self.bits {
            false => FSP_A::_0,
            true => FSP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FSP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FSP_A::_1
    }
}
#[doc = "Field `FSP` writer - Frame Sync Polarity"]
pub type FSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR4_SPEC, FSP_A, O>;
impl<'a, const O: u8> FSP_W<'a, O> {
    #[doc = "Frame sync is active high."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSP_A::_0)
    }
    #[doc = "Frame sync is active low."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSP_A::_1)
    }
}
#[doc = "Field `FSE` reader - Frame Sync Early"]
pub type FSE_R = crate::BitReader<FSE_A>;
#[doc = "Frame Sync Early\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSE_A {
    #[doc = "0: Frame sync asserts with the first bit of the frame."]
    _0 = 0,
    #[doc = "1: Frame sync asserts one bit before the first bit of the frame."]
    _1 = 1,
}
impl From<FSE_A> for bool {
    #[inline(always)]
    fn from(variant: FSE_A) -> Self {
        variant as u8 != 0
    }
}
impl FSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSE_A {
        match self.bits {
            false => FSE_A::_0,
            true => FSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FSE_A::_1
    }
}
#[doc = "Field `FSE` writer - Frame Sync Early"]
pub type FSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR4_SPEC, FSE_A, O>;
impl<'a, const O: u8> FSE_W<'a, O> {
    #[doc = "Frame sync asserts with the first bit of the frame."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FSE_A::_0)
    }
    #[doc = "Frame sync asserts one bit before the first bit of the frame."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FSE_A::_1)
    }
}
#[doc = "Field `MF` reader - MSB First"]
pub type MF_R = crate::BitReader<MF_A>;
#[doc = "MSB First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MF_A {
    #[doc = "0: LSB is received first."]
    _0 = 0,
    #[doc = "1: MSB is received first."]
    _1 = 1,
}
impl From<MF_A> for bool {
    #[inline(always)]
    fn from(variant: MF_A) -> Self {
        variant as u8 != 0
    }
}
impl MF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MF_A {
        match self.bits {
            false => MF_A::_0,
            true => MF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MF_A::_1
    }
}
#[doc = "Field `MF` writer - MSB First"]
pub type MF_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR4_SPEC, MF_A, O>;
impl<'a, const O: u8> MF_W<'a, O> {
    #[doc = "LSB is received first."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(MF_A::_0)
    }
    #[doc = "MSB is received first."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(MF_A::_1)
    }
}
#[doc = "Field `SYWD` reader - Sync Width"]
pub type SYWD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYWD` writer - Sync Width"]
pub type SYWD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCR4_SPEC, u8, u8, 5, O>;
#[doc = "Field `FRSZ` reader - Frame Size"]
pub type FRSZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRSZ` writer - Frame Size"]
pub type FRSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCR4_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bit 0 - Frame Sync Direction"]
    #[inline(always)]
    pub fn fsd(&self) -> FSD_R {
        FSD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame Sync Polarity"]
    #[inline(always)]
    pub fn fsp(&self) -> FSP_R {
        FSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame Sync Early"]
    #[inline(always)]
    pub fn fse(&self) -> FSE_R {
        FSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MSB First"]
    #[inline(always)]
    pub fn mf(&self) -> MF_R {
        MF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Sync Width"]
    #[inline(always)]
    pub fn sywd(&self) -> SYWD_R {
        SYWD_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Frame Size"]
    #[inline(always)]
    pub fn frsz(&self) -> FRSZ_R {
        FRSZ_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Frame Sync Direction"]
    #[inline(always)]
    #[must_use]
    pub fn fsd(&mut self) -> FSD_W<0> {
        FSD_W::new(self)
    }
    #[doc = "Bit 1 - Frame Sync Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn fsp(&mut self) -> FSP_W<1> {
        FSP_W::new(self)
    }
    #[doc = "Bit 3 - Frame Sync Early"]
    #[inline(always)]
    #[must_use]
    pub fn fse(&mut self) -> FSE_W<3> {
        FSE_W::new(self)
    }
    #[doc = "Bit 4 - MSB First"]
    #[inline(always)]
    #[must_use]
    pub fn mf(&mut self) -> MF_W<4> {
        MF_W::new(self)
    }
    #[doc = "Bits 8:12 - Sync Width"]
    #[inline(always)]
    #[must_use]
    pub fn sywd(&mut self) -> SYWD_W<8> {
        SYWD_W::new(self)
    }
    #[doc = "Bits 16:20 - Frame Size"]
    #[inline(always)]
    #[must_use]
    pub fn frsz(&mut self) -> FRSZ_W<16> {
        FRSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SAI Receive Configuration 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr4](index.html) module"]
pub struct RCR4_SPEC;
impl crate::RegisterSpec for RCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcr4::R](R) reader structure"]
impl crate::Readable for RCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcr4::W](W) writer structure"]
impl crate::Writable for RCR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCR4 to value 0"]
impl crate::Resettable for RCR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
