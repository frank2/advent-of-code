use std::io;
use std::collections::HashSet;

type Survey = HashSet<char>;

#[derive(Clone, Eq, PartialEq, Debug)]
struct SurveyGroup {
    individuals: Vec<Survey>,
    total: Survey,
    unanimous: Survey,
}
impl SurveyGroup {
    fn new() -> Self {
        Self { individuals: Vec::<Survey>::new(), total: Survey::new(), unanimous: ('a' as u8..='z' as u8).map(|x| x as char).collect() }
    }
    fn add_survey(&mut self, survey: &Survey) {
        self.individuals.push(survey.clone());
        self.total = self.total.union(survey).copied().collect();
        self.unanimous = self.unanimous.intersection(survey).copied().collect();
    }
}

fn read_surveys() -> Result<Vec<SurveyGroup>, ()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut surveys = Vec::<SurveyGroup>::new();
    let mut current_survey = SurveyGroup::new();

    while let Ok(size) = stdin.read_line(&mut buffer) {
        if size == 0 { break; }
        else if size == 1 {
            surveys.push(current_survey);
            current_survey = SurveyGroup::new();
        }
        else {
            current_survey.add_survey(&buffer.trim().chars().collect());
        }

        buffer.clear();
    }

    if current_survey.total.len() > 0 { surveys.push(current_survey); }

    if surveys.len() == 0 { Err(()) }
    else { Ok(surveys) }
}

fn part1() {
    if let Ok(surveys) = read_surveys() {
        println!("{}", surveys.iter().map(|x| x.total.len()).sum::<usize>());
    }
    else { panic!("couldn't read surveys!"); }
}

fn part2() {
    if let Ok(surveys) = read_surveys() {
        println!("{}", surveys.iter().map(|x| x.unanimous.len()).sum::<usize>());
    }
    else { panic!("couldn't read surveys!"); }
}

fn main() {
    // part1();
    part2();
}
