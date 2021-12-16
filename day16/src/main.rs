use bitvec::prelude::*;
use num::Integer;

struct BitSliceReader<'a> {
    bit_slice: &'a BitSlice<Msb0, u8>,
    cur_bit: usize,
}
impl <'a> BitSliceReader<'a> {
    fn new(input: &[u8]) -> BitSliceReader {
        let bit_slice = BitSlice::<Msb0, u8>::from_slice(input)
            .expect("Could not parse to BitSlice");
        BitSliceReader {
            bit_slice,
            cur_bit: 0,
        }
    }

    fn read_bits<N: Integer + PartialOrd + Copy>(&mut self, num_bits: usize) -> N {
        let result: N = (self.cur_bit..(self.cur_bit+num_bits)).into_iter()
            .fold(N::zero(), |acc, bit_index| {
                let b = self.bit_slice[bit_index];
                let mut result = acc * (N::one() + N::one());
                if b {
                    result = result + N::one();
                }
                result
            });
        self.cur_bit += num_bits;
        result
    }

    fn read_bool(&mut self) -> bool {
        let result = self.bit_slice[self.cur_bit];
        self.cur_bit += 1;
        result
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Header {
    packet_version: u8,
    packet_type: u8,
}

#[derive(Eq, PartialEq, Debug)]
enum Packet {
    Literal(Header, u128),
    Operator(Header, Vec<Packet>),
}
impl Packet {
    fn parse(input: &str) -> Packet {
        let parsed_hex = hex::decode(input).expect("Could not parse hex");
        let mut parser = PacketParser::new(&parsed_hex);
        parser.packet()
    }

    fn sum_version_numbers(&self) -> u128 {
        match self {
            Packet::Literal(header, _) => header.packet_version as u128,
            Packet::Operator(header, subpackets) => {
                header.packet_version as u128 + subpackets.iter().map(|p| p.sum_version_numbers()).sum::<u128>()
            }
        }
    }

    fn value(&self) -> u128 {
        match self {
            Packet::Literal(_, v) => *v,
            Packet::Operator(h, subpackets) => {
                ops::from(&h.packet_type)(subpackets)
            }
        }
    }
}

mod ops {
    use super::*;

    pub(crate) fn from(packet_type: &u8) -> fn(&Vec<Packet>) -> u128 {
        match packet_type {
            0 => sum,
            1 => product,
            2 => minimum,
            3 => maximum,
            5 => greater_than,
            6 => less_than,
            7 => equal_to,
            _ => panic!(),
        }
    }

    fn sum(packets: &Vec<Packet>) -> u128 {
        packets.iter().map(|p| p.value()).sum()
    }

    fn product(packets: &Vec<Packet>) -> u128 {
        packets.iter().map(|p| p.value()).fold(1, |acc, v| acc * v)
    }

    fn minimum(packets: &Vec<Packet>) -> u128 {
        packets.iter().map(|p| p.value()).min().unwrap()
    }

    fn maximum(packets: &Vec<Packet>) -> u128 {
        packets.iter().map(|p| p.value()).max().unwrap()
    }

    fn greater_than(packets: &Vec<Packet>) -> u128 {
        let (first, second) = get_two_packets(packets);
        if first.value() > second.value() { 1 } else { 0 }
    }

    fn less_than(packets: &Vec<Packet>) -> u128 {
        let (first, second) = get_two_packets(packets);
        if first.value() < second.value() { 1 } else { 0 }
    }

    fn equal_to(packets: &Vec<Packet>) -> u128 {
        let (first, second) = get_two_packets(packets);
        if first.value() == second.value() { 1 } else { 0 }
    }

    fn get_two_packets(packets: &Vec<Packet>) -> (&Packet, &Packet) {
        assert_eq!(packets.len(), 2);
        let first = &packets[0];
        let second = &packets[1];
        (first, second)
    }
}

struct PacketParser<'a> {
    bit_slice_reader: BitSliceReader<'a>
}
impl <'a> PacketParser<'a> {
    fn new(input: &[u8]) -> PacketParser {
        PacketParser {
            bit_slice_reader: BitSliceReader::new(input)
        }
    }

    fn packet(&mut self) -> Packet {
        let header = self.header();

        if header.packet_type == 4 {
            self.literal_packet(header)
        } else {
            self.operator_packet(header)
        }
    }

    fn header(&mut self) -> Header {
        let version_byte = self.bit_slice_reader.read_bits(3);
        let type_byte = self.bit_slice_reader.read_bits(3);
        Header { packet_version: version_byte, packet_type: type_byte }
    }

    fn literal_packet(&mut self, header: Header) -> Packet {
        let mut should_continue = true;
        let mut result = 0;

        while should_continue {
            should_continue = self.bit_slice_reader.read_bool();
            let byte: u128 = self.bit_slice_reader.read_bits(4);
            result *= 16;
            result += byte;
        }

        Packet::Literal(header, result)
    }

    fn operator_packet(&mut self, header: Header) -> Packet {
        let is_length_type_1 = self.bit_slice_reader.read_bool();
        let subpackets = if is_length_type_1 {
            self.operator_packet_length_type_1()
        } else {
            self.operator_packet_length_type_0()
        };
        Packet::Operator(header, subpackets)
    }

    fn operator_packet_length_type_0(&mut self) -> Vec<Packet> {
        let subpackets_bit_length: u16 = self.bit_slice_reader.read_bits(15);
        let terminate_at = self.bit_slice_reader.cur_bit + subpackets_bit_length as usize;

        let mut subpackets = vec![];
        while self.bit_slice_reader.cur_bit < terminate_at {
            subpackets.push(self.packet())
        }

        subpackets
    }

    fn operator_packet_length_type_1(&mut self) -> Vec<Packet> {
        let num_subpackets: u16 = self.bit_slice_reader.read_bits(11);

        let mut subpackets = vec![];
        for _ in 0..num_subpackets {
            subpackets.push(self.packet())
        }

        subpackets
    }
}

fn main() {
    let input = "6051639005B56008C1D9BB3CC9DAD5BE97A4A9104700AE76E672DC95AAE91425EF6AD8BA5591C00F92073004AC0171007E0BC248BE0008645982B1CA680A7A0CC60096802723C94C265E5B9699E7E94D6070C016958F99AC015100760B45884600087C6E88B091C014959C83E740440209FC89C2896A50765A59CE299F3640D300827902547661964D2239180393AF92A8B28F4401BCC8ED52C01591D7E9D2591D7E9D273005A5D127C99802C095B044D5A19A73DC0E9C553004F000DE953588129E372008F2C0169FDB44FA6C9219803E00085C378891F00010E8FF1AE398803D1BE25C743005A6477801F59CC4FA1F3989F420C0149ED9CF006A000084C5386D1F4401F87310E313804D33B4095AFBED32ABF2CA28007DC9D3D713300524BCA940097CA8A4AF9F4C00F9B6D00088654867A7BC8BCA4829402F9D6895B2E4DF7E373189D9BE6BF86B200B7E3C68021331CD4AE6639A974232008E663C3FE00A4E0949124ED69087A848002749002151561F45B3007218C7A8FE600FC228D50B8C01097EEDD7001CF9DE5C0E62DEB089805330ED30CD3C0D3A3F367A40147E8023221F221531C9681100C717002100B36002A19809D15003900892601F950073630024805F400150D400A70028C00F5002C00252600698400A700326C0E44590039687B313BF669F35C9EF974396EF0A647533F2011B340151007637C46860200D43085712A7E4FE60086003E5234B5A56129C91FC93F1802F12EC01292BD754BCED27B92BD754BCED27B100264C4C40109D578CA600AC9AB5802B238E67495391D5CFC402E8B325C1E86F266F250B77ECC600BE006EE00085C7E8DF044001088E31420BCB08A003A72BF87D7A36C994CE76545030047801539F649BF4DEA52CBCA00B4EF3DE9B9CFEE379F14608";
    let packet = Packet::parse(input);
    println!("Part 1: {}", packet.sum_version_numbers());
    println!("Part 2: {}", packet.value());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_literal_packet_example() {
        let input = "D2FE28";
        let packet = Packet::parse(input);
        assert_eq!(packet, Packet::Literal(Header { packet_version: 6, packet_type: 4}, 2021));
    }

    #[test]
    fn test_operator_packet_example_type_0() {
        let input = "38006F45291200";
        let packet = Packet::parse(input);
        assert_eq!(packet, Packet::Operator(
            Header { packet_version: 1, packet_type: 6 },
            vec![
                Packet::Literal(
                    Header { packet_version: 6, packet_type: 4 },
                    10
                ),
                Packet::Literal(
                    Header { packet_version: 2, packet_type: 4 },
                    20
                )
            ]
        ));
    }

    #[test]
    fn test_operator_packet_example_type_1() {
        let input = "EE00D40C823060";
        let packet = Packet::parse(input);
        assert_eq!(packet, Packet::Operator(
            Header { packet_version: 7, packet_type: 3 },
            vec![
                Packet::Literal(
                    Header { packet_version: 2, packet_type: 4 },
                    1
                ),
                Packet::Literal(
                    Header { packet_version: 4, packet_type: 4 },
                    2
                ) ,
                Packet::Literal(
                    Header { packet_version: 1, packet_type: 4 },
                    3
                )
            ]
        ));
    }

    #[test]
    fn test_sum_version_numbers() {
        assert_eq!(Packet::parse("8A004A801A8002F478").sum_version_numbers(), 16);
        assert_eq!(Packet::parse("620080001611562C8802118E34").sum_version_numbers(), 12);
        assert_eq!(Packet::parse("C0015000016115A2E0802F182340").sum_version_numbers(), 23);
        assert_eq!(Packet::parse("A0016C880162017C3686B18A3D4780").sum_version_numbers(), 31);
    }

    #[test]
    fn test_value() {
        assert_eq!(Packet::parse("C200B40A82").value(), 3);
        assert_eq!(Packet::parse("04005AC33890").value(), 54);
        assert_eq!(Packet::parse("880086C3E88112").value(), 7);
        assert_eq!(Packet::parse("CE00C43D881120").value(), 9);
        assert_eq!(Packet::parse("D8005AC2A8F0").value(), 1);
        assert_eq!(Packet::parse("F600BC2D8F").value(), 0);
        assert_eq!(Packet::parse("9C005AC2F8F0").value(), 0);
        assert_eq!(Packet::parse("9C0141080250320F1802104A08").value(), 1);
    }
}