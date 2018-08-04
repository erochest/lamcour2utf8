mod translate_byte {
    use lamcour::translate_byte;
    use spectral::prelude::*;

    macro_rules! test_char {
        ($lamcour:expr, $unicode:expr, $name:expr) => {{
            assert_that(&translate_byte($lamcour))
                .named($name)
                .is_equal_to(&$unicode);
        }};
    }

    #[test]
    fn test_all_the_things() {
        test_char!(132u8, "\u{02D5}", "U+02D5 MODIFIER LETTER DOWN TACK");
        test_char!(133u8, "\u{02D4}", "U+02D4 MODIFIER LETTER UP TACK");
        test_char!(134u8, "\u{030B}", "U+030B COMBINING DOUBLE ACUTE ACCENT");
        test_char!(135u8, "\u{0319}", "U+0319 COMBINING RIGHT TACK BELOW");
        test_char!(136u8, "\u{0318}", "U+0318 COMBINING LEFT TACK BELOW");
        test_char!(137u8, "\u{0311}", "U+0311 COMBINING INVERTED BREVE");
        test_char!(138u8, "\u{030A}", "U+030A COMBINING RING ABOVE");
        test_char!(139u8, "\u{0302}", "U+0302 COMBINING CIRCUMFLEX ACCENT");
        test_char!(140u8, "\u{0361}", "U+0361 COMBINING DOUBLE INVERTED BREVE");
        test_char!(141u8, "\u{0317}", "U+0317 COMBINING ACUTE ACCENT BELOW");
        test_char!(142u8, "\u{0323}", "U+0323 COMBINING DOT BELOW");
        test_char!(143u8, "\u{0325}", "U+0325 COMBINING RING BELOW");
        test_char!(144u8, "\u{032C}", "U+032C COMBINING CARON BELOW");
        test_char!(145u8, "\u{032F}", "U+032F COMBINING INVERTED BREVE BELOW");
        test_char!(146u8, "\u{032A}", "U+032A COMBINING BRIDGE BELOW");
        test_char!(147u8, "\u{033A}", "U+033A COMBINING INVERTED BRIDGE BELOW");
        test_char!(148u8, "\u{0331}", "U+0331 COMBINING MACRON BELOW");
        test_char!(
            149u8,
            "\u{032B}",
            "U+032B COMBINING INVERTED DOUBLE ARCH BELOW"
        );
        test_char!(150u8, "\u{0329}", "U+0329 COMBINING VERTICAL LINE BELOW");
        test_char!(151u8, "\u{032E}", "U+032E COMBINING BREVE BELOW");
        test_char!(152u8, "\u{031E}", "U+031E COMBINING DOWN TACK BELOW");
        test_char!(153u8, "\u{031D}", "U+031D COMBINING UP TACK BELOW");
        test_char!(154u8, "\u{031F}", "U+031F COMBINING PLUS SIGN BELOW");
        test_char!(
            155u8,
            "\u{032D}",
            "U+032D COMBINING CIRCUMFLEX ACCENT BELOW"
        );
        test_char!(156u8, "\u{0318}", "U+0318 COMBINING LEFT TACK BELOW");
        test_char!(157u8, "\u{0319}", "U+0319 COMBINING RIGHT TACK BELOW");
        test_char!(158u8, "\u{031C}", "U+031C COMBINING LEFT HALF RING BELOW");
        test_char!(159u8, "\u{203F}", "U+203F UNDERTIE");
        test_char!(160u8, "\u{02B2}", "U+02B2 MODIFIER LETTER SMALL J");
        test_char!(161u8, "\u{028F}", "U+028F LATIN LETTER SMALL CAPITAL Y");
        test_char!(162u8, "\u{201D}", "U+201D RIGHT DOUBLE QUOTATION MARK");
        test_char!(163u8, "\u{029A}", "U+029A LATIN SMALL LETTER CLOSED OPEN E");
        test_char!(164u8, "\u{0258}", "U+0258 LATIN SMALL LETTER REVERSED E");
        test_char!(165u8, "\u{0250}", "U+0250 LATIN SMALL LETTER TURNED A");
        test_char!(166u8, "\u{0263}", "U+0263 LATIN SMALL LETTER GAMMA");
        test_char!(167u8, "\u{0027}", "U+0027 APOSTROPHE");
        test_char!(168u8, "\u{0277}", "U+0277 LATIN SMALL LETTER CLOSED OMEGA");
        test_char!(169u8, "\u{026F}", "U+026F LATIN SMALL LETTER TURNED M");
        test_char!(170u8, "\u{0153}", "U+0153 LATIN SMALL LIGATURE OE");
        test_char!(171u8, "\u{02C4}", "U+02C4 MODIFIER LETTER UP ARROWHEAD");
        test_char!(172u8, "\u{0327}", "U+0327 COMBINING CEDILLA");
        test_char!(173u8, "\u{2013}", "U+2013 EN DASH");
        test_char!(174u8, "\u{00AE}", "U+00AE REGISTERED SIGN");
        test_char!(175u8, "\u{006A}", "U+006A LATIN SMALL LETTER J");
        test_char!(176u8, "\u{0259}", "U+0259 LATIN SMALL LETTER SCHWA");
        test_char!(177u8, "\u{026A}", "U+026A LATIN LETTER SMALL CAPITAL I");
        test_char!(178u8, "\u{025B}", "U+025B LATIN SMALL LETTER OPEN E");
        test_char!(
            179u8,
            "\u{025C}",
            "U+025C LATIN SMALL LETTER REVERSED OPEN E"
        );
        test_char!(
            180u8,
            "\u{025A}",
            "U+025A LATIN SMALL LETTER SCHWA WITH HOOK"
        );
        test_char!(181u8, "\u{0251}", "U+0251 LATIN SMALL LETTER ALPHA");
        test_char!(182u8, "\u{0252}", "U+0252 LATIN SMALL LETTER TURNED ALPHA");
        test_char!(183u8, "\u{0254}", "U+0254 LATIN SMALL LETTER OPEN O");
        test_char!(184u8, "\u{028C}", "U+028C LATIN SMALL LETTER TURNED V");
        test_char!(185u8, "\u{028A}", "U+028A LATIN SMALL LETTER UPSILON");
        test_char!(186u8, "\u{00B7}", "U+00B7 MIDDLE DOT");
        test_char!(187u8, "\u{0307}", "U+0307 COMBINING DOT ABOVE");
        test_char!(188u8, "\u{02C2}", "U+02C2 MODIFIER LETTER LEFT ARROWHEAD");
        test_char!(189u8, "\u{02C5}", "U+02C5 MODIFIER LETTER DOWN ARROWHEAD");
        test_char!(190u8, "\u{02C3}", "U+02C3 MODIFIER LETTER RIGHT ARROWHEAD");
        test_char!(191u8, "\u{02D7}", "U+02D7 MODIFIER LETTER MINUS SIGN");
        test_char!(
            192u8,
            "\u{025E}",
            "U+025E LATIN SMALL LETTER CLOSED REVERSED OPEN E"
        );
        test_char!(193u8, "\u{00C6}", "U+00C6 LATIN CAPITAL LETTER AE");
        test_char!(194u8, "\u{0299}", "U+0299 LATIN LETTER SMALL CAPITAL B");
        test_char!(195u8, "\u{02B9}", "U+02B9 MODIFIER LETTER PRIME");
        test_char!(196u8, "\u{00D0}", "U+00D0 LATIN CAPITAL LETTER ETH");
        test_char!(
            197u8,
            "\u{027A}",
            "U+027A LATIN SMALL LETTER TURNED R WITH LONG LEG"
        );
        test_char!(198u8, "\u{0046}", "U+0046 LATIN CAPITAL LETTER F");
        test_char!(
            199u8,
            "\u{00C7}",
            "U+00C7 LATIN CAPITAL LETTER C WITH CEDILLA"
        );
        test_char!(200u8, "\u{029C}", "U+029C LATIN LETTER SMALL CAPITAL H");
        test_char!(201u8, "\u{0269}", "U+0269 LATIN SMALL LETTER IOTA");
        test_char!(
            202u8,
            "\u{00CA}",
            "U+00CA LATIN CAPITAL LETTER E WITH CIRCUMFLEX"
        );
        test_char!(203u8, "\u{0266}", "U+0266 LATIN SMALL LETTER H WITH HOOK");
        test_char!(204u8, "\u{029F}", "U+029F LATIN LETTER SMALL CAPITAL L");
        test_char!(205u8, "\u{02CB}", "U+02CB MODIFIER LETTER GRAVE ACCENT");
        test_char!(
            206u8,
            "\u{0272}",
            "U+0272 LATIN SMALL LETTER N WITH LEFT HOOK"
        );
        test_char!(
            207u8,
            "\u{00D8}",
            "U+00D8 LATIN CAPITAL LETTER O WITH STROKE"
        );
        test_char!(208u8, "\u{00D0}", "U+00D0 LATIN CAPITAL LETTER ETH");
        test_char!(209u8, "\u{0287}", "U+0287 LATIN SMALL LETTER TURNED T");
        test_char!(210u8, "\u{0280}", "U+0280 LATIN LETTER SMALL CAPITAL R");
        test_char!(
            211u8,
            "\u{027C}",
            "U+027C LATIN SMALL LETTER R WITH LONG LEG"
        );
        test_char!(212u8, "\u{00DE}", "U+00DE LATIN CAPITAL LETTER THORN");
        test_char!(213u8, "\u{027a}\u{0336}", "U+027A LATIN SMALL LETTER TURNED R WITH LONG LEG | U+0336 COMBINING LONG STROKE OVERLAY");
        test_char!(
            214u8,
            "\u{0281}",
            "U+0281 LATIN LETTER SMALL CAPITAL INVERTED R"
        );
        test_char!(215u8, "\u{0194}", "U+0194 LATIN CAPITAL LETTER GAMMA");
        test_char!(
            216u8,
            "\u{00D8}",
            "U+00D8 LATIN CAPITAL LETTER O WITH STROKE"
        );
        test_char!(217u8, "\u{028E}", "U+028E LATIN SMALL LETTER TURNED Y");
        test_char!(218u8, "\u{01B7}", "U+01B7 LATIN CAPITAL LETTER EZH");
        test_char!(219u8, "\u{030C}", "U+030C COMBINING CARON");
        test_char!(220u8, "\u{0300}", "U+0300 COMBINING GRAVE ACCENT");
        test_char!(221u8, "\u{0306}", "U+0306 COMBINING BREVE");
        test_char!(
            222u8,
            "\u{0252}\u{032f}",
            "U+0252 LATIN SMALL LETTER TURNED ALPHA | U+032F COMBINING INVERTED BREVE BELOW"
        );
        test_char!(223u8, "\u{0304}", "U+0304 COMBINING MACRON");
        test_char!(224u8, "\u{0308}", "U+0308 COMBINING DIAERESIS");
        test_char!(225u8, "\u{00E6}", "U+00E6 LATIN SMALL LETTER AE");
        test_char!(226u8, "\u{00DF}", "U+00DF LATIN SMALL LETTER SHARP S");
        test_char!(227u8, "\u{0393}", "U+0393 GREEK CAPITAL LETTER GAMMA");
        test_char!(228u8, "\u{00F0}", "U+00F0 LATIN SMALL LETTER ETH");
        test_char!(229u8, "\u{02C8}", "U+02C8 MODIFIER LETTER VERTICAL LINE");
        test_char!(230u8, "\u{03A6}", "U+03A6 GREEK CAPITAL LETTER PHI");
        test_char!(231u8, "\u{0294}", "U+0294 LATIN LETTER GLOTTAL STOP");
        test_char!(232u8, "\u{0265}", "U+0265 LATIN SMALL LETTER TURNED H");
        test_char!(233u8, "\u{02D8}", "U+02D8 BREVE");
        test_char!(234u8, "\u{0301}", "U+0301 COMBINING ACUTE ACCENT");
        test_char!(235u8, "\u{029E}", "U+029E LATIN SMALL LETTER TURNED K");
        test_char!(
            236u8,
            "\u{026B}",
            "U+026B LATIN SMALL LETTER L WITH MIDDLE TILDE"
        );
        test_char!(237u8, "\u{0271}", "U+0271 LATIN SMALL LETTER M WITH HOOK");
        test_char!(238u8, "\u{014B}", "U+014B LATIN SMALL LETTER ENG");
        test_char!(239u8, "\u{00F8}", "U+00F8 LATIN SMALL LETTER O WITH STROKE");
        test_char!(240u8, "\u{00F0}", "U+00F0 LATIN SMALL LETTER ETH");
        test_char!(
            241u8,
            "\u{027E}",
            "U+027E LATIN SMALL LETTER R WITH FISHHOOK"
        );
        test_char!(242u8, "\u{0279}", "U+0279 LATIN SMALL LETTER TURNED R");
        test_char!(243u8, "\u{0283}", "U+0283 LATIN SMALL LETTER ESH");
        test_char!(244u8, "\u{00FE}", "U+00FE LATIN SMALL LETTER THORN");
        test_char!(
            245u8,
            "\u{02CC}",
            "U+02CC MODIFIER LETTER LOW VERTICAL LINE"
        );
        test_char!(246u8, "\u{028B}", "U+028B LATIN SMALL LETTER V WITH HOOK");
        test_char!(247u8, "\u{028D}", "U+028D LATIN SMALL LETTER TURNED W");
        test_char!(248u8, "\u{027D}", "U+027D LATIN SMALL LETTER R WITH TAIL");
        test_char!(249u8, "\u{03B8}", "U+03B8 GREEK SMALL LETTER THETA");
        test_char!(250u8, "\u{0292}", "U+0292 LATIN SMALL LETTER EZH");
        test_char!(251u8, "\u{007E}", "U+007E TILDE");
        test_char!(252u8, "\u{220F}", "U+220F N-ARY PRODUCT");
        test_char!(253u8, "\u{00FD}", "U+00FD LATIN SMALL LETTER Y WITH ACUTE");
        test_char!(254u8, "\u{0303}", "U+0303 COMBINING TILDE");
        test_char!(255u8, "\u{02E1}", "U+02E1 MODIFIER LETTER SMALL L");
    }

    #[test]
    fn test_passes_through_non_alpha_pron_characters() {
        test_char!(1u8, "\u{0001}", "U+0001 <unknown>");
        test_char!(2u8, "\u{0002}", "U+0002 <unknown>");
        test_char!(3u8, "\u{0003}", "U+0003 <unknown>");
        test_char!(4u8, "\u{0004}", "U+0004 <unknown>");
        test_char!(5u8, "\u{0005}", "U+0005 <unknown>");
        test_char!(6u8, "\u{0006}", "U+0006 <unknown>");
        test_char!(7u8, "\u{0007}", "U+0007 <unknown>");
        test_char!(8u8, "\u{0008}", "U+0008 <unknown>");
        test_char!(9u8, "\u{0009}", "U+0009 <unknown>");
        test_char!(10u8, "\u{000A}", "U+000A <unknown>");
        test_char!(11u8, "\u{000B}", "U+000B <unknown>");
        test_char!(12u8, "\u{000C}", "U+000C <unknown>");
        test_char!(13u8, "\u{000D}", "U+000D <unknown>");
        test_char!(14u8, "\u{000E}", "U+000E <unknown>");
        test_char!(15u8, "\u{000F}", "U+000F <unknown>");
        test_char!(16u8, "\u{0010}", "U+0010 <unknown>");
        test_char!(17u8, "\u{0011}", "U+0011 <unknown>");
        test_char!(18u8, "\u{0012}", "U+0012 <unknown>");
        test_char!(19u8, "\u{0013}", "U+0013 <unknown>");
        test_char!(20u8, "\u{0014}", "U+0014 <unknown>");
        test_char!(21u8, "\u{0015}", "U+0015 <unknown>");
        test_char!(22u8, "\u{0016}", "U+0016 <unknown>");
        test_char!(23u8, "\u{0017}", "U+0017 <unknown>");
        test_char!(24u8, "\u{0018}", "U+0018 <unknown>");
        test_char!(25u8, "\u{0019}", "U+0019 <unknown>");
        test_char!(26u8, "\u{001A}", "U+001A <unknown>");
        test_char!(27u8, "\u{001B}", "U+001B <unknown>");
        test_char!(28u8, "\u{001C}", "U+001C <unknown>");
        test_char!(29u8, "\u{001D}", "U+001D <unknown>");
        test_char!(30u8, "\u{001E}", "U+001E <unknown>");
        test_char!(31u8, "\u{001F}", "U+001F <unknown>");
        test_char!(32u8, "\u{0020}", "U+0020 SPACE");
        test_char!(33u8, "\u{0021}", "U+0021 EXCLAMATION MARK");
        test_char!(34u8, "\u{0022}", "U+0022 QUOTATION MARK");
        test_char!(35u8, "\u{0023}", "U+0023 NUMBER SIGN");
        test_char!(36u8, "\u{0024}", "U+0024 DOLLAR SIGN");
        test_char!(37u8, "\u{0025}", "U+0025 PERCENT SIGN");
        test_char!(38u8, "\u{0026}", "U+0026 AMPERSAND");
        test_char!(39u8, "\u{0027}", "U+0027 APOSTROPHE");
        test_char!(40u8, "\u{0028}", "U+0028 LEFT PARENTHESIS");
        test_char!(41u8, "\u{0029}", "U+0029 RIGHT PARENTHESIS");
        test_char!(42u8, "\u{002A}", "U+002A ASTERISK");
        test_char!(43u8, "\u{002B}", "U+002B PLUS SIGN");
        test_char!(44u8, "\u{002C}", "U+002C COMMA");
        test_char!(45u8, "\u{002D}", "U+002D HYPHEN-MINUS");
        test_char!(46u8, "\u{002E}", "U+002E FULL STOP");
        test_char!(47u8, "\u{002F}", "U+002F SOLIDUS");
        test_char!(48u8, "\u{0030}", "U+0030 DIGIT ZERO");
        test_char!(49u8, "\u{0031}", "U+0031 DIGIT ONE");
        test_char!(50u8, "\u{0032}", "U+0032 DIGIT TWO");
        test_char!(51u8, "\u{0033}", "U+0033 DIGIT THREE");
        test_char!(52u8, "\u{0034}", "U+0034 DIGIT FOUR");
        test_char!(53u8, "\u{0035}", "U+0035 DIGIT FIVE");
        test_char!(54u8, "\u{0036}", "U+0036 DIGIT SIX");
        test_char!(55u8, "\u{0037}", "U+0037 DIGIT SEVEN");
        test_char!(56u8, "\u{0038}", "U+0038 DIGIT EIGHT");
        test_char!(57u8, "\u{0039}", "U+0039 DIGIT NINE");
        test_char!(58u8, "\u{003A}", "U+003A COLON");
        test_char!(59u8, "\u{003B}", "U+003B SEMICOLON");
        test_char!(60u8, "\u{003C}", "U+003C LESS-THAN SIGN");
        test_char!(61u8, "\u{003D}", "U+003D EQUALS SIGN");
        test_char!(62u8, "\u{003E}", "U+003E GREATER-THAN SIGN");
        test_char!(63u8, "\u{003F}", "U+003F QUESTION MARK");
        test_char!(64u8, "\u{0040}", "U+0040 COMMERCIAL AT");
        test_char!(91u8, "\u{005B}", "U+005B LEFT SQUARE BRACKET");
        test_char!(92u8, "\u{005C}", "U+005C REVERSE SOLIDUS");
        test_char!(93u8, "\u{005D}", "U+005D RIGHT SQUARE BRACKET");
        test_char!(94u8, "\u{005E}", "U+005E CIRCUMFLEX ACCENT");
        test_char!(95u8, "\u{005F}", "U+005F LOW LINE");
        test_char!(96u8, "\u{0060}", "U+0060 GRAVE ACCENT");
        test_char!(123u8, "\u{007B}", "U+007B LEFT CURLY BRACKET");
        test_char!(124u8, "\u{007C}", "U+007C VERTICAL LINE");
        test_char!(125u8, "\u{007D}", "U+007D RIGHT CURLY BRACKET");
        test_char!(126u8, "\u{007E}", "U+007E TILDE");
        test_char!(127u8, "\u{007F}", "U+007F <unknown>");
        test_char!(128u8, "\u{0080}", "U+0080 <unknown>");
        test_char!(129u8, "\u{0081}", "U+0081 <unknown>");
        test_char!(130u8, "\u{0082}", "U+0082 <unknown>");
        test_char!(131u8, "\u{0083}", "U+0083 <unknown>");
    }

    #[test]
    fn test_translates_latin_letters() {
        test_char!(65u8, "\u{0041}", "U+0041 LATIN CAPITAL LETTER A");
        test_char!(66u8, "\u{0042}", "U+0042 LATIN CAPITAL LETTER B");
        test_char!(67u8, "\u{0043}", "U+0043 LATIN CAPITAL LETTER C");
        test_char!(68u8, "\u{0044}", "U+0044 LATIN CAPITAL LETTER D");
        test_char!(69u8, "\u{0045}", "U+0045 LATIN CAPITAL LETTER E");
        test_char!(70u8, "\u{0046}", "U+0046 LATIN CAPITAL LETTER F");
        test_char!(71u8, "\u{0047}", "U+0047 LATIN CAPITAL LETTER G");
        test_char!(72u8, "\u{0048}", "U+0048 LATIN CAPITAL LETTER H");
        test_char!(73u8, "\u{0049}", "U+0049 LATIN CAPITAL LETTER I");
        test_char!(74u8, "\u{004A}", "U+004A LATIN CAPITAL LETTER J");
        test_char!(75u8, "\u{004B}", "U+004B LATIN CAPITAL LETTER K");
        test_char!(76u8, "\u{004C}", "U+004C LATIN CAPITAL LETTER L");
        test_char!(77u8, "\u{004D}", "U+004D LATIN CAPITAL LETTER M");
        test_char!(78u8, "\u{004E}", "U+004E LATIN CAPITAL LETTER N");
        test_char!(79u8, "\u{004F}", "U+004F LATIN CAPITAL LETTER O");
        test_char!(80u8, "\u{0050}", "U+0050 LATIN CAPITAL LETTER P");
        test_char!(81u8, "\u{0051}", "U+0051 LATIN CAPITAL LETTER Q");
        test_char!(82u8, "\u{0052}", "U+0052 LATIN CAPITAL LETTER R");
        test_char!(83u8, "\u{0053}", "U+0053 LATIN CAPITAL LETTER S");
        test_char!(84u8, "\u{0054}", "U+0054 LATIN CAPITAL LETTER T");
        test_char!(85u8, "\u{0055}", "U+0055 LATIN CAPITAL LETTER U");
        test_char!(86u8, "\u{0056}", "U+0056 LATIN CAPITAL LETTER V");
        test_char!(87u8, "\u{0057}", "U+0057 LATIN CAPITAL LETTER W");
        test_char!(88u8, "\u{0058}", "U+0058 LATIN CAPITAL LETTER X");
        test_char!(89u8, "\u{0059}", "U+0059 LATIN CAPITAL LETTER Y");
        test_char!(90u8, "\u{005A}", "U+005A LATIN CAPITAL LETTER Z");
        test_char!(97u8, "\u{0061}", "U+0061 LATIN SMALL LETTER A");
        test_char!(98u8, "\u{0062}", "U+0062 LATIN SMALL LETTER B");
        test_char!(99u8, "\u{0063}", "U+0063 LATIN SMALL LETTER C");
        test_char!(100u8, "\u{0064}", "U+0064 LATIN SMALL LETTER D");
        test_char!(101u8, "\u{0065}", "U+0065 LATIN SMALL LETTER E");
        test_char!(102u8, "\u{0066}", "U+0066 LATIN SMALL LETTER F");
        test_char!(103u8, "\u{0067}", "U+0067 LATIN SMALL LETTER G");
        test_char!(104u8, "\u{0068}", "U+0068 LATIN SMALL LETTER H");
        test_char!(105u8, "\u{0069}", "U+0069 LATIN SMALL LETTER I");
        test_char!(106u8, "\u{006A}", "U+006A LATIN SMALL LETTER J");
        test_char!(107u8, "\u{006B}", "U+006B LATIN SMALL LETTER K");
        test_char!(108u8, "\u{006C}", "U+006C LATIN SMALL LETTER L");
        test_char!(109u8, "\u{006D}", "U+006D LATIN SMALL LETTER M");
        test_char!(110u8, "\u{006E}", "U+006E LATIN SMALL LETTER N");
        test_char!(111u8, "\u{006F}", "U+006F LATIN SMALL LETTER O");
        test_char!(112u8, "\u{0070}", "U+0070 LATIN SMALL LETTER P");
        test_char!(113u8, "\u{0071}", "U+0071 LATIN SMALL LETTER Q");
        test_char!(114u8, "\u{0072}", "U+0072 LATIN SMALL LETTER R");
        test_char!(115u8, "\u{0073}", "U+0073 LATIN SMALL LETTER S");
        test_char!(116u8, "\u{0074}", "U+0074 LATIN SMALL LETTER T");
        test_char!(117u8, "\u{0075}", "U+0075 LATIN SMALL LETTER U");
        test_char!(118u8, "\u{0076}", "U+0076 LATIN SMALL LETTER V");
        test_char!(119u8, "\u{0077}", "U+0077 LATIN SMALL LETTER W");
        test_char!(120u8, "\u{0078}", "U+0078 LATIN SMALL LETTER X");
        test_char!(121u8, "\u{0079}", "U+0079 LATIN SMALL LETTER Y");
        test_char!(122u8, "\u{007A}", "U+007A LATIN SMALL LETTER Z");
    }
}

mod translate_bytes {
    use lamcour::translate_bytes;
    use spectral::prelude::*;

    #[test]
    fn test_virginia() {
        let input = b"v\xb4d\xfa\xb1nj\xb0";
        let expected = "v\u{025a}d\u{0292}\u{026a}nj\u{0259}".to_string();
        assert_that(&translate_bytes(input)).is_equal_to(&expected);

        let input = b"v\xb1\xbfd\xfa\xb1\xbanj\xb0";
        let expected = "v\u{026a}\u{02d7}d\u{0292}\u{026a}\u{00b7}nj\u{0259}".to_string();
        assert_that(&translate_bytes(input)).is_equal_to(&expected);

        let input = b"v\xb0d\xfa\xb1\xab{\xb0}n\xa0j\xb0";
        let expected = "v\u{0259}d\u{0292}\u{026a}\u{02c4}{\u{0259}}n\u{02b2}j\u{0259}".to_string();
        assert_that(&translate_bytes(input)).is_equal_to(&expected);
    }

    #[test]
    fn test_eight() {
        let input = b"\xb0\xab\xbc\xb1\xbet";
        let expected = "\u{0259}\u{02c4}\u{02c2}\u{026a}\u{02c3}t".to_string();
        assert_that(&translate_bytes(input)).is_equal_to(&expected);

        let input = b"e\xba{\xb1\xbf}t";
        let expected = "e\u{00b7}{\u{026a}\u{02d7}}t".to_string();
        assert_that(&translate_bytes(input)).is_equal_to(&expected);

        let input = b"e\xbat";
        let expected = "e\u{00b7}t".to_string();
        assert_that(&translate_bytes(input)).is_equal_to(&expected);
    }

    #[test]
    fn test_dragonfly() {
        let input = b"sne\xb1\xbfk$d\xb5\xdd\xabkt\xb0";
        let expected = "sne\u{026a}\u{02d7}k$d\u{0251}\u{0306}\u{02c4}kt\u{0259}".to_string();
        assert_that(&translate_bytes(input)).is_equal_to(&expected);

        let input = b"sne\xb1\xbfk$fi\xbad\xb4";
        let expected = "sne\u{026a}\u{02d7}k$fi\u{00b7}d\u{025a}".to_string();
        assert_that(&translate_bytes(input)).is_equal_to(&expected);

        let input = b"\xe5sk\x88i\xbat\x90\xb0\xdd\xf5h\xb7\xbd\xba{\xb0}k";
        let expected = "\u{02c8}sk\u{0318}i\u{00b7}t\u{032c}\u{0259}\u{0306}\u{02cc}h\u{0254}\u{02c5}\u{00b7}{\u{0259}}k".to_string();
        assert_that(&translate_bytes(input)).is_equal_to(&expected);
    }
}
