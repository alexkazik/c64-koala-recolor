pub(crate) struct Koala<'a> {
    file: &'a mut [u8; 10003],
}

impl<'a> Koala<'a> {
    #[inline]
    pub(crate) fn new(file: &'a mut [u8]) -> Option<Self> {
        Some(Koala {
            file: file.try_into().ok()?,
        })
    }

    #[inline]
    pub(crate) fn bgcolor(&self) -> u8 {
        self.file[10002]
    }

    #[inline]
    pub(crate) fn set_bgcolor(&mut self, value: u8) {
        self.file[10002] = value;
    }

    #[inline]
    pub(crate) fn chars_mut<'b>(&'b mut self) -> CharsMut<'a, 'b> {
        CharsMut {
            koala: self,
            pos: 0,
        }
    }
}

pub(crate) struct CharsMut<'a, 'b> {
    koala: &'b mut Koala<'a>,
    pos: usize,
}

impl<'b> Iterator for CharsMut<'_, 'b> {
    type Item = (&'b mut [u8; 8], &'b mut u8, &'b mut u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == 1000 {
            None
        } else {
            let result = (
                #[expect(unsafe_code, reason = "see below")]
                // Safety: the three references do point to distinct memory, for all calls of next
                unsafe {
                    &mut *(self.koala.file[2 + self.pos * 8..2 + self.pos * 8 + 8]
                        .as_mut_ptr()
                        .cast::<[u8; 8]>())
                },
                #[expect(unsafe_code, reason = "see below")]
                // Safety: the three references do point to distinct memory, for all calls of next
                unsafe {
                    &mut *(&raw mut self.koala.file[8002 + self.pos])
                },
                #[expect(unsafe_code, reason = "see below")]
                // Safety: the three references do point to distinct memory, for all calls of next
                unsafe {
                    &mut *(&raw mut self.koala.file[9002 + self.pos])
                },
                self.koala.bgcolor(),
            );
            self.pos += 1;

            Some(result)
        }
    }
}
