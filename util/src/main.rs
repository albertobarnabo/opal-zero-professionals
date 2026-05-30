//! OpalZero Util — Wasm Professional
//!
//! Compiled to `wasm32-wasip1` and executed inside the OpalZero `WasmExecutor`.
//!
//! Generates UUID v4 identifiers and SHA-256 hashes of text.
//!
//! Protocol:
//!   stdin  → JSON: `{"op": "uuid"}` or `{"op": "sha256", "text": "hello"}`
//!   stdout → `"Result: <value>"` on success
//!   stderr → error message + exit(1) on failure

use std::io::Read;

use serde::Deserialize;

#[derive(Deserialize)]
struct UtilArgs {
    op: String,
    text: Option<String>,
}

fn compute(input: &str) -> Result<String, String> {
    let args: UtilArgs =
        serde_json::from_str(input).map_err(|e| format!("Failed to parse util arguments: {}", e))?;

    match args.op.as_str() {
        "uuid" => {
            // Generate a UUID v4 using random bytes
            let uuid = generate_uuid_v4();
            Ok(format!("Result: {}", uuid))
        }
        "sha256" => {
            let text = args
                .text
                .as_deref()
                .ok_or_else(|| "Missing required parameter 'text' for sha256 operation".to_string())?;
            let hash = sha256(text);
            Ok(format!("Result: {}", hash))
        }
        _ => Err(format!("Unknown operation: '{}'. Supported: uuid, sha256", args.op)),
    }
}

/// Generate a UUID v4 (random) identifier.
///
/// Format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
/// where x is any hex digit and y is one of 8, 9, a, or b.
fn generate_uuid_v4() -> String {
    let mut bytes = [0u8; 16];
    get_random_bytes(&mut bytes);

    // Set version (4) in the 4 most significant bits of byte 6 (time_hi_and_version)
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    // Set variant (10xx) in the 2 most significant bits of byte 8 (clock_seq_hi_and_reserved)
    bytes[8] = (bytes[8] & 0x3f) | 0x80;

    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7],
        bytes[8], bytes[9],
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
    )
}

/// Simple SHA-256 implementation using a const-capable approach.
/// Uses the standard FIPS 180-4 SHA-256 algorithm.
fn sha256(input: &str) -> String {
    // Initial hash values (H0-H7) — first 32 bits of the fractional parts
    // of the square roots of the first 8 primes
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a,
        0x510e527f, 0x9b05688c, 0x1f83d9ab, 0x5be0cd19,
    ];

    // Round constants (K0-K63) — first 32 bits of the fractional parts
    // of the cube roots of the first 64 primes
    let k: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5,
        0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3,
        0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc,
        0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13,
        0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3,
        0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5,
        0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208,
        0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2,
    ];

    // Pre-processing: append 0x80, then zeros, then length in bits as 64-bit big-endian
    let msg = input.as_bytes();
    let msg_len_bits = (msg.len() as u64) * 8;

    // Message padding: we'll work with blocks of 64 bytes
    // Pad length = (55 - msg.len() % 64 + 64) % 64 + 1 + 8 for the length field
    let pad_len = if msg.len() % 64 < 56 {
        56 - msg.len() % 64
    } else {
        120 - msg.len() % 64
    };
    let total_len = msg.len() + pad_len + 8;
    let mut padded = Vec::with_capacity(total_len);
    padded.extend_from_slice(msg);
    padded.push(0x80);
    padded.extend(std::iter::repeat(0).take(pad_len - 1));
    // Append length in bits as big-endian u64
    padded.extend_from_slice(&msg_len_bits.to_be_bytes());

    // Process each 64-byte (512-bit) block
    for chunk in padded.chunks(64) {
        let mut w = [0u32; 64];

        // Prepare the message schedule (w[0..15])
        for t in 0..16 {
            w[t] = u32::from_be_bytes([
                chunk[t * 4],
                chunk[t * 4 + 1],
                chunk[t * 4 + 2],
                chunk[t * 4 + 3],
            ]);
        }

        // Extend the 16-word block into 64 words
        for t in 16..64 {
            let s0 = w[t - 15].rotate_right(7) ^ w[t - 15].rotate_right(18) ^ (w[t - 15] >> 3);
            let s1 = w[t - 2].rotate_right(17) ^ w[t - 2].rotate_right(19) ^ (w[t - 2] >> 10);
            w[t] = w[t - 16]
                .wrapping_add(s0)
                .wrapping_add(w[t - 7])
                .wrapping_add(s1);
        }

        // Initialize working variables
        let (mut a, mut b, mut c, mut d) = (h[0], h[1], h[2], h[3]);
        let (mut e, mut f, mut g, mut hh) = (h[4], h[5], h[6], h[7]);

        // Compression function main loop
        for t in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(k[t])
                .wrapping_add(w[t]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        // Compute the intermediate hash values
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    // Produce the final hash as hex string
    h.iter()
        .map(|word| format!("{:08x}", word))
        .collect::<Vec<_>>()
        .concat()
}

fn get_random_bytes(buf: &mut [u8]) {
    // On wasm32-wasip1, use the getrandom via Wasi
    // For Linux/macOS fallback, use /dev/urandom
    #[cfg(target_os = "wasi")]
    {
        use std::fs::File;
        let mut f = File::open("/dev/urandom").unwrap_or_else(|_| {
            // Fallback: use a simple seed from time + address
            let seed = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            fill_seeded(buf, seed);
            return;
        });
        use std::io::Read;
        f.read_exact(buf).ok();
    }

    #[cfg(not(target_os = "wasi"))]
    {
        fill_seeded(buf, rand_seed());
    }
}

fn rand_seed() -> u128 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}

fn fill_seeded(buf: &mut [u8], seed: u128) {
    let mut state = seed;
    for byte in buf.iter_mut() {
        // Simple xorshift-style PRNG
        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;
        *byte = (state & 0xff) as u8;
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    match compute(input.trim()) {
        Ok(result) => print!("{}", result),
        Err(e) => {
            eprintln!("Util error: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uuid_format() {
        let uuid = generate_uuid_v4();
        assert_eq!(uuid.len(), 36);
        assert_eq!(&uuid[14..15], "4"); // version nibble
        let variant = uuid[19..20].chars().next().unwrap();
        assert!(
            variant == '8' || variant == '9' || variant == 'a' || variant == 'b',
            "UUID variant nibble should be 8/9/a/b, got {}",
            variant
        );
    }

    #[test]
    fn test_uuid_uniqueness() {
        let a = generate_uuid_v4();
        let b = generate_uuid_v4();
        assert_ne!(a, b, "Two UUIDs should not collide");
    }

    #[test]
    fn test_sha256_empty() {
        // SHA-256 of empty string
        let result = sha256("");
        assert_eq!(
            result,
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
        );
    }

    #[test]
    fn test_sha256_hello() {
        // SHA-256 of "hello"
        let result = sha256("hello");
        assert_eq!(
            result,
            "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_sha256_known() {
        // SHA-256 of "abc"
        let result = sha256("abc");
        assert_eq!(
            result,
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn test_compute_uuid() {
        let result = compute(r#"{"op": "uuid"}"#).unwrap();
        assert!(result.starts_with("Result: "));
        assert_eq!(result.len(), 43); // "Result: " + 36-char UUID
    }

    #[test]
    fn test_compute_sha256() {
        let result = compute(r#"{"op": "sha256", "text": "hello"}"#).unwrap();
        assert_eq!(
            result,
            "Result: 2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"
        );
    }

    #[test]
    fn test_compute_sha256_missing_text() {
        let err = compute(r#"{"op": "sha256"}"#).unwrap_err();
        assert!(err.contains("Missing required parameter 'text'"));
    }

    #[test]
    fn test_compute_unknown_op() {
        let err = compute(r#"{"op": "md5"}"#).unwrap_err();
        assert!(err.contains("Unknown operation"));
    }

    #[test]
    fn test_compute_invalid_json() {
        let err = compute("not json").unwrap_err();
        assert!(err.contains("Failed to parse"));
    }
}
