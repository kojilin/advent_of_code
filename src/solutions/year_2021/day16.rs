use std::error::Error;
use std::fs;
use std::iter::FromIterator;
use std::ops::Add;

fn solve_day16_1() -> Result<i32, Box<dyn Error>> {
    return Ok(solve_1(parse_input()?));
}

fn solve_day16_2() -> Result<i64, Box<dyn Error>> {
    return Ok(solve_2(parse_input()?));
}

fn parse_input() -> Result<Vec<char>, Box<dyn Error>> {
    let line = fs::read_to_string("src/solutions/year_2021/day16.txt")?;
    let chars: Vec<String> = line.trim().chars().map(|c| {
        format!("{:04b}", c.to_digit(16).unwrap())
    }).collect();
    Ok(chars.join("").chars().collect())
}

fn solve_1(chars: Vec<char>) -> i32 {
    let mut index = 0;
    let mut result = 0;
    while index != -1 {
        let packet = read_packet(index, &chars);
        index = packet.0;
        if let (_, Some(packet)) = packet {
            result += packet.sum_version();
        }
    }
    return result;
}

fn solve_2(chars: Vec<char>) -> i64 {
    let mut index = 0;
    let mut result = 0;
    while index != -1 {
        let packet = read_packet(index, &chars);
        index = packet.0;
        if let (_, Some(packet)) = packet {
            result += packet.calculate();
        }
    }
    return result;
}


// better to use multiple struct...
#[derive(Debug)]
struct Packet {
    type_id: i32,
    version: i32,
    packets: Vec<Packet>,
    literal: i64,
}

impl Packet {
    pub fn sum_version(&self) -> i32 {
        return self.version + self.packets.iter().map(|p| p.sum_version()).sum::<i32>();
    }

    pub fn calculate(&self) -> i64 {
        return match self.type_id {
            0 => self.packets.iter().map(|p| p.calculate()).sum::<i64>(),
            1 => self.packets.iter().map(|p| p.calculate()).product::<i64>(),
            2 => self.packets.iter().map(|p| p.calculate()).min().unwrap(),
            3 => self.packets.iter().map(|p| p.calculate()).max().unwrap(),
            5 => {
                return if self.packets[0].calculate() > self.packets[1].calculate() {
                    1
                } else {
                    0
                };
            }
            6 => {
                return if self.packets[0].calculate() < self.packets[1].calculate() {
                    1
                } else {
                    0
                };
            }
            7 => {
                return if self.packets[0].calculate() == self.packets[1].calculate() {
                    1
                } else {
                    0
                };
            }
            4 => self.literal as i64,
            _ => panic!(),
        };
    }
}

fn read_packet(index: i32, chars: &Vec<char>) -> (i32, Option<Packet>) {
    // How to know the 0 is not useful?
    if index + 8 > chars.len() as i32 {
        return (-1, None);
    }
    let mut index = index as usize;
    let version = &chars[index..(index + 3)];
    index += 3;
    let type_id = &chars[index..(index + 3)];
    index += 3;
    // is 4
    let type_id = i32::from_str_radix(&String::from_iter(type_id), 2).unwrap();
    return if type_id == 4 {
        let mut is_last = false;
        let mut literal = String::new();
        while !is_last {
            let leading = chars[index];
            if leading == '0' {
                is_last = true;
            }
            index += 1;
            literal = literal.add(&String::from_iter(&chars[index..index + 4]));
            //literal
            index += 4;
        }
        (index as i32,
         Some(Packet {
             type_id,
             version: i32::from_str_radix(&String::from_iter(version), 2).unwrap(),
             literal: i64::from_str_radix(&literal, 2).unwrap(),
             packets: vec![],
         }))
    } else {
        let mut sub_packets = vec![];
        let length_type = chars[index];
        index += 1;
        if length_type == '0' {
            // 0
            let length_of_total = &chars[index..(index + 15)];
            index += 15;
            let length_of_total = i32::from_str_radix(&String::from_iter(length_of_total), 2).unwrap();
            let mark = index;
            while (mark + length_of_total as usize) > index {
                let packet = read_packet(index as i32, chars);
                index = packet.0 as usize;
                for p in packet.1 {
                    sub_packets.push(p);
                }
            }
        } else {
            // 1
            let number_of_total = &chars[index..(index + 11)];
            index += 11;
            let number_of_total = i32::from_str_radix(&String::from_iter(number_of_total), 2).unwrap();
            for _ in 0..number_of_total {
                let packet = read_packet(index as i32, chars);
                index = packet.0 as usize;
                for p in packet.1 {
                    sub_packets.push(p);
                }
            }
        }
        (index as i32,
         Some(Packet {
             type_id,
             version: i32::from_str_radix(&String::from_iter(version), 2).unwrap(),
             literal: -1,
             packets: sub_packets,
         })
        )
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day16_1()?);
        Ok(())
    }

    #[test]
    fn test_second() -> Result<(), Box<dyn Error>> {
        println!("-----real-----");
        println!("result:{}", solve_day16_2()?);
        Ok(())
    }
}

