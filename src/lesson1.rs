
use bevy::prelude::*;


#[derive(Component)]
pub struct Person {
    pub name: String,
}


#[derive(Component)]
pub struct Employed {
    pub job: Job,
}

#[derive(Debug)]
pub enum Job {
    Scientist, 
    Musician,
    BladeRunner, 
    SpaceCaptain
}

pub fn setup(mut commands: Commands) {
    
    commands.spawn((
        Person {
            name: "Deckard".to_string()
        }, 
        Employed {
            job: Job::BladeRunner
        }
    ));
    commands.spawn(Person {
        name: "Ripply".to_string()
    });
    commands.spawn(Person {
        name: "Doc".to_string()
    });
    commands.spawn((
        Person {
            name: "Marty".to_string()
        },
        Employed {
            job: Job::Musician
        }
    ));
}

pub fn person_does_job(person_query: Query<(&Person, &Employed)>) {
    for (person, employed) in person_query.iter() {
        let job_name = match employed.job {
            Job::BladeRunner => "BladeRunner", 
            Job::Musician => "Musician", 
            Job::Scientist => "Scientist",
            Job::SpaceCaptain => "SpaceCaptain"
        };
        println!("{} is a {}", person.name, job_name);
    }
}

pub fn people_with_jobs(person_query: Query<&Person, With<Employed>>) {
    for person in person_query.iter() {
        println!("{} has a job", person.name);
    }
}

pub fn people_ready_for_hire(person_query: Query<&Person, Without<Employed>>) {
    for person in person_query.iter() {
        println!("{} is available for hire", person.name);
    }
}

pub fn print_names(person_query: Query<&Person>) {
    for person in person_query.iter() {     
        println!("Name: {}", person.name);
    }
}

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(print_names)
            .add_system(people_with_jobs)
            .add_system(people_ready_for_hire)
            .add_system(person_does_job);
    }
}
