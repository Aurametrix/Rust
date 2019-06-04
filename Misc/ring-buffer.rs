
struct ContiguousAsyncBuffer {
  buf: *mut u8,
  len: usize,
  read: AtomicUsize,
  write: AtomicUsize,
  watermark: AtomicUsize,
}


// [writer thread]
buffer.write.store(buffer.write.load() + write_len)

// [writer thread]
if buffer.len.saturating_sub(buffer.write.load()) >= write_len {
  // not shown: check `read` to make sure there's enough free room
  buffer.watermark.store(buffer.write.load() + write_len);
  buffer.write.store(buffer.write.load() + write_len);
} else { // not enough space, wrap around
  // not shown: check `read` to make sure there's enough free room at the beginning of the buffer
  buffer.watermark.store(buffer.write.load());
  buffer.write.store(0 + write_len);
}


// contra-example
write = 100 (at the start)
write_len = 10 for both threads.
