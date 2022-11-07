#[doc = "Register `CHCFG%s` reader"]
pub struct R(crate::R<CHCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCFG%s` writer"]
pub struct W(crate::W<CHCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCFG_SPEC>;
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
impl From<crate::W<CHCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOURCE` reader - DMA Channel Source (Slot)"]
pub type SOURCE_R = crate::FieldReader<u8, SOURCE_A>;
#[doc = "DMA Channel Source (Slot)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOURCE_A {
    #[doc = "0: Disable_Signal"]
    _0 = 0,
    #[doc = "2: UART0_Rx_Signal"]
    _2 = 2,
    #[doc = "3: UART0_Tx_Signal"]
    _3 = 3,
    #[doc = "4: UART1_Rx_Signal"]
    _4 = 4,
    #[doc = "5: UART1_Tx_Signal"]
    _5 = 5,
    #[doc = "6: UART2_Rx_Signal"]
    _6 = 6,
    #[doc = "7: UART2_Tx_Signal"]
    _7 = 7,
    #[doc = "8: UART3_Rx_Signal"]
    _8 = 8,
    #[doc = "9: UART3_Tx_Signal"]
    _9 = 9,
    #[doc = "10: UART4_Signal"]
    _10 = 10,
    #[doc = "11: UART5_Signal"]
    _11 = 11,
    #[doc = "12: I2S0_Rx_Signal"]
    _12 = 12,
    #[doc = "13: I2S0_Tx_Signal"]
    _13 = 13,
    #[doc = "14: SPI0_Rx_Signal"]
    _14 = 14,
    #[doc = "15: SPI0_Tx_Signal"]
    _15 = 15,
    #[doc = "16: SPI1_Signal"]
    _16 = 16,
    #[doc = "17: SPI2_Signal"]
    _17 = 17,
    #[doc = "18: I2C0_Signal"]
    _18 = 18,
    #[doc = "19: I2C1_I2C2_Signal"]
    _19 = 19,
    #[doc = "20: FTM0_Channel0_Signal"]
    _20 = 20,
    #[doc = "21: FTM0_Channel1_Signal"]
    _21 = 21,
    #[doc = "22: FTM0_Channel2_Signal"]
    _22 = 22,
    #[doc = "23: FTM0_Channel3_Signal"]
    _23 = 23,
    #[doc = "24: FTM0_Channel4_Signal"]
    _24 = 24,
    #[doc = "25: FTM0_Channel5_Signal"]
    _25 = 25,
    #[doc = "26: FTM0_Channel6_Signal"]
    _26 = 26,
    #[doc = "27: FTM0_Channel7_Signal"]
    _27 = 27,
    #[doc = "28: FTM1_Channel0_Signal"]
    _28 = 28,
    #[doc = "29: FTM1_Channel1_Signal"]
    _29 = 29,
    #[doc = "30: FTM2_Channel0_Signal"]
    _30 = 30,
    #[doc = "31: FTM2_Channel1_Signal"]
    _31 = 31,
    #[doc = "32: FTM3_Channel0_Signal"]
    _32 = 32,
    #[doc = "33: FTM3_Channel1_Signal"]
    _33 = 33,
    #[doc = "34: FTM3_Channel2_Signal"]
    _34 = 34,
    #[doc = "35: FTM3_Channel3_Signal"]
    _35 = 35,
    #[doc = "36: FTM3_Channel4_Signal"]
    _36 = 36,
    #[doc = "37: FTM3_Channel5_Signal"]
    _37 = 37,
    #[doc = "38: FTM3_Channel6_Signal"]
    _38 = 38,
    #[doc = "39: FTM3_Channel7_Signal"]
    _39 = 39,
    #[doc = "40: ADC0_Signal"]
    _40 = 40,
    #[doc = "41: ADC1_Signal"]
    _41 = 41,
    #[doc = "42: CMP0_Signal"]
    _42 = 42,
    #[doc = "43: CMP1_Signal"]
    _43 = 43,
    #[doc = "44: CMP2_Signal"]
    _44 = 44,
    #[doc = "45: DAC0_Signal"]
    _45 = 45,
    #[doc = "46: DAC1_Signal"]
    _46 = 46,
    #[doc = "47: CMT_Signal"]
    _47 = 47,
    #[doc = "48: PDB_Signal"]
    _48 = 48,
    #[doc = "49: PortA_Signal"]
    _49 = 49,
    #[doc = "50: PortB_Signal"]
    _50 = 50,
    #[doc = "51: PortC_Signal"]
    _51 = 51,
    #[doc = "52: PortD_Signal"]
    _52 = 52,
    #[doc = "53: PortE_Signal"]
    _53 = 53,
    #[doc = "54: IEEE1588Timer0_Signal"]
    _54 = 54,
    #[doc = "55: IEEE1588Timer1_Signal"]
    _55 = 55,
    #[doc = "56: IEEE1588Timer2_Signal"]
    _56 = 56,
    #[doc = "57: IEEE1588Timer3_Signal"]
    _57 = 57,
    #[doc = "58: AlwaysOn58_Signal"]
    _58 = 58,
    #[doc = "59: AlwaysOn59_Signal"]
    _59 = 59,
    #[doc = "60: AlwaysOn60_Signal"]
    _60 = 60,
    #[doc = "61: AlwaysOn61_Signal"]
    _61 = 61,
    #[doc = "62: AlwaysOn62_Signal"]
    _62 = 62,
    #[doc = "63: AlwaysOn63_Signal"]
    _63 = 63,
}
impl From<SOURCE_A> for u8 {
    #[inline(always)]
    fn from(variant: SOURCE_A) -> Self {
        variant as _
    }
}
impl SOURCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SOURCE_A> {
        match self.bits {
            0 => Some(SOURCE_A::_0),
            2 => Some(SOURCE_A::_2),
            3 => Some(SOURCE_A::_3),
            4 => Some(SOURCE_A::_4),
            5 => Some(SOURCE_A::_5),
            6 => Some(SOURCE_A::_6),
            7 => Some(SOURCE_A::_7),
            8 => Some(SOURCE_A::_8),
            9 => Some(SOURCE_A::_9),
            10 => Some(SOURCE_A::_10),
            11 => Some(SOURCE_A::_11),
            12 => Some(SOURCE_A::_12),
            13 => Some(SOURCE_A::_13),
            14 => Some(SOURCE_A::_14),
            15 => Some(SOURCE_A::_15),
            16 => Some(SOURCE_A::_16),
            17 => Some(SOURCE_A::_17),
            18 => Some(SOURCE_A::_18),
            19 => Some(SOURCE_A::_19),
            20 => Some(SOURCE_A::_20),
            21 => Some(SOURCE_A::_21),
            22 => Some(SOURCE_A::_22),
            23 => Some(SOURCE_A::_23),
            24 => Some(SOURCE_A::_24),
            25 => Some(SOURCE_A::_25),
            26 => Some(SOURCE_A::_26),
            27 => Some(SOURCE_A::_27),
            28 => Some(SOURCE_A::_28),
            29 => Some(SOURCE_A::_29),
            30 => Some(SOURCE_A::_30),
            31 => Some(SOURCE_A::_31),
            32 => Some(SOURCE_A::_32),
            33 => Some(SOURCE_A::_33),
            34 => Some(SOURCE_A::_34),
            35 => Some(SOURCE_A::_35),
            36 => Some(SOURCE_A::_36),
            37 => Some(SOURCE_A::_37),
            38 => Some(SOURCE_A::_38),
            39 => Some(SOURCE_A::_39),
            40 => Some(SOURCE_A::_40),
            41 => Some(SOURCE_A::_41),
            42 => Some(SOURCE_A::_42),
            43 => Some(SOURCE_A::_43),
            44 => Some(SOURCE_A::_44),
            45 => Some(SOURCE_A::_45),
            46 => Some(SOURCE_A::_46),
            47 => Some(SOURCE_A::_47),
            48 => Some(SOURCE_A::_48),
            49 => Some(SOURCE_A::_49),
            50 => Some(SOURCE_A::_50),
            51 => Some(SOURCE_A::_51),
            52 => Some(SOURCE_A::_52),
            53 => Some(SOURCE_A::_53),
            54 => Some(SOURCE_A::_54),
            55 => Some(SOURCE_A::_55),
            56 => Some(SOURCE_A::_56),
            57 => Some(SOURCE_A::_57),
            58 => Some(SOURCE_A::_58),
            59 => Some(SOURCE_A::_59),
            60 => Some(SOURCE_A::_60),
            61 => Some(SOURCE_A::_61),
            62 => Some(SOURCE_A::_62),
            63 => Some(SOURCE_A::_63),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOURCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == SOURCE_A::_2
    }
    #[doc = "Checks if the value of the field is `_3`"]
    #[inline(always)]
    pub fn is_3(&self) -> bool {
        *self == SOURCE_A::_3
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == SOURCE_A::_4
    }
    #[doc = "Checks if the value of the field is `_5`"]
    #[inline(always)]
    pub fn is_5(&self) -> bool {
        *self == SOURCE_A::_5
    }
    #[doc = "Checks if the value of the field is `_6`"]
    #[inline(always)]
    pub fn is_6(&self) -> bool {
        *self == SOURCE_A::_6
    }
    #[doc = "Checks if the value of the field is `_7`"]
    #[inline(always)]
    pub fn is_7(&self) -> bool {
        *self == SOURCE_A::_7
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == SOURCE_A::_8
    }
    #[doc = "Checks if the value of the field is `_9`"]
    #[inline(always)]
    pub fn is_9(&self) -> bool {
        *self == SOURCE_A::_9
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == SOURCE_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == SOURCE_A::_11
    }
    #[doc = "Checks if the value of the field is `_12`"]
    #[inline(always)]
    pub fn is_12(&self) -> bool {
        *self == SOURCE_A::_12
    }
    #[doc = "Checks if the value of the field is `_13`"]
    #[inline(always)]
    pub fn is_13(&self) -> bool {
        *self == SOURCE_A::_13
    }
    #[doc = "Checks if the value of the field is `_14`"]
    #[inline(always)]
    pub fn is_14(&self) -> bool {
        *self == SOURCE_A::_14
    }
    #[doc = "Checks if the value of the field is `_15`"]
    #[inline(always)]
    pub fn is_15(&self) -> bool {
        *self == SOURCE_A::_15
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == SOURCE_A::_16
    }
    #[doc = "Checks if the value of the field is `_17`"]
    #[inline(always)]
    pub fn is_17(&self) -> bool {
        *self == SOURCE_A::_17
    }
    #[doc = "Checks if the value of the field is `_18`"]
    #[inline(always)]
    pub fn is_18(&self) -> bool {
        *self == SOURCE_A::_18
    }
    #[doc = "Checks if the value of the field is `_19`"]
    #[inline(always)]
    pub fn is_19(&self) -> bool {
        *self == SOURCE_A::_19
    }
    #[doc = "Checks if the value of the field is `_20`"]
    #[inline(always)]
    pub fn is_20(&self) -> bool {
        *self == SOURCE_A::_20
    }
    #[doc = "Checks if the value of the field is `_21`"]
    #[inline(always)]
    pub fn is_21(&self) -> bool {
        *self == SOURCE_A::_21
    }
    #[doc = "Checks if the value of the field is `_22`"]
    #[inline(always)]
    pub fn is_22(&self) -> bool {
        *self == SOURCE_A::_22
    }
    #[doc = "Checks if the value of the field is `_23`"]
    #[inline(always)]
    pub fn is_23(&self) -> bool {
        *self == SOURCE_A::_23
    }
    #[doc = "Checks if the value of the field is `_24`"]
    #[inline(always)]
    pub fn is_24(&self) -> bool {
        *self == SOURCE_A::_24
    }
    #[doc = "Checks if the value of the field is `_25`"]
    #[inline(always)]
    pub fn is_25(&self) -> bool {
        *self == SOURCE_A::_25
    }
    #[doc = "Checks if the value of the field is `_26`"]
    #[inline(always)]
    pub fn is_26(&self) -> bool {
        *self == SOURCE_A::_26
    }
    #[doc = "Checks if the value of the field is `_27`"]
    #[inline(always)]
    pub fn is_27(&self) -> bool {
        *self == SOURCE_A::_27
    }
    #[doc = "Checks if the value of the field is `_28`"]
    #[inline(always)]
    pub fn is_28(&self) -> bool {
        *self == SOURCE_A::_28
    }
    #[doc = "Checks if the value of the field is `_29`"]
    #[inline(always)]
    pub fn is_29(&self) -> bool {
        *self == SOURCE_A::_29
    }
    #[doc = "Checks if the value of the field is `_30`"]
    #[inline(always)]
    pub fn is_30(&self) -> bool {
        *self == SOURCE_A::_30
    }
    #[doc = "Checks if the value of the field is `_31`"]
    #[inline(always)]
    pub fn is_31(&self) -> bool {
        *self == SOURCE_A::_31
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == SOURCE_A::_32
    }
    #[doc = "Checks if the value of the field is `_33`"]
    #[inline(always)]
    pub fn is_33(&self) -> bool {
        *self == SOURCE_A::_33
    }
    #[doc = "Checks if the value of the field is `_34`"]
    #[inline(always)]
    pub fn is_34(&self) -> bool {
        *self == SOURCE_A::_34
    }
    #[doc = "Checks if the value of the field is `_35`"]
    #[inline(always)]
    pub fn is_35(&self) -> bool {
        *self == SOURCE_A::_35
    }
    #[doc = "Checks if the value of the field is `_36`"]
    #[inline(always)]
    pub fn is_36(&self) -> bool {
        *self == SOURCE_A::_36
    }
    #[doc = "Checks if the value of the field is `_37`"]
    #[inline(always)]
    pub fn is_37(&self) -> bool {
        *self == SOURCE_A::_37
    }
    #[doc = "Checks if the value of the field is `_38`"]
    #[inline(always)]
    pub fn is_38(&self) -> bool {
        *self == SOURCE_A::_38
    }
    #[doc = "Checks if the value of the field is `_39`"]
    #[inline(always)]
    pub fn is_39(&self) -> bool {
        *self == SOURCE_A::_39
    }
    #[doc = "Checks if the value of the field is `_40`"]
    #[inline(always)]
    pub fn is_40(&self) -> bool {
        *self == SOURCE_A::_40
    }
    #[doc = "Checks if the value of the field is `_41`"]
    #[inline(always)]
    pub fn is_41(&self) -> bool {
        *self == SOURCE_A::_41
    }
    #[doc = "Checks if the value of the field is `_42`"]
    #[inline(always)]
    pub fn is_42(&self) -> bool {
        *self == SOURCE_A::_42
    }
    #[doc = "Checks if the value of the field is `_43`"]
    #[inline(always)]
    pub fn is_43(&self) -> bool {
        *self == SOURCE_A::_43
    }
    #[doc = "Checks if the value of the field is `_44`"]
    #[inline(always)]
    pub fn is_44(&self) -> bool {
        *self == SOURCE_A::_44
    }
    #[doc = "Checks if the value of the field is `_45`"]
    #[inline(always)]
    pub fn is_45(&self) -> bool {
        *self == SOURCE_A::_45
    }
    #[doc = "Checks if the value of the field is `_46`"]
    #[inline(always)]
    pub fn is_46(&self) -> bool {
        *self == SOURCE_A::_46
    }
    #[doc = "Checks if the value of the field is `_47`"]
    #[inline(always)]
    pub fn is_47(&self) -> bool {
        *self == SOURCE_A::_47
    }
    #[doc = "Checks if the value of the field is `_48`"]
    #[inline(always)]
    pub fn is_48(&self) -> bool {
        *self == SOURCE_A::_48
    }
    #[doc = "Checks if the value of the field is `_49`"]
    #[inline(always)]
    pub fn is_49(&self) -> bool {
        *self == SOURCE_A::_49
    }
    #[doc = "Checks if the value of the field is `_50`"]
    #[inline(always)]
    pub fn is_50(&self) -> bool {
        *self == SOURCE_A::_50
    }
    #[doc = "Checks if the value of the field is `_51`"]
    #[inline(always)]
    pub fn is_51(&self) -> bool {
        *self == SOURCE_A::_51
    }
    #[doc = "Checks if the value of the field is `_52`"]
    #[inline(always)]
    pub fn is_52(&self) -> bool {
        *self == SOURCE_A::_52
    }
    #[doc = "Checks if the value of the field is `_53`"]
    #[inline(always)]
    pub fn is_53(&self) -> bool {
        *self == SOURCE_A::_53
    }
    #[doc = "Checks if the value of the field is `_54`"]
    #[inline(always)]
    pub fn is_54(&self) -> bool {
        *self == SOURCE_A::_54
    }
    #[doc = "Checks if the value of the field is `_55`"]
    #[inline(always)]
    pub fn is_55(&self) -> bool {
        *self == SOURCE_A::_55
    }
    #[doc = "Checks if the value of the field is `_56`"]
    #[inline(always)]
    pub fn is_56(&self) -> bool {
        *self == SOURCE_A::_56
    }
    #[doc = "Checks if the value of the field is `_57`"]
    #[inline(always)]
    pub fn is_57(&self) -> bool {
        *self == SOURCE_A::_57
    }
    #[doc = "Checks if the value of the field is `_58`"]
    #[inline(always)]
    pub fn is_58(&self) -> bool {
        *self == SOURCE_A::_58
    }
    #[doc = "Checks if the value of the field is `_59`"]
    #[inline(always)]
    pub fn is_59(&self) -> bool {
        *self == SOURCE_A::_59
    }
    #[doc = "Checks if the value of the field is `_60`"]
    #[inline(always)]
    pub fn is_60(&self) -> bool {
        *self == SOURCE_A::_60
    }
    #[doc = "Checks if the value of the field is `_61`"]
    #[inline(always)]
    pub fn is_61(&self) -> bool {
        *self == SOURCE_A::_61
    }
    #[doc = "Checks if the value of the field is `_62`"]
    #[inline(always)]
    pub fn is_62(&self) -> bool {
        *self == SOURCE_A::_62
    }
    #[doc = "Checks if the value of the field is `_63`"]
    #[inline(always)]
    pub fn is_63(&self) -> bool {
        *self == SOURCE_A::_63
    }
}
#[doc = "Field `SOURCE` writer - DMA Channel Source (Slot)"]
pub type SOURCE_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CHCFG_SPEC, u8, SOURCE_A, 6, O>;
impl<'a, const O: u8> SOURCE_W<'a, O> {
    #[doc = "Disable_Signal"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SOURCE_A::_0)
    }
    #[doc = "UART0_Rx_Signal"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(SOURCE_A::_2)
    }
    #[doc = "UART0_Tx_Signal"]
    #[inline(always)]
    pub fn _3(self) -> &'a mut W {
        self.variant(SOURCE_A::_3)
    }
    #[doc = "UART1_Rx_Signal"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(SOURCE_A::_4)
    }
    #[doc = "UART1_Tx_Signal"]
    #[inline(always)]
    pub fn _5(self) -> &'a mut W {
        self.variant(SOURCE_A::_5)
    }
    #[doc = "UART2_Rx_Signal"]
    #[inline(always)]
    pub fn _6(self) -> &'a mut W {
        self.variant(SOURCE_A::_6)
    }
    #[doc = "UART2_Tx_Signal"]
    #[inline(always)]
    pub fn _7(self) -> &'a mut W {
        self.variant(SOURCE_A::_7)
    }
    #[doc = "UART3_Rx_Signal"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(SOURCE_A::_8)
    }
    #[doc = "UART3_Tx_Signal"]
    #[inline(always)]
    pub fn _9(self) -> &'a mut W {
        self.variant(SOURCE_A::_9)
    }
    #[doc = "UART4_Signal"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(SOURCE_A::_10)
    }
    #[doc = "UART5_Signal"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(SOURCE_A::_11)
    }
    #[doc = "I2S0_Rx_Signal"]
    #[inline(always)]
    pub fn _12(self) -> &'a mut W {
        self.variant(SOURCE_A::_12)
    }
    #[doc = "I2S0_Tx_Signal"]
    #[inline(always)]
    pub fn _13(self) -> &'a mut W {
        self.variant(SOURCE_A::_13)
    }
    #[doc = "SPI0_Rx_Signal"]
    #[inline(always)]
    pub fn _14(self) -> &'a mut W {
        self.variant(SOURCE_A::_14)
    }
    #[doc = "SPI0_Tx_Signal"]
    #[inline(always)]
    pub fn _15(self) -> &'a mut W {
        self.variant(SOURCE_A::_15)
    }
    #[doc = "SPI1_Signal"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(SOURCE_A::_16)
    }
    #[doc = "SPI2_Signal"]
    #[inline(always)]
    pub fn _17(self) -> &'a mut W {
        self.variant(SOURCE_A::_17)
    }
    #[doc = "I2C0_Signal"]
    #[inline(always)]
    pub fn _18(self) -> &'a mut W {
        self.variant(SOURCE_A::_18)
    }
    #[doc = "I2C1_I2C2_Signal"]
    #[inline(always)]
    pub fn _19(self) -> &'a mut W {
        self.variant(SOURCE_A::_19)
    }
    #[doc = "FTM0_Channel0_Signal"]
    #[inline(always)]
    pub fn _20(self) -> &'a mut W {
        self.variant(SOURCE_A::_20)
    }
    #[doc = "FTM0_Channel1_Signal"]
    #[inline(always)]
    pub fn _21(self) -> &'a mut W {
        self.variant(SOURCE_A::_21)
    }
    #[doc = "FTM0_Channel2_Signal"]
    #[inline(always)]
    pub fn _22(self) -> &'a mut W {
        self.variant(SOURCE_A::_22)
    }
    #[doc = "FTM0_Channel3_Signal"]
    #[inline(always)]
    pub fn _23(self) -> &'a mut W {
        self.variant(SOURCE_A::_23)
    }
    #[doc = "FTM0_Channel4_Signal"]
    #[inline(always)]
    pub fn _24(self) -> &'a mut W {
        self.variant(SOURCE_A::_24)
    }
    #[doc = "FTM0_Channel5_Signal"]
    #[inline(always)]
    pub fn _25(self) -> &'a mut W {
        self.variant(SOURCE_A::_25)
    }
    #[doc = "FTM0_Channel6_Signal"]
    #[inline(always)]
    pub fn _26(self) -> &'a mut W {
        self.variant(SOURCE_A::_26)
    }
    #[doc = "FTM0_Channel7_Signal"]
    #[inline(always)]
    pub fn _27(self) -> &'a mut W {
        self.variant(SOURCE_A::_27)
    }
    #[doc = "FTM1_Channel0_Signal"]
    #[inline(always)]
    pub fn _28(self) -> &'a mut W {
        self.variant(SOURCE_A::_28)
    }
    #[doc = "FTM1_Channel1_Signal"]
    #[inline(always)]
    pub fn _29(self) -> &'a mut W {
        self.variant(SOURCE_A::_29)
    }
    #[doc = "FTM2_Channel0_Signal"]
    #[inline(always)]
    pub fn _30(self) -> &'a mut W {
        self.variant(SOURCE_A::_30)
    }
    #[doc = "FTM2_Channel1_Signal"]
    #[inline(always)]
    pub fn _31(self) -> &'a mut W {
        self.variant(SOURCE_A::_31)
    }
    #[doc = "FTM3_Channel0_Signal"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(SOURCE_A::_32)
    }
    #[doc = "FTM3_Channel1_Signal"]
    #[inline(always)]
    pub fn _33(self) -> &'a mut W {
        self.variant(SOURCE_A::_33)
    }
    #[doc = "FTM3_Channel2_Signal"]
    #[inline(always)]
    pub fn _34(self) -> &'a mut W {
        self.variant(SOURCE_A::_34)
    }
    #[doc = "FTM3_Channel3_Signal"]
    #[inline(always)]
    pub fn _35(self) -> &'a mut W {
        self.variant(SOURCE_A::_35)
    }
    #[doc = "FTM3_Channel4_Signal"]
    #[inline(always)]
    pub fn _36(self) -> &'a mut W {
        self.variant(SOURCE_A::_36)
    }
    #[doc = "FTM3_Channel5_Signal"]
    #[inline(always)]
    pub fn _37(self) -> &'a mut W {
        self.variant(SOURCE_A::_37)
    }
    #[doc = "FTM3_Channel6_Signal"]
    #[inline(always)]
    pub fn _38(self) -> &'a mut W {
        self.variant(SOURCE_A::_38)
    }
    #[doc = "FTM3_Channel7_Signal"]
    #[inline(always)]
    pub fn _39(self) -> &'a mut W {
        self.variant(SOURCE_A::_39)
    }
    #[doc = "ADC0_Signal"]
    #[inline(always)]
    pub fn _40(self) -> &'a mut W {
        self.variant(SOURCE_A::_40)
    }
    #[doc = "ADC1_Signal"]
    #[inline(always)]
    pub fn _41(self) -> &'a mut W {
        self.variant(SOURCE_A::_41)
    }
    #[doc = "CMP0_Signal"]
    #[inline(always)]
    pub fn _42(self) -> &'a mut W {
        self.variant(SOURCE_A::_42)
    }
    #[doc = "CMP1_Signal"]
    #[inline(always)]
    pub fn _43(self) -> &'a mut W {
        self.variant(SOURCE_A::_43)
    }
    #[doc = "CMP2_Signal"]
    #[inline(always)]
    pub fn _44(self) -> &'a mut W {
        self.variant(SOURCE_A::_44)
    }
    #[doc = "DAC0_Signal"]
    #[inline(always)]
    pub fn _45(self) -> &'a mut W {
        self.variant(SOURCE_A::_45)
    }
    #[doc = "DAC1_Signal"]
    #[inline(always)]
    pub fn _46(self) -> &'a mut W {
        self.variant(SOURCE_A::_46)
    }
    #[doc = "CMT_Signal"]
    #[inline(always)]
    pub fn _47(self) -> &'a mut W {
        self.variant(SOURCE_A::_47)
    }
    #[doc = "PDB_Signal"]
    #[inline(always)]
    pub fn _48(self) -> &'a mut W {
        self.variant(SOURCE_A::_48)
    }
    #[doc = "PortA_Signal"]
    #[inline(always)]
    pub fn _49(self) -> &'a mut W {
        self.variant(SOURCE_A::_49)
    }
    #[doc = "PortB_Signal"]
    #[inline(always)]
    pub fn _50(self) -> &'a mut W {
        self.variant(SOURCE_A::_50)
    }
    #[doc = "PortC_Signal"]
    #[inline(always)]
    pub fn _51(self) -> &'a mut W {
        self.variant(SOURCE_A::_51)
    }
    #[doc = "PortD_Signal"]
    #[inline(always)]
    pub fn _52(self) -> &'a mut W {
        self.variant(SOURCE_A::_52)
    }
    #[doc = "PortE_Signal"]
    #[inline(always)]
    pub fn _53(self) -> &'a mut W {
        self.variant(SOURCE_A::_53)
    }
    #[doc = "IEEE1588Timer0_Signal"]
    #[inline(always)]
    pub fn _54(self) -> &'a mut W {
        self.variant(SOURCE_A::_54)
    }
    #[doc = "IEEE1588Timer1_Signal"]
    #[inline(always)]
    pub fn _55(self) -> &'a mut W {
        self.variant(SOURCE_A::_55)
    }
    #[doc = "IEEE1588Timer2_Signal"]
    #[inline(always)]
    pub fn _56(self) -> &'a mut W {
        self.variant(SOURCE_A::_56)
    }
    #[doc = "IEEE1588Timer3_Signal"]
    #[inline(always)]
    pub fn _57(self) -> &'a mut W {
        self.variant(SOURCE_A::_57)
    }
    #[doc = "AlwaysOn58_Signal"]
    #[inline(always)]
    pub fn _58(self) -> &'a mut W {
        self.variant(SOURCE_A::_58)
    }
    #[doc = "AlwaysOn59_Signal"]
    #[inline(always)]
    pub fn _59(self) -> &'a mut W {
        self.variant(SOURCE_A::_59)
    }
    #[doc = "AlwaysOn60_Signal"]
    #[inline(always)]
    pub fn _60(self) -> &'a mut W {
        self.variant(SOURCE_A::_60)
    }
    #[doc = "AlwaysOn61_Signal"]
    #[inline(always)]
    pub fn _61(self) -> &'a mut W {
        self.variant(SOURCE_A::_61)
    }
    #[doc = "AlwaysOn62_Signal"]
    #[inline(always)]
    pub fn _62(self) -> &'a mut W {
        self.variant(SOURCE_A::_62)
    }
    #[doc = "AlwaysOn63_Signal"]
    #[inline(always)]
    pub fn _63(self) -> &'a mut W {
        self.variant(SOURCE_A::_63)
    }
}
#[doc = "Field `TRIG` reader - DMA Channel Trigger Enable"]
pub type TRIG_R = crate::BitReader<TRIG_A>;
#[doc = "DMA Channel Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG_A {
    #[doc = "0: Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    _0 = 0,
    #[doc = "1: Triggering is enabled. If triggering is enabled and ENBL is set, the DMAMUX is in Periodic Trigger mode."]
    _1 = 1,
}
impl From<TRIG_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG_A {
        match self.bits {
            false => TRIG_A::_0,
            true => TRIG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRIG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRIG_A::_1
    }
}
#[doc = "Field `TRIG` writer - DMA Channel Trigger Enable"]
pub type TRIG_W<'a, const O: u8> = crate::BitWriter<'a, u8, CHCFG_SPEC, TRIG_A, O>;
impl<'a, const O: u8> TRIG_W<'a, O> {
    #[doc = "Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRIG_A::_0)
    }
    #[doc = "Triggering is enabled. If triggering is enabled and ENBL is set, the DMAMUX is in Periodic Trigger mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRIG_A::_1)
    }
}
#[doc = "Field `ENBL` reader - DMA Channel Enable"]
pub type ENBL_R = crate::BitReader<ENBL_A>;
#[doc = "DMA Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENBL_A {
    #[doc = "0: DMA channel is disabled. This mode is primarily used during configuration of the DMAMux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel."]
    _0 = 0,
    #[doc = "1: DMA channel is enabled"]
    _1 = 1,
}
impl From<ENBL_A> for bool {
    #[inline(always)]
    fn from(variant: ENBL_A) -> Self {
        variant as u8 != 0
    }
}
impl ENBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENBL_A {
        match self.bits {
            false => ENBL_A::_0,
            true => ENBL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENBL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENBL_A::_1
    }
}
#[doc = "Field `ENBL` writer - DMA Channel Enable"]
pub type ENBL_W<'a, const O: u8> = crate::BitWriter<'a, u8, CHCFG_SPEC, ENBL_A, O>;
impl<'a, const O: u8> ENBL_W<'a, O> {
    #[doc = "DMA channel is disabled. This mode is primarily used during configuration of the DMAMux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENBL_A::_0)
    }
    #[doc = "DMA channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENBL_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:5 - DMA Channel Source (Slot)"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - DMA Channel Trigger Enable"]
    #[inline(always)]
    pub fn trig(&self) -> TRIG_R {
        TRIG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Channel Enable"]
    #[inline(always)]
    pub fn enbl(&self) -> ENBL_R {
        ENBL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - DMA Channel Source (Slot)"]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SOURCE_W<0> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 6 - DMA Channel Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trig(&mut self) -> TRIG_W<6> {
        TRIG_W::new(self)
    }
    #[doc = "Bit 7 - DMA Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enbl(&mut self) -> ENBL_W<7> {
        ENBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chcfg](index.html) module"]
pub struct CHCFG_SPEC;
impl crate::RegisterSpec for CHCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [chcfg::R](R) reader structure"]
impl crate::Readable for CHCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chcfg::W](W) writer structure"]
impl crate::Writable for CHCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCFG%s to value 0"]
impl crate::Resettable for CHCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
