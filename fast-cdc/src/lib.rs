#![allow(clippy::unused_io_amount)]
pub use cdchunking;

use cdchunking::ChunkerImpl;
use std::cmp::min;
use std::io::{Result as IoResult, Seek, Write};

const TABLE: [u64; 256] = [
    1553318008, 574654857, 759734804, 310648967, 1393527547, 1195718329, 694400241, 1154184075,
    1319583805, 1298164590, 122602963, 989043992, 1918895050, 933636724, 1369634190, 1963341198,
    1565176104, 1296753019, 1105746212, 1191982839, 1195494369, 29065008, 1635524067, 722221599,
    1355059059, 564669751, 1620421856, 1100048288, 1018120624, 1087284781, 1723604070, 1415454125,
    737834957, 1854265892, 1605418437, 1697446953, 973791659, 674750707, 1669838606, 320299026,
    1130545851, 1725494449, 939321396, 748475270, 554975894, 1651665064, 1695413559, 671470969,
    992078781, 1935142196, 1062778243, 1901125066, 1935811166, 1644847216, 744420649, 2068980838,
    1988851904, 1263854878, 1979320293, 111370182, 817303588, 478553825, 694867320, 685227566,
    345022554, 2095989693, 1770739427, 165413158, 1322704750, 46251975, 710520147, 700507188,
    2104251000, 1350123687, 1593227923, 1756802846, 1179873910, 1629210470, 358373501, 807118919,
    751426983, 172199468, 174707988, 1951167187, 1328704411, 2129871494, 1242495143, 1793093310,
    1721521010, 306195915, 1609230749, 1992815783, 1790818204, 234528824, 551692332, 1930351755,
    110996527, 378457918, 638641695, 743517326, 368806918, 1583529078, 1767199029, 182158924,
    1114175764, 882553770, 552467890, 1366456705, 934589400, 1574008098, 1798094820, 1548210079,
    821697741, 601807702, 332526858, 1693310695, 136360183, 1189114632, 506273277, 397438002,
    620771032, 676183860, 1747529440, 909035644, 142389739, 1991534368, 272707803, 1905681287,
    1210958911, 596176677, 1380009185, 1153270606, 1150188963, 1067903737, 1020928348, 978324723,
    962376754, 1368724127, 1133797255, 1367747748, 1458212849, 537933020, 1295159285, 2104731913,
    1647629177, 1691336604, 922114202, 170715530, 1608833393, 62657989, 1140989235, 381784875,
    928003604, 449509021, 1057208185, 1239816707, 525522922, 476962140, 102897870, 132620570,
    419788154, 2095057491, 1240747817, 1271689397, 973007445, 1380110056, 1021668229, 12064370,
    1186917580, 1017163094, 597085928, 2018803520, 1795688603, 1722115921, 2015264326, 506263638,
    1002517905, 1229603330, 1376031959, 763839898, 1970623926, 1109937345, 524780807, 1976131071,
    905940439, 1313298413, 772929676, 1578848328, 1108240025, 577439381, 1293318580, 1512203375,
    371003697, 308046041, 320070446, 1252546340, 568098497, 1341794814, 1922466690, 480833267,
    1060838440, 969079660, 1836468543, 2049091118, 2023431210, 383830867, 2112679659, 231203270,
    1551220541, 1377927987, 275637462, 2110145570, 1700335604, 738389040, 1688841319, 1506456297,
    1243730675, 258043479, 599084776, 41093802, 792486733, 1897397356, 28077829, 1520357900,
    361516586, 1119263216, 209458355, 45979201, 363681532, 477245280, 2107748241, 601938891,
    244572459, 1689418013, 1141711990, 1485744349, 1181066840, 1950794776, 410494836, 1445347454,
    2137242950, 852679640, 1014566730, 1999335993, 1871390758, 1736439305, 231222289, 603972436,
    783045542, 370384393, 184356284, 709706295, 1453549767, 591603172, 768512391, 854125182,
];

const BLOCK_MIN_SIZE: usize = 1024 * 2; // 2 KB
const BLOCK_AVG_SIZE: usize = 1024 * 8; // 8 KB
const BLOCK_MAX_SIZE: usize = 1024 * 64; // 64 KB
                                         // const BLOCK_MIN_SIZE: usize = 4;
                                         // const BLOCK_AVG_SIZE: usize = 8;
                                         // const BLOCK_MAX_SIZE: usize = 16;

// Empirically derived values where the padded zero bits are almost evenly
// distributed for slightly higher deduplication ratio according to our
// large scale tests
const MASK_SMALL: u64 = 0x0000d90303530000; // 15 '1' bits
const _MASK_AVERAGE: u64 = 0x0000d90303530000; // 13 '1' bits
const MASK_LARGE: u64 = 0x0000d90003530000; // 11 '1' bits

pub fn cut(buffer: &[u8]) -> usize {
    let mut fp = 0;
    let mut i = BLOCK_MIN_SIZE;
    let mut n = buffer.len();
    let mut avg_size = BLOCK_AVG_SIZE;

    if n <= BLOCK_MIN_SIZE {
        return n;
    }

    if n >= BLOCK_MAX_SIZE {
        n = BLOCK_MAX_SIZE;
    } else if n <= BLOCK_AVG_SIZE {
        avg_size = n;
    }

    while i < avg_size {
        fp = (fp << 1) + TABLE[buffer[i] as usize];
        if fp & MASK_SMALL == 0 {
            return i;
        }
        i += 1;
    }

    while i < n {
        fp = (fp << 1) + TABLE[buffer[i] as usize];
        if fp & MASK_LARGE == 0 {
            return i;
        }
        i += 1;
    }

    i
}

const MAX_BUFFER_SIZE: usize = BLOCK_MAX_SIZE * 2;

#[derive(Debug)]
pub struct Chunker<W: Write + Seek> {
    dst: W,       // destination writer
    buf: Vec<u8>, // chunker buffer ()
    len: usize,
}

impl<W: Write + Seek> Chunker<W> {
    /// Create a new Chunker
    pub fn new(dst: W) -> Self {
        Self {
            dst,
            buf: vec![0u8; MAX_BUFFER_SIZE],
            len: 0,
        }
    }

    pub fn into_inner(&self) -> &W {
        &self.dst
    }

    pub fn into_owned(self) -> W {
        self.dst
    }
}

fn cut_without_limit(buffer: &[u8]) -> Option<usize> {
    let mut fp = 0;
    let mut i = BLOCK_MIN_SIZE;
    let mut n = buffer.len();

    if n < BLOCK_MIN_SIZE {
        return None;
    }

    if n >= BLOCK_MAX_SIZE {
        n = BLOCK_MAX_SIZE;
    }
    //  else {
    //     return None;
    // }

    while i < n {
        fp = (fp << 1) + TABLE[buffer[i] as usize];
        if fp & MASK_LARGE == 0 {
            return Some(i);
        }
        i += 1;
    }

    Some(i)
}

impl<W: Write + Seek> Write for Chunker<W> {
    fn write(&mut self, buffer: &[u8]) -> IoResult<usize> {
        if buffer.is_empty() {
            return Ok(0);
        }

        let buf = &mut self.buf;
        let len = self.len;
        let buffer_len = buffer.len();
        let mut data_written = 0;
        let mut pos = 0;

        // copy source data into chunker buffer
        let mut in_len = min(MAX_BUFFER_SIZE - len, buffer_len);
        assert!(in_len > 0);
        buf[len..len + in_len].copy_from_slice(&buffer[..in_len]);
        self.len += in_len;

        // find chunks
        while let Some(cut_pos) = cut_without_limit(&buf[pos..self.len]) {
            data_written += self.dst.write(&buf[pos..pos + cut_pos])?;
            pos += cut_pos;

            let left_len = buf[pos..].len();
            if in_len < buffer_len && left_len < BLOCK_MAX_SIZE {
                // copy data that left in the beginning of the chunker buffer
                buf.copy_within(pos..pos + left_len, 0);
                self.len = left_len;

                // if we have source data, copy them into chunker buffer
                if in_len < buffer_len {
                    let len_to_copy = min(pos, buffer_len - in_len);
                    buf[left_len..left_len + len_to_copy]
                        .copy_from_slice(&buffer[in_len..in_len + len_to_copy]);
                    self.len += len_to_copy;
                    in_len += len_to_copy;
                }

                //
                pos = 0;
            }
        }

        assert!(buffer_len >= data_written);
        self.len = buffer_len - data_written;
        if self.len > 0 {
            buf.copy_within(pos..pos + self.len, 0);
        }

        Ok(buffer_len)
    }

    fn flush(&mut self) -> IoResult<()> {
        // flush remaining data
        if self.len > 0 {
            self.dst.write(&self.buf[0..self.len])?;
        }

        // reset chunker
        self.len = 0;

        // flush destination
        self.dst.flush()
    }
}

pub struct FastCDC {}

impl ChunkerImpl for FastCDC {
    fn find_boundary(&mut self, data: &[u8]) -> Option<usize> {
        Some(cut(data) - 1)
    }

    fn reset(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::Fake;
    use std::cell::RefCell;
    use std::io::Cursor;

    #[test]
    fn cut_test() {
        let buf: [u8; 3] = [0xb, 0xc, 0xff];
        let i = cut(&buf);
        assert_eq!(i, buf.len());
    }

    #[test]
    fn cut_test2() {
        // perpare test data
        let mut data: Vec<u8> = Vec::with_capacity(1075);
        for _ in 0..1024 {
            data.push((0..255).fake::<u8>())
        }

        // Create buf with Write trait
        let buf: Vec<u8> = Vec::with_capacity(1024);
        let mut cursor = RefCell::new(Cursor::new(buf));

        // chunk data
        let mut chunker = Chunker::new(cursor.get_mut());
        let size_written = chunker.write(&data).unwrap();
        assert!(chunker.flush().is_ok()); // Chunker flush work
        assert_eq!(cursor.get_mut().get_ref(), &data); // Data is the same
        assert_eq!(data.len(), size_written); // data len is the same
    }
}
