#[doc = "Register `OTGICR` reader"]
pub struct R(crate::R<OTGICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTGICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTGICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTGICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTGICR` writer"]
pub struct W(crate::W<OTGICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTGICR_SPEC>;
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
impl From<crate::W<OTGICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTGICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVBUSEN` reader - A VBUS Valid Interrupt Enable"]
pub type AVBUSEN_R = crate::BitReader<AVBUSEN_A>;
#[doc = "A VBUS Valid Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVBUSEN_A {
    #[doc = "0: Disables the AVBUSCHG interrupt."]
    _0 = 0,
    #[doc = "1: Enables the AVBUSCHG interrupt."]
    _1 = 1,
}
impl From<AVBUSEN_A> for bool {
    #[inline(always)]
    fn from(variant: AVBUSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl AVBUSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVBUSEN_A {
        match self.bits {
            false => AVBUSEN_A::_0,
            true => AVBUSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AVBUSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AVBUSEN_A::_1
    }
}
#[doc = "Field `AVBUSEN` writer - A VBUS Valid Interrupt Enable"]
pub type AVBUSEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGICR_SPEC, AVBUSEN_A, O>;
impl<'a, const O: u8> AVBUSEN_W<'a, O> {
    #[doc = "Disables the AVBUSCHG interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVBUSEN_A::_0)
    }
    #[doc = "Enables the AVBUSCHG interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVBUSEN_A::_1)
    }
}
#[doc = "Field `BSESSEN` reader - B Session END Interrupt Enable"]
pub type BSESSEN_R = crate::BitReader<BSESSEN_A>;
#[doc = "B Session END Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSESSEN_A {
    #[doc = "0: Disables the B_SESS_CHG interrupt."]
    _0 = 0,
    #[doc = "1: Enables the B_SESS_CHG interrupt."]
    _1 = 1,
}
impl From<BSESSEN_A> for bool {
    #[inline(always)]
    fn from(variant: BSESSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BSESSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSESSEN_A {
        match self.bits {
            false => BSESSEN_A::_0,
            true => BSESSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSESSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSESSEN_A::_1
    }
}
#[doc = "Field `BSESSEN` writer - B Session END Interrupt Enable"]
pub type BSESSEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGICR_SPEC, BSESSEN_A, O>;
impl<'a, const O: u8> BSESSEN_W<'a, O> {
    #[doc = "Disables the B_SESS_CHG interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSESSEN_A::_0)
    }
    #[doc = "Enables the B_SESS_CHG interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSESSEN_A::_1)
    }
}
#[doc = "Field `SESSVLDEN` reader - Session Valid Interrupt Enable"]
pub type SESSVLDEN_R = crate::BitReader<SESSVLDEN_A>;
#[doc = "Session Valid Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SESSVLDEN_A {
    #[doc = "0: Disables the SESSVLDCHG interrupt."]
    _0 = 0,
    #[doc = "1: Enables the SESSVLDCHG interrupt."]
    _1 = 1,
}
impl From<SESSVLDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SESSVLDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SESSVLDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SESSVLDEN_A {
        match self.bits {
            false => SESSVLDEN_A::_0,
            true => SESSVLDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SESSVLDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SESSVLDEN_A::_1
    }
}
#[doc = "Field `SESSVLDEN` writer - Session Valid Interrupt Enable"]
pub type SESSVLDEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGICR_SPEC, SESSVLDEN_A, O>;
impl<'a, const O: u8> SESSVLDEN_W<'a, O> {
    #[doc = "Disables the SESSVLDCHG interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SESSVLDEN_A::_0)
    }
    #[doc = "Enables the SESSVLDCHG interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SESSVLDEN_A::_1)
    }
}
#[doc = "Field `LINESTATEEN` reader - Line State Change Interrupt Enable"]
pub type LINESTATEEN_R = crate::BitReader<LINESTATEEN_A>;
#[doc = "Line State Change Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINESTATEEN_A {
    #[doc = "0: Disables the LINE_STAT_CHG interrupt."]
    _0 = 0,
    #[doc = "1: Enables the LINE_STAT_CHG interrupt."]
    _1 = 1,
}
impl From<LINESTATEEN_A> for bool {
    #[inline(always)]
    fn from(variant: LINESTATEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl LINESTATEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINESTATEEN_A {
        match self.bits {
            false => LINESTATEEN_A::_0,
            true => LINESTATEEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LINESTATEEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LINESTATEEN_A::_1
    }
}
#[doc = "Field `LINESTATEEN` writer - Line State Change Interrupt Enable"]
pub type LINESTATEEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGICR_SPEC, LINESTATEEN_A, O>;
impl<'a, const O: u8> LINESTATEEN_W<'a, O> {
    #[doc = "Disables the LINE_STAT_CHG interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LINESTATEEN_A::_0)
    }
    #[doc = "Enables the LINE_STAT_CHG interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LINESTATEEN_A::_1)
    }
}
#[doc = "Field `ONEMSECEN` reader - One Millisecond Interrupt Enable"]
pub type ONEMSECEN_R = crate::BitReader<ONEMSECEN_A>;
#[doc = "One Millisecond Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONEMSECEN_A {
    #[doc = "0: Diables the 1ms timer interrupt."]
    _0 = 0,
    #[doc = "1: Enables the 1ms timer interrupt."]
    _1 = 1,
}
impl From<ONEMSECEN_A> for bool {
    #[inline(always)]
    fn from(variant: ONEMSECEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ONEMSECEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONEMSECEN_A {
        match self.bits {
            false => ONEMSECEN_A::_0,
            true => ONEMSECEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ONEMSECEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ONEMSECEN_A::_1
    }
}
#[doc = "Field `ONEMSECEN` writer - One Millisecond Interrupt Enable"]
pub type ONEMSECEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGICR_SPEC, ONEMSECEN_A, O>;
impl<'a, const O: u8> ONEMSECEN_W<'a, O> {
    #[doc = "Diables the 1ms timer interrupt."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ONEMSECEN_A::_0)
    }
    #[doc = "Enables the 1ms timer interrupt."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ONEMSECEN_A::_1)
    }
}
#[doc = "Field `IDEN` reader - ID Interrupt Enable"]
pub type IDEN_R = crate::BitReader<IDEN_A>;
#[doc = "ID Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDEN_A {
    #[doc = "0: The ID interrupt is disabled"]
    _0 = 0,
    #[doc = "1: The ID interrupt is enabled"]
    _1 = 1,
}
impl From<IDEN_A> for bool {
    #[inline(always)]
    fn from(variant: IDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDEN_A {
        match self.bits {
            false => IDEN_A::_0,
            true => IDEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IDEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IDEN_A::_1
    }
}
#[doc = "Field `IDEN` writer - ID Interrupt Enable"]
pub type IDEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGICR_SPEC, IDEN_A, O>;
impl<'a, const O: u8> IDEN_W<'a, O> {
    #[doc = "The ID interrupt is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IDEN_A::_0)
    }
    #[doc = "The ID interrupt is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IDEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - A VBUS Valid Interrupt Enable"]
    #[inline(always)]
    pub fn avbusen(&self) -> AVBUSEN_R {
        AVBUSEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - B Session END Interrupt Enable"]
    #[inline(always)]
    pub fn bsessen(&self) -> BSESSEN_R {
        BSESSEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Session Valid Interrupt Enable"]
    #[inline(always)]
    pub fn sessvlden(&self) -> SESSVLDEN_R {
        SESSVLDEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Line State Change Interrupt Enable"]
    #[inline(always)]
    pub fn linestateen(&self) -> LINESTATEEN_R {
        LINESTATEEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - One Millisecond Interrupt Enable"]
    #[inline(always)]
    pub fn onemsecen(&self) -> ONEMSECEN_R {
        ONEMSECEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ID Interrupt Enable"]
    #[inline(always)]
    pub fn iden(&self) -> IDEN_R {
        IDEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A VBUS Valid Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn avbusen(&mut self) -> AVBUSEN_W<0> {
        AVBUSEN_W::new(self)
    }
    #[doc = "Bit 2 - B Session END Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bsessen(&mut self) -> BSESSEN_W<2> {
        BSESSEN_W::new(self)
    }
    #[doc = "Bit 3 - Session Valid Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sessvlden(&mut self) -> SESSVLDEN_W<3> {
        SESSVLDEN_W::new(self)
    }
    #[doc = "Bit 5 - Line State Change Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn linestateen(&mut self) -> LINESTATEEN_W<5> {
        LINESTATEEN_W::new(self)
    }
    #[doc = "Bit 6 - One Millisecond Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn onemsecen(&mut self) -> ONEMSECEN_W<6> {
        ONEMSECEN_W::new(self)
    }
    #[doc = "Bit 7 - ID Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iden(&mut self) -> IDEN_W<7> {
        IDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG Interrupt Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otgicr](index.html) module"]
pub struct OTGICR_SPEC;
impl crate::RegisterSpec for OTGICR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [otgicr::R](R) reader structure"]
impl crate::Readable for OTGICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otgicr::W](W) writer structure"]
impl crate::Writable for OTGICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTGICR to value 0"]
impl crate::Resettable for OTGICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
