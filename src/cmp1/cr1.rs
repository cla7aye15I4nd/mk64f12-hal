#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Comparator Module Enable"]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "Comparator Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: Analog Comparator is disabled."]
    _0 = 0,
    #[doc = "1: Analog Comparator is enabled."]
    _1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::_0,
            true => EN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN_A::_1
    }
}
#[doc = "Field `EN` writer - Comparator Module Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR1_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Analog Comparator is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN_A::_0)
    }
    #[doc = "Analog Comparator is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN_A::_1)
    }
}
#[doc = "Field `OPE` reader - Comparator Output Pin Enable"]
pub type OPE_R = crate::BitReader<OPE_A>;
#[doc = "Comparator Output Pin Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPE_A {
    #[doc = "0: CMPO is not available on the associated CMPO output pin. If the comparator does not own the pin, this field has no effect."]
    _0 = 0,
    #[doc = "1: CMPO is available on the associated CMPO output pin. The comparator output (CMPO) is driven out on the associated CMPO output pin if the comparator owns the pin. If the comparator does not own the field, this bit has no effect."]
    _1 = 1,
}
impl From<OPE_A> for bool {
    #[inline(always)]
    fn from(variant: OPE_A) -> Self {
        variant as u8 != 0
    }
}
impl OPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPE_A {
        match self.bits {
            false => OPE_A::_0,
            true => OPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OPE_A::_1
    }
}
#[doc = "Field `OPE` writer - Comparator Output Pin Enable"]
pub type OPE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR1_SPEC, OPE_A, O>;
impl<'a, const O: u8> OPE_W<'a, O> {
    #[doc = "CMPO is not available on the associated CMPO output pin. If the comparator does not own the pin, this field has no effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(OPE_A::_0)
    }
    #[doc = "CMPO is available on the associated CMPO output pin. The comparator output (CMPO) is driven out on the associated CMPO output pin if the comparator owns the pin. If the comparator does not own the field, this bit has no effect."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(OPE_A::_1)
    }
}
#[doc = "Field `COS` reader - Comparator Output Select"]
pub type COS_R = crate::BitReader<COS_A>;
#[doc = "Comparator Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COS_A {
    #[doc = "0: Set the filtered comparator output (CMPO) to equal COUT."]
    _0 = 0,
    #[doc = "1: Set the unfiltered comparator output (CMPO) to equal COUTA."]
    _1 = 1,
}
impl From<COS_A> for bool {
    #[inline(always)]
    fn from(variant: COS_A) -> Self {
        variant as u8 != 0
    }
}
impl COS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COS_A {
        match self.bits {
            false => COS_A::_0,
            true => COS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == COS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == COS_A::_1
    }
}
#[doc = "Field `COS` writer - Comparator Output Select"]
pub type COS_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR1_SPEC, COS_A, O>;
impl<'a, const O: u8> COS_W<'a, O> {
    #[doc = "Set the filtered comparator output (CMPO) to equal COUT."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(COS_A::_0)
    }
    #[doc = "Set the unfiltered comparator output (CMPO) to equal COUTA."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(COS_A::_1)
    }
}
#[doc = "Field `INV` reader - Comparator INVERT"]
pub type INV_R = crate::BitReader<INV_A>;
#[doc = "Comparator INVERT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INV_A {
    #[doc = "0: Does not invert the comparator output."]
    _0 = 0,
    #[doc = "1: Inverts the comparator output."]
    _1 = 1,
}
impl From<INV_A> for bool {
    #[inline(always)]
    fn from(variant: INV_A) -> Self {
        variant as u8 != 0
    }
}
impl INV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INV_A {
        match self.bits {
            false => INV_A::_0,
            true => INV_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == INV_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == INV_A::_1
    }
}
#[doc = "Field `INV` writer - Comparator INVERT"]
pub type INV_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR1_SPEC, INV_A, O>;
impl<'a, const O: u8> INV_W<'a, O> {
    #[doc = "Does not invert the comparator output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(INV_A::_0)
    }
    #[doc = "Inverts the comparator output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(INV_A::_1)
    }
}
#[doc = "Field `PMODE` reader - Power Mode Select"]
pub type PMODE_R = crate::BitReader<PMODE_A>;
#[doc = "Power Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMODE_A {
    #[doc = "0: Low-Speed (LS) Comparison mode selected. In this mode, CMP has slower output propagation delay and lower current consumption."]
    _0 = 0,
    #[doc = "1: High-Speed (HS) Comparison mode selected. In this mode, CMP has faster output propagation delay and higher current consumption."]
    _1 = 1,
}
impl From<PMODE_A> for bool {
    #[inline(always)]
    fn from(variant: PMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl PMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMODE_A {
        match self.bits {
            false => PMODE_A::_0,
            true => PMODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PMODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PMODE_A::_1
    }
}
#[doc = "Field `PMODE` writer - Power Mode Select"]
pub type PMODE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR1_SPEC, PMODE_A, O>;
impl<'a, const O: u8> PMODE_W<'a, O> {
    #[doc = "Low-Speed (LS) Comparison mode selected. In this mode, CMP has slower output propagation delay and lower current consumption."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PMODE_A::_0)
    }
    #[doc = "High-Speed (HS) Comparison mode selected. In this mode, CMP has faster output propagation delay and higher current consumption."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PMODE_A::_1)
    }
}
#[doc = "Field `WE` reader - Windowing Enable"]
pub type WE_R = crate::BitReader<WE_A>;
#[doc = "Windowing Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WE_A {
    #[doc = "0: Windowing mode is not selected."]
    _0 = 0,
    #[doc = "1: Windowing mode is selected."]
    _1 = 1,
}
impl From<WE_A> for bool {
    #[inline(always)]
    fn from(variant: WE_A) -> Self {
        variant as u8 != 0
    }
}
impl WE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WE_A {
        match self.bits {
            false => WE_A::_0,
            true => WE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WE_A::_1
    }
}
#[doc = "Field `WE` writer - Windowing Enable"]
pub type WE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR1_SPEC, WE_A, O>;
impl<'a, const O: u8> WE_W<'a, O> {
    #[doc = "Windowing mode is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WE_A::_0)
    }
    #[doc = "Windowing mode is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WE_A::_1)
    }
}
#[doc = "Field `SE` reader - Sample Enable"]
pub type SE_R = crate::BitReader<SE_A>;
#[doc = "Sample Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SE_A {
    #[doc = "0: Sampling mode is not selected."]
    _0 = 0,
    #[doc = "1: Sampling mode is selected."]
    _1 = 1,
}
impl From<SE_A> for bool {
    #[inline(always)]
    fn from(variant: SE_A) -> Self {
        variant as u8 != 0
    }
}
impl SE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SE_A {
        match self.bits {
            false => SE_A::_0,
            true => SE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SE_A::_1
    }
}
#[doc = "Field `SE` writer - Sample Enable"]
pub type SE_W<'a, const O: u8> = crate::BitWriter<'a, u8, CR1_SPEC, SE_A, O>;
impl<'a, const O: u8> SE_W<'a, O> {
    #[doc = "Sampling mode is not selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SE_A::_0)
    }
    #[doc = "Sampling mode is selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SE_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator Module Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator Output Pin Enable"]
    #[inline(always)]
    pub fn ope(&self) -> OPE_R {
        OPE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator Output Select"]
    #[inline(always)]
    pub fn cos(&self) -> COS_R {
        COS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator INVERT"]
    #[inline(always)]
    pub fn inv(&self) -> INV_R {
        INV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Power Mode Select"]
    #[inline(always)]
    pub fn pmode(&self) -> PMODE_R {
        PMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Windowing Enable"]
    #[inline(always)]
    pub fn we(&self) -> WE_R {
        WE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sample Enable"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator Module Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Comparator Output Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ope(&mut self) -> OPE_W<1> {
        OPE_W::new(self)
    }
    #[doc = "Bit 2 - Comparator Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn cos(&mut self) -> COS_W<2> {
        COS_W::new(self)
    }
    #[doc = "Bit 3 - Comparator INVERT"]
    #[inline(always)]
    #[must_use]
    pub fn inv(&mut self) -> INV_W<3> {
        INV_W::new(self)
    }
    #[doc = "Bit 4 - Power Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn pmode(&mut self) -> PMODE_W<4> {
        PMODE_W::new(self)
    }
    #[doc = "Bit 6 - Windowing Enable"]
    #[inline(always)]
    #[must_use]
    pub fn we(&mut self) -> WE_W<6> {
        WE_W::new(self)
    }
    #[doc = "Bit 7 - Sample Enable"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<7> {
        SE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMP Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
