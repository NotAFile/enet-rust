// An adaptive order-2 PPM range coder
// translated from enet/compress.c

/*
use std::cell::Cell;

const CONTEXT_ESCAPE_MINIMUM: u8 = 1;
const CONTEXT_SYMBOL_MINIMUM: u8 = 1;
const RANGE_CODER_TOP: i32 = 1 << 24;
const RANGE_CODER_BOTTOM: i32 = 1 << 16;

trait Compressor {
    #[must_use]
    fn compress(&self, in_slices: &[&[u8]], out_data: &[u8]) -> Result<usize, ()>;
    #[must_use]
    fn decompress(&self, in_data: &[u8], out_data: &[u8]) -> Result<usize, ()>;
}
type symbindex = u16;

#[derive(PartialEq, Default)]
struct Symbol {
    index: symbindex,
    // binary indexed tree of symbols
    value: u8,
    count: u8,
    under: u8,
    left: Option<symbindex>, // index of byte left next to it (?) (offset in original code)
    right: Option<symbindex>,

    // context defined by this symbol
    symbols: Option<symbindex>, // index of some other node
    escapes: u16,
    total: u16,
    parent: u16,
}

impl Symbol {
    fn new(value: u8, count: u8) -> Self {
        Symbol {
            value,
            count,
            under: count,
            ..Default::default()
        }
    }

    fn new_with_context(escapes: u16, minimum: u16) -> Self {
        Symbol {
            symbols: None,
            escapes,
            total: escapes + 255 * minimum,
            parent: 0,
            ..Default::default()
        }
    }
}

struct RangeCoder {
    // symbols: [Symbol; 4096]
    symbols: Vec<Symbol>,
}

impl RangeCoder {
    fn new() -> RangeCoder {
        // RangeCoder { symbols: [Symbol; 4086] }
        RangeCoder {
            symbols: Vec::new(),
        }
    }

    //// Create a symbol and return it's index
    fn create_symbol(&mut self, value: u8, update: u8) -> symbindex {
        let symbol = Symbol::new(value, update);
        self.symbols.push(symbol);
        (self.symbols.len() - 1) as symbindex
    }

    fn get_symbol(&self, index: symbindex) -> &Symbol {
        &self.symbols[index as usize]
    }

    fn range_encode(under: u16, count: u16, total: u16, out: &mut [u8]) {
        let mut write_index = 0;
        let mut encode_low: i32 = 0; // from enclosing
        let mut encode_range: i32 = 0; // from enclosing

        encode_range /= total as i32;

        encode_low += under as i32 * encode_range;
        encode_range *= count as i32;
        loop {
            if encode_low ^ (encode_low + encode_range) >= RANGE_CODER_TOP {
                if encode_range >= RANGE_CODER_BOTTOM {
                    break;
                }
                encode_range = -encode_low & (RANGE_CODER_BOTTOM - 1);
            }
            out[write_index] = (encode_low >> 24) as u8;
            write_index += 1;
            encode_range <<= 8;
            encode_low <<= 8;
        }
    }

    fn encode_context(
        &mut self,
        context: &mut Symbol,
        symbol: symbindex,
        value: u8,
        update: u8,
        minimum: u16,
    ) -> (symbindex, u16, u16) {
        let mut under = value as u16 * minimum ;
        let mut count = minimum;

        let mut symbol_return = symbol;

        match context.symbols {
            None => {
                context.symbols = Some(self.create_symbol(value, update));
                (symbol_return, under, count)
            }
            Some(index) => {
                let mut node = &mut self.symbols[index as usize];
                loop {
                    if value < node.value {
                        node.under += update;
                        if let Some(left_node) = node.left {
                            node = &mut self.symbols[left_node as usize];
                            continue;
                        }
                        node.left = Some(self.create_symbol(value, update))
                    } else if value > node.value {
                        node.under += update;
                        if let Some(right_node) = node.right {
                            node = &mut self.symbols[right_node as usize];
                            continue;
                        }
                        node.right = Some(self.create_symbol(value, update))
                    } else {
                        // value is identical
                        count += node.count as u16;
                        under += (node.under - node.count) as u16;
                        node.under += update;
                        node.count += update;
                        symbol_return = node.index;
                    }
                    break;
                }
                (symbol_return, under, count)
            }
        }
    }
}

impl Compressor for RangeCoder {
    fn compress(&self, in_slices: &[&[u8]], out: &[u8]) -> Result<usize, ()> {
        let encode_low: i32 = 0;
        let encode_range: i32 = !0;

        let root = &Symbol::new(CONTEXT_ESCAPE_MINIMUM, CONTEXT_SYMBOL_MINIMUM);
        let predicted = 0u16;
        let order: usize = 0;
        let next_symbol: usize = 0;

        for slice in in_slices {
            let value_iter = slice.iter();
            let symbol: &Symbol;
            let count: u16;
            let under: u16;
            let total: u16;
            let parent = &predicted;

            loop {
                let subcontext = &self.symbols[predicted as usize];

                if subcontext.index == root.index {
                    break;
                }
            }
        }

        Ok(0)
    }

    fn decompress(&self, in_data: &[u8], out_data: &[u8]) -> Result<usize, ()> {
        Ok(0)
    }
}
*/
