#[doc = "Register `ADDINFO` reader"]
pub struct R(crate::R<ADDINFO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDINFO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDINFO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDINFO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IEHOST` reader - This bit is set if host mode is enabled."]
pub type IEHOST_R = crate::BitReader<bool>;
#[doc = "Field `IRQNUM` reader - Assigned Interrupt Request Number"]
pub type IRQNUM_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - This bit is set if host mode is enabled."]
    #[inline(always)]
    pub fn iehost(&self) -> IEHOST_R {
        IEHOST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:7 - Assigned Interrupt Request Number"]
    #[inline(always)]
    pub fn irqnum(&self) -> IRQNUM_R {
        IRQNUM_R::new((self.bits >> 3) & 0x1f)
    }
}
#[doc = "Peripheral Additional Info register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addinfo](index.html) module"]
pub struct ADDINFO_SPEC;
impl crate::RegisterSpec for ADDINFO_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [addinfo::R](R) reader structure"]
impl crate::Readable for ADDINFO_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ADDINFO to value 0x01"]
impl crate::Resettable for ADDINFO_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
