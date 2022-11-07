#[doc = "Register `MSCR` reader"]
pub struct R(crate::R<MSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSCR` writer"]
pub struct W(crate::W<MSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSCR_SPEC>;
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
impl From<crate::W<MSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MII_SPEED` reader - MII Speed"]
pub type MII_SPEED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MII_SPEED` writer - MII Speed"]
pub type MII_SPEED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MSCR_SPEC, u8, u8, 6, O>;
#[doc = "Field `DIS_PRE` reader - Disable Preamble"]
pub type DIS_PRE_R = crate::BitReader<DIS_PRE_A>;
#[doc = "Disable Preamble\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIS_PRE_A {
    #[doc = "0: Preamble enabled."]
    _0 = 0,
    #[doc = "1: Preamble (32 ones) is not prepended to the MII management frame."]
    _1 = 1,
}
impl From<DIS_PRE_A> for bool {
    #[inline(always)]
    fn from(variant: DIS_PRE_A) -> Self {
        variant as u8 != 0
    }
}
impl DIS_PRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIS_PRE_A {
        match self.bits {
            false => DIS_PRE_A::_0,
            true => DIS_PRE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DIS_PRE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DIS_PRE_A::_1
    }
}
#[doc = "Field `DIS_PRE` writer - Disable Preamble"]
pub type DIS_PRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MSCR_SPEC, DIS_PRE_A, O>;
impl<'a, const O: u8> DIS_PRE_W<'a, O> {
    #[doc = "Preamble enabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DIS_PRE_A::_0)
    }
    #[doc = "Preamble (32 ones) is not prepended to the MII management frame."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DIS_PRE_A::_1)
    }
}
#[doc = "Field `HOLDTIME` reader - Hold time On MDIO Output"]
pub type HOLDTIME_R = crate::FieldReader<u8, HOLDTIME_A>;
#[doc = "Hold time On MDIO Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HOLDTIME_A {
    #[doc = "0: 1 internal module clock cycle"]
    _000 = 0,
    #[doc = "1: 2 internal module clock cycles"]
    _001 = 1,
    #[doc = "2: 3 internal module clock cycles"]
    _010 = 2,
    #[doc = "7: 8 internal module clock cycles"]
    _111 = 7,
}
impl From<HOLDTIME_A> for u8 {
    #[inline(always)]
    fn from(variant: HOLDTIME_A) -> Self {
        variant as _
    }
}
impl HOLDTIME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HOLDTIME_A> {
        match self.bits {
            0 => Some(HOLDTIME_A::_000),
            1 => Some(HOLDTIME_A::_001),
            2 => Some(HOLDTIME_A::_010),
            7 => Some(HOLDTIME_A::_111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == HOLDTIME_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == HOLDTIME_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == HOLDTIME_A::_010
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == HOLDTIME_A::_111
    }
}
#[doc = "Field `HOLDTIME` writer - Hold time On MDIO Output"]
pub type HOLDTIME_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MSCR_SPEC, u8, HOLDTIME_A, 3, O>;
impl<'a, const O: u8> HOLDTIME_W<'a, O> {
    #[doc = "1 internal module clock cycle"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(HOLDTIME_A::_000)
    }
    #[doc = "2 internal module clock cycles"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(HOLDTIME_A::_001)
    }
    #[doc = "3 internal module clock cycles"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(HOLDTIME_A::_010)
    }
    #[doc = "8 internal module clock cycles"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(HOLDTIME_A::_111)
    }
}
impl R {
    #[doc = "Bits 1:6 - MII Speed"]
    #[inline(always)]
    pub fn mii_speed(&self) -> MII_SPEED_R {
        MII_SPEED_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Disable Preamble"]
    #[inline(always)]
    pub fn dis_pre(&self) -> DIS_PRE_R {
        DIS_PRE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Hold time On MDIO Output"]
    #[inline(always)]
    pub fn holdtime(&self) -> HOLDTIME_R {
        HOLDTIME_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 1:6 - MII Speed"]
    #[inline(always)]
    #[must_use]
    pub fn mii_speed(&mut self) -> MII_SPEED_W<1> {
        MII_SPEED_W::new(self)
    }
    #[doc = "Bit 7 - Disable Preamble"]
    #[inline(always)]
    #[must_use]
    pub fn dis_pre(&mut self) -> DIS_PRE_W<7> {
        DIS_PRE_W::new(self)
    }
    #[doc = "Bits 8:10 - Hold time On MDIO Output"]
    #[inline(always)]
    #[must_use]
    pub fn holdtime(&mut self) -> HOLDTIME_W<8> {
        HOLDTIME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MII Speed Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mscr](index.html) module"]
pub struct MSCR_SPEC;
impl crate::RegisterSpec for MSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mscr::R](R) reader structure"]
impl crate::Readable for MSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mscr::W](W) writer structure"]
impl crate::Writable for MSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSCR to value 0"]
impl crate::Resettable for MSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
