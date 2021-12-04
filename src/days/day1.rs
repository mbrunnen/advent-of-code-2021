use crate::utils::challenge::Challenge;

pub struct Day1 {
    lines: Vec<String>,
}

#[derive(Debug)]
struct Passport {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<u16>,
}

impl Passport {
    fn empty() -> Self {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn decode(line: &str) -> Result<Self, String> {
        let mut pp = Passport::empty();
        for i in line.split(' ') {
            let pair: Vec<&str> = i.split(':').collect();
            match pair[0] {
                "byr" => pp.byr = Some(pair[1].parse().unwrap()),
                "iyr" => pp.iyr = Some(pair[1].parse().unwrap()),
                "eyr" => pp.eyr = Some(pair[1].parse().unwrap()),
                "hgt" => pp.hgt = Some(pair[1].to_string()),
                "hcl" => pp.hcl = Some(pair[1].to_string()),
                "ecl" => pp.ecl = Some(pair[1].to_string()),
                "pid" => pp.pid = Some(pair[1].to_string()),
                "cid" => pp.cid = Some(pair[1].parse().unwrap()),
                e => return Err(format!("Unknown field: {}", e)),
            };
        }

        Ok(pp)
    }

    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
        // && self.cid.is_some()
    }
}

// XXX: Use derive
impl PartialEq for Passport {
    fn eq(&self, other: &Self) -> bool {
        self.byr == other.byr
            && self.iyr == other.iyr
            && self.eyr == other.eyr
            && self.hgt == other.hgt
            && self.hcl == other.hcl
            && self.ecl == other.ecl
            && self.pid == other.pid
            && self.cid == other.cid
    }
}

impl Challenge for Day1 {
    fn new(lines: Vec<String>) -> Self {
        Day1 {
            lines: {
                let mut data: Vec<String> = vec![];
                let mut first: usize = 0;
                for (pos, l) in lines.iter().enumerate() {
                    if l.is_empty() {
                        data.push(lines[first..pos].join(" "));
                        first = pos + 1;
                    }
                }

                // Add last item
                if lines.last() != Some(&String::from("")) {
                    data.push(lines[first..lines.len()].join(" "));
                }

                data
            },
        }
    }

    fn run(&self) -> Result<String, String> {
        let passports: Vec<Passport> = self
            .lines
            .iter()
            .map(|x| Passport::decode(x).expect("Something went wrong while decoding"))
            .collect();
        let valid_passports = passports.iter().filter(|p| p.is_valid()).count();
        Ok(format!("valid: {} of {}", valid_passports, passports.len()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregate() {
        let data1 = vec![
            String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884"),
            String::from("hcl:#cfa07d byr:1929"),
            String::from(""),
            String::from("hcl:#ae17e1 iyr:2013"),
            String::from("eyr:2024"),
            String::from("ecl:brn pid:760753108 byr:1931"),
            String::from("hgt:179cm"),
            String::from(""),
            String::from("hcl:#cfa07d eyr:2025 pid:166559648"),
            String::from("iyr:2011 ecl:brn hgt:59in"),
        ];

        let expected = vec![
            String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884 hcl:#cfa07d byr:1929"),
            String::from("hcl:#ae17e1 iyr:2013 eyr:2024 ecl:brn pid:760753108 byr:1931 hgt:179cm"),
            String::from("hcl:#cfa07d eyr:2025 pid:166559648 iyr:2011 ecl:brn hgt:59in"),
        ];

        let d1 = Day1::new(data1);
        assert_eq!(d1.lines, expected);

        let data2 = vec![
            String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884"),
            String::from("hcl:#cfa07d byr:1929"),
            String::from(""),
            String::from("hcl:#ae17e1 iyr:2013"),
            String::from("eyr:2024"),
            String::from("ecl:brn pid:760753108 byr:1931"),
            String::from("hgt:179cm"),
            String::from(""),
            String::from("hcl:#cfa07d eyr:2025 pid:166559648"),
            String::from("iyr:2011 ecl:brn hgt:59in"),
            String::from(""),
        ];
        let d2 = Day1::new(data2);
        assert_eq!(d2.lines, expected);
    }

    #[test]
    fn test_decode_valid() {
        let data = String::from(
            "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd byr:1937 iyr:2017 cid:147 hgt:183cm",
        );

        let pp_actual = Passport::decode(&data).unwrap();
        let pp_expected = Passport {
            byr: Some(1937),
            iyr: Some(2017),
            eyr: Some(2020),
            hgt: Some("183cm".to_string()),
            hcl: Some("#fffffd".to_string()),
            ecl: Some("gry".to_string()),
            pid: Some("860033327".to_string()),
            cid: Some(147),
        };

        assert_eq!(pp_actual, pp_expected);
    }

    #[test]
    fn valid_test() {
        let pp_valid1 = Passport {
            byr: Some(1937),
            iyr: Some(2017),
            eyr: Some(2020),
            hgt: Some("183cm".to_string()),
            hcl: Some("#fffffd".to_string()),
            ecl: Some("gry".to_string()),
            pid: Some("860033327".to_string()),
            cid: Some(147),
        };
        let pp_valid2 = Passport {
            byr: Some(1937),
            iyr: Some(2017),
            eyr: Some(2020),
            hgt: Some("183cm".to_string()),
            hcl: Some("#fffffd".to_string()),
            ecl: Some("gry".to_string()),
            pid: Some("860033327".to_string()),
            cid: None,
        };
        let pp_invalid1 = Passport {
            byr: Some(1937),
            iyr: Some(2017),
            eyr: Some(2020),
            hgt: None,
            hcl: Some("#fffffd".to_string()),
            ecl: Some("gry".to_string()),
            pid: Some("860033327".to_string()),
            cid: Some(147),
        };
        let pp_invalid2 = Passport {
            byr: None,
            iyr: Some(2017),
            eyr: Some(2020),
            hgt: Some("183cm".to_string()),
            hcl: Some("#fffffd".to_string()),
            ecl: Some("gry".to_string()),
            pid: Some("860033327".to_string()),
            cid: None,
        };
        assert!(pp_valid1.is_valid());
        assert!(pp_valid2.is_valid());
        assert!(!pp_invalid1.is_valid());
        assert!(!pp_invalid2.is_valid());
    }
}
