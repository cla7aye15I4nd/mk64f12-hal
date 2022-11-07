#[doc = "Register `SC2` reader"]
pub struct R(crate::R<SC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SC2` writer"]
pub struct W(crate::W<SC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SC2_SPEC>;
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
impl From<crate::W<SC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REFSEL` reader - Voltage Reference Selection"]
pub type REFSEL_R = crate::FieldReader<u8, REFSEL_A>;
#[doc = "Voltage Reference Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFSEL_A {
    #[doc = "0: Default voltage reference pin pair, that is, external pins VREFH and VREFL"]
    _00 = 0,
    #[doc = "1: Alternate reference pair, that is, VALTH and VALTL . This pair may be additional external pins or internal sources depending on the MCU configuration. See the chip configuration information for details specific to this MCU"]
    _01 = 1,
}
impl From<REFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFSEL_A) -> Self {
        variant as _
    }
}
impl REFSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<REFSEL_A> {
        match self.bits {
            0 => Some(REFSEL_A::_00),
            1 => Some(REFSEL_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == REFSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == REFSEL_A::_01
    }
}
#[doc = "Field `REFSEL` writer - Voltage Reference Selection"]
pub type REFSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SC2_SPEC, u8, REFSEL_A, 2, O>;
impl<'a, const O: u8> REFSEL_W<'a, O> {
    #[doc = "Default voltage reference pin pair, that is, external pins VREFH and VREFL"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(REFSEL_A::_00)
    }
    #[doc = "Alternate reference pair, that is, VALTH and VALTL . This pair may be additional external pins or internal sources depending on the MCU configuration. See the chip configuration information for details specific to this MCU"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(REFSEL_A::_01)
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: DMA is disabled."]
    _0 = 0,
    #[doc = "1: DMA is enabled and will assert the ADC DMA request during an ADC conversion complete event noted when any of the SC1n\\[COCO\\]
flags is asserted."]
    _1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::_0,
            true => DMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAEN_A::_1
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC2_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEN_A::_0)
    }
    #[doc = "DMA is enabled and will assert the ADC DMA request during an ADC conversion complete event noted when any of the SC1n\\[COCO\\]
flags is asserted."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAEN_A::_1)
    }
}
#[doc = "Field `ACREN` reader - Compare Function Range Enable"]
pub type ACREN_R = crate::BitReader<ACREN_A>;
#[doc = "Compare Function Range Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACREN_A {
    #[doc = "0: Range function disabled. Only CV1 is compared."]
    _0 = 0,
    #[doc = "1: Range function enabled. Both CV1 and CV2 are compared."]
    _1 = 1,
}
impl From<ACREN_A> for bool {
    #[inline(always)]
    fn from(variant: ACREN_A) -> Self {
        variant as u8 != 0
    }
}
impl ACREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACREN_A {
        match self.bits {
            false => ACREN_A::_0,
            true => ACREN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACREN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACREN_A::_1
    }
}
#[doc = "Field `ACREN` writer - Compare Function Range Enable"]
pub type ACREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC2_SPEC, ACREN_A, O>;
impl<'a, const O: u8> ACREN_W<'a, O> {
    #[doc = "Range function disabled. Only CV1 is compared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACREN_A::_0)
    }
    #[doc = "Range function enabled. Both CV1 and CV2 are compared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACREN_A::_1)
    }
}
#[doc = "Field `ACFGT` reader - Compare Function Greater Than Enable"]
pub type ACFGT_R = crate::BitReader<ACFGT_A>;
#[doc = "Compare Function Greater Than Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACFGT_A {
    #[doc = "0: Configures less than threshold, outside range not inclusive and inside range not inclusive; functionality based on the values placed in CV1 and CV2."]
    _0 = 0,
    #[doc = "1: Configures greater than or equal to threshold, outside and inside ranges inclusive; functionality based on the values placed in CV1 and CV2."]
    _1 = 1,
}
impl From<ACFGT_A> for bool {
    #[inline(always)]
    fn from(variant: ACFGT_A) -> Self {
        variant as u8 != 0
    }
}
impl ACFGT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACFGT_A {
        match self.bits {
            false => ACFGT_A::_0,
            true => ACFGT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACFGT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACFGT_A::_1
    }
}
#[doc = "Field `ACFGT` writer - Compare Function Greater Than Enable"]
pub type ACFGT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC2_SPEC, ACFGT_A, O>;
impl<'a, const O: u8> ACFGT_W<'a, O> {
    #[doc = "Configures less than threshold, outside range not inclusive and inside range not inclusive; functionality based on the values placed in CV1 and CV2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACFGT_A::_0)
    }
    #[doc = "Configures greater than or equal to threshold, outside and inside ranges inclusive; functionality based on the values placed in CV1 and CV2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACFGT_A::_1)
    }
}
#[doc = "Field `ACFE` reader - Compare Function Enable"]
pub type ACFE_R = crate::BitReader<ACFE_A>;
#[doc = "Compare Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACFE_A {
    #[doc = "0: Compare function disabled."]
    _0 = 0,
    #[doc = "1: Compare function enabled."]
    _1 = 1,
}
impl From<ACFE_A> for bool {
    #[inline(always)]
    fn from(variant: ACFE_A) -> Self {
        variant as u8 != 0
    }
}
impl ACFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACFE_A {
        match self.bits {
            false => ACFE_A::_0,
            true => ACFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACFE_A::_1
    }
}
#[doc = "Field `ACFE` writer - Compare Function Enable"]
pub type ACFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC2_SPEC, ACFE_A, O>;
impl<'a, const O: u8> ACFE_W<'a, O> {
    #[doc = "Compare function disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ACFE_A::_0)
    }
    #[doc = "Compare function enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ACFE_A::_1)
    }
}
#[doc = "Field `ADTRG` reader - Conversion Trigger Select"]
pub type ADTRG_R = crate::BitReader<ADTRG_A>;
#[doc = "Conversion Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADTRG_A {
    #[doc = "0: Software trigger selected."]
    _0 = 0,
    #[doc = "1: Hardware trigger selected."]
    _1 = 1,
}
impl From<ADTRG_A> for bool {
    #[inline(always)]
    fn from(variant: ADTRG_A) -> Self {
        variant as u8 != 0
    }
}
impl ADTRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADTRG_A {
        match self.bits {
            false => ADTRG_A::_0,
            true => ADTRG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADTRG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADTRG_A::_1
    }
}
#[doc = "Field `ADTRG` writer - Conversion Trigger Select"]
pub type ADTRG_W<'a, const O: u8> = crate::BitWriter<'a, u32, SC2_SPEC, ADTRG_A, O>;
impl<'a, const O: u8> ADTRG_W<'a, O> {
    #[doc = "Software trigger selected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADTRG_A::_0)
    }
    #[doc = "Hardware trigger selected."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADTRG_A::_1)
    }
}
#[doc = "Field `ADACT` reader - Conversion Active"]
pub type ADACT_R = crate::BitReader<ADACT_A>;
#[doc = "Conversion Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADACT_A {
    #[doc = "0: Conversion not in progress."]
    _0 = 0,
    #[doc = "1: Conversion in progress."]
    _1 = 1,
}
impl From<ADACT_A> for bool {
    #[inline(always)]
    fn from(variant: ADACT_A) -> Self {
        variant as u8 != 0
    }
}
impl ADACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADACT_A {
        match self.bits {
            false => ADACT_A::_0,
            true => ADACT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADACT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADACT_A::_1
    }
}
impl R {
    #[doc = "Bits 0:1 - Voltage Reference Selection"]
    #[inline(always)]
    pub fn refsel(&self) -> REFSEL_R {
        REFSEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare Function Range Enable"]
    #[inline(always)]
    pub fn acren(&self) -> ACREN_R {
        ACREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare Function Greater Than Enable"]
    #[inline(always)]
    pub fn acfgt(&self) -> ACFGT_R {
        ACFGT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Compare Function Enable"]
    #[inline(always)]
    pub fn acfe(&self) -> ACFE_R {
        ACFE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Conversion Trigger Select"]
    #[inline(always)]
    pub fn adtrg(&self) -> ADTRG_R {
        ADTRG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Conversion Active"]
    #[inline(always)]
    pub fn adact(&self) -> ADACT_R {
        ADACT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Voltage Reference Selection"]
    #[inline(always)]
    #[must_use]
    pub fn refsel(&mut self) -> REFSEL_W<0> {
        REFSEL_W::new(self)
    }
    #[doc = "Bit 2 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<2> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 3 - Compare Function Range Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acren(&mut self) -> ACREN_W<3> {
        ACREN_W::new(self)
    }
    #[doc = "Bit 4 - Compare Function Greater Than Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acfgt(&mut self) -> ACFGT_W<4> {
        ACFGT_W::new(self)
    }
    #[doc = "Bit 5 - Compare Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acfe(&mut self) -> ACFE_W<5> {
        ACFE_W::new(self)
    }
    #[doc = "Bit 6 - Conversion Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn adtrg(&mut self) -> ADTRG_W<6> {
        ADTRG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status and Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sc2](index.html) module"]
pub struct SC2_SPEC;
impl crate::RegisterSpec for SC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sc2::R](R) reader structure"]
impl crate::Readable for SC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sc2::W](W) writer structure"]
impl crate::Writable for SC2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SC2 to value 0"]
impl crate::Resettable for SC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
