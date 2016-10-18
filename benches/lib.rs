#![feature(test)]

#[macro_use]
extern crate fix_rs;
extern crate test;

use std::collections::HashMap;
use fix_rs::message::Message;
use fix_rs::dictionary::NewOrderSingle;
use fix_rs::fix::Parser;
use test::Bencher;

const MESSAGE_BYTES: &'static [u8] = b"8=FIX.4.2\x019=197\x0135=D\x0149=AFUNDMGR\x0156=ABROKER\x0134=2\x0152=20030615-01:14:49\x0111=12345\x011=111111\x0163=0\x0164=20030621\x0121=3\x01110=1000\x01111=50000\x0155=IBM\x0148=459200101\x0122=1\x0154=1\x0160=2003061501:14:49\x0138=5000\x0140=1\x0144=15.75\x0115=USD\x0159=0\x0110=230\x01";

#[bench]
fn parse_simple_message_bench(b: &mut Bencher) {
    define_dictionary!(
        b"D" => NewOrderSingle : NewOrderSingle,
    );

    let mut parser = Parser::new(build_dictionary());
    b.iter(|| {
        let (bytes_read,result) = parser.parse(MESSAGE_BYTES);
        assert!(result.is_ok());
        assert!(bytes_read == MESSAGE_BYTES.len());
    });
}

#[bench]
fn serialize_simple_message_bench(b: &mut Bencher) {
    define_dictionary!(
        b"D" => NewOrderSingle : NewOrderSingle,
    );

    let mut parser = Parser::new(build_dictionary());
    let (bytes_read,result) = parser.parse(MESSAGE_BYTES);
    assert!(result.is_ok());
    assert!(bytes_read == MESSAGE_BYTES.len());
    match message_to_enum(&**(parser.messages.first().unwrap())) {
        MessageEnum::NewOrderSingle(message) => {
            b.iter(|| {
                let mut data = Vec::new();
                message.read(&mut data);
            });
        },
    }
}