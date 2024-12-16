use std::fs::read_to_string;

#[derive(Clone, Copy)]
struct Bits<'a>(&'a [bool]);

impl<'a> Bits<'a> {
    fn new(bits: &'a [bool]) -> Self {
        Self(bits)
    }

    fn value(&self) -> usize {
        let mut total = 0;
        for bit in self.0 {
            total *= 2;
            if *bit {
                total += 1
            }
        }
        total
    }

    fn is_only_zeroes(&self) -> bool {
        self.0.iter().all(|b| !b)
    }

    fn take(&mut self, at: usize) -> Self {
        let (output, _) = self.0.split_at(at);
        self.0 = &self.0[at..];
        Bits::new(output)
    }

    fn convert_from_hex(input: &str) -> Vec<bool> {
        input
            .trim()
            .chars()
            .flat_map(|c| match c {
                '0' => [false, false, false, false],
                '1' => [false, false, false, true],
                '2' => [false, false, true, false],
                '3' => [false, false, true, true],
                '4' => [false, true, false, false],
                '5' => [false, true, false, true],
                '6' => [false, true, true, false],
                '7' => [false, true, true, true],
                '8' => [true, false, false, false],
                '9' => [true, false, false, true],
                'A' => [true, false, true, false],
                'B' => [true, false, true, true],
                'C' => [true, true, false, false],
                'D' => [true, true, false, true],
                'E' => [true, true, true, false],
                'F' => [true, true, true, true],
                c => panic!("invalid character: {c}"),
            })
            .collect()
    }
}

enum Operation {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    EqualTo,
}

enum PacketType {
    Literal(usize),
    Operation(Packets, Operation),
}

impl PacketType {
    fn new(bits: &mut Bits, id: usize) -> Self {
        if id == 4 {
            let mut value: Vec<bool> = vec![];
            let mut is_there_more = true;
            while is_there_more {
                let mut current_chunk = bits.take(5);
                is_there_more = current_chunk.take(1).0[0];
                value.extend(current_chunk.0);
            }

            PacketType::Literal(Bits::new(&value).value())
        } else {
            let operation = match id {
                0 => Operation::Sum,
                1 => Operation::Product,
                2 => Operation::Min,
                3 => Operation::Max,
                5 => Operation::GreaterThan,
                6 => Operation::LessThan,
                7 => Operation::EqualTo,
                _ => panic!("invalid id"),
            };
            let length_type_id = bits.take(1).0[0];

            if length_type_id {
                let number_of_subpackets = bits.take(11).value();
                let mut subpackets = vec![];
                for _ in 0..number_of_subpackets {
                    subpackets.push(Packet::new(bits))
                }

                PacketType::Operation(Packets(subpackets), operation)
            } else {
                let length = bits.take(15).value();
                let mut subpackets = bits.take(length);
                PacketType::Operation(Packets::new(&mut subpackets), operation)
            }
        }
    }
}

struct Packet {
    version: usize,
    packet_type: PacketType,
}

impl Packet {
    fn new(bits: &mut Bits) -> Self {
        let version = bits.take(3).value();
        let id = bits.take(3).value();

        let packet_type = PacketType::new(bits, id);

        Self {
            version,
            packet_type,
        }
    }

    fn version_sum(&self) -> usize {
        let mut total = self.version;
        if let PacketType::Operation(packets, _) = &self.packet_type {
            total += packets.version_sum();
        }

        total
    }

    fn evaluate(&self) -> usize {
        match &self.packet_type {
            PacketType::Literal(x) => *x,
            PacketType::Operation(packets, operation) => {
                let values: Vec<_> = packets.0.iter().map(|p| p.evaluate()).collect();
                match operation {
                    Operation::Sum => values.iter().sum(),
                    Operation::Product => values.iter().product(),
                    Operation::Min => *values.iter().min().unwrap(),
                    Operation::Max => *values.iter().max().unwrap(),
                    Operation::LessThan => (values[0] < values[1]) as usize,
                    Operation::GreaterThan => (values[0] > values[1]) as usize,
                    Operation::EqualTo => (values[0] == values[1]) as usize,
                }
            }
        }
    }
}

struct Packets(Vec<Packet>);

impl Packets {
    fn new(bits: &mut Bits) -> Self {
        let mut packets = vec![];

        while !bits.is_only_zeroes() {
            packets.push(Packet::new(bits));
        }

        Packets(packets)
    }

    fn version_sum(&self) -> usize {
        self.0.iter().map(|p| p.version_sum()).sum()
    }
}

fn solve(path: &str) -> (usize, usize) {
    let input = read_to_string(path).unwrap();
    let bits = Bits::convert_from_hex(&input);
    let packets = Packets::new(&mut Bits::new(&bits));
    let output_1 = packets.version_sum();
    let output_2 = packets.0[0].evaluate();

    (output_1, output_2)
}

fn main() {
    let (output_1, output_2) = solve("input");
    println!("part 1: {output_1} part 2: {output_2}")
}

#[test]
fn practice() {
    let (_, output) = solve("practice");
    assert_eq!(output, 1)
}
