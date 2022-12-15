use nom::{
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Debug, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn distance_to_point(lhs: &Position, rhs: &Position) -> i64 {
        (rhs.x.abs_diff(lhs.x) + rhs.y.abs_diff(lhs.y)) as i64
    }
}

#[derive(Debug)]
struct Sensor {
    beacon: Position,
    position: Position,
    range: i64,
}

impl Sensor {
    fn new(sensor_pos: (i64, i64), beacon_pos: (i64, i64)) -> Self {
        let sensor_pos = Position::new(sensor_pos.0, sensor_pos.1);
        let beacon_pos = Position::new(beacon_pos.0, beacon_pos.1);

        Self {
            range: Position::distance_to_point(&sensor_pos, &beacon_pos),
            position: sensor_pos,
            beacon: beacon_pos,
        }
    }

    fn distance_to_beacon(&self) -> i64 {
        self.range
    }

    fn in_range_of(&self, other: &Position) -> bool {
        match self.beacon == *other {
            true => false,
            false => {
                let sensor_to_point_distance = Position::distance_to_point(&self.position, other);
                let sensor_range = self.distance_to_beacon();

                sensor_range >= sensor_to_point_distance
            }
        }
    }
}

fn parse_coords(input: &str) -> IResult<&str, (i64, i64)> {
    let (input, coords) = separated_pair(
        preceded(tag("x="), nom::character::complete::i64),
        tag(", "),
        preceded(tag("y="), nom::character::complete::i64),
    )(input)?;
    Ok((input, coords))
}

fn parse(input: &str) -> IResult<&str, Vec<Sensor>> {
    let (input, result) = separated_list1(
        line_ending,
        map(
            separated_pair(
                preceded(tag("Sensor at "), parse_coords),
                tag(": "),
                preceded(tag("closest beacon is at "), parse_coords),
            ),
            |(l, r)| Sensor::new(l, r),
        ),
    )(input)?;

    Ok((input, result))
}

fn solution_1(input: &str, line: i64) -> usize {
    let (_, sensors) = parse(input).unwrap();

    let min_x = sensors
        .iter()
        .map(|sensor| sensor.position.x - sensor.distance_to_beacon())
        .min()
        .unwrap();

    let max_x = sensors
        .iter()
        .map(|sensor| sensor.position.x + sensor.distance_to_beacon())
        .max()
        .unwrap();

    (min_x..=max_x)
        .filter(|&x_pos| {
            sensors
                .iter()
                .any(|sensor| sensor.in_range_of(&Position::new(x_pos, line)))
        })
        .count()
}

fn solution_2(input: &str, line: i64) -> Option<i64> {
    let (_, sensors) = parse(input).unwrap();

    sensors.iter().find_map(|sensor| {
        let x_pos = (sensor.position.x - (sensor.distance_to_beacon() + 1)).max(0);
        (x_pos..=sensor.position.x)
            .zip(sensor.position.y..=line)
            .find(|pos| {
                sensors
                    .iter()
                    .all(|sensor| !sensor.in_range_of(&Position::new(pos.0, pos.1)))
            })
            .map(|(x, y)| x * 4000000 + y)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution_1_example() {
        let input = include_str!("input/example/day15.txt");

        assert_eq!(26, solution_1(input, 10));
    }

    #[test]
    fn solution_1_input() {
        let input = include_str!("input/day15.txt");

        assert_eq!(4724228, solution_1(input, 2000000));
    }

    #[test]
    fn solution_2_example() {
        let input = include_str!("input/example/day15.txt");

        assert_eq!(56000011, solution_2(input, 20).unwrap());
    }

    #[test]
    fn solution_2_input() {
        let input = include_str!("input/day15.txt");

        assert_eq!(13622251246513, solution_2(input, 4000000).unwrap());
    }
}
