#[doc = "Register `OTGSTAT` reader"]
pub struct R(crate::R<OTGSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTGSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTGSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTGSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTGSTAT` writer"]
pub struct W(crate::W<OTGSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTGSTAT_SPEC>;
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
impl From<crate::W<OTGSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTGSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AVBUSVLD` reader - A VBUS Valid"]
pub type AVBUSVLD_R = crate::BitReader<AVBUSVLD_A>;
#[doc = "A VBUS Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVBUSVLD_A {
    #[doc = "0: The VBUS voltage is below the A VBUS Valid threshold."]
    _0 = 0,
    #[doc = "1: The VBUS voltage is above the A VBUS Valid threshold."]
    _1 = 1,
}
impl From<AVBUSVLD_A> for bool {
    #[inline(always)]
    fn from(variant: AVBUSVLD_A) -> Self {
        variant as u8 != 0
    }
}
impl AVBUSVLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVBUSVLD_A {
        match self.bits {
            false => AVBUSVLD_A::_0,
            true => AVBUSVLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AVBUSVLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AVBUSVLD_A::_1
    }
}
#[doc = "Field `AVBUSVLD` writer - A VBUS Valid"]
pub type AVBUSVLD_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGSTAT_SPEC, AVBUSVLD_A, O>;
impl<'a, const O: u8> AVBUSVLD_W<'a, O> {
    #[doc = "The VBUS voltage is below the A VBUS Valid threshold."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AVBUSVLD_A::_0)
    }
    #[doc = "The VBUS voltage is above the A VBUS Valid threshold."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AVBUSVLD_A::_1)
    }
}
#[doc = "Field `BSESSEND` reader - B Session End"]
pub type BSESSEND_R = crate::BitReader<BSESSEND_A>;
#[doc = "B Session End\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSESSEND_A {
    #[doc = "0: The VBUS voltage is above the B session end threshold."]
    _0 = 0,
    #[doc = "1: The VBUS voltage is below the B session end threshold."]
    _1 = 1,
}
impl From<BSESSEND_A> for bool {
    #[inline(always)]
    fn from(variant: BSESSEND_A) -> Self {
        variant as u8 != 0
    }
}
impl BSESSEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSESSEND_A {
        match self.bits {
            false => BSESSEND_A::_0,
            true => BSESSEND_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSESSEND_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSESSEND_A::_1
    }
}
#[doc = "Field `BSESSEND` writer - B Session End"]
pub type BSESSEND_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGSTAT_SPEC, BSESSEND_A, O>;
impl<'a, const O: u8> BSESSEND_W<'a, O> {
    #[doc = "The VBUS voltage is above the B session end threshold."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSESSEND_A::_0)
    }
    #[doc = "The VBUS voltage is below the B session end threshold."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSESSEND_A::_1)
    }
}
#[doc = "Field `SESS_VLD` reader - Session Valid"]
pub type SESS_VLD_R = crate::BitReader<SESS_VLD_A>;
#[doc = "Session Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SESS_VLD_A {
    #[doc = "0: The VBUS voltage is below the B session valid threshold"]
    _0 = 0,
    #[doc = "1: The VBUS voltage is above the B session valid threshold."]
    _1 = 1,
}
impl From<SESS_VLD_A> for bool {
    #[inline(always)]
    fn from(variant: SESS_VLD_A) -> Self {
        variant as u8 != 0
    }
}
impl SESS_VLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SESS_VLD_A {
        match self.bits {
            false => SESS_VLD_A::_0,
            true => SESS_VLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SESS_VLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SESS_VLD_A::_1
    }
}
#[doc = "Field `SESS_VLD` writer - Session Valid"]
pub type SESS_VLD_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGSTAT_SPEC, SESS_VLD_A, O>;
impl<'a, const O: u8> SESS_VLD_W<'a, O> {
    #[doc = "The VBUS voltage is below the B session valid threshold"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SESS_VLD_A::_0)
    }
    #[doc = "The VBUS voltage is above the B session valid threshold."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SESS_VLD_A::_1)
    }
}
#[doc = "Field `LINESTATESTABLE` reader - Indicates that the internal signals that control the LINE_STATE_CHG field of OTGISTAT are stable for at least 1 millisecond"]
pub type LINESTATESTABLE_R = crate::BitReader<LINESTATESTABLE_A>;
#[doc = "Indicates that the internal signals that control the LINE_STATE_CHG field of OTGISTAT are stable for at least 1 millisecond\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LINESTATESTABLE_A {
    #[doc = "0: The LINE_STAT_CHG bit is not yet stable."]
    _0 = 0,
    #[doc = "1: The LINE_STAT_CHG bit has been debounced and is stable."]
    _1 = 1,
}
impl From<LINESTATESTABLE_A> for bool {
    #[inline(always)]
    fn from(variant: LINESTATESTABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl LINESTATESTABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LINESTATESTABLE_A {
        match self.bits {
            false => LINESTATESTABLE_A::_0,
            true => LINESTATESTABLE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LINESTATESTABLE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LINESTATESTABLE_A::_1
    }
}
#[doc = "Field `LINESTATESTABLE` writer - Indicates that the internal signals that control the LINE_STATE_CHG field of OTGISTAT are stable for at least 1 millisecond"]
pub type LINESTATESTABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u8, OTGSTAT_SPEC, LINESTATESTABLE_A, O>;
impl<'a, const O: u8> LINESTATESTABLE_W<'a, O> {
    #[doc = "The LINE_STAT_CHG bit is not yet stable."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LINESTATESTABLE_A::_0)
    }
    #[doc = "The LINE_STAT_CHG bit has been debounced and is stable."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LINESTATESTABLE_A::_1)
    }
}
#[doc = "Field `ONEMSECEN` reader - This bit is reserved for the 1ms count, but it is not useful to software."]
pub type ONEMSECEN_R = crate::BitReader<bool>;
#[doc = "Field `ONEMSECEN` writer - This bit is reserved for the 1ms count, but it is not useful to software."]
pub type ONEMSECEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGSTAT_SPEC, bool, O>;
#[doc = "Field `ID` reader - Indicates the current state of the ID pin on the USB connector"]
pub type ID_R = crate::BitReader<ID_A>;
#[doc = "Indicates the current state of the ID pin on the USB connector\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ID_A {
    #[doc = "0: Indicates a Type A cable is plugged into the USB connector."]
    _0 = 0,
    #[doc = "1: Indicates no cable is attached or a Type B cable is plugged into the USB connector."]
    _1 = 1,
}
impl From<ID_A> for bool {
    #[inline(always)]
    fn from(variant: ID_A) -> Self {
        variant as u8 != 0
    }
}
impl ID_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ID_A {
        match self.bits {
            false => ID_A::_0,
            true => ID_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ID_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ID_A::_1
    }
}
#[doc = "Field `ID` writer - Indicates the current state of the ID pin on the USB connector"]
pub type ID_W<'a, const O: u8> = crate::BitWriter<'a, u8, OTGSTAT_SPEC, ID_A, O>;
impl<'a, const O: u8> ID_W<'a, O> {
    #[doc = "Indicates a Type A cable is plugged into the USB connector."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ID_A::_0)
    }
    #[doc = "Indicates no cable is attached or a Type B cable is plugged into the USB connector."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ID_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - A VBUS Valid"]
    #[inline(always)]
    pub fn avbusvld(&self) -> AVBUSVLD_R {
        AVBUSVLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - B Session End"]
    #[inline(always)]
    pub fn bsessend(&self) -> BSESSEND_R {
        BSESSEND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Session Valid"]
    #[inline(always)]
    pub fn sess_vld(&self) -> SESS_VLD_R {
        SESS_VLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates that the internal signals that control the LINE_STATE_CHG field of OTGISTAT are stable for at least 1 millisecond"]
    #[inline(always)]
    pub fn linestatestable(&self) -> LINESTATESTABLE_R {
        LINESTATESTABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit is reserved for the 1ms count, but it is not useful to software."]
    #[inline(always)]
    pub fn onemsecen(&self) -> ONEMSECEN_R {
        ONEMSECEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates the current state of the ID pin on the USB connector"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - A VBUS Valid"]
    #[inline(always)]
    #[must_use]
    pub fn avbusvld(&mut self) -> AVBUSVLD_W<0> {
        AVBUSVLD_W::new(self)
    }
    #[doc = "Bit 2 - B Session End"]
    #[inline(always)]
    #[must_use]
    pub fn bsessend(&mut self) -> BSESSEND_W<2> {
        BSESSEND_W::new(self)
    }
    #[doc = "Bit 3 - Session Valid"]
    #[inline(always)]
    #[must_use]
    pub fn sess_vld(&mut self) -> SESS_VLD_W<3> {
        SESS_VLD_W::new(self)
    }
    #[doc = "Bit 5 - Indicates that the internal signals that control the LINE_STATE_CHG field of OTGISTAT are stable for at least 1 millisecond"]
    #[inline(always)]
    #[must_use]
    pub fn linestatestable(&mut self) -> LINESTATESTABLE_W<5> {
        LINESTATESTABLE_W::new(self)
    }
    #[doc = "Bit 6 - This bit is reserved for the 1ms count, but it is not useful to software."]
    #[inline(always)]
    #[must_use]
    pub fn onemsecen(&mut self) -> ONEMSECEN_W<6> {
        ONEMSECEN_W::new(self)
    }
    #[doc = "Bit 7 - Indicates the current state of the ID pin on the USB connector"]
    #[inline(always)]
    #[must_use]
    pub fn id(&mut self) -> ID_W<7> {
        ID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otgstat](index.html) module"]
pub struct OTGSTAT_SPEC;
impl crate::RegisterSpec for OTGSTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [otgstat::R](R) reader structure"]
impl crate::Readable for OTGSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otgstat::W](W) writer structure"]
impl crate::Writable for OTGSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTGSTAT to value 0"]
impl crate::Resettable for OTGSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
