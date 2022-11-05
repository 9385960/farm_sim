mod Plant;

struct Plant {
    lifetime: u16,
    plant_type: Type,
    nourishment: f32,
}

enum Type {
    Wheat,
    Carrot,
    Beet,
    Potato,
    Berries,
}
