use regex::Regex;

pub struct Formatter {
    text: String,
}

impl Formatter {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn print(&self) {
        for line in self.text.lines() {
            println!("{}", line);
        }
    }

    pub fn output(self) -> String {
        self.text
    }

    // Formatter modules
    pub fn remove_double_lines(&mut self) {
        log::info!("Removing double lines");
        let mut lines: Vec<&str> = self.text.lines().collect();
        while let Some(index) = lines
            .windows(2)
            .position(|pair| pair[0] == "" && pair[1] == "")
        {
            lines.remove(index);
        }
        self.text = lines.iter().map(|line| format!("{}\n", line)).collect();
    }

    pub fn fix_spacing(&mut self) {
        log::info!("Fixing labels");
        let mut lines: Vec<String> = self.text.lines().map(|x| String::from(x)).collect();
        let is_spot = Regex::new(r#"(^[^"]*:)|(^[^"]*;[^;])"#).unwrap();
        for line in lines.iter_mut() {
            if let Some(position) = is_spot.shortest_match(line) {
                line.insert(position, ' ');
            }
        }
        self.text = lines.iter().map(|line| format!("{}\n", line)).collect();
    }

    pub fn fix_indenting(&mut self) {
        log::info!("Fixing indentation");
        let mut lines: Vec<String> = self.text.lines().map(|x| String::from(x)).collect();
        let dont_indent = Regex::new(r#"(^[^"]*:)|(^[\s]*$)"#).unwrap();
        for line in lines.iter_mut() {
            *line = line.trim().to_string();
            if !dont_indent.is_match(line) {
                line.insert(0, '\t');
            }
        }
        self.text = lines.iter().map(|line| format!("{}\n", line)).collect();
    }

    pub fn fix_whitespace(&mut self) {
        log::info!("Fixing whitespace");
        let mut lines: Vec<String> = self.text.lines().map(|x| String::from(x)).collect();
        let words = Regex::new(r#""[^"]*",?|\s?[^\s]+\s?"#).unwrap();
        for line in lines.iter_mut() {
            let mut new_line: String = String::new();
            for matched in words.find_iter(line) {
                new_line.push_str(format!("{} ", matched.as_str().trim()).as_str());
            }
            *line = new_line;
        }
        self.text = lines.iter().map(|line| format!("{}\n", line)).collect();
    }
}
