# 8XP File Structure Documentation

The documentation on TI-BASIC bytecode (8XP) files and how they're created/parsed is very little to none, so I figured I'd create a comprehensive documentation here for anyone in the future looking to do this.

## Character Encoding

Whenever text is used in the file, for example, writing out a program name or comment within the file header, it's encoded with standard ASCII. So, if the name of the program is `TEST`, the respective ASCII will be `0x84 0x69 0x83 0x84`

## Data Portion

Some entries in the table will mention the "Data Portion" of the 8XP file. This is the portion of the bytecode which actually contains the instructions. The data portion starts after the header (74 bytes long) and ends before the footer (last 2 bytes). The length of the data portion is just the number of bytes within it.

## Storing Integers

Since many integers are greater than 255 (the max integer that can be contained within one byte), the 8XP format stores them as two bytes. The first byte in the sequence is the size byte, and the second byte is the carry byte. For example, 257 would be stored as (in decimal) `2 1`. We multiply the carry byte by 255 and then add the sum byte to get the total, in this case being 2+255. Storing numbers less than 256 works the same way, with the carry byte just being 0. For example, 85 would be `85 0`. If the integer is big enough for the carry byte to go over 255, the carry byte should just overflow to 0. As an example implementation, here is my Rust implementation for converting base-10 integers to this format:

```rs
pub fn int_to_bytes(size: usize) -> [u8; 2] {
    let mut bytes: [u8; 2] = [0; 2];

    // size byte
    bytes[0] = (size & 0xFF) as u8;
    // carry byte
    bytes[1] = ((size >> 8) & 0xFF) as u8;

    bytes
}
```

## File Header

The first part of an 8XP file, obviously, is the header. This section contains metadata about the program. I've chosen to organize the bytes into a table and group them by byte-range, so that you can skip to any particular range that you're not sure about and use it as a reference.

| Byte Range | Field                    | Description                                                                                                                                                                                                                                                            | Example (Decimal)                                                                                                                           | Example (Hex)                                                                                                                                                                                                       | ASCII Equivalent (if applicable)      |
| ---------- | ------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------- |
| 0-9        | File Header              | This is a 10-byte file header that is the same for every 8XP file                                                                                                                                                                                                      | `42 42 84 73 56 51 70 42 26 10`                                                                                                             | `0x2A 0x2A 0x54 0x49 0x38 0x33 0x46 0x2A 0x1A 0xA`                                                                                                                                                                  |                                       |
| 10         | Null Byte                | A null byte that marks the beginning of the program comment                                                                                                                                                                                                            | `0`                                                                                                                                         | `0x00`                                                                                                                                                                                                              |                                       |
| 11-54      | Program Comment          | A 42-byte long ASCII comment about the program. This can be anything at all. If it is shorter than 42 characters, you can pad the right side with null bytes until it does reach a length of 42.                                                                       | `83 105 110 103 108 101 32 102 105 108 101 32 100 97 116 101 100 32 84 104 117 32 77 97 121 32 49 51 32 49 49 58 49 54 58 50 54 32 0 0 0 0` | `0x53 0x69 0x6E 0x67 0x6C 0x65 0x20 0x66 0x69 0x6C 0x65 0x20 0x64 0x61 0x74 0x65 0x64 0x20 0x54 0x68 0x75 0x20 0x4D 0x61 0x79 0x20 0x31 0x33 0x20 0x31 0x31 0x3A 0x31 0x36 0x3A 0x32 0x36 0x20 0x00 0x00 0x00 0x00` | Single file dated Thu May 13 11:16:26 |
| 53-56      | Data Checksum            | This is a checksum of the data portion of the 8XP file. The checksum is calculated by taking the length of the [data portion](#data-portion) and adding 19. Then, this should be converted into two bytes in the [carry format](#storing-integers).                    | `41 2` (data length of 532, checksum value of 551)                                                                                          | `0x29 0x02` (data length of 532, checksum value of 551)                                                                                                                                                             |                                       |
| 55-58      | ???                      | I really don't know what these bytes are for, but they're needed.                                                                                                                                                                                                      | `13 0`                                                                                                                                      | `0xD 0x00`                                                                                                                                                                                                          |                                       |
| 57-60      | Data Checksum            | Another data checksum like the one above, however it's slightly different. This checksum is calculated by taking the length of the [data portion](#data-portion) and adding 2. Then, this should be converted into two bytes in the [carry format](#storing-integers). | `24 2` (data length of 532, checksum value of 534)                                                                                          | `0x18 0x02` (data length of 532, checksum value of 534)                                                                                                                                                             |                                       |
| 59         | [File Type](#file-types) | The type of file. See the table below for more details.                                                                                                                                                                                                                | `5`, `6`                                                                                                                                    | `0x05`, `0x06`, etc                                                                                                                                                                                                 |                                       |
| 60-69      | Truncated Program Name   | The all-uppercase name of the program, truncated to 8 bytes. If shorter than 8 bytes, it should be null-padded on the right.                                                                                                                                           | `84 69 83 84 00 00 00 00`                                                                                                                   | `0x54 0x45 0x53 0x54 0x00 0x00 0x00 0x00`                                                                                                                                                                           | TEST                                  |
| 68         | Version                  | I'm not sure what this actually means, but it's usually `0x00`. Apparently this field is present if byte 55 is `0x0D`. [Source](https://gist.github.com/SimonEast/244a0fd04526ea1acbec2e2ceb2e7924?utm_source=chatgpt.com#file-8xp-file-format-ksy-L131)               | `00`                                                                                                                                        | `0x00`                                                                                                                                                                                                              |                                       |
| 69         | Archived Flag            | Whether or not this program is archived. `0x00` for normal programs, `0x80` for archived.                                                                                                                                                                              | `00` OR `128`                                                                                                                               | `0x00` OR `0x80`                                                                                                                                                                                                    |                                       |
| 70-73      | Program Size             | The size of the data section. This time, there is nothing extra added. This should be converted into two bytes in the [carry format](#storing-integers).                                                                                                               | `22 2` (data length of 532)                                                                                                                 | `0x16 0x02` (data length of 532)                                                                                                                                                                                    |                                       |
| 72-75      | Program Size - 2         | The size of the data section minus 2. This should be converted into two bytes in the [carry format](#storing-integers).                                                                                                                                                | `20 2` (data length of 532, resulting value of 530)                                                                                         | `0x14 0x02` (data length of 532, resulting value of 530)                                                                                                                                                            |                                       |

### File Types

| Hex Value | Variable Type        | Notes                                                                                                       |
| --------- | -------------------- | ----------------------------------------------------------------------------------------------------------- |
| `0x00`    | Real                 |                                                                                                             |
| `0x01`    | List                 |                                                                                                             |
| `0x02`    | Matrix               |                                                                                                             |
| `0x03`    | Y-Var/Equation       | Older calculators, such as the 82, store equations as "Y-Vars", which are upgraded to "Equations" on the 84 |
| `0x04`    | String               |                                                                                                             |
| `0x05`    | Program              |                                                                                                             |
| `0x06`    | Locked Program       | Assembly programs are usually set to this type                                                              |
| `0x07`    | Picture              |                                                                                                             |
| `0x08`    | Graph Database (GDB) |                                                                                                             |
| `0x0C`    | Complex              |                                                                                                             |
| `0x0F`    | Window setup         |                                                                                                             |
| `0x10`    | Zoom Settings        |                                                                                                             |
| `0x11`    | Table Setup          |                                                                                                             |
| `0x13`    | Backup               |                                                                                                             |
| `0x15`    | App Vars             |                                                                                                             |
| `0x17`    | Group                |                                                                                                             |
| `0x24`    | Flash Application    |                                                                                                             |

## File Footer (checksum)

The last 2 bytes in an 8XP file are the checksum of the file. This checksum is very simple if you read these directions carefully. Take the second half of the header, starting at the 55th byte (These are the two unknown bytes, `0xD` and `0x0`), and sum up all of the byte values. Then, sum up the byte values of the data portion of the program. Now, add these two numbers together, and convert them into the integer [carry format](#storing-integers).

### Example

Let's pretend that the header starting at byte 55 is as follows:

```asm
0x0D 0x00 0x18 0x02 0x05 0x54 0x45 0x53 0x54 0x00 0x00 0x00 0x00 0x00 0x00 0x16 0x02 0x14 0x02
```

Summing up all of these values, we get 410. Next, we need to sum up the data portion of the program. Let's use this as an example:

```asm
0xE1 0x3F 0xDC 0x2A 0x57 0x45 0x49 0x47 0x48 0x54 0x29 0x2A 0x2B 0x57 0x3F 0xDE 0x57
```

Summing up all of these values, we get 1591. If we add this to the header sum of 410, we get 2001. To find the value in the carry format, we can use `2001 & 0xFF` to get the size byte (209) and `(2001 >> 8) & 0xFF` to get the carry byte (7). Therefore, the checksum would be `209 7` in decimal or `0xD1 0x07` in hex.
