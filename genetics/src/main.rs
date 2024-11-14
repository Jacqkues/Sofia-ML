extern crate csv;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::path::Path;
use csv::ReaderBuilder;
use rand::seq::SliceRandom;
use rand::Rng;

#[derive(Debug)]
struct Coordinates{
    x:f64,
    y:f64,
}

fn dist_city(c1: &Coordinates , c2: &Coordinates) -> f64{
    let dx = c1.x - c2.x;
    let dy= c1.y - c2.y;
    (dx*dx + dy*dy).sqrt()
}




fn read_city_coordinates(city_file: &str, coordinates_file: &str) -> Result<HashMap<String, Coordinates>, Box<dyn Error>> {
    // Création d'une HashMap pour stocker les coordonnées des villes
    let mut city_coordinates: HashMap<String, Coordinates> = HashMap::new();

    // Ouverture et lecture du fichier des noms de villes
    let city_path = Path::new(city_file);
    let city_file = File::open(city_path)?;
    let mut city_reader = ReaderBuilder::new().has_headers(false).from_reader(city_file);

    // Ouverture et lecture du fichier des coordonnées
    let coordinates_path = Path::new(coordinates_file);
    let coordinates_file = File::open(coordinates_path)?;
    let mut coordinates_reader = ReaderBuilder::new().has_headers(false).from_reader(coordinates_file);

    // Itération sur les deux fichiers CSV en parallèle
    for (city_result, coordinates_result) in city_reader.records().zip(coordinates_reader.records()) {
        // Récupération des enregistrements des deux fichiers
        let city_record = city_result?;
        let coordinates_record = coordinates_result?;

        // Récupération du nom de la ville et de ses coordonnées
        let city_name = city_record.get(0).ok_or("Missing city name")?.to_owned();
        let x = coordinates_record.get(0).ok_or("Missing x coordinate")?.parse::<f64>()?;
        let y = coordinates_record.get(1).ok_or("Missing y coordinate")?.parse::<f64>()?;

        // Création de la structure Coordinates
        let coordinates = Coordinates { x, y };

        // Association de la ville à ses coordonnées dans la HashMap
        city_coordinates.insert(city_name, coordinates);
    }

    Ok(city_coordinates)
}

fn initialize_population(size: usize, cities: &Vec<String>) -> Vec<Vec<String>> {
    let mut rng = rand::thread_rng();
    let mut population = Vec::new();
    for _ in 0..size {
        let mut individual = cities.clone();
        individual.shuffle(&mut rng);
        population.push(individual);
    }
    population
}



fn build_distance_matrix(city_coordinates: &HashMap<String, Coordinates>) -> HashMap<(String, String), f64> {

    let mut distance_matrix: HashMap<(String, String), f64> = HashMap::new();
    for (city1, coordinates1) in city_coordinates.iter() {
        for (city2, coordinates2) in city_coordinates.iter() {
            let distance = dist_city(coordinates1, coordinates2);
            distance_matrix.insert((city1.clone(), city2.clone()), distance);
        }
    }
    distance_matrix
}

fn fitness(individual: &Vec<String>, distances: &HashMap<(String, String), f64>) -> f64 {
    let mut total_distance = 0.0;
    for cities in individual.windows(2) {
        total_distance += distances.get(&(cities[0].clone(), cities[1].clone())).unwrap();
    }
    // Inverse de la distance totale pour que les individus avec une distance plus courte aient une fitness plus élevée
   // 1.0 / total_distance
  1.0/ total_distance
}

fn selection(population: &Vec<Vec<String>>, distances: &HashMap<(String, String), f64>) -> Vec<Vec<String>> {
    let mut rng = rand::thread_rng();
    let mut selected = Vec::new();
    for _ in 0..population.len() {
        let individual1 = population.choose(&mut rng).unwrap();
        let individual2 = population.choose(&mut rng).unwrap();
        selected.push(if fitness(individual1, distances) > fitness(individual2, distances) {
            individual1.clone()
        } else {
            individual2.clone()
        });
    }
    selected
}



fn crossover(parent1: &Vec<String>, parent2: &Vec<String>) -> Vec<String> {
    let mut rng = rand::thread_rng();
    let cut = rng.gen_range(0..parent1.len());
    let mut child = parent1[0..cut].to_vec();
    for city in parent2 {
        if !child.contains(city) {
            child.push(city.clone());
        }
    }
    child
}

fn mutate(individual: &mut Vec<String>) {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.01) { // 1% de chance de mutation
        let idx1 = rng.gen_range(0..individual.len());
        let idx2 = rng.gen_range(0..individual.len());
        individual.swap(idx1, idx2);
    }
}

fn genetic_algorithm(cities: Vec<String>, distances: &HashMap<(String, String), f64>) -> Vec<String>{
    let mut population = initialize_population(500, &cities);
    for _ in 0..1000 {
        population = selection(&population, &distances);
        let mut new_population = Vec::new();
        while new_population.len() < population.len() {
             let parent1 = population.choose(&mut rand::thread_rng()).unwrap();
            let parent2 = population.choose(&mut rand::thread_rng()).unwrap();
          //  let parent1 = &population[0];
           // let parent2 = &population[1];
            let mut child = crossover(parent1, parent2);
            mutate(&mut child);
            new_population.push(child);
        }
        population = new_population;
    }
    population.sort_by(|a, b| fitness(b, &distances).partial_cmp(&fitness(a, &distances)).unwrap());
    population[0].clone()

}


fn main() -> Result<(), Box<dyn Error>> {

  

    let city_coordinates = read_city_coordinates("./UK_TSP/uk12_name.csv", "./UK_TSP/uk12_xy.csv")?;
    let city_names: Vec<_> = city_coordinates.keys().cloned().collect();

    let distance_matrix = build_distance_matrix(&city_coordinates);

    let best_individual = genetic_algorithm(city_names, &distance_matrix);
    println!("{:?}", &best_individual);
    println!("{:?}", 1.0/fitness(&best_individual, &distance_matrix));
  
    Ok(())
}
