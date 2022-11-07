#[doc = "Register `SCR` reader"]
pub struct R(crate::R<SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUT` reader - Analog Comparator Output"]
pub type COUT_R = crate::BitReader<bool>;
#[doc = "Field `CFF` reader - Analog Comparator Flag Falling"]
pub type CFF_R = crate::BitReader<CFF_A>;
#[doc = "Analog Comparator Flag Falling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFF_A {
    #[doc = "0: Falling-edge on COUT has not been detected."]
    _0 = 0,
    #[doc = "1: Falling-edge on COUT has occurred."]
    _1 = 1,
}
impl From<CFF_A> for bool {
    #[inline(always)]
    fn from(variant: CFF_A) -> Self {
        variant as u8 != 0
    }
}
impl CFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFF_A {
        match self.bits {
            false => CFF_A::_0,
            true => CFF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFF_A::_1
    }
}
#[doc = "Field `CFF` writer - Analog Comparator Flag Falling"]
pub type CFF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCR_SPEC, CFF_A, O>;
impl<'a, const O: u8> CFF_W<'a, O> {
    #[doc = "Falling-edge on COUT has not been detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFF_A::_0)
    }
    #[doc = "Falling-edge on COUT has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFF_A::_1)
    }
}
#[doc = "Field `CFR` reader - Analog Comparator Flag Rising"]
pub type CFR_R = crate::BitReader<CFR_A>;
#[doc = "Analog Comparator Flag Rising\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CFR_A {
    #[doc = "0: Rising-edge on COUT has not been detected."]
    _0 = 0,
    #[doc = "1: Rising-edge on COUT has occurred."]
    _1 = 1,
}
impl From<CFR_A> for bool {
    #[inline(always)]
    fn from(variant: CFR_A) -> Self {
        variant as u8 != 0
    }
}
impl CFR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CFR_A {
        match self.bits {
            false => CFR_A::_0,
            true => CFR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CFR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CFR_A::_1
    }
}
#[doc = "Field `CFR` writer - Analog Comparator Flag Rising"]
pub type CFR_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCR_SPEC, CFR_A, O>;
impl<'a, const O: u8> CFR_W<'a, O> {
    #[doc = "Rising-edge on COUT has not been detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CFR_A::_0)
    }
    #[doc = "Rising-edge on COUT has occurred."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CFR_A::_1)
    }
}
#[doc = "Field `IEF` reader - Comparator Interrupt Enable Falling"]
pub type IEF_R = crate::BitReader<IEF_A>;
#[doc = "Comparator Interrupt Enable Falling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEF_A {
    #[doc = "0: Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt is enabled."]
    _1 = 1,
}
impl From<IEF_A> for bool {
    #[inline(always)]
    fn from(variant: IEF_A) -> Self {
        variant as u8 != 0
    }
}
impl IEF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEF_A {
        match self.bits {
            false => IEF_A::_0,
            true => IEF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IEF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IEF_A::_1
    }
}
#[doc = "Field `IEF` writer - Comparator Interrupt Enable Falling"]
pub type IEF_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCR_SPEC, IEF_A, O>;
impl<'a, const O: u8> IEF_W<'a, O> {
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IEF_A::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IEF_A::_1)
    }
}
#[doc = "Field `IER` reader - Comparator Interrupt Enable Rising"]
pub type IER_R = crate::BitReader<IER_A>;
#[doc = "Comparator Interrupt Enable Rising\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IER_A {
    #[doc = "0: Interrupt is disabled."]
    _0 = 0,
    #[doc = "1: Interrupt is enabled."]
    _1 = 1,
}
impl From<IER_A> for bool {
    #[inline(always)]
    fn from(variant: IER_A) -> Self {
        variant as u8 != 0
    }
}
impl IER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IER_A {
        match self.bits {
            false => IER_A::_0,
            true => IER_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IER_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IER_A::_1
    }
}
#[doc = "Field `IER` writer - Comparator Interrupt Enable Rising"]
pub type IER_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCR_SPEC, IER_A, O>;
impl<'a, const O: u8> IER_W<'a, O> {
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(IER_A::_0)
    }
    #[doc = "Interrupt is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(IER_A::_1)
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable Control"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "DMA Enable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: DMA is disabled."]
    _0 = 0,
    #[doc = "1: DMA is enabled."]
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
#[doc = "Field `DMAEN` writer - DMA Enable Control"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SCR_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    #[doc = "DMA is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAEN_A::_0)
    }
    #[doc = "DMA is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAEN_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Analog Comparator Output"]
    #[inline(always)]
    pub fn cout(&self) -> COUT_R {
        COUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Analog Comparator Flag Falling"]
    #[inline(always)]
    pub fn cff(&self) -> CFF_R {
        CFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Analog Comparator Flag Rising"]
    #[inline(always)]
    pub fn cfr(&self) -> CFR_R {
        CFR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Comparator Interrupt Enable Falling"]
    #[inline(always)]
    pub fn ief(&self) -> IEF_R {
        IEF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Comparator Interrupt Enable Rising"]
    #[inline(always)]
    pub fn ier(&self) -> IER_R {
        IER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Enable Control"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Analog Comparator Flag Falling"]
    #[inline(always)]
    #[must_use]
    pub fn cff(&mut self) -> CFF_W<1> {
        CFF_W::new(self)
    }
    #[doc = "Bit 2 - Analog Comparator Flag Rising"]
    #[inline(always)]
    #[must_use]
    pub fn cfr(&mut self) -> CFR_W<2> {
        CFR_W::new(self)
    }
    #[doc = "Bit 3 - Comparator Interrupt Enable Falling"]
    #[inline(always)]
    #[must_use]
    pub fn ief(&mut self) -> IEF_W<3> {
        IEF_W::new(self)
    }
    #[doc = "Bit 4 - Comparator Interrupt Enable Rising"]
    #[inline(always)]
    #[must_use]
    pub fn ier(&mut self) -> IER_W<4> {
        IER_W::new(self)
    }
    #[doc = "Bit 6 - DMA Enable Control"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<6> {
        DMAEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMP Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [scr::R](R) reader structure"]
impl crate::Readable for SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
