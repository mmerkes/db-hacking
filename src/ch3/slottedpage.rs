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

const BLOCKS_PER_PAGE: u32 = 8;
const BYTES_PER_BLOCK: u32 = 512;
const PAGE_SIZE: usize = (BLOCKS_PER_PAGE * BYTES_PER_BLOCK) as usize; // 4KiB

// Start of the header
const HEADER_OFFSET: u32 = 0;
// Offset for the number of offsets stored in header
const NUM_OF_OFFSETS_OFFSET: u32 = 1;
// Location in page bytes with the first offset
const FIRST_OFFSET: u32 = 24;

pub fn test() {
    println!("Testing");
}

fn gen_blank_page() -> [u8; PAGE_SIZE] {
    // Initialize an empty byte array
    let blank_page: [u8; PAGE_SIZE] = [0; PAGE_SIZE];

    blank_page
}

fn insert_into_page(key: &str, value: &str, page: [u8; PAGE_SIZE]) {

}

fn _get_num_of_offsets(page: [u8; PAGE_SIZE]) -> u32 {
    0
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
