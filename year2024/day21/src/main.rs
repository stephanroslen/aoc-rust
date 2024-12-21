use std::collections::HashMap;
use util::coord2d::ICoord2D;
use util::error::Errors;

mod detail {
    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub struct SequenceElement<T: Eq> {
        pub key: T,
        pub repetitions: isize,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Sequence<T: Copy + Eq> {
        pub sequence: Vec<SequenceElement<T>>,
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum NumericKeyboard {
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyA,
}

impl NumericKeyboard {
    fn location(self) -> ICoord2D {
        use NumericKeyboard::*;
        match self {
            Key7 => ICoord2D { x: 0, y: 0 },
            Key8 => ICoord2D { x: 1, y: 0 },
            Key9 => ICoord2D { x: 2, y: 0 },
            Key4 => ICoord2D { x: 0, y: 1 },
            Key5 => ICoord2D { x: 1, y: 1 },
            Key6 => ICoord2D { x: 2, y: 1 },
            Key1 => ICoord2D { x: 0, y: 2 },
            Key2 => ICoord2D { x: 1, y: 2 },
            Key3 => ICoord2D { x: 2, y: 2 },
            Key0 => ICoord2D { x: 1, y: 3 },
            KeyA => ICoord2D { x: 2, y: 3 },
        }
    }

    fn parent_sequence_candidates(
        self,
        to: NumericKeyboard,
    ) -> Vec<detail::Sequence<DirectionalKeyboard>> {
        let loc_self = self.location();
        let loc_to = to.location();

        use DirectionalKeyboard::*;

        match (loc_self, loc_to) {
            (f, t) if t == f => vec![detail::Sequence { sequence: vec![] }],
            (f, t) if t.x < f.x && t.y == f.y => vec![detail::Sequence {
                sequence: vec![
                    detail::SequenceElement {
                        key: KeyL,
                        repetitions: f.x - t.x,
                    },
                    detail::SequenceElement {
                        key: DirectionalKeyboard::KeyA,
                        repetitions: 1,
                    },
                ],
            }],
            (f, t) if t.x > f.x && t.y == f.y => vec![detail::Sequence {
                sequence: vec![
                    detail::SequenceElement {
                        key: KeyR,
                        repetitions: t.x - f.x,
                    },
                    detail::SequenceElement {
                        key: DirectionalKeyboard::KeyA,
                        repetitions: 1,
                    },
                ],
            }],
            (f, t) if t.x == f.x && t.y < f.y => vec![detail::Sequence {
                sequence: vec![
                    detail::SequenceElement {
                        key: KeyU,
                        repetitions: f.y - t.y,
                    },
                    detail::SequenceElement {
                        key: DirectionalKeyboard::KeyA,
                        repetitions: 1,
                    },
                ],
            }],
            (f, t) if t.x == f.x && t.y > f.y => vec![detail::Sequence {
                sequence: vec![
                    detail::SequenceElement {
                        key: KeyD,
                        repetitions: t.y - f.y,
                    },
                    detail::SequenceElement {
                        key: DirectionalKeyboard::KeyA,
                        repetitions: 1,
                    },
                ],
            }],
            (f, t) if f.x == 0 && t.y == 3 => vec![detail::Sequence {
                sequence: vec![
                    detail::SequenceElement {
                        key: KeyR,
                        repetitions: t.x - f.x,
                    },
                    detail::SequenceElement {
                        key: KeyD,
                        repetitions: t.y - f.y,
                    },
                    detail::SequenceElement {
                        key: DirectionalKeyboard::KeyA,
                        repetitions: 1,
                    },
                ],
            }],
            (f, t) if f.y == 3 && t.x == 0 => vec![detail::Sequence {
                sequence: vec![
                    detail::SequenceElement {
                        key: KeyU,
                        repetitions: f.y - t.y,
                    },
                    detail::SequenceElement {
                        key: KeyL,
                        repetitions: f.x - t.x,
                    },
                    detail::SequenceElement {
                        key: DirectionalKeyboard::KeyA,
                        repetitions: 1,
                    },
                ],
            }],
            (f, t) => {
                let diff = t - f;
                let key_x = if diff.x < 0 { KeyL } else { KeyR };
                let key_y = if diff.y < 0 { KeyU } else { KeyD };
                let repetitions_x = diff.x.abs();
                let repetitions_y = diff.y.abs();

                assert_ne!(repetitions_x, 0);
                assert_ne!(repetitions_y, 0);

                vec![
                    detail::Sequence {
                        sequence: vec![
                            detail::SequenceElement {
                                key: key_x,
                                repetitions: repetitions_x,
                            },
                            detail::SequenceElement {
                                key: key_y,
                                repetitions: repetitions_y,
                            },
                            detail::SequenceElement {
                                key: DirectionalKeyboard::KeyA,
                                repetitions: 1,
                            },
                        ],
                    },
                    detail::Sequence {
                        sequence: vec![
                            detail::SequenceElement {
                                key: key_y,
                                repetitions: repetitions_y,
                            },
                            detail::SequenceElement {
                                key: key_x,
                                repetitions: repetitions_x,
                            },
                            detail::SequenceElement {
                                key: DirectionalKeyboard::KeyA,
                                repetitions: 1,
                            },
                        ],
                    },
                ]
            }
        }
    }

    fn cost_push(
        state: NumericKeyboard,
        button: NumericKeyboard,
        directional_robot_keypads: usize,
        cache: &mut HashMap<(DirectionalKeyboard, DirectionalKeyboard, usize), usize>,
    ) -> usize {
        let sequences = state.parent_sequence_candidates(button);

        let min_cost = sequences
            .iter()
            .map(|sequence| {
                let mut cost = 0usize;
                let mut current_parent = DirectionalKeyboard::KeyA;
                for &detail::SequenceElement {
                    key: key_parent,
                    repetitions,
                } in &sequence.sequence
                {
                    for _ in 0..repetitions {
                        cost += DirectionalKeyboard::cost_push(
                            current_parent,
                            key_parent,
                            directional_robot_keypads,
                            cache,
                        );
                        current_parent = key_parent;
                    }
                }
                cost
            })
            .min()
            .expect("Minimum expected to exist");

        min_cost
    }
}

impl TryFrom<char> for NumericKeyboard {
    type Error = Errors;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        use NumericKeyboard::*;
        match value {
            '0' => Ok(Key0),
            '1' => Ok(Key1),
            '2' => Ok(Key2),
            '3' => Ok(Key3),
            '4' => Ok(Key4),
            '5' => Ok(Key5),
            '6' => Ok(Key6),
            '7' => Ok(Key7),
            '8' => Ok(Key8),
            '9' => Ok(Key9),
            'A' => Ok(KeyA),
            _ => Err(Errors::ConversionError),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum DirectionalKeyboard {
    KeyU,
    KeyR,
    KeyD,
    KeyL,
    KeyA,
}

impl DirectionalKeyboard {
    fn location(self) -> ICoord2D {
        use DirectionalKeyboard::*;
        match self {
            KeyU => ICoord2D { x: 1, y: 0 },
            KeyA => ICoord2D { x: 2, y: 0 },
            KeyL => ICoord2D { x: 0, y: 1 },
            KeyD => ICoord2D { x: 1, y: 1 },
            KeyR => ICoord2D { x: 2, y: 1 },
        }
    }

    fn parent_sequence_candidates(
        self,
        to: DirectionalKeyboard,
    ) -> Vec<detail::Sequence<DirectionalKeyboard>> {
        let loc_self = self.location();
        let loc_to = to.location();

        use DirectionalKeyboard::*;

        match (loc_self, loc_to) {
            (f, t) if t == f => vec![detail::Sequence {
                sequence: vec![detail::SequenceElement {
                    key: DirectionalKeyboard::KeyA,
                    repetitions: 1,
                }],
            }],
            (f, t) if t.x < f.x && t.y == f.y => vec![detail::Sequence {
                sequence: vec![
                    detail::SequenceElement {
                        key: KeyL,
                        repetitions: f.x - t.x,
                    },
                    detail::SequenceElement {
                        key: DirectionalKeyboard::KeyA,
                        repetitions: 1,
                    },
                ],
            }],
            (f, t) if t.x > f.x && t.y == f.y => vec![detail::Sequence {
                sequence: vec![
                    detail::SequenceElement {
                        key: KeyR,
                        repetitions: t.x - f.x,
                    },
                    detail::SequenceElement {
                        key: DirectionalKeyboard::KeyA,
                        repetitions: 1,
                    },
                ],
            }],
            (f, t) if t.x == f.x && t.y < f.y => vec![detail::Sequence {
                sequence: vec![
                    detail::SequenceElement {
                        key: KeyU,
                        repetitions: f.y - t.y,
                    },
                    detail::SequenceElement {
                        key: DirectionalKeyboard::KeyA,
                        repetitions: 1,
                    },
                ],
            }],
            (f, t) if t.x == f.x && t.y > f.y => vec![detail::Sequence {
                sequence: vec![
                    detail::SequenceElement {
                        key: KeyD,
                        repetitions: t.y - f.y,
                    },
                    detail::SequenceElement {
                        key: DirectionalKeyboard::KeyA,
                        repetitions: 1,
                    },
                ],
            }],
            (f, t) if f.x == 0 && t.y == 0 => vec![detail::Sequence {
                sequence: vec![
                    detail::SequenceElement {
                        key: KeyR,
                        repetitions: t.x - f.x,
                    },
                    detail::SequenceElement {
                        key: KeyU,
                        repetitions: f.y - t.y,
                    },
                    detail::SequenceElement {
                        key: DirectionalKeyboard::KeyA,
                        repetitions: 1,
                    },
                ],
            }],
            (f, t) if f.y == 0 && t.x == 0 => vec![detail::Sequence {
                sequence: vec![
                    detail::SequenceElement {
                        key: KeyD,
                        repetitions: t.y - f.y,
                    },
                    detail::SequenceElement {
                        key: KeyL,
                        repetitions: f.x - t.x,
                    },
                    detail::SequenceElement {
                        key: DirectionalKeyboard::KeyA,
                        repetitions: 1,
                    },
                ],
            }],
            (f, t) => {
                let diff = t - f;
                let key_x = if diff.x < 0 { KeyL } else { KeyR };
                let key_y = if diff.y < 0 { KeyU } else { KeyD };
                let repetitions_x = diff.x.abs();
                let repetitions_y = diff.y.abs();

                assert_ne!(repetitions_x, 0);
                assert_ne!(repetitions_y, 0);

                vec![
                    detail::Sequence {
                        sequence: vec![
                            detail::SequenceElement {
                                key: key_x,
                                repetitions: repetitions_x,
                            },
                            detail::SequenceElement {
                                key: key_y,
                                repetitions: repetitions_y,
                            },
                            detail::SequenceElement {
                                key: DirectionalKeyboard::KeyA,
                                repetitions: 1,
                            },
                        ],
                    },
                    detail::Sequence {
                        sequence: vec![
                            detail::SequenceElement {
                                key: key_y,
                                repetitions: repetitions_y,
                            },
                            detail::SequenceElement {
                                key: key_x,
                                repetitions: repetitions_x,
                            },
                            detail::SequenceElement {
                                key: DirectionalKeyboard::KeyA,
                                repetitions: 1,
                            },
                        ],
                    },
                ]
            }
        }
    }

    fn cost_push(
        state: DirectionalKeyboard,
        button: DirectionalKeyboard,
        layer: usize,
        cache: &mut HashMap<(DirectionalKeyboard, DirectionalKeyboard, usize), usize>,
    ) -> usize {
        use DirectionalKeyboard::*;

        if layer == 0 {
            return 1;
        }

        if let Some(result) = cache.get(&(state, button, layer)) {
            return *result;
        }

        let sequences = state.parent_sequence_candidates(button);

        let min_cost = sequences
            .iter()
            .map(|sequence| {
                let mut cost = 0usize;
                let mut current_parent = KeyA;
                for &detail::SequenceElement {
                    key: key_parent,
                    repetitions,
                } in &sequence.sequence
                {
                    for _ in 0..repetitions {
                        cost += Self::cost_push(current_parent, key_parent, layer - 1, cache);
                        current_parent = key_parent;
                    }
                }
                cost
            })
            .min()
            .expect("Minimum expected to exist");

        cache.insert((state, button, layer), min_cost);

        min_cost
    }
}

struct Line {
    sequence: Vec<NumericKeyboard>,
    value: usize,
}

fn result(
    lines: &Vec<Line>,
    directional_robot_keypads: usize,
    cache: &mut HashMap<(DirectionalKeyboard, DirectionalKeyboard, usize), usize>,
) -> usize {
    lines
        .iter()
        .map(|Line { sequence, value }| {
            let mut accum = 0usize;
            let mut from = NumericKeyboard::KeyA;
            for &to in sequence.iter() {
                accum += NumericKeyboard::cost_push(from, to, directional_robot_keypads, cache);
                from = to;
            }
            accum * *value
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_input = std::fs::read_to_string("input")?;
    let raw_lines = raw_input.trim_end().split('\n').collect::<Vec<_>>();

    let lines = raw_lines
        .iter()
        .map(|l| -> Result<_, Errors> {
            let sequence = l
                .chars()
                .map(|c| c.try_into())
                .collect::<Result<Vec<NumericKeyboard>, Errors>>()?;
            let value = l[0..3].parse::<usize>()?;
            Ok(Line { sequence, value })
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut cache = HashMap::new();

    println!("{}", result(&lines, 2, &mut cache));
    println!("{}", result(&lines, 25, &mut cache));

    Ok(())
}
