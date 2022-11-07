#[doc = "Register `CSCR%s` reader"]
pub struct R(crate::R<CSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCR%s` writer"]
pub struct W(crate::W<CSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCR_SPEC>;
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
impl From<crate::W<CSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BSTW` reader - Burst-Write Enable"]
pub type BSTW_R = crate::BitReader<BSTW_A>;
#[doc = "Burst-Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSTW_A {
    #[doc = "0: Disabled. Data exceeding the specified port size is broken into individual, port-sized, non-burst writes. For example, a 32-bit write to an 8-bit port takes four byte writes."]
    _0 = 0,
    #[doc = "1: Enabled. Enables burst write of data larger than the specified port size, including 32-bit writes to 8- and 16-bit ports, 16-bit writes to 8-bit ports, and line writes to 8-, 16-, and 32-bit ports."]
    _1 = 1,
}
impl From<BSTW_A> for bool {
    #[inline(always)]
    fn from(variant: BSTW_A) -> Self {
        variant as u8 != 0
    }
}
impl BSTW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSTW_A {
        match self.bits {
            false => BSTW_A::_0,
            true => BSTW_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSTW_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSTW_A::_1
    }
}
#[doc = "Field `BSTW` writer - Burst-Write Enable"]
pub type BSTW_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSCR_SPEC, BSTW_A, O>;
impl<'a, const O: u8> BSTW_W<'a, O> {
    #[doc = "Disabled. Data exceeding the specified port size is broken into individual, port-sized, non-burst writes. For example, a 32-bit write to an 8-bit port takes four byte writes."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSTW_A::_0)
    }
    #[doc = "Enabled. Enables burst write of data larger than the specified port size, including 32-bit writes to 8- and 16-bit ports, 16-bit writes to 8-bit ports, and line writes to 8-, 16-, and 32-bit ports."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSTW_A::_1)
    }
}
#[doc = "Field `BSTR` reader - Burst-Read Enable"]
pub type BSTR_R = crate::BitReader<BSTR_A>;
#[doc = "Burst-Read Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSTR_A {
    #[doc = "0: Disabled. Data exceeding the specified port size is broken into individual, port-sized, non-burst reads. For example, a 32-bit read from an 8-bit port is broken into four 8-bit reads."]
    _0 = 0,
    #[doc = "1: Enabled. Enables data burst reads larger than the specified port size, including 32-bit reads from 8- and 16-bit ports, 16-bit reads from 8-bit ports, and line reads from 8-, 16-, and 32-bit ports."]
    _1 = 1,
}
impl From<BSTR_A> for bool {
    #[inline(always)]
    fn from(variant: BSTR_A) -> Self {
        variant as u8 != 0
    }
}
impl BSTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSTR_A {
        match self.bits {
            false => BSTR_A::_0,
            true => BSTR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BSTR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BSTR_A::_1
    }
}
#[doc = "Field `BSTR` writer - Burst-Read Enable"]
pub type BSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSCR_SPEC, BSTR_A, O>;
impl<'a, const O: u8> BSTR_W<'a, O> {
    #[doc = "Disabled. Data exceeding the specified port size is broken into individual, port-sized, non-burst reads. For example, a 32-bit read from an 8-bit port is broken into four 8-bit reads."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BSTR_A::_0)
    }
    #[doc = "Enabled. Enables data burst reads larger than the specified port size, including 32-bit reads from 8- and 16-bit ports, 16-bit reads from 8-bit ports, and line reads from 8-, 16-, and 32-bit ports."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BSTR_A::_1)
    }
}
#[doc = "Field `BEM` reader - Byte-Enable Mode"]
pub type BEM_R = crate::BitReader<BEM_A>;
#[doc = "Byte-Enable Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BEM_A {
    #[doc = "0: FB_BE is asserted for data write only."]
    _0 = 0,
    #[doc = "1: FB_BE is asserted for data read and write accesses."]
    _1 = 1,
}
impl From<BEM_A> for bool {
    #[inline(always)]
    fn from(variant: BEM_A) -> Self {
        variant as u8 != 0
    }
}
impl BEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BEM_A {
        match self.bits {
            false => BEM_A::_0,
            true => BEM_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BEM_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BEM_A::_1
    }
}
#[doc = "Field `BEM` writer - Byte-Enable Mode"]
pub type BEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSCR_SPEC, BEM_A, O>;
impl<'a, const O: u8> BEM_W<'a, O> {
    #[doc = "FB_BE is asserted for data write only."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BEM_A::_0)
    }
    #[doc = "FB_BE is asserted for data read and write accesses."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BEM_A::_1)
    }
}
#[doc = "Field `PS` reader - Port Size"]
pub type PS_R = crate::FieldReader<u8, PS_A>;
#[doc = "Port Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PS_A {
    #[doc = "0: 32-bit port size. Valid data is sampled and driven on FB_D\\[31:0\\]."]
    _00 = 0,
    #[doc = "1: 8-bit port size. Valid data is sampled and driven on FB_D\\[31:24\\]
when BLS is 0b, or FB_D\\[7:0\\]
when BLS is 1b."]
    _01 = 1,
}
impl From<PS_A> for u8 {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as _
    }
}
impl PS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PS_A> {
        match self.bits {
            0 => Some(PS_A::_00),
            1 => Some(PS_A::_01),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PS_A::_01
    }
}
#[doc = "Field `PS` writer - Port Size"]
pub type PS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSCR_SPEC, u8, PS_A, 2, O>;
impl<'a, const O: u8> PS_W<'a, O> {
    #[doc = "32-bit port size. Valid data is sampled and driven on FB_D\\[31:0\\]."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PS_A::_00)
    }
    #[doc = "8-bit port size. Valid data is sampled and driven on FB_D\\[31:24\\]
when BLS is 0b, or FB_D\\[7:0\\]
when BLS is 1b."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PS_A::_01)
    }
}
#[doc = "Field `AA` reader - Auto-Acknowledge Enable"]
pub type AA_R = crate::BitReader<AA_A>;
#[doc = "Auto-Acknowledge Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AA_A {
    #[doc = "0: Disabled. No internal transfer acknowledge is asserted and the cycle is terminated externally."]
    _0 = 0,
    #[doc = "1: Enabled. Internal transfer acknowledge is asserted as specified by WS."]
    _1 = 1,
}
impl From<AA_A> for bool {
    #[inline(always)]
    fn from(variant: AA_A) -> Self {
        variant as u8 != 0
    }
}
impl AA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AA_A {
        match self.bits {
            false => AA_A::_0,
            true => AA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == AA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == AA_A::_1
    }
}
#[doc = "Field `AA` writer - Auto-Acknowledge Enable"]
pub type AA_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSCR_SPEC, AA_A, O>;
impl<'a, const O: u8> AA_W<'a, O> {
    #[doc = "Disabled. No internal transfer acknowledge is asserted and the cycle is terminated externally."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(AA_A::_0)
    }
    #[doc = "Enabled. Internal transfer acknowledge is asserted as specified by WS."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(AA_A::_1)
    }
}
#[doc = "Field `BLS` reader - Byte-Lane Shift"]
pub type BLS_R = crate::BitReader<BLS_A>;
#[doc = "Byte-Lane Shift\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BLS_A {
    #[doc = "0: Not shifted. Data is left-aligned on FB_AD."]
    _0 = 0,
    #[doc = "1: Shifted. Data is right-aligned on FB_AD."]
    _1 = 1,
}
impl From<BLS_A> for bool {
    #[inline(always)]
    fn from(variant: BLS_A) -> Self {
        variant as u8 != 0
    }
}
impl BLS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BLS_A {
        match self.bits {
            false => BLS_A::_0,
            true => BLS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BLS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BLS_A::_1
    }
}
#[doc = "Field `BLS` writer - Byte-Lane Shift"]
pub type BLS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSCR_SPEC, BLS_A, O>;
impl<'a, const O: u8> BLS_W<'a, O> {
    #[doc = "Not shifted. Data is left-aligned on FB_AD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BLS_A::_0)
    }
    #[doc = "Shifted. Data is right-aligned on FB_AD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BLS_A::_1)
    }
}
#[doc = "Field `WS` reader - Wait States"]
pub type WS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WS` writer - Wait States"]
pub type WS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSCR_SPEC, u8, u8, 6, O>;
#[doc = "Field `WRAH` reader - Write Address Hold or Deselect"]
pub type WRAH_R = crate::FieldReader<u8, WRAH_A>;
#[doc = "Write Address Hold or Deselect\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WRAH_A {
    #[doc = "0: 1 cycle (default for all but FB_CS0 )"]
    _00 = 0,
    #[doc = "1: 2 cycles"]
    _01 = 1,
    #[doc = "2: 3 cycles"]
    _10 = 2,
    #[doc = "3: 4 cycles (default for FB_CS0 )"]
    _11 = 3,
}
impl From<WRAH_A> for u8 {
    #[inline(always)]
    fn from(variant: WRAH_A) -> Self {
        variant as _
    }
}
impl WRAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WRAH_A {
        match self.bits {
            0 => WRAH_A::_00,
            1 => WRAH_A::_01,
            2 => WRAH_A::_10,
            3 => WRAH_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == WRAH_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == WRAH_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == WRAH_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == WRAH_A::_11
    }
}
#[doc = "Field `WRAH` writer - Write Address Hold or Deselect"]
pub type WRAH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CSCR_SPEC, u8, WRAH_A, 2, O>;
impl<'a, const O: u8> WRAH_W<'a, O> {
    #[doc = "1 cycle (default for all but FB_CS0 )"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(WRAH_A::_00)
    }
    #[doc = "2 cycles"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(WRAH_A::_01)
    }
    #[doc = "3 cycles"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(WRAH_A::_10)
    }
    #[doc = "4 cycles (default for FB_CS0 )"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(WRAH_A::_11)
    }
}
#[doc = "Field `RDAH` reader - Read Address Hold or Deselect"]
pub type RDAH_R = crate::FieldReader<u8, RDAH_A>;
#[doc = "Read Address Hold or Deselect\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RDAH_A {
    #[doc = "0: When AA is 0b, 1 cycle. When AA is 1b, 0 cycles."]
    _00 = 0,
    #[doc = "1: When AA is 0b, 2 cycles. When AA is 1b, 1 cycle."]
    _01 = 1,
    #[doc = "2: When AA is 0b, 3 cycles. When AA is 1b, 2 cycles."]
    _10 = 2,
    #[doc = "3: When AA is 0b, 4 cycles. When AA is 1b, 3 cycles."]
    _11 = 3,
}
impl From<RDAH_A> for u8 {
    #[inline(always)]
    fn from(variant: RDAH_A) -> Self {
        variant as _
    }
}
impl RDAH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDAH_A {
        match self.bits {
            0 => RDAH_A::_00,
            1 => RDAH_A::_01,
            2 => RDAH_A::_10,
            3 => RDAH_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == RDAH_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == RDAH_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == RDAH_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == RDAH_A::_11
    }
}
#[doc = "Field `RDAH` writer - Read Address Hold or Deselect"]
pub type RDAH_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CSCR_SPEC, u8, RDAH_A, 2, O>;
impl<'a, const O: u8> RDAH_W<'a, O> {
    #[doc = "When AA is 0b, 1 cycle. When AA is 1b, 0 cycles."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(RDAH_A::_00)
    }
    #[doc = "When AA is 0b, 2 cycles. When AA is 1b, 1 cycle."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(RDAH_A::_01)
    }
    #[doc = "When AA is 0b, 3 cycles. When AA is 1b, 2 cycles."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(RDAH_A::_10)
    }
    #[doc = "When AA is 0b, 4 cycles. When AA is 1b, 3 cycles."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(RDAH_A::_11)
    }
}
#[doc = "Field `ASET` reader - Address Setup"]
pub type ASET_R = crate::FieldReader<u8, ASET_A>;
#[doc = "Address Setup\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ASET_A {
    #[doc = "0: Assert FB_CSn on the first rising clock edge after the address is asserted (default for all but FB_CS0 )."]
    _00 = 0,
    #[doc = "1: Assert FB_CSn on the second rising clock edge after the address is asserted."]
    _01 = 1,
    #[doc = "2: Assert FB_CSn on the third rising clock edge after the address is asserted."]
    _10 = 2,
    #[doc = "3: Assert FB_CSn on the fourth rising clock edge after the address is asserted (default for FB_CS0 )."]
    _11 = 3,
}
impl From<ASET_A> for u8 {
    #[inline(always)]
    fn from(variant: ASET_A) -> Self {
        variant as _
    }
}
impl ASET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASET_A {
        match self.bits {
            0 => ASET_A::_00,
            1 => ASET_A::_01,
            2 => ASET_A::_10,
            3 => ASET_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ASET_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ASET_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ASET_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ASET_A::_11
    }
}
#[doc = "Field `ASET` writer - Address Setup"]
pub type ASET_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CSCR_SPEC, u8, ASET_A, 2, O>;
impl<'a, const O: u8> ASET_W<'a, O> {
    #[doc = "Assert FB_CSn on the first rising clock edge after the address is asserted (default for all but FB_CS0 )."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ASET_A::_00)
    }
    #[doc = "Assert FB_CSn on the second rising clock edge after the address is asserted."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ASET_A::_01)
    }
    #[doc = "Assert FB_CSn on the third rising clock edge after the address is asserted."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ASET_A::_10)
    }
    #[doc = "Assert FB_CSn on the fourth rising clock edge after the address is asserted (default for FB_CS0 )."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ASET_A::_11)
    }
}
#[doc = "Field `EXTS` reader - Extended Transfer Start/Extended Address Latch Enable Controls how long FB_TS /FB_ALE is asserted."]
pub type EXTS_R = crate::BitReader<EXTS_A>;
#[doc = "Extended Transfer Start/Extended Address Latch Enable Controls how long FB_TS /FB_ALE is asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTS_A {
    #[doc = "0: Disabled. FB_TS /FB_ALE asserts for one bus clock cycle."]
    _0 = 0,
    #[doc = "1: Enabled. FB_TS /FB_ALE remains asserted until the first positive clock edge after FB_CSn asserts."]
    _1 = 1,
}
impl From<EXTS_A> for bool {
    #[inline(always)]
    fn from(variant: EXTS_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTS_A {
        match self.bits {
            false => EXTS_A::_0,
            true => EXTS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EXTS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EXTS_A::_1
    }
}
#[doc = "Field `EXTS` writer - Extended Transfer Start/Extended Address Latch Enable Controls how long FB_TS /FB_ALE is asserted."]
pub type EXTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSCR_SPEC, EXTS_A, O>;
impl<'a, const O: u8> EXTS_W<'a, O> {
    #[doc = "Disabled. FB_TS /FB_ALE asserts for one bus clock cycle."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EXTS_A::_0)
    }
    #[doc = "Enabled. FB_TS /FB_ALE remains asserted until the first positive clock edge after FB_CSn asserts."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EXTS_A::_1)
    }
}
#[doc = "Field `SWSEN` reader - Secondary Wait State Enable"]
pub type SWSEN_R = crate::BitReader<SWSEN_A>;
#[doc = "Secondary Wait State Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWSEN_A {
    #[doc = "0: Disabled. A number of wait states (specified by WS) are inserted before an internal transfer acknowledge is generated for all transfers."]
    _0 = 0,
    #[doc = "1: Enabled. A number of wait states (specified by SWS) are inserted before an internal transfer acknowledge is generated for burst transfer secondary terminations."]
    _1 = 1,
}
impl From<SWSEN_A> for bool {
    #[inline(always)]
    fn from(variant: SWSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SWSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWSEN_A {
        match self.bits {
            false => SWSEN_A::_0,
            true => SWSEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWSEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWSEN_A::_1
    }
}
#[doc = "Field `SWSEN` writer - Secondary Wait State Enable"]
pub type SWSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSCR_SPEC, SWSEN_A, O>;
impl<'a, const O: u8> SWSEN_W<'a, O> {
    #[doc = "Disabled. A number of wait states (specified by WS) are inserted before an internal transfer acknowledge is generated for all transfers."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWSEN_A::_0)
    }
    #[doc = "Enabled. A number of wait states (specified by SWS) are inserted before an internal transfer acknowledge is generated for burst transfer secondary terminations."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWSEN_A::_1)
    }
}
#[doc = "Field `SWS` reader - Secondary Wait States"]
pub type SWS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SWS` writer - Secondary Wait States"]
pub type SWS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSCR_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bit 3 - Burst-Write Enable"]
    #[inline(always)]
    pub fn bstw(&self) -> BSTW_R {
        BSTW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Burst-Read Enable"]
    #[inline(always)]
    pub fn bstr(&self) -> BSTR_R {
        BSTR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Byte-Enable Mode"]
    #[inline(always)]
    pub fn bem(&self) -> BEM_R {
        BEM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Port Size"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Auto-Acknowledge Enable"]
    #[inline(always)]
    pub fn aa(&self) -> AA_R {
        AA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Byte-Lane Shift"]
    #[inline(always)]
    pub fn bls(&self) -> BLS_R {
        BLS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - Wait States"]
    #[inline(always)]
    pub fn ws(&self) -> WS_R {
        WS_R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bits 16:17 - Write Address Hold or Deselect"]
    #[inline(always)]
    pub fn wrah(&self) -> WRAH_R {
        WRAH_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Read Address Hold or Deselect"]
    #[inline(always)]
    pub fn rdah(&self) -> RDAH_R {
        RDAH_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Address Setup"]
    #[inline(always)]
    pub fn aset(&self) -> ASET_R {
        ASET_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Extended Transfer Start/Extended Address Latch Enable Controls how long FB_TS /FB_ALE is asserted."]
    #[inline(always)]
    pub fn exts(&self) -> EXTS_R {
        EXTS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Secondary Wait State Enable"]
    #[inline(always)]
    pub fn swsen(&self) -> SWSEN_R {
        SWSEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 26:31 - Secondary Wait States"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 3 - Burst-Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bstw(&mut self) -> BSTW_W<3> {
        BSTW_W::new(self)
    }
    #[doc = "Bit 4 - Burst-Read Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bstr(&mut self) -> BSTR_W<4> {
        BSTR_W::new(self)
    }
    #[doc = "Bit 5 - Byte-Enable Mode"]
    #[inline(always)]
    #[must_use]
    pub fn bem(&mut self) -> BEM_W<5> {
        BEM_W::new(self)
    }
    #[doc = "Bits 6:7 - Port Size"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<6> {
        PS_W::new(self)
    }
    #[doc = "Bit 8 - Auto-Acknowledge Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aa(&mut self) -> AA_W<8> {
        AA_W::new(self)
    }
    #[doc = "Bit 9 - Byte-Lane Shift"]
    #[inline(always)]
    #[must_use]
    pub fn bls(&mut self) -> BLS_W<9> {
        BLS_W::new(self)
    }
    #[doc = "Bits 10:15 - Wait States"]
    #[inline(always)]
    #[must_use]
    pub fn ws(&mut self) -> WS_W<10> {
        WS_W::new(self)
    }
    #[doc = "Bits 16:17 - Write Address Hold or Deselect"]
    #[inline(always)]
    #[must_use]
    pub fn wrah(&mut self) -> WRAH_W<16> {
        WRAH_W::new(self)
    }
    #[doc = "Bits 18:19 - Read Address Hold or Deselect"]
    #[inline(always)]
    #[must_use]
    pub fn rdah(&mut self) -> RDAH_W<18> {
        RDAH_W::new(self)
    }
    #[doc = "Bits 20:21 - Address Setup"]
    #[inline(always)]
    #[must_use]
    pub fn aset(&mut self) -> ASET_W<20> {
        ASET_W::new(self)
    }
    #[doc = "Bit 22 - Extended Transfer Start/Extended Address Latch Enable Controls how long FB_TS /FB_ALE is asserted."]
    #[inline(always)]
    #[must_use]
    pub fn exts(&mut self) -> EXTS_W<22> {
        EXTS_W::new(self)
    }
    #[doc = "Bit 23 - Secondary Wait State Enable"]
    #[inline(always)]
    #[must_use]
    pub fn swsen(&mut self) -> SWSEN_W<23> {
        SWSEN_W::new(self)
    }
    #[doc = "Bits 26:31 - Secondary Wait States"]
    #[inline(always)]
    #[must_use]
    pub fn sws(&mut self) -> SWS_W<26> {
        SWS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Chip Select Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cscr](index.html) module"]
pub struct CSCR_SPEC;
impl crate::RegisterSpec for CSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cscr::R](R) reader structure"]
impl crate::Readable for CSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cscr::W](W) writer structure"]
impl crate::Writable for CSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CSCR%s to value 0x003f_fc00"]
impl crate::Resettable for CSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x003f_fc00;
}
