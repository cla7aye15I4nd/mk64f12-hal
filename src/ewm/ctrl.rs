#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EWMEN` reader - EWM enable."]
pub type EWMEN_R = crate::BitReader<bool>;
#[doc = "Field `EWMEN` writer - EWM enable."]
pub type EWMEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRL_SPEC, bool, O>;
#[doc = "Field `ASSIN` reader - EWM_in's Assertion State Select."]
pub type ASSIN_R = crate::BitReader<bool>;
#[doc = "Field `ASSIN` writer - EWM_in's Assertion State Select."]
pub type ASSIN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRL_SPEC, bool, O>;
#[doc = "Field `INEN` reader - Input Enable."]
pub type INEN_R = crate::BitReader<bool>;
#[doc = "Field `INEN` writer - Input Enable."]
pub type INEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRL_SPEC, bool, O>;
#[doc = "Field `INTEN` reader - Interrupt Enable."]
pub type INTEN_R = crate::BitReader<bool>;
#[doc = "Field `INTEN` writer - Interrupt Enable."]
pub type INTEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - EWM enable."]
    #[inline(always)]
    pub fn ewmen(&self) -> EWMEN_R {
        EWMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EWM_in's Assertion State Select."]
    #[inline(always)]
    pub fn assin(&self) -> ASSIN_R {
        ASSIN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input Enable."]
    #[inline(always)]
    pub fn inen(&self) -> INEN_R {
        INEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Enable."]
    #[inline(always)]
    pub fn inten(&self) -> INTEN_R {
        INTEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EWM enable."]
    #[inline(always)]
    #[must_use]
    pub fn ewmen(&mut self) -> EWMEN_W<0> {
        EWMEN_W::new(self)
    }
    #[doc = "Bit 1 - EWM_in's Assertion State Select."]
    #[inline(always)]
    #[must_use]
    pub fn assin(&mut self) -> ASSIN_W<1> {
        ASSIN_W::new(self)
    }
    #[doc = "Bit 2 - Input Enable."]
    #[inline(always)]
    #[must_use]
    pub fn inen(&mut self) -> INEN_W<2> {
        INEN_W::new(self)
    }
    #[doc = "Bit 3 - Interrupt Enable."]
    #[inline(always)]
    #[must_use]
    pub fn inten(&mut self) -> INTEN_W<3> {
        INTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
