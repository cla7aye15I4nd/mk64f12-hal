#[doc = "Register `REG%s` reader"]
pub struct R(crate::R<REG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG%s` writer"]
pub struct W(crate::W<REG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_SPEC>;
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
impl From<crate::W<REG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LL` reader - Low lower byte"]
pub type LL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LL` writer - Low lower byte"]
pub type LL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_SPEC, u8, u8, 8, O>;
#[doc = "Field `LH` reader - Low higher byte"]
pub type LH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LH` writer - Low higher byte"]
pub type LH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_SPEC, u8, u8, 8, O>;
#[doc = "Field `HL` reader - High lower byte"]
pub type HL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HL` writer - High lower byte"]
pub type HL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_SPEC, u8, u8, 8, O>;
#[doc = "Field `HH` reader - High higher byte"]
pub type HH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HH` writer - High higher byte"]
pub type HH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Low lower byte"]
    #[inline(always)]
    pub fn ll(&self) -> LL_R {
        LL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Low higher byte"]
    #[inline(always)]
    pub fn lh(&self) -> LH_R {
        LH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - High lower byte"]
    #[inline(always)]
    pub fn hl(&self) -> HL_R {
        HL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - High higher byte"]
    #[inline(always)]
    pub fn hh(&self) -> HH_R {
        HH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Low lower byte"]
    #[inline(always)]
    #[must_use]
    pub fn ll(&mut self) -> LL_W<0> {
        LL_W::new(self)
    }
    #[doc = "Bits 8:15 - Low higher byte"]
    #[inline(always)]
    #[must_use]
    pub fn lh(&mut self) -> LH_W<8> {
        LH_W::new(self)
    }
    #[doc = "Bits 16:23 - High lower byte"]
    #[inline(always)]
    #[must_use]
    pub fn hl(&mut self) -> HL_W<16> {
        HL_W::new(self)
    }
    #[doc = "Bits 24:31 - High higher byte"]
    #[inline(always)]
    #[must_use]
    pub fn hh(&mut self) -> HH_W<24> {
        HH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VBAT register file register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg](index.html) module"]
pub struct REG_SPEC;
impl crate::RegisterSpec for REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg::R](R) reader structure"]
impl crate::Readable for REG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg::W](W) writer structure"]
impl crate::Writable for REG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG%s to value 0"]
impl crate::Resettable for REG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
