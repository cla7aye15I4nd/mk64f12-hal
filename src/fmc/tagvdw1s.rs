#[doc = "Register `TAGVDW1S%s` reader"]
pub struct R(crate::R<TAGVDW1S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAGVDW1S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAGVDW1S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAGVDW1S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAGVDW1S%s` writer"]
pub struct W(crate::W<TAGVDW1S_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAGVDW1S_SPEC>;
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
impl From<crate::W<TAGVDW1S_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAGVDW1S_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `valid` reader - 1-bit valid for cache entry"]
pub type VALID_R = crate::BitReader<bool>;
#[doc = "Field `valid` writer - 1-bit valid for cache entry"]
pub type VALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, TAGVDW1S_SPEC, bool, O>;
#[doc = "Field `tag` reader - 14-bit tag for cache entry"]
pub type TAG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `tag` writer - 14-bit tag for cache entry"]
pub type TAG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TAGVDW1S_SPEC, u16, u16, 14, O>;
impl R {
    #[doc = "Bit 0 - 1-bit valid for cache entry"]
    #[inline(always)]
    pub fn valid(&self) -> VALID_R {
        VALID_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 5:18 - 14-bit tag for cache entry"]
    #[inline(always)]
    pub fn tag(&self) -> TAG_R {
        TAG_R::new(((self.bits >> 5) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 1-bit valid for cache entry"]
    #[inline(always)]
    #[must_use]
    pub fn valid(&mut self) -> VALID_W<0> {
        VALID_W::new(self)
    }
    #[doc = "Bits 5:18 - 14-bit tag for cache entry"]
    #[inline(always)]
    #[must_use]
    pub fn tag(&mut self) -> TAG_W<5> {
        TAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Tag Storage\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tagvdw1s](index.html) module"]
pub struct TAGVDW1S_SPEC;
impl crate::RegisterSpec for TAGVDW1S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tagvdw1s::R](R) reader structure"]
impl crate::Readable for TAGVDW1S_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tagvdw1s::W](W) writer structure"]
impl crate::Writable for TAGVDW1S_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TAGVDW1S%s to value 0"]
impl crate::Resettable for TAGVDW1S_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
