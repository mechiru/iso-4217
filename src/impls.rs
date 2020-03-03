use crate::impl_curr_code;

impl_curr_code! {
    /// [`CurrencyCode`] represents [`ISO-4217`] currency code enumeration.
    ///
    /// [`CurrencyCode`]: ./enum.CurrencyCode.html
    /// [`ISO-4217`]: https://en.wikipedia.org/wiki/ISO_4217
    #[derive(Debug, PartialEq, Clone, Copy)]
    (pub) enum CurrencyCode {
        AED { num: 784, digit: Some(2), name: "United Arab Emirates dirham", },
        AFN { num: 971, digit: Some(2), name: "Afghan afghani", },
        ALL { num: 008, digit: Some(2), name: "Albanian lek", },
        AMD { num: 051, digit: Some(2), name: "Armenian dram", },
        ANG { num: 532, digit: Some(2), name: "Netherlands Antillean guilder", },
        AOA { num: 973, digit: Some(2), name: "Angolan kwanza", },
        ARS { num: 032, digit: Some(2), name: "Argentine peso", },
        AUD { num: 036, digit: Some(2), name: "Australian dollar", },
        AWG { num: 533, digit: Some(2), name: "Aruban florin", },
        AZN { num: 944, digit: Some(2), name: "Azerbaijani manat", },
        BAM { num: 977, digit: Some(2), name: "Bosnia and Herzegovina convertible mark", },
        BBD { num: 052, digit: Some(2), name: "Barbados dollar", },
        BDT { num: 050, digit: Some(2), name: "Bangladeshi taka", },
        BGN { num: 975, digit: Some(2), name: "Bulgarian lev", },
        BHD { num: 048, digit: Some(3), name: "Bahraini dinar", },
        BIF { num: 108, digit: Some(0), name: "Burundian franc", },
        BMD { num: 060, digit: Some(2), name: "Bermudian dollar", },
        BND { num: 096, digit: Some(2), name: "Brunei dollar", },
        BOB { num: 068, digit: Some(2), name: "Boliviano", },
        BOV { num: 984, digit: Some(2), name: "Bolivian Mvdol (funds code)", },
        BRL { num: 986, digit: Some(2), name: "Brazilian real", },
        BSD { num: 044, digit: Some(2), name: "Bahamian dollar", },
        BTN { num: 064, digit: Some(2), name: "Bhutanese ngultrum", },
        BWP { num: 072, digit: Some(2), name: "Botswana pula", },
        BYN { num: 933, digit: Some(2), name: "Belarusian ruble", },
        BZD { num: 084, digit: Some(2), name: "Belize dollar", },
        CAD { num: 124, digit: Some(2), name: "Canadian dollar", },
        CDF { num: 976, digit: Some(2), name: "Congolese franc", },
        CHE { num: 947, digit: Some(2), name: "WIR Euro (complementary currency)", },
        CHF { num: 756, digit: Some(2), name: "Swiss franc", },
        CHW { num: 948, digit: Some(2), name: "WIR Franc (complementary currency)", },
        CLF { num: 990, digit: Some(4), name: "Unidad de Fomento (funds code)", },
        CLP { num: 152, digit: Some(0), name: "Chilean peso", },
        CNY { num: 156, digit: Some(2), name: "Renminbi (Chinese) yuan", },
        COP { num: 170, digit: Some(2), name: "Colombian peso", },
        COU { num: 970, digit: Some(2), name: "Unidad de Valor Real (UVR) (funds code)", },
        CRC { num: 188, digit: Some(2), name: "Costa Rican colon", },
        CUC { num: 931, digit: Some(2), name: "Cuban convertible peso", },
        CUP { num: 192, digit: Some(2), name: "Cuban peso", },
        CVE { num: 132, digit: Some(2), name: "Cape Verdean escudo", },
        CZK { num: 203, digit: Some(2), name: "Czech koruna", },
        DJF { num: 262, digit: Some(0), name: "Djiboutian franc", },
        DKK { num: 208, digit: Some(2), name: "Danish krone", },
        DOP { num: 214, digit: Some(2), name: "Dominican peso", },
        DZD { num: 012, digit: Some(2), name: "Algerian dinar", },
        EGP { num: 818, digit: Some(2), name: "Egyptian pound", },
        ERN { num: 232, digit: Some(2), name: "Eritrean nakfa", },
        ETB { num: 230, digit: Some(2), name: "Ethiopian birr", },
        EUR { num: 978, digit: Some(2), name: "Euro", },
        FJD { num: 242, digit: Some(2), name: "Fiji dollar", },
        FKP { num: 238, digit: Some(2), name: "Falkland Islands pound", },
        GBP { num: 826, digit: Some(2), name: "Pound sterling", },
        GEL { num: 981, digit: Some(2), name: "Georgian lari", },
        GHS { num: 936, digit: Some(2), name: "Ghanaian cedi", },
        GIP { num: 292, digit: Some(2), name: "Gibraltar pound", },
        GMD { num: 270, digit: Some(2), name: "Gambian dalasi", },
        GNF { num: 324, digit: Some(0), name: "Guinean franc", },
        GTQ { num: 320, digit: Some(2), name: "Guatemalan quetzal", },
        GYD { num: 328, digit: Some(2), name: "Guyanese dollar", },
        HKD { num: 344, digit: Some(2), name: "Hong Kong dollar", },
        HNL { num: 340, digit: Some(2), name: "Honduran lempira", },
        HRK { num: 191, digit: Some(2), name: "Croatian kuna", },
        HTG { num: 332, digit: Some(2), name: "Haitian gourde", },
        HUF { num: 348, digit: Some(2), name: "Hungarian forint", },
        IDR { num: 360, digit: Some(2), name: "Indonesian rupiah", },
        ILS { num: 376, digit: Some(2), name: "Israeli new shekel", },
        INR { num: 356, digit: Some(2), name: "Indian rupee", },
        IQD { num: 368, digit: Some(3), name: "Iraqi dinar", },
        IRR { num: 364, digit: Some(2), name: "Iranian rial", },
        ISK { num: 352, digit: Some(0), name: "Icelandic króna", },
        JMD { num: 388, digit: Some(2), name: "Jamaican dollar", },
        JOD { num: 400, digit: Some(3), name: "Jordanian dinar", },
        JPY { num: 392, digit: Some(0), name: "Japanese yen", },
        KES { num: 404, digit: Some(2), name: "Kenyan shilling", },
        KGS { num: 417, digit: Some(2), name: "Kyrgyzstani som", },
        KHR { num: 116, digit: Some(2), name: "Cambodian riel", },
        KMF { num: 174, digit: Some(0), name: "Comoro franc", },
        KPW { num: 408, digit: Some(2), name: "North Korean won", },
        KRW { num: 410, digit: Some(0), name: "South Korean won", },
        KWD { num: 414, digit: Some(3), name: "Kuwaiti dinar", },
        KYD { num: 136, digit: Some(2), name: "Cayman Islands dollar", },
        KZT { num: 398, digit: Some(2), name: "Kazakhstani tenge", },
        LAK { num: 418, digit: Some(2), name: "Lao kip", },
        LBP { num: 422, digit: Some(2), name: "Lebanese pound", },
        LKR { num: 144, digit: Some(2), name: "Sri Lankan rupee", },
        LRD { num: 430, digit: Some(2), name: "Liberian dollar", },
        LSL { num: 426, digit: Some(2), name: "Lesotho loti", },
        LYD { num: 434, digit: Some(3), name: "Libyan dinar", },
        MAD { num: 504, digit: Some(2), name: "Moroccan dirham", },
        MDL { num: 498, digit: Some(2), name: "Moldovan leu", },
        MGA { num: 969, digit: Some(2), name: "Malagasy ariary", },
        MKD { num: 807, digit: Some(2), name: "Macedonian denar", },
        MMK { num: 104, digit: Some(2), name: "Myanmar kyat", },
        MNT { num: 496, digit: Some(2), name: "Mongolian tögrög", },
        MOP { num: 446, digit: Some(2), name: "Macanese pataca", },
        MRU { num: 929, digit: Some(2), name: "Mauritanian ouguiya", },
        MUR { num: 480, digit: Some(2), name: "Mauritian rupee", },
        MVR { num: 462, digit: Some(2), name: "Maldivian rufiyaa", },
        MWK { num: 454, digit: Some(2), name: "Malawian kwacha", },
        MXN { num: 484, digit: Some(2), name: "Mexican peso", },
        MXV { num: 979, digit: Some(2), name: "Mexican Unidad de Inversion (UDI) (funds code)", },
        MYR { num: 458, digit: Some(2), name: "Malaysian ringgit", },
        MZN { num: 943, digit: Some(2), name: "Mozambican metical", },
        NAD { num: 516, digit: Some(2), name: "Namibian dollar", },
        NGN { num: 566, digit: Some(2), name: "Nigerian naira", },
        NIO { num: 558, digit: Some(2), name: "Nicaraguan córdoba", },
        NOK { num: 578, digit: Some(2), name: "Norwegian krone", },
        NPR { num: 524, digit: Some(2), name: "Nepalese rupee", },
        NZD { num: 554, digit: Some(2), name: "New Zealand dollar", },
        OMR { num: 512, digit: Some(3), name: "Omani rial", },
        PAB { num: 590, digit: Some(2), name: "Panamanian balboa", },
        PEN { num: 604, digit: Some(2), name: "Peruvian sol", },
        PGK { num: 598, digit: Some(2), name: "Papua New Guinean kina", },
        PHP { num: 608, digit: Some(2), name: "Philippine peso", },
        PKR { num: 586, digit: Some(2), name: "Pakistani rupee", },
        PLN { num: 985, digit: Some(2), name: "Polish złoty", },
        PYG { num: 600, digit: Some(0), name: "Paraguayan guaraní", },
        QAR { num: 634, digit: Some(2), name: "Qatari riyal", },
        RON { num: 946, digit: Some(2), name: "Romanian leu", },
        RSD { num: 941, digit: Some(2), name: "Serbian dinar", },
        RUB { num: 643, digit: Some(2), name: "Russian ruble", },
        RWF { num: 646, digit: Some(0), name: "Rwandan franc", },
        SAR { num: 682, digit: Some(2), name: "Saudi riyal", },
        SBD { num: 090, digit: Some(2), name: "Solomon Islands dollar", },
        SCR { num: 690, digit: Some(2), name: "Seychelles rupee", },
        SDG { num: 938, digit: Some(2), name: "Sudanese pound", },
        SEK { num: 752, digit: Some(2), name: "Swedish krona/kronor", },
        SGD { num: 702, digit: Some(2), name: "Singapore dollar", },
        SHP { num: 654, digit: Some(2), name: "Saint Helena pound", },
        SLL { num: 694, digit: Some(2), name: "Sierra Leonean leone", },
        SOS { num: 706, digit: Some(2), name: "Somali shilling", },
        SRD { num: 968, digit: Some(2), name: "Surinamese dollar", },
        SSP { num: 728, digit: Some(2), name: "South Sudanese pound", },
        STN { num: 930, digit: Some(2), name: "São Tomé and Príncipe dobra", },
        SVC { num: 222, digit: Some(2), name: "Salvadoran colón", },
        SYP { num: 760, digit: Some(2), name: "Syrian pound", },
        SZL { num: 748, digit: Some(2), name: "Swazi lilangeni", },
        THB { num: 764, digit: Some(2), name: "Thai baht", },
        TJS { num: 972, digit: Some(2), name: "Tajikistani somoni", },
        TMT { num: 934, digit: Some(2), name: "Turkmenistan manat", },
        TND { num: 788, digit: Some(3), name: "Tunisian dinar", },
        TOP { num: 776, digit: Some(2), name: "Tongan paʻanga", },
        TRY { num: 949, digit: Some(2), name: "Turkish lira", },
        TTD { num: 780, digit: Some(2), name: "Trinidad and Tobago dollar", },
        TWD { num: 901, digit: Some(2), name: "New Taiwan dollar", },
        TZS { num: 834, digit: Some(2), name: "Tanzanian shilling", },
        UAH { num: 980, digit: Some(2), name: "Ukrainian hryvnia", },
        UGX { num: 800, digit: Some(0), name: "Ugandan shilling", },
        USD { num: 840, digit: Some(2), name: "United States dollar", },
        USN { num: 997, digit: Some(2), name: "United States dollar (next day) (funds code)", },
        UYI { num: 940, digit: Some(0), name: "Uruguay Peso en Unidades Indexadas (URUIURUI) (funds code)", },
        UYU { num: 858, digit: Some(2), name: "Uruguayan peso", },
        UYW { num: 927, digit: Some(4), name: "Unidad previsional", },
        UZS { num: 860, digit: Some(2), name: "Uzbekistan som", },
        VES { num: 928, digit: Some(2), name: "Venezuelan bolívar soberano", },
        VND { num: 704, digit: Some(0), name: "Vietnamese đồng", },
        VUV { num: 548, digit: Some(0), name: "Vanuatu vatu", },
        WST { num: 882, digit: Some(2), name: "Samoan tala", },
        XAF { num: 950, digit: Some(0), name: "CFA franc BEAC", },
        XAG { num: 961, digit: None, name: "Silver (one troy ounce)", },
        XAU { num: 959, digit: None, name: "Gold (one troy ounce)", },
        XBA { num: 955, digit: None, name: "European Composite Unit (EURCO) (bond market unit)", },
        XBB { num: 956, digit: None, name: "European Monetary Unit (E.M.U.-6) (bond market unit)", },
        XBC { num: 957, digit: None, name: "European Unit of Account 9 (E.U.A.-9) (bond market unit)", },
        XBD { num: 958, digit: None, name: "European Unit of Account 17 (E.U.A.-17) (bond market unit)", },
        XCD { num: 951, digit: Some(2), name: "East Caribbean dollar", },
        XDR { num: 960, digit: None, name: "Special drawing rights", },
        XOF { num: 952, digit: Some(0), name: "CFA franc BCEAO", },
        XPD { num: 964, digit: None, name: "Palladium (one troy ounce)", },
        XPF { num: 953, digit: Some(0), name: "CFP franc (franc Pacifique)", },
        XPT { num: 962, digit: None, name: "Platinum (one troy ounce)", },
        XSU { num: 994, digit: None, name: "SUCRE", },
        XTS { num: 963, digit: None, name: "Code reserved for testing", },
        XUA { num: 965, digit: None, name: "ADB Unit of Account", },
        XXX { num: 999, digit: None, name: "No currency", },
        YER { num: 886, digit: Some(2), name: "Yemeni rial", },
        ZAR { num: 710, digit: Some(2), name: "South African rand", },
        ZMW { num: 967, digit: Some(2), name: "Zambian kwacha", },
        ZWL { num: 932, digit: Some(2), name: "Zimbabwean dollar", },
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn currency_code_impl_test() {
        let jpy = CurrencyCode::JPY;

        assert_eq!(jpy.alpha(), "JPY");
        assert_eq!(jpy.num(), 392);
        assert_eq!(jpy.name(), "Japanese yen");
        assert_eq!(jpy.digit(), Some(0));
    }

    #[test]
    fn trait_impl_test() -> Result<(), Box<dyn std::error::Error>> {
        use std::convert::{From, TryFrom};

        {
            let code = "JPY";
            let curr: CurrencyCode = code.parse()?;
            assert_eq!(curr, CurrencyCode::JPY);
        }

        {
            let code = "JPY";

            let curr: CurrencyCode = TryFrom::try_from(code)?;
            assert_eq!(curr, CurrencyCode::JPY);

            let v: &str = From::from(curr);
            assert_eq!(v, code);
        }

        {
            let code = 392;

            let curr: CurrencyCode = TryFrom::try_from(code)?;
            assert_eq!(curr, CurrencyCode::JPY);

            let v: u32 = From::from(curr);
            assert_eq!(v, code);
        }

        Ok(())
    }
}
