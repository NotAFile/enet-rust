// An adaptive order-2 PPM range coder
// translated from enet/compress.c

/*
struct EnetSymbol {
    // binary indexed tree of symbols
    value: u8,
    count: u8,
    under: u8,
    left: u16,
    right: u16,

    // context defined by this symbol
    symbols: u16,
    escapes: u16,
    total: u16,
    parent: u16,
}

struct RangeCoder {
    symbols: [EnetSymbol; 4096]
}

impl RangeCoder {
    fn new() -> EnetRangeCoder {
        RangeCoder { symbols: EnetSymbol[4086] }
    }
}
*/
