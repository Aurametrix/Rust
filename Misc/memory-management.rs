pub struct Table {
	pub entries: [Entry; 512],
}

impl Table {
	pub fn len() -> usize {
		512
	}
}

pub struct Entry {
	pub entry: i64,
}
impl Entry {
	pub fn is_valid(&self) -> bool {
		self.get_entry() & EntryBits::Valid.val() != 0
	}

	// The first bit (bit index #0) is the V bit for
	// valid.
	pub fn is_invalid(&self) -> bool {
		!self.is_valid()
	}

	// A leaf has one or more RWX bits set
	pub fn is_leaf(&self) -> bool {
		self.get_entry() & 0xe != 0
	}

	pub fn is_branch(&self) -> bool {
		!self.is_leaf()
	}

	pub fn set_entry(&mut self, entry: i64) {
		self.entry = entry;
	}

	pub fn get_entry(&self) -> i64 {
		self.entry
	}
}

pub fn map(root: &mut Table, vaddr: usize, paddr: usize, bits: i64, level: usize) {
	// Make sure that Read, Write, or Execute have been provided
	// otherwise, we'll leak memory and always create a page fault.
	assert!(bits & 0xe != 0);
	// Extract out each VPN from the virtual address
	// On the virtual address, each VPN is exactly 9 bits,
	// which is why we use the mask 0x1ff = 0b1_1111_1111 (9 bits)
	let vpn = [
				// VPN[0] = vaddr[20:12]
				(vaddr >> 12) & 0x1ff,
				// VPN[1] = vaddr[29:21]
				(vaddr >> 21) & 0x1ff,
				// VPN[2] = vaddr[38:30]
				(vaddr >> 30) & 0x1ff,
	];

	// Just like the virtual address, extract the physical address
	// numbers (PPN). However, PPN[2] is different in that it stores
	// 26 bits instead of 9. Therefore, we use,
	// 0x3ff_ffff = 0b11_1111_1111_1111_1111_1111_1111 (26 bits).
	let ppn = [
				// PPN[0] = paddr[20:12]
				(paddr >> 12) & 0x1ff,
				// PPN[1] = paddr[29:21]
				(paddr >> 21) & 0x1ff,
				// PPN[2] = paddr[55:30]
				(paddr >> 30) & 0x3ff_ffff,
	];
  
  
  // We will use this as a floating reference so that we can set
// individual entries as we walk the table.
let mut v = &mut root.entries[vpn[2]];
// Now, we're going to traverse the page table and set the bits
// properly. We expect the root to be valid, however we're required to
// create anything beyond the root.
// In Rust, we create a range iterator using the .. operator.
// The .rev() will reverse the iteration since we need to start with
// VPN[2] The .. operator is inclusive on start but exclusive on end.
// So, (0..2) will iterate 0 and 1.
for i in (level..2).rev() {
	if !v.is_valid() {
		// Allocate a page
		let page = zalloc(1);
		// The page is already aligned by 4,096, so store it
		// directly The page is stored in the entry shifted
		// right by 2 places.
		v.set_entry(
					(page as i64 >> 2)
					| EntryBits::Valid.val(),
		);
	}
	let entry = ((v.get_entry() & !0x3ff) << 2) as *mut Entry;
	v = unsafe { entry.add(vpn[i]).as_mut().unwrap() };
}
// When we get here, we should be at VPN[0] and v should be pointing to
// our entry.
// The entry structure is Figure 4.18 in the RISC-V Privileged
// Specification
let entry = (ppn[2] << 28) as i64 |   // PPN[2] = [53:28]
			(ppn[1] << 19) as i64 |   // PPN[1] = [27:19]
			(ppn[0] << 10) as i64 |   // PPN[0] = [18:10]
			bits |                    // Specified bits, such as User, Read, Write, etc
			EntryBits::Valid.val();   // Valid bit
			// Set the entry. V should be set to the correct pointer by the loop
			// above.
v.set_entry(entry);
