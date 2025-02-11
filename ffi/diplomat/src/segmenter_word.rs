// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use crate::errors::ffi::ICU4XError;
    use crate::provider::ffi::ICU4XDataProvider;
    use alloc::boxed::Box;
    use core::convert::TryFrom;
    use diplomat_runtime::DiplomatResult;
    use icu_provider::DataProvider;
    use icu_segmenter::{
        LstmDataV1Marker, UCharDictionaryBreakDataV1Marker, WordBreakDataV1Marker,
        WordBreakIteratorLatin1, WordBreakIteratorUtf16, WordBreakIteratorUtf8, WordBreakSegmenter,
    };

    #[diplomat::opaque]
    /// An ICU4X word-break segmenter, capable of finding word breakpoints in strings.
    #[diplomat::rust_link(icu_segmenter::WordBreakSegmenter, Struct)]
    pub struct ICU4XWordBreakSegmenter(WordBreakSegmenter);

    #[diplomat::opaque]
    pub struct ICU4XWordBreakIteratorUtf8<'a>(WordBreakIteratorUtf8<'a, 'a>);

    #[diplomat::opaque]
    pub struct ICU4XWordBreakIteratorUtf16<'a>(WordBreakIteratorUtf16<'a, 'a>);

    #[diplomat::opaque]
    pub struct ICU4XWordBreakIteratorLatin1<'a>(WordBreakIteratorLatin1<'a, 'a>);

    impl ICU4XWordBreakSegmenter {
        /// Construct an [`ICU4XWordBreakSegmenter`].
        #[diplomat::rust_link(icu_segmenter::WordBreakSegmenter::try_new, FnInStruct)]
        pub fn try_new(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XWordBreakSegmenter>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();
            Self::try_new_impl(&provider)
        }

        fn try_new_impl<D>(provider: &D) -> DiplomatResult<Box<ICU4XWordBreakSegmenter>, ICU4XError>
        where
            D: DataProvider<WordBreakDataV1Marker>
                + DataProvider<UCharDictionaryBreakDataV1Marker>
                + DataProvider<LstmDataV1Marker>
                + ?Sized,
        {
            WordBreakSegmenter::try_new(provider)
                .map(|o| Box::new(ICU4XWordBreakSegmenter(o)))
                .map_err(Into::into)
                .into()
        }

        /// Segments a UTF-8 string.
        #[diplomat::rust_link(icu_segmenter::WordBreakSegmenter::segment_str, FnInStruct)]
        pub fn segment_utf8<'a>(&'a self, input: &'a str) -> Box<ICU4XWordBreakIteratorUtf8<'a>> {
            Box::new(ICU4XWordBreakIteratorUtf8(self.0.segment_str(input)))
        }

        /// Segments a UTF-16 string.
        #[diplomat::rust_link(icu_segmenter::WordBreakSegmenter::segment_utf16, FnInStruct)]
        pub fn segment_utf16<'a>(
            &'a self,
            input: &'a [u16],
        ) -> Box<ICU4XWordBreakIteratorUtf16<'a>> {
            Box::new(ICU4XWordBreakIteratorUtf16(self.0.segment_utf16(input)))
        }

        /// Segments a Latin-1 string.
        #[diplomat::rust_link(icu_segmenter::WordBreakSegmenter::segment_latin1, FnInStruct)]
        pub fn segment_latin1<'a>(
            &'a self,
            input: &'a [u8],
        ) -> Box<ICU4XWordBreakIteratorLatin1<'a>> {
            Box::new(ICU4XWordBreakIteratorLatin1(self.0.segment_latin1(input)))
        }
    }

    impl<'a> ICU4XWordBreakIteratorUtf8<'a> {
        /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
        /// out of range of a 32-bit signed integer.
        #[allow(clippy::should_implement_trait)]
        pub fn next(&mut self) -> i32 {
            self.0
                .next()
                .and_then(|u| i32::try_from(u).ok())
                .unwrap_or(-1)
        }
    }

    impl<'a> ICU4XWordBreakIteratorUtf16<'a> {
        /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
        /// out of range of a 32-bit signed integer.
        #[allow(clippy::should_implement_trait)]
        pub fn next(&mut self) -> i32 {
            self.0
                .next()
                .and_then(|u| i32::try_from(u).ok())
                .unwrap_or(-1)
        }
    }

    impl<'a> ICU4XWordBreakIteratorLatin1<'a> {
        /// Finds the next breakpoint. Returns -1 if at the end of the string or if the index is
        /// out of range of a 32-bit signed integer.
        #[allow(clippy::should_implement_trait)]
        pub fn next(&mut self) -> i32 {
            self.0
                .next()
                .and_then(|u| i32::try_from(u).ok())
                .unwrap_or(-1)
        }
    }
}
