enum CelestialBodyType
{
    Star,
    Planet,
    Moon,
    AsteroidOrComet,
    Nebulae,
    Galaxy,
    Pulsar,
    BlackHole,
}

struct CelestialBody {
    name: String,
    body_type: CelestialBodyType,
    mass: String,
    radius: String,
    age: String,
    chemical_composition: Vec<String>, 
}

impl CelestialBody {
    fn show_name(&self) {
        println!("This celestial body name is {}", self.name);
    }

    fn describe_body(&self) {
        match self.body_type
        {
            CelestialBodyType::Star => println!("Estrela: Bola de gás quente, gera energia por fusão."),
            CelestialBodyType::Planet => println!("Planeta: Corpo que orbita uma estrela, sem fusão nuclear."),
            CelestialBodyType::Moon => println!("Lua: Satélite natural que orbita um planeta."),
            CelestialBodyType::AsteroidOrComet => println!("Asteroide/Cometa: Corpo rochoso ou de gelo, viaja pelo espaço."),
            CelestialBodyType::Nebulae => println!("Nebulosa: Nuvem de gás e poeira no espaço."),
            CelestialBodyType::Galaxy => println!("Galáxia: Conjunto de estrelas, planetas e outros corpos."), 
            CelestialBodyType::Pulsar => println!("Pulsar: Estrelas de nêutrons que emitem radiação."),
            CelestialBodyType::BlackHole => println!("Buraco Negro: Região com gravidade intensa, nada escapa."),
        }
    }

    fn show_mass(&self) {
        println!("Mass is: {} kg", self.mass);
    }

    fn show_radius(&self) {
        println!("Radius is: {} km", self.radius)
    }

    fn show_age(&self) {
        println!("Age is: {} years", self.age)
    }

    fn show_chemical_compisition(&self) {
        for chemical in self.chemical_composition.iter() {
            println!("{} contains {}", self.name, chemical);
        }
    }

    fn display_information(&self)
    {
        self.show_name();
        self.describe_body();
        self.show_mass();
        self.show_radius();
        self.show_age();
        self.show_chemical_compisition();
    }

}

fn main() {
    let sun = CelestialBody {
        name: String::from("Sun"),
        body_type: CelestialBodyType::Star,
        mass: String::from("1.989 * 10^30"),
        radius: String::from("696 340"),
        age: String::from("4.603 * 10^9"),
        chemical_composition: vec![
            String::from("Hydrogen"),
            String::from("Helium"),
            String::from("Oxygen"),
            String::from("Carbon"),
            String::from("Iron"),
        ]
    };

    let mars = CelestialBody {
        name: String::from("Mars"),
        body_type: CelestialBodyType::Planet,
        mass: String::from("6.4171 * 10^23"),
        radius: String::from("3 396"),
        age: String::from("4.6 * 10^9"),
        chemical_composition: vec![
            String::from("Carbon dioxide"),
            String::from("Nitrogen"),
            String::from("Argon"),
            String::from("Water"),
        ]
    };

    let europa = CelestialBody {
        name: String::from("Europa (moon of Jupiter)"),
        body_type: CelestialBodyType::Moon,
        mass: String::from("4.799 * 10^22"),
        radius: String::from("1 560.8"),
        age: String::from("4.5 * 10^9"),
        chemical_composition: vec![
            String::from("Water"),
            String::from("Carbon dioxide"),
            String::from("Methane"),
            String::from("Ammonia"),
        ]
    };

    let vesta = CelestialBody {
        name: String::from("Vesta"),
        body_type: CelestialBodyType::AsteroidOrComet,
        mass: String::from("2.590 * 10^20"),
        radius: String::from("525.4"),
        age: String::from("4.56 * 10^9"),
        chemical_composition: vec![
            String::from("Iron"),
            String::from("Nickel"),
            String::from("Silicon"),
            String::from("Magnesium")
        ]
    };  

    let orion_nebula = CelestialBody {
        name: String::from("Orion Nebula"),
        body_type: CelestialBodyType::Nebulae,
        mass: String::from("4.0 * 10^35"),
        radius: String::from("4.629 * 10^16"),
        age: String::from("About 1 million"),
        chemical_composition: vec![
            String::from("Hydrogen"),
            String::from("Helium"),
            String::from("Carbon"),
            String::from("Nitrogen"),
            String::from("Oxygen"),
        ]
    };

    let milky_way = CelestialBody {
        name: String::from("Milky Way"),
        body_type: CelestialBodyType::Galaxy,
        mass: String::from("1.5 * 10^42"),
        radius: String::from("9.461 * 10^17"),
        age: String::from("13.6 billion"),
        chemical_composition: vec![
            String::from("Hydrogen"),
            String::from("Helium"),
            String::from("Carbon"),
            String::from("Nitrogen"),
            String::from("Oxygen"),
        ]
    };

    let psr = CelestialBody {
        name: String::from("PSR B1919+21"),
        body_type: CelestialBodyType::Pulsar,
        mass: String::from("2.786 * 10^30"),
        radius: String::from("10"),
        age: String::from("10^7"),
        chemical_composition: vec![
            String::from("Neutrons"),
            String::from("Iron"),
            String::from("Carbon"),
        ]
    };

    let sagittarius_a = CelestialBody {
        name: String::from("Sagittarius A*"),
        body_type: CelestialBodyType::BlackHole,
        mass: String::from("7.956 * 10^36"),
        radius: String::from("Schwarzschild radius is about 12 million"),
        age: String::from("About 13 billion"),
        chemical_composition: vec![
            String::from("Unknown, but likely"),
            String::from("Compressed matter"),
            String::from("Atoms of heavy elements"),
        ]
    };

    let celestial_bodys: Vec<CelestialBody> = vec![
        sun,
        mars,
        europa,
        vesta,
        orion_nebula,
        milky_way,
        psr,
        sagittarius_a,
    ];

    println!("");
    for celestial_body in celestial_bodys {
        celestial_body.display_information();
        println!("");
    }

}


//demorei 23:17 -> 1:00 da manha pra fazer
//O intervalo entre 23:17 e 1:00 da manhã é 1 hora e 43 minutos.
/*
    -- Compensou acho estou feliz com o resultado, usei bastaste os documentos o learnrust, o
       stack overflow mais esta tudo certo acho aprendi bastante 200 linhas de codigo em 1:43 em
       uma linguagem que estou só aprendendo agora acho que compensou bastante, adeus comentario
       provavelmente nunca mais vou escrever em você e isso é bem triste, mais fazer oque 
       tchau tchau tiago 04/03/2025 - 01:05

*/