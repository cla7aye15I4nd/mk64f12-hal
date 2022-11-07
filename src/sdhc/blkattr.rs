#[doc = "Register `BLKATTR` reader"]
pub struct R(crate::R<BLKATTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLKATTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLKATTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLKATTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLKATTR` writer"]
pub struct W(crate::W<BLKATTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLKATTR_SPEC>;
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
impl From<crate::W<BLKATTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLKATTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLKSIZE` reader - Transfer Block Size"]
pub type BLKSIZE_R = crate::FieldReader<u16, BLKSIZE_A>;
#[doc = "Transfer Block Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum BLKSIZE_A {
    #[doc = "0: No data transfer."]
    _0 = 0,
    #[doc = "1: 1 Byte"]
    _1 = 1,
    #[doc = "2: 2 Bytes"]
    _10 = 2,
    #[doc = "3: 3 Bytes"]
    _11 = 3,
    #[doc = "4: 4 Bytes"]
    _100 = 4,
    #[doc = "511: 511 Bytes"]
    _111111111 = 511,
    #[doc = "512: 512 Bytes"]
    _1000000000 = 512,
    #[doc = "2048: 2048 Bytes"]
    _100000000000 = 2048,
    #[doc = "4096: 4096 Bytes"]
    _1000000000000 = 4096,
}
impl From<BLKSIZE_A> for u16 {
    #[inline(always)]
    fn from(variant: BLKSIZE_A) -> Self {
        variant as _
    }
}
impl BLKSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BLKSIZE_A> {
        match self.bits {
            0 => Some(BLKSIZE_A::_0),
            1 => Some(BLKSIZE_A::_1),
            2 => Some(BLKSIZE_A::_10),
            3 => Some(BLKSIZE_A::_11),
            4 => Some(BLKSIZE_A::_100),
            511 => Some(BLKSIZE_A::_111111111),
            512 => Some(BLKSIZE_A::_1000000000),
            2048 => Some(BLKSIZE_A::_100000000000),
            4096 => Some(BLKSIZE_A::_1000000000000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BLKSIZE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BLKSIZE_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BLKSIZE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == BLKSIZE_A::_11
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == BLKSIZE_A::_100
    }
    #[doc = "Checks if the value of the field is `_111111111`"]
    #[inline(always)]
    pub fn is_111111111(&self) -> bool {
        *self == BLKSIZE_A::_111111111
    }
    #[doc = "Checks if the value of the field is `_1000000000`"]
    #[inline(always)]
    pub fn is_1000000000(&self) -> bool {
        *self == BLKSIZE_A::_1000000000
    }
    #[doc = "Checks if the value of the field is `_100000000000`"]
    #[inline(always)]
    pub fn is_100000000000(&self) -> bool {
        *self == BLKSIZE_A::_100000000000
    }
    #[doc = "Checks if the value of the field is `_1000000000000`"]
    #[inline(always)]
    pub fn is_1000000000000(&self) -> bool {
        *self == BLKSIZE_A::_1000000000000
    }
}
#[doc = "Field `BLKSIZE` writer - Transfer Block Size"]
pub type BLKSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BLKATTR_SPEC, u16, BLKSIZE_A, 13, O>;
impl<'a, const O: u8> BLKSIZE_W<'a, O> {
    #[doc = "No data transfer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BLKSIZE_A::_0)
    }
    #[doc = "1 Byte"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BLKSIZE_A::_1)
    }
    #[doc = "2 Bytes"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(BLKSIZE_A::_10)
    }
    #[doc = "3 Bytes"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(BLKSIZE_A::_11)
    }
    #[doc = "4 Bytes"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(BLKSIZE_A::_100)
    }
    #[doc = "511 Bytes"]
    #[inline(always)]
    pub fn _111111111(self) -> &'a mut W {
        self.variant(BLKSIZE_A::_111111111)
    }
    #[doc = "512 Bytes"]
    #[inline(always)]
    pub fn _1000000000(self) -> &'a mut W {
        self.variant(BLKSIZE_A::_1000000000)
    }
    #[doc = "2048 Bytes"]
    #[inline(always)]
    pub fn _100000000000(self) -> &'a mut W {
        self.variant(BLKSIZE_A::_100000000000)
    }
    #[doc = "4096 Bytes"]
    #[inline(always)]
    pub fn _1000000000000(self) -> &'a mut W {
        self.variant(BLKSIZE_A::_1000000000000)
    }
}
#[doc = "Field `BLKCNT` reader - Blocks Count For Current Transfer"]
pub type BLKCNT_R = crate::FieldReader<u16, BLKCNT_A>;
#[doc = "Blocks Count For Current Transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum BLKCNT_A {
    #[doc = "0: Stop count."]
    _0 = 0,
    #[doc = "1: 1 block"]
    _1 = 1,
    #[doc = "2: 2 blocks"]
    _10 = 2,
    #[doc = "65535: 65535 blocks"]
    _1111111111111111 = 65535,
}
impl From<BLKCNT_A> for u16 {
    #[inline(always)]
    fn from(variant: BLKCNT_A) -> Self {
        variant as _
    }
}
impl BLKCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BLKCNT_A> {
        match self.bits {
            0 => Some(BLKCNT_A::_0),
            1 => Some(BLKCNT_A::_1),
            2 => Some(BLKCNT_A::_10),
            65535 => Some(BLKCNT_A::_1111111111111111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BLKCNT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BLKCNT_A::_1
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == BLKCNT_A::_10
    }
    #[doc = "Checks if the value of the field is `_1111111111111111`"]
    #[inline(always)]
    pub fn is_1111111111111111(&self) -> bool {
        *self == BLKCNT_A::_1111111111111111
    }
}
#[doc = "Field `BLKCNT` writer - Blocks Count For Current Transfer"]
pub type BLKCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BLKATTR_SPEC, u16, BLKCNT_A, 16, O>;
impl<'a, const O: u8> BLKCNT_W<'a, O> {
    #[doc = "Stop count."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BLKCNT_A::_0)
    }
    #[doc = "1 block"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BLKCNT_A::_1)
    }
    #[doc = "2 blocks"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(BLKCNT_A::_10)
    }
    #[doc = "65535 blocks"]
    #[inline(always)]
    pub fn _1111111111111111(self) -> &'a mut W {
        self.variant(BLKCNT_A::_1111111111111111)
    }
}
impl R {
    #[doc = "Bits 0:12 - Transfer Block Size"]
    #[inline(always)]
    pub fn blksize(&self) -> BLKSIZE_R {
        BLKSIZE_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:31 - Blocks Count For Current Transfer"]
    #[inline(always)]
    pub fn blkcnt(&self) -> BLKCNT_R {
        BLKCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Transfer Block Size"]
    #[inline(always)]
    #[must_use]
    pub fn blksize(&mut self) -> BLKSIZE_W<0> {
        BLKSIZE_W::new(self)
    }
    #[doc = "Bits 16:31 - Blocks Count For Current Transfer"]
    #[inline(always)]
    #[must_use]
    pub fn blkcnt(&mut self) -> BLKCNT_W<16> {
        BLKCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Attributes register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blkattr](index.html) module"]
pub struct BLKATTR_SPEC;
impl crate::RegisterSpec for BLKATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blkattr::R](R) reader structure"]
impl crate::Readable for BLKATTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blkattr::W](W) writer structure"]
impl crate::Writable for BLKATTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLKATTR to value 0"]
impl crate::Resettable for BLKATTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
