// Slotted Page Layout
// | header 24 bytes | offets variable | free space | cells |
//
// Header Layout
// 0 flags
// 1-4 Number of offsets
//
// Cell Layout
// 0 flags
// 1-4 key size
// 5-8 value size
// 9+ key bytes
// x+ value bytes
use std::mem::transmute;

const BLOCKS_PER_PAGE: usize = 8;
const BYTES_PER_BLOCK: usize = 512;
const PAGE_SIZE: usize = BLOCKS_PER_PAGE * BYTES_PER_BLOCK; // 4KiB

// Offset for the number of cells
const NUM_OF_CELLS_OFFSET: usize = 1;
// Size of header
const HEADER_SIZE: usize = 24;
// Number of bytes per offset
const BYTES_PER_OFFSET: usize = 4;

// Cell offsets
const CELL_KEY_SIZE_OFFSET: usize = 1;
const CELL_VALUE_SIZE_OFFSET: usize = 5;
const CELL_KEY_OFFSET: usize = 9;
const CELL_METADATA_SIZE: usize = 9;

pub fn test() {
    println!("Testing");
}

fn gen_blank_page() -> [u8; PAGE_SIZE] {
    // Initialize an empty byte array
    let blank_page: [u8; PAGE_SIZE] = [0; PAGE_SIZE];

    blank_page
}

fn _insert_into_page(key: &str, value: &str, page: &mut [u8; PAGE_SIZE]) {
    let num_of_cells = _get_num_of_cells(*page);

    let flags: u8 = 0;
    let key_size = key.len();
    let value_size = value.len();
    // Need to update
    let cell_offset = PAGE_SIZE - (CELL_METADATA_SIZE + key_size + value_size + 1);

    // Insert record
    page[cell_offset] = 0; // Empty flag right now
    _write_int(page, cell_offset + CELL_KEY_SIZE_OFFSET, _to_u32(key_size));
    _write_int(page, cell_offset + CELL_VALUE_SIZE_OFFSET, _to_u32(value_size));
    let key_offset = cell_offset + CELL_KEY_OFFSET;
    for (i, c) in key.chars().enumerate() {
        page[key_offset + i] = c as u8;
    }
    let value_offset = key_offset + key_size;
    for (i, c) in value.chars().enumerate() {
        page[value_offset + i] = c as u8;
    }

    // Update offset counts and references
    _set_num_of_cells(page, num_of_cells + 1);
    _write_int(page, HEADER_SIZE, _to_u32(cell_offset));
}

fn _read_from_page(key: &str, page: [u8; PAGE_SIZE]) -> String {
    let cell_count = _get_num_of_cells(page) as usize;

    // Replace with binary search
    for i in 0..cell_count {
        let cell_offset = _read_int(page, HEADER_SIZE + i * BYTES_PER_OFFSET) as usize;
        let key_size = _read_int(page, cell_offset + CELL_KEY_SIZE_OFFSET) as usize;
        let key_offset = cell_offset + CELL_KEY_OFFSET;

        let cell_key = _read_string(page, key_offset, key_size);
        let value = if cell_key == key {
            let value_offset = key_offset + key_size;
            let value_size = _read_int(page, cell_offset + CELL_VALUE_SIZE_OFFSET) as usize;
            _read_string(page, value_offset, value_size)
        } else {
            continue;
        };

        return value;
    }

    panic!("Could not find key {}", key);
}

fn _read_string(page: [u8; PAGE_SIZE], string_offset: usize, string_size: usize) -> String {
    let mut s: Vec<u8> = Vec::new();
    for i in 0..string_size {
        s.push(page[string_offset + i]);
    }
    std::str::from_utf8(&s)
        .unwrap()
        .to_string()
}

fn _to_u32(n: usize) -> u32 {
    n.try_into().unwrap()
}

fn _get_num_of_cells(page: [u8; PAGE_SIZE]) -> u32 {
    _read_int(page, NUM_OF_CELLS_OFFSET)
}

fn _set_num_of_cells(page: &mut [u8; PAGE_SIZE], num: u32) {
    _write_int(page, NUM_OF_CELLS_OFFSET, num)
}

fn write_page(filename: &str, offset: u32, bytes: [u8; PAGE_SIZE]) {

}

fn read_page(filename: &str, offset: u32) -> [u8; PAGE_SIZE] {
    // Initialize an empty byte array
    let blank_page: [u8; PAGE_SIZE] = [0; PAGE_SIZE];

    blank_page
}

fn _write_int(page: &mut [u8; PAGE_SIZE], offset: usize, num: u32) {
    let bytes = unsafe {
        transmute::<u32, [u8; 4]>(num.to_le())
    };

    for i in 0..4 {
        page[offset + i] = bytes[i];
    }
}

fn _read_int(page: [u8; PAGE_SIZE], offset: usize) -> u32 {
    unsafe {
        transmute::<[u8; 4], u32>([page[offset], page[offset + 1], page[offset + 2], page[offset + 3]]) 
    }.to_le()
}

#[test]
fn test_slotted_page() {
    let blank_page = gen_blank_page();
}

#[test]
fn _read_int_should_get_int_from_bytes() {
    let mut page = gen_blank_page();
    // Verify that blank page returns zero at whatever point
    assert_eq!(0, _read_int(page, page.len() - 20));

    // Verify max number when set
    let first_num = page.len() - 20;
    for i in 0..4 {
        page[first_num + i] = u8::MAX;
    }
    assert_eq!(u32::MAX, _read_int(page, first_num));

    // Verify endianness is right
    let second_num = page.len() - 20;
    page[second_num] = 1;
    for i in 1..4 {
        page[second_num + i] = 0;
    }
    assert_eq!(1, _read_int(page, second_num));
}

#[test]
fn _write_int_should_write_bytes_from_int() {
    let mut page = gen_blank_page();
    let offset = page.len() - 20;
    // Verify that zero is written correctly
    _write_int(&mut page, offset, 0);
    assert_eq!(0, _read_int(page, offset));

    // Verify max number is written correctly
    _write_int(&mut page, offset, u32::MAX);
    assert_eq!(u32::MAX, _read_int(page, offset));

    // Verify endianness is right
    _write_int(&mut page, offset, 1);
    assert_eq!(1, _read_int(page, offset));
}

#[test]
fn _insert_into_page_should_insert_entries_to_page() {
    let mut page = gen_blank_page();
    let key = "some-key";
    let value = "some-value";

    _insert_into_page(key, value, &mut page);

    let result = _read_from_page(key, page);
    assert_eq!(value, result);
}
