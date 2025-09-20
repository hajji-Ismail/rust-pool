#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
    A,
    AB,
    B,
    O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum RhFactor {
    Positive,
    Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
    pub antigen: Antigen,
    pub rh_factor: RhFactor,
}

use std::char::from_digit;
use std::cmp::{Ord, Ordering};

use std::str::FromStr;

impl FromStr for Antigen {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Antigen::A),
            "B" => Ok(Antigen::B),
            "AB" => Ok(Antigen::AB),
            "O" => Ok(Antigen::O),
            _ => Err("not in the enum".to_string()),
        }
    }
}

impl FromStr for RhFactor {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(RhFactor::Negative),
            "+" => Ok(RhFactor::Positive),
            _ => Err("not in the enum".to_string()),
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.antigen
            .cmp(&other.antigen)
            .then(self.rh_factor.cmp(&other.rh_factor))
    }
}

impl FromStr for BloodType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let an: String = s.to_string()[0..s.len() - 1].to_string();
        let rh = s.to_string()[s.len() - 1..].to_string();

        let valid_rh = RhFactor::from_str(rh.as_str());
        let valid_n = Antigen::from_str(an.as_str());
        if valid_n.is_ok() && valid_rh.is_ok() {
            return Ok(BloodType {
                antigen: valid_n.unwrap(),
                rh_factor: valid_rh.unwrap(),
            });
        }
        Err("Invalide blod type".to_string())
    }
}

use std::fmt::{self, Debug, write};

impl Debug for BloodType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}{:?}", self.antigen, self.rh_factor)
    }
}

impl BloodType {
    pub fn can_receive_from(&self, other: &Self) -> bool {
        let donors = self.donors();
        donors.contains(other)
    }

    pub fn donors(&self) -> Vec<Self> {
        match self.rh_factor {
            RhFactor::Positive => match self.antigen {
                Antigen::AB => vec![
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Positive,
                    },
                ],
                Antigen::O => vec![
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Positive,
                    },
                ],
                Antigen::B => vec![
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Positive,
                    },
                ],
                Antigen::A => vec![
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Positive,
                    },
                ],
            },
            RhFactor::Negative => match self.antigen {
                Antigen::AB => vec![
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Negative,
                    },
                ],
                Antigen::O => vec![ BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Positive,
                    },],
                Antigen::B => vec![
                       BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Negative,
                    },
                           BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Positive,
                    },
                           BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Negative,
                    },
                ],
                Antigen::A => vec![
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Negative,
                    },
                       BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Positive,
                    },
                      BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Positive,
                    },
                ],
            },
        }
    }

    pub fn recipients(&self) -> Vec<BloodType> {
        match self.rh_factor {
            RhFactor::Positive => match self.antigen {
                Antigen::AB => vec![BloodType {
                    antigen: Antigen::AB,
                    rh_factor: RhFactor::Positive,
                }],
                Antigen::O => vec![
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Positive,
                    },
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Positive,
                    },
					        BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Positive,
                    },
					
					        BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Positive,
                    },
					

                ],
                Antigen::B => vec![

                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Positive,
                    },

                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Positive,
                    },
                ],
                Antigen::A => vec![
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Positive,
                    },


                    BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Positive,
                    },
                ],
            },
            RhFactor::Negative => match self.antigen {
                Antigen::AB => vec![
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Positive,
                    },
           
                    BloodType {
                        antigen: Antigen::AB,
                        rh_factor: RhFactor::Negative,
                    },
         
                ],
                Antigen::O => vec![BloodType {
                    antigen: Antigen::O,
                    rh_factor: RhFactor::Negative,
                }],
                Antigen::B => vec![
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::B,
                        rh_factor: RhFactor::Negative,
                    },
                ],
                Antigen::A => vec![
                    BloodType {
                        antigen: Antigen::O,
                        rh_factor: RhFactor::Negative,
                    },
                    BloodType {
                        antigen: Antigen::A,
                        rh_factor: RhFactor::Negative,
                    },
                ],
            },
        }
    }
}
