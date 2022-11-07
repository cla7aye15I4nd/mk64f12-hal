#[doc = "Register `VENDOR` reader"]
pub struct R(crate::R<VENDOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VENDOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VENDOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VENDOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VENDOR` writer"]
pub struct W(crate::W<VENDOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VENDOR_SPEC>;
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
impl From<crate::W<VENDOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VENDOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTDMAEN` reader - External DMA Request Enable"]
pub type EXTDMAEN_R = crate::BitReader<EXTDMAEN_A>;
#[doc = "External DMA Request Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTDMAEN_A {
    #[doc = "0: In any scenario, SDHC does not send out the external DMA request."]
    _0 = 0,
    #[doc = "1: When internal DMA is not active, the external DMA request will be sent out."]
    _1 = 1,
}
impl From<EXTDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: EXTDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTDMAEN_A {
        match self.bits {
            false => EXTDMAEN_A::_0,
            true => EXTDMAEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXTDMAEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXTDMAEN_A::_1
    }
}
#[doc = "Field `EXTDMAEN` writer - External DMA Request Enable"]
pub type EXTDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, VENDOR_SPEC, EXTDMAEN_A, O>;
impl<'a, const O: u8> EXTDMAEN_W<'a, O> {
    #[doc = "In any scenario, SDHC does not send out the external DMA request."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXTDMAEN_A::_0)
    }
    #[doc = "When internal DMA is not active, the external DMA request will be sent out."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXTDMAEN_A::_1)
    }
}
#[doc = "Field `EXBLKNU` reader - Exact Block Number Block Read Enable For SDIO CMD53"]
pub type EXBLKNU_R = crate::BitReader<EXBLKNU_A>;
#[doc = "Exact Block Number Block Read Enable For SDIO CMD53\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXBLKNU_A {
    #[doc = "0: None exact block read."]
    _0 = 0,
    #[doc = "1: Exact block read for SDIO CMD53."]
    _1 = 1,
}
impl From<EXBLKNU_A> for bool {
    #[inline(always)]
    fn from(variant: EXBLKNU_A) -> Self {
        variant as u8 != 0
    }
}
impl EXBLKNU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXBLKNU_A {
        match self.bits {
            false => EXBLKNU_A::_0,
            true => EXBLKNU_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXBLKNU_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXBLKNU_A::_1
    }
}
#[doc = "Field `EXBLKNU` writer - Exact Block Number Block Read Enable For SDIO CMD53"]
pub type EXBLKNU_W<'a, const O: u8> = crate::BitWriter<'a, u32, VENDOR_SPEC, EXBLKNU_A, O>;
impl<'a, const O: u8> EXBLKNU_W<'a, O> {
    #[doc = "None exact block read."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXBLKNU_A::_0)
    }
    #[doc = "Exact block read for SDIO CMD53."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXBLKNU_A::_1)
    }
}
#[doc = "Field `INTSTVAL` reader - Internal State Value"]
pub type INTSTVAL_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - External DMA Request Enable"]
    #[inline(always)]
    pub fn extdmaen(&self) -> EXTDMAEN_R {
        EXTDMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Exact Block Number Block Read Enable For SDIO CMD53"]
    #[inline(always)]
    pub fn exblknu(&self) -> EXBLKNU_R {
        EXBLKNU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Internal State Value"]
    #[inline(always)]
    pub fn intstval(&self) -> INTSTVAL_R {
        INTSTVAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External DMA Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn extdmaen(&mut self) -> EXTDMAEN_W<0> {
        EXTDMAEN_W::new(self)
    }
    #[doc = "Bit 1 - Exact Block Number Block Read Enable For SDIO CMD53"]
    #[inline(always)]
    #[must_use]
    pub fn exblknu(&mut self) -> EXBLKNU_W<1> {
        EXBLKNU_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Vendor Specific register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vendor](index.html) module"]
pub struct VENDOR_SPEC;
impl crate::RegisterSpec for VENDOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vendor::R](R) reader structure"]
impl crate::Readable for VENDOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vendor::W](W) writer structure"]
impl crate::Writable for VENDOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VENDOR to value 0x01"]
impl crate::Resettable for VENDOR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
