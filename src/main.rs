mod models {
    pub mod sir;
}
use models::sir::*;

fn main() {
    let params = SirParameters {
        beta: 0.05,
        sigma: 0.01,
        gamma: 0.05,
        mu: 0.001
    };
    
    let mut population: Vec<PopulationGroup> = vec![];

    // healthy population of 990 people
    for _ in 0..990 {
        population.push(PopulationGroup::Susceptible);
    }

    // infected population of 10 people
    for _ in 0..10 {
        population.push(PopulationGroup::Infected);
    }

    for day in 1..=365 {
        simulate_day(&mut population, &params);

        let (s, e, i, r, d) = count_groups(&population);
        println!("Day {}: S={}, E={}, I={}, R={}, D={}", day, s, e, i, r, d);

    }
}
