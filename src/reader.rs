use std::io::Bytes;
use std::iter::Iterator;
use std::io::Read;
use cp437;

pub struct Reader<'a, R : 'a> {
	buffer: &'a mut Bytes<R>
}

impl<'a, R : Read> Reader<'a, R> {
	pub fn new(buffer : &'a mut Bytes<R>) -> Self {
		Reader {
			buffer: buffer
		}
	}
	
	pub fn consume(&mut self, length : usize) -> String {
		let s : String = self.buffer
			.take(length)
			.filter_map(|x| {
				match x {
					Ok(ref b) => Some(cp437::convert_byte(b)),
					Err(e) => None
				}
			})
			.collect();
			
		s
		//s.trim().to_string()
	}
	
	pub fn foo() -> String { vec!["a", "b"].into_iter().collect() }
	
}