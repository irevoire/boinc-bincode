use std::io::{self, BufReader, Read};

#[derive(Debug)]
pub struct Truc<T: Read> {
    size: u64,
    reader: BufReader<T>,
}

impl<T: Read> Truc<T> {
    pub fn deserialize(reader: T) -> Truc<T> {
        let mut reader = BufReader::new(reader);
        let mut size = [0_u8; std::mem::size_of::<u64>()];
        reader.read_exact(&mut size).expect("malformed reader");
        let size = u64::from_le_bytes(size);

        Truc { size, reader }
    }

    pub fn get_minute(mut self, n: u64) -> Vec<u64> {
        assert!(
            self.size >= n,
            "queried the a minute that doesn't exists. queried {n} while only {} minutes exists",
            self.size
        );

        let mut i = 0;
        for _ in 0..n {
            println!("skipping the {i} bucket");

            let mut size = [0_u8; std::mem::size_of::<u64>()];
            self.reader.read_exact(&mut size).expect("malformed reader");
            let size = u64::from_le_bytes(size);
            println!("of {size} elements");

            io::copy(
                &mut self
                    .reader
                    .by_ref()
                    .take(size * std::mem::size_of::<u64>() as u64),
                &mut io::sink(),
            )
            .unwrap();
            i += 1;
        }

        println!("getting the {i} bucket");
        let mut size = [0_u8; std::mem::size_of::<u64>()];
        self.reader.read_exact(&mut size).expect("malformed reader");
        let size = u64::from_le_bytes(size);
        println!("contains {size} elements");

        let mut buf = Vec::with_capacity(size as usize);
        io::copy(
            &mut self
                .reader
                .by_ref()
                .take(size * std::mem::size_of::<u64>() as u64),
            &mut buf,
        )
        .unwrap();

        println!("u8 bucket {buf:?}");

        unsafe {
            buf.set_len(buf.len() / 8);
            std::mem::transmute(buf)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read() {
        let base: Vec<Vec<u64>> = vec![
            vec![0, 1, 2, 4, 5],
            vec![2, 1, 4, 3, 4, 5],
            vec![5, 2, 6, 4, 5],
            vec![3, 15, 21, 3, 8, 54],
        ];

        let serialized = bincode::serialize(&base).unwrap();
        println!("{:?}", serialized);

        for minute in 0..base.len() {
            let res = Truc::deserialize(serialized.as_slice());
            dbg!(&res);
            assert_eq!(base[minute], res.get_minute(minute as u64));
        }
    }
}
