pub struct SirParameters {
    pub beta: f64,      // base level rate of infection
    pub sigma: f64,     // base rate of becoming infectious
    pub gamma: f64,     // base level rate of recovery
    pub mu: f64         // base level rate of death
}

pub enum PopulationGroup {
    Susceptible,
    Exposed,
    Infected,
    Recovered,
    Dead
}

pub fn next_state(group: &PopulationGroup, params: &SirParameters, rand: f64) -> PopulationGroup {
    match group {
        PopulationGroup::Susceptible => {
            if rand < params.beta {
                PopulationGroup::Exposed
            } else {
                PopulationGroup::Susceptible
            }
        }
        PopulationGroup::Exposed => {
            if rand < params.sigma {
                PopulationGroup::Infected
            } else {
                PopulationGroup::Exposed
            }
        }
        PopulationGroup::Infected => {
            if rand < params.mu {
                PopulationGroup::Dead
            } else if rand < params.mu + params.gamma {
                PopulationGroup::Recovered
            } 
            else {
                PopulationGroup::Infected
            }
        }
        PopulationGroup::Recovered => PopulationGroup::Recovered,
        PopulationGroup::Dead => PopulationGroup::Dead
    }
}

pub fn simulate_day(population: &mut Vec<PopulationGroup>, params: &SirParameters) {
    for person in population.iter_mut() {
        let rand: f64 = rand::random();
        *person = next_state(&person, &params, rand);
    }
}

pub fn count_groups(population: &[PopulationGroup]) -> (usize, usize, usize, usize, usize) {
    let mut susceptible = 0;
    let mut exposed = 0;
    let mut infected = 0;
    let mut recovered = 0;
    let mut dead: usize = 0;

    for person in population.iter() {
        match person {
            PopulationGroup::Susceptible => susceptible += 1,
            PopulationGroup::Exposed => exposed += 1,
            PopulationGroup::Infected => infected += 1,
            PopulationGroup::Recovered => recovered += 1,
            PopulationGroup::Dead => dead += 1
        }
    }

    (susceptible, exposed, infected, recovered, dead)
}