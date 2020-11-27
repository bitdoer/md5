pub mod lib {
    const S: [u32; 64] = [ 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22,
                           5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20, 5,  9, 14, 20,
                           4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23,
                           6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21];

    const K: [u32; 64] = [0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee,
                          0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
                          0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
                          0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
                          0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 
                          0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8, 
                          0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 
                          0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 
                          0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 
                          0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 
                          0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 
                          0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 
                          0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 
                          0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
                          0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
                          0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391];


    pub fn convert_and_pad(ascii: &String) -> Vec<u32> {
        let mut output = Vec::new();
        let chars = ascii.chars().collect::<Vec<_>>();
        for slice in chars.chunks_exact(4) {
        // stack word into u32 in rev byte order
            output.push((slice[0] as u32)
                    + (slice[1] as u32).rotate_left(8)
                    + (slice[2] as u32).rotate_left(16)
                    + (slice[3] as u32).rotate_left(24));
        }
        let rem = chars.chunks_exact(4).remainder();
        match rem.len() {
            1 => output.push((rem[0] as u32) + 0x00008000),
            2 => output.push((rem[0] as u32)
                        + (rem[1] as u32).rotate_left(8) + 0x00800000),
            3 => output.push((rem[0] as u32)
                        + (rem[1] as u32).rotate_left(8)
                        + (rem[2] as u32).rotate_left(16) + 0x80000000),
            _ => output.push(0x00000080),
        }
        
        let amount_of_padding = ((16 - (output.len() % 16) as isize) - 2).rem_euclid(16);
        for _ in 0..amount_of_padding {
            output.push(0);
        }
        let length = (chars.len() * 8) as u64;
        // push length as usual --- input as LE and length as BE, as here, is isomorphic to the way MD5 does it,
        // which is to store input in straight byte order and length in rev byte order, and then read everything in
        // rev byte order
        output.push(length as u32);
        output.push((length >> 32) as u32);
        output
    }

    pub fn md5_hash(data: Vec<u32>) -> [u32; 4] {
        // set up registers
        let mut a0: u32 = 0x67452301;
        let mut b0: u32 = 0xefcdab89;
        let mut c0: u32 = 0x98badcfe;
        let mut d0: u32 = 0x10325476;
        
        for chunk in data.chunks(16) {
            // copy registers to work with
            let mut a = a0;
            let mut b = b0;
            let mut c = c0;
            let mut d = d0;
            // rounds---each 16 i's is 1 round
            for i in 0..64 {
                let (mut f, g);
                if i <= 15 {
                    f = (b & c) | ((!b) & d);
                    g = i;
                } else if i <= 31 {
                    f = (d & b) | ((!d) & c);
                    g = (5*i + 1) % 16;
                } else if i <= 47 {
                    f = b ^ c ^ d;
                    g = (3*i + 5) % 16;
                } else {
                    f = c ^ (b | (!d));
                    g = (7*i) % 16;
                }
                f = f.wrapping_add(a).wrapping_add(K[i]).wrapping_add(chunk[g]);
                a = d;
                d = c;
                c = b;
                b = b.wrapping_add(f.rotate_left(S[i]));
            }
            // add result to output registers
            a0 = a0.wrapping_add(a);
            b0 = b0.wrapping_add(b);
            c0 = c0.wrapping_add(c);
            d0 = d0.wrapping_add(d);
        }
        // output is LE
        [u32::from_le_bytes(a0.to_be_bytes()), u32::from_le_bytes(b0.to_be_bytes()), u32::from_le_bytes(c0.to_be_bytes()), u32::from_le_bytes(d0.to_be_bytes())]
    }
}