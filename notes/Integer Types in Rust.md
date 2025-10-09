
This is number without any fractional component

### Types of Integer types

1. Unsigned : cannot hold negative numbers
2. Signed : can hold negative numbers , this numbers are stored using the [2's complement](https://en.wikipedia.org/wiki/Two%27s_complement) representation

| Length                 | Signed  | Unsigned |
| ---------------------- | ------- | -------- |
| 8-bit                  | `i8`    | `u8`     |
| 16-bit                 | `i16`   | `u16`    |
| 32-bit                 | `i32`   | `u32`    |
| 64-bit                 | `i64`   | `u64`    |
| 128-bit                | `i128`  | `u128`   |
| architecture dependent | `isize` | `usize`  |

Range
2 ^ 8 = 256
Therefore Unsigned can store about : 0 - 255
Signed can store about : -128 to 127

therefore general formula is this Each signed variant can store numbers from −(2^(n − 1)) to 2^(n − 1) − 1 inclusive, where _n_ is the number of bits that variant uses.

Unsigned variants can store numbers from 0 to 2^n − 1, so a `u8` can store numbers from 0 to 2^8 − 1, which equals 0 to 255.

we cab write integers in any form

|Number literals|Example|
|---|---|
|Decimal|`98_222`|
|Hex|`0xff`|
|Octal|`0o77`|
|Binary|`0b1111_0000`|
|Byte (`u8` only)|`b'A'`|
