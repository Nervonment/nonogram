pub type Line = u64;

#[derive(Clone)]
pub struct Domain(pub Vec<Line>);

impl Domain {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn insert(&mut self, line: Line) {
        self.0.push(line);
    }

    pub fn size(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    Column,
    Row,
}

pub fn enumerate_domain(
    line_info: &Vec<i32>,
    num_idx: usize,
    line_width: usize,
    start_pos: usize,
    line: Line,
    listed: &mut Domain,
) {
    if num_idx >= line_info.len() {
        listed.insert(line);
        return;
    }

    let bar_len = line_info[num_idx];
    let bar = (1 << bar_len) - 1;
    for place_pos in start_pos..line_width {
        if place_pos + bar_len as usize > line_width {
            break;
        }

        let new_line = line | (bar << place_pos);
        enumerate_domain(
            line_info,
            num_idx + 1,
            line_width,
            place_pos + bar_len as usize + 1,
            new_line,
            listed,
        );
    }
}

#[test]
fn test_enumerate_domain() {
    let mut domain = Domain::new();
    let line_info = vec![5, 3, 1];
    let line_width = 15;
    enumerate_domain(&line_info, 0, line_width, 0, 0, &mut domain);
    for line in domain.0 {
        for i in 0..line_width {
            print!("{}", if line & (1 << i) != 0 { "🟩" } else { "⬛" });
        }
        println!();
    }
}
