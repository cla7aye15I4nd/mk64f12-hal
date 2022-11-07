#[doc = "Register `ID2` reader"]
pub struct R(crate::R<ID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ID2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ID2` writer"]
pub struct W(crate::W<ID2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ID2_SPEC>;
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
impl From<crate::W<ID2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ID2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXT` reader - Contains extended (LOW word) identifier of message buffer."]
pub type EXT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `EXT` writer - Contains extended (LOW word) identifier of message buffer."]
pub type EXT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ID2_SPEC, u32, u32, 18, O>;
#[doc = "Field `STD` reader - Contains standard/extended (HIGH word) identifier of message buffer."]
pub type STD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `STD` writer - Contains standard/extended (HIGH word) identifier of message buffer."]
pub type STD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ID2_SPEC, u16, u16, 11, O>;
#[doc = "Field `PRIO` reader - Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
pub type PRIO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRIO` writer - Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
pub type PRIO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ID2_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:17 - Contains extended (LOW word) identifier of message buffer."]
    #[inline(always)]
    pub fn ext(&self) -> EXT_R {
        EXT_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:28 - Contains standard/extended (HIGH word) identifier of message buffer."]
    #[inline(always)]
    pub fn std(&self) -> STD_R {
        STD_R::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bits 29:31 - Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    #[inline(always)]
    pub fn prio(&self) -> PRIO_R {
        PRIO_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:17 - Contains extended (LOW word) identifier of message buffer."]
    #[inline(always)]
    #[must_use]
    pub fn ext(&mut self) -> EXT_W<0> {
        EXT_W::new(self)
    }
    #[doc = "Bits 18:28 - Contains standard/extended (HIGH word) identifier of message buffer."]
    #[inline(always)]
    #[must_use]
    pub fn std(&mut self) -> STD_W<18> {
        STD_W::new(self)
    }
    #[doc = "Bits 29:31 - Local priority. This 3-bit fieldis only used when LPRIO_EN bit is set in MCR and it only makes sense for Tx buffers. These bits are not transmitted. They are appended to the regular ID to define the transmission priority."]
    #[inline(always)]
    #[must_use]
    pub fn prio(&mut self) -> PRIO_W<29> {
        PRIO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Buffer 2 ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [id2](index.html) module"]
pub struct ID2_SPEC;
impl crate::RegisterSpec for ID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [id2::R](R) reader structure"]
impl crate::Readable for ID2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [id2::W](W) writer structure"]
impl crate::Writable for ID2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ID2 to value 0"]
impl crate::Resettable for ID2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
