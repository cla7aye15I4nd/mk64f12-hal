#[doc = "Register `FILTER` reader"]
pub struct R(crate::R<FILTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FILTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FILTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FILTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FILTER` writer"]
pub struct W(crate::W<FILTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FILTER_SPEC>;
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
impl From<crate::W<FILTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FILTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0FVAL` reader - Channel 0 Input Filter"]
pub type CH0FVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH0FVAL` writer - Channel 0 Input Filter"]
pub type CH0FVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FILTER_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH1FVAL` reader - Channel 1 Input Filter"]
pub type CH1FVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH1FVAL` writer - Channel 1 Input Filter"]
pub type CH1FVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FILTER_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH2FVAL` reader - Channel 2 Input Filter"]
pub type CH2FVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH2FVAL` writer - Channel 2 Input Filter"]
pub type CH2FVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FILTER_SPEC, u8, u8, 4, O>;
#[doc = "Field `CH3FVAL` reader - Channel 3 Input Filter"]
pub type CH3FVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CH3FVAL` writer - Channel 3 Input Filter"]
pub type CH3FVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FILTER_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Channel 0 Input Filter"]
    #[inline(always)]
    pub fn ch0fval(&self) -> CH0FVAL_R {
        CH0FVAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Channel 1 Input Filter"]
    #[inline(always)]
    pub fn ch1fval(&self) -> CH1FVAL_R {
        CH1FVAL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Channel 2 Input Filter"]
    #[inline(always)]
    pub fn ch2fval(&self) -> CH2FVAL_R {
        CH2FVAL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Channel 3 Input Filter"]
    #[inline(always)]
    pub fn ch3fval(&self) -> CH3FVAL_R {
        CH3FVAL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Channel 0 Input Filter"]
    #[inline(always)]
    #[must_use]
    pub fn ch0fval(&mut self) -> CH0FVAL_W<0> {
        CH0FVAL_W::new(self)
    }
    #[doc = "Bits 4:7 - Channel 1 Input Filter"]
    #[inline(always)]
    #[must_use]
    pub fn ch1fval(&mut self) -> CH1FVAL_W<4> {
        CH1FVAL_W::new(self)
    }
    #[doc = "Bits 8:11 - Channel 2 Input Filter"]
    #[inline(always)]
    #[must_use]
    pub fn ch2fval(&mut self) -> CH2FVAL_W<8> {
        CH2FVAL_W::new(self)
    }
    #[doc = "Bits 12:15 - Channel 3 Input Filter"]
    #[inline(always)]
    #[must_use]
    pub fn ch3fval(&mut self) -> CH3FVAL_W<12> {
        CH3FVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Capture Filter Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [filter](index.html) module"]
pub struct FILTER_SPEC;
impl crate::RegisterSpec for FILTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [filter::R](R) reader structure"]
impl crate::Readable for FILTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [filter::W](W) writer structure"]
impl crate::Writable for FILTER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FILTER to value 0"]
impl crate::Resettable for FILTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
