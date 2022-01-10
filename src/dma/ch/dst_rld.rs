#[doc = "Register `DST_RLD` reader"]
pub struct R(crate::R<DST_RLD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DST_RLD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DST_RLD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DST_RLD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DST_RLD` writer"]
pub struct W(crate::W<DST_RLD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DST_RLD_SPEC>;
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
impl From<crate::W<DST_RLD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DST_RLD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DST_RLD` reader - Destination Address Reload Value."]
pub struct DST_RLD_R(crate::FieldReader<u32, u32>);
impl DST_RLD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        DST_RLD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DST_RLD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DST_RLD` writer - Destination Address Reload Value."]
pub struct DST_RLD_W<'a> {
    w: &'a mut W,
}
impl<'a> DST_RLD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | (value as u32 & 0x7fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:30 - Destination Address Reload Value."]
    #[inline(always)]
    pub fn dst_rld(&self) -> DST_RLD_R {
        DST_RLD_R::new((self.bits & 0x7fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:30 - Destination Address Reload Value."]
    #[inline(always)]
    pub fn dst_rld(&mut self) -> DST_RLD_W {
        DST_RLD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Destination Address Reload Value. The value of this register is loaded into DMA0_DST upon a count-to-zero condition.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dst_rld](index.html) module"]
pub struct DST_RLD_SPEC;
impl crate::RegisterSpec for DST_RLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dst_rld::R](R) reader structure"]
impl crate::Readable for DST_RLD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dst_rld::W](W) writer structure"]
impl crate::Writable for DST_RLD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DST_RLD to value 0"]
impl crate::Resettable for DST_RLD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
