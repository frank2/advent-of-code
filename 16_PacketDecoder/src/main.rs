use std::io;

#[derive(Clone, Eq, PartialEq, Debug)]
struct Bits {
    bits: Vec<u8>,
}
impl Bits {
    fn new() -> Self {
        Self { bits: Vec::<u8>::new() }
    }
    fn push(&mut self, bit: u8) {
        self.bits.push(bit)
    }
    fn append(&mut self, bits: &mut Bits) {
        self.bits.append(&mut bits.bits);
    }
    fn as_int(&self) -> usize {
        let mut result = 0usize;
        
        for i in 0..self.bits.len() {
            result <<= 1;
            result |= self.bits[i] as usize;
        }

        result
    }
    fn len(&self) -> usize { self.bits.len() }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Block {
    block: u8,
    bits: usize,
}
impl Block {
    fn new(block: u8) -> Self {
        Self { block: block, bits: 8 }
    }
    fn read_bits(&mut self, size: usize) -> Result<Bits, ()> {
        if self.bits == 0 { return Err(()); }
        
        let total;
        
        if size > self.bits { total = self.bits; }
        else { total = size; }

        let mut bits = Bits::new();

        (0..total).for_each(|_| bits.push(self.read_bit().unwrap()));

        Ok(bits)
    }
    fn read_bit(&mut self) -> Result<u8, ()> {
        if self.bits == 0 { return Err(()); }

        self.bits -= 1;
        let result = self.block >> (self.bits);

        self.block &= 2u8.pow(self.bits as u32)-1;

        Ok(result)
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Bitstream {
    stream: Vec<Block>,
}
impl Bitstream {
    fn new(s: &str) -> Result<Self, ()> {
        let mut stream = Vec::<Block>::new();

        for i in (0..s.len()).step_by(2) {
            let value = u8::from_str_radix(&s[i..i+2], 16);

            if value.is_err() { return Err(()); }
            else { stream.push(Block::new(value.unwrap())); }
        }

        Ok(Self { stream })
    }
    fn first(&mut self) -> Option<&mut Block> {
        if self.stream.len() == 0 { None }
        else { Some(&mut self.stream[0]) }
    }
    fn consume(&mut self) {
        if self.stream.len() > 0 { self.stream.remove(0); }
    }
    fn read(&mut self, size: usize) -> Result<Bits, ()> {
        let mut bits = Bits::new();
        let mut total = size;

        while total > 0 {
            if let Some(first) = self.first() {
                if let Ok(mut new_bits) = first.read_bits(total) {
                    total -= new_bits.len();
                    bits.append(&mut new_bits);
                }
                else { return Err(()); }

                if first.bits == 0 { self.consume(); }
            }
            else { return Err(()); }
        }

        Ok(bits)
    }
    fn eop(&mut self) {
        if let Some(first) = self.first() {
            if first.bits < 8 { self.consume(); }
        }
    }
    fn read_packets(&mut self) -> Result<Vec<Packet>, ()> {
        let mut packets = Vec::<Packet>::new();

        while self.stream.len() > 0 {
            if let Ok((packet, _)) = Packet::parse(self) {
                // println!("Consumed {} bits for {:?}", consumed, packet);
                packets.push(packet);
                self.eop();
            }
            else { return Err(()); }
        }

        Ok(packets)
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    packet_type: PacketType,
}
impl Packet {
    fn parse(stream: &mut Bitstream) -> Result<(Packet, usize), ()> {
        let mut consumed = 0usize;
        let version;
        let type_id;

        if let Ok(version_bits) = stream.read(3) {
            version = version_bits.as_int() as u8;
        }
        else { return Err(()); }

        consumed += 3;

        if let Ok(type_bits) = stream.read(3) {
            type_id = type_bits.as_int() as u8;
        }
        else { return Err(()); }

        consumed += 3;

        if let Ok((packet_type, packet_consumed)) = PacketType::parse(stream, type_id) {
            Ok((Packet { version, type_id, packet_type }, consumed+packet_consumed))
        }
        else { Err(()) }
    }
    fn version_sum(&self) -> usize {
        let mut result = self.version as usize;

        match &self.packet_type {
            PacketType::LiteralPacket(_) => (),
            PacketType::OperatorPacket(operator) => result += operator.version_sum(),
        }

        result
    }
    fn evaluate(&self) -> usize {
        match &self.packet_type {
            PacketType::LiteralPacket(literal) => literal.evaluate(),
            PacketType::OperatorPacket(operator) => operator.evaluate(self.type_id),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum PacketType {
    LiteralPacket(LiteralPacket),
    OperatorPacket(OperatorPacket),
}
impl PacketType {
    fn parse(stream: &mut Bitstream, type_id: u8) -> Result<(Self, usize), ()> {
        let consumed;
        
        let packet_type = match type_id {
            4 => {
                if let Ok((literal, literal_consumed)) = LiteralPacket::parse(stream) {
                    consumed = literal_consumed;
                    
                    Self::LiteralPacket(literal)
                }
                else { return Err(()); }
            },
            _ => {
                if let Ok((operator, operator_consumed)) = OperatorPacket::parse(stream) {
                    consumed = operator_consumed;

                    Self::OperatorPacket(operator)
                }
                else { return Err(()); }
            },
        };

        Ok((packet_type, consumed))
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct LiteralBlock {
    more: bool,
    value: Bits,
}
impl LiteralBlock {
    fn parse(stream: &mut Bitstream) -> Result<(Self, usize), ()> {
        let more;

        if let Ok(more_bits) = stream.read(1) {
            more = more_bits.as_int() == 1;
        }
        else { return Err(()); }

        if let Ok(value) = stream.read(4) {
            Ok((Self { more, value }, 5))
        }
        else { Err(()) }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct LiteralPacket {
    blocks: Vec<LiteralBlock>,
}
impl LiteralPacket {
    fn parse(stream: &mut Bitstream) -> Result<(Self, usize), ()> {
        let mut blocks = Vec::<LiteralBlock>::new();
        let mut consumed = 0usize;

        loop {
            if let Ok((block, block_consumed)) = LiteralBlock::parse(stream) {
                let has_more = block.more;
                
                blocks.push(block);
                consumed += block_consumed;

                if !has_more { break; }
            }
            else { return Err(()); }
        }

        Ok((Self { blocks }, consumed))
    }
    fn as_int(&self) -> usize {
        let mut bits = Bits::new();

        for block in &self.blocks {
            bits.append(&mut block.value.clone());
        }

        bits.as_int()
    }
    fn evaluate(&self) -> usize {
        self.as_int()
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct OperatorPacket {
    subpackets: Vec<Packet>,
}
impl OperatorPacket {
    fn parse(stream: &mut Bitstream) -> Result<(Self, usize), ()> {
        let mut consumed = 0usize;
        let mut subpackets = Vec::<Packet>::new();
        let parse_switch;
        let consume_or_packets;

        if let Ok(parse_bits) = stream.read(1) {
            parse_switch = parse_bits.as_int() == 1;
            consumed += 1;
        }
        else { return Err(()); }

        if parse_switch {
            if let Ok(parse_count) = stream.read(11) {
                consume_or_packets = parse_count.as_int();
                consumed += 11;
            }
            else { return Err(()); }
        }
        else {
            if let Ok(parse_count) = stream.read(15) {
                consume_or_packets = parse_count.as_int();
                consumed += 15;
            }
            else { return Err(()); }
        }

        if parse_switch {
            let expected_packets = consume_or_packets;

            for _ in 0..expected_packets {
                if let Ok((subpacket, subpacket_consumed)) = Packet::parse(stream) {
                    subpackets.push(subpacket);
                    consumed += subpacket_consumed;
                }
                else { return Err(()); }
            }
        }
        else {
            let mut expected_consumption = consume_or_packets;

            while expected_consumption > 0 {
                if let Ok((subpacket, subpacket_consumed)) = Packet::parse(stream) {
                    subpackets.push(subpacket);
                    consumed += subpacket_consumed;
                    expected_consumption -= subpacket_consumed;
                }
                else { return Err(()); }
            }
        }

        Ok((Self { subpackets }, consumed))
    }

    fn version_sum(&self) -> usize {
        let mut result = 0usize;

        for packet in &self.subpackets {
            result += packet.version_sum();
        }

        result
    }

    fn evaluate(&self, type_id: u8) -> usize {
        match type_id {
            0 => self.subpackets.iter().map(|x| x.evaluate()).sum(),
            1 => self.subpackets.iter().map(|x| x.evaluate()).product(),
            2 => self.subpackets.iter().map(|x| x.evaluate()).min().unwrap(),
            3 => self.subpackets.iter().map(|x| x.evaluate()).max().unwrap(),
            5 => {
                let (left, right) = (self.subpackets[0].evaluate(), self.subpackets[1].evaluate());

                (left > right) as usize
            },
            6 => {
                let (left, right) = (self.subpackets[0].evaluate(), self.subpackets[1].evaluate());

                (left < right) as usize
            },
            7 => {
                let (left, right) = (self.subpackets[0].evaluate(), self.subpackets[1].evaluate());

                (left == right) as usize
            },
            _ => panic!("cannot evaluate unknown type id"),
        }
    }
}

fn read_pcap() -> Result<Bitstream, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();

    if let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { Err(()) }
        else { Bitstream::new(&buffer.trim()) }
    }
    else { Err(()) }
}

fn part1() {
    if let Ok(mut stream) = read_pcap() {
        if let Ok(packets) = stream.read_packets() {
            let mut version_sum = 0usize;
            
            for packet in &packets {
                version_sum += packet.version_sum();
            }

            println!("{}", version_sum);
        }
    }
    else { panic!("couldn't read pcap!"); }
}

fn part2() {
    if let Ok(mut stream) = read_pcap() {
        if let Ok(packets) = stream.read_packets() {
            for packet in &packets {
                println!("{}", packet.evaluate());
            }
        }
    }
    else { panic!("couldn't read pcap!"); }
}

fn main() {
    // part1();
    part2();
}
