#![feature(slice_patterns)]

#[derive(Clone, Copy, Debug)]
enum Frame {
    Open(u16, u16),
    Spare(u16),
    Bonus(u16),
    Strike,
}

use Frame::*;

#[derive(Clone, Copy, Debug, PartialEq)]
enum Rolling {
    Regular,
    Extra(u16),
    Over,
}

use Rolling::*;

impl Frame {
    pub fn throw1(&self) -> u16 {
        match *self {
            Open(t1, _) => t1,
            Spare(t1) => t1,
            Strike => 10,
            Bonus(t) => t,
        }
    }

    pub fn throw2(&self) -> u16 {
        match *self {
            Open(_, t2) => t2,
            Spare(t1) => 10 - t1,
            Strike => unimplemented!(),
            Bonus(_) => unimplemented!(),
        }
    }

    pub fn score(&self) -> u16 {
        match *self {
            Bonus(t) => t,
            Open(t1, t2) => t1 + t2,
            Spare(..) | Strike => 10,
        }
    }
}

#[derive(Debug)]
pub struct BowlingGame {
    frames: Vec<Frame>,
    rolling: Rolling,
    partial: Option<u16>,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: Vec::with_capacity(10),
            rolling: Regular,
            partial: None,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), ()> {
        if pins > 10 {
            return Err(());
        }

        match self.rolling {
            Over => {
                return Err(());
            }
            Regular => {
                match (self.partial, pins) {
                    (None, 10) => {
                        self.frames.push(Strike);
                    }
                    (None, pins) => {
                        self.partial = Some(pins);
                    }
                    (Some(prev_pins), pins) if prev_pins + pins > 10 => {
                        return Err(());
                    }
                    (Some(prev_pins), pins) if prev_pins + pins == 10 => {
                        self.frames.push(Spare(prev_pins));
                        self.partial = None;
                    }
                    (Some(prev_pins), pins) => {
                        self.frames.push(Open(prev_pins, pins));
                        self.partial = None;
                    }
                }

                if self.frames.len() == 10 {
                    self.rolling = match self.frames.last() {
                        Some(&Spare(..)) => Extra(1),
                        Some(&Strike) => Extra(2),
                        _ => Over,
                    };
                }
            }
            Extra(remaining) => {
                let frame = match pins {
                    10 => Strike,
                    _ => Bonus(pins),
                };
                self.frames.push(frame);

                self.rolling = if remaining == 1 {
                    Over
                } else {
                    Extra(remaining - 1)
                };
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Result<u16, ()> {
        if self.rolling != Over {
            return Err(());
        }

        let pins: u16 = self.frames.iter().map(Frame::score).sum();
        let bonus: u16 = self.frames[0..10]
            .windows(3)
            .map(|window| {
                match (window[0], window[1], window[2]) {
                    (Strike, Strike, frame) => 10 + frame.throw1(),
                    (Spare(..), frame, _) => frame.throw1(),
                    (Strike, frame, _) => frame.throw1() + frame.throw2(),
                    _ => 0,
                }
            })
            .sum();
        let extra_bonus: u16 = if self.frames.len() > 10 {
            match &self.frames[9..] {
                &[Strike, Strike, Strike] => 20,
                &[_, Strike, frame] => 10 + frame.throw1(),
                _ => 0,
            }
        } else {
            0
        };

        println!("frames: {:?}, extra_bonus: {:?}", self.frames, extra_bonus);

        Ok(pins + bonus + extra_bonus)
    }
}
