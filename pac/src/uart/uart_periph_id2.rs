#[doc = "Register `UART_PeriphID2` reader"]
pub struct R(crate::R<UART_PERIPHID2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_PERIPHID2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_PERIPHID2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_PERIPHID2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UART_PeriphID2` reader - UART Peripheral ID 2 register"]
pub struct UART_PERIPHID2_R(crate::FieldReader<u8, u8>);
impl UART_PERIPHID2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UART_PERIPHID2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_PERIPHID2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - UART Peripheral ID 2 register"]
    #[inline(always)]
    pub fn uart_periph_id2(&self) -> UART_PERIPHID2_R {
        UART_PERIPHID2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "UART Peripheral ID 2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_periph_id2](index.html) module"]
pub struct UART_PERIPHID2_SPEC;
impl crate::RegisterSpec for UART_PERIPHID2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_periph_id2::R](R) reader structure"]
impl crate::Readable for UART_PERIPHID2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_PeriphID2 to value 0x34"]
impl crate::Resettable for UART_PERIPHID2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x34
    }
}
