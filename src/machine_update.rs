use bevy::prelude::*;

const MARGINE : f32 = 0.01;

use crate::machine::Machine;

fn machine_update(
    machine : Query<(&Parent, &Machine)>,
    machine_location : Query<&Transform>,
)
{
    for(parent, vehicle) in machine.iter()
    {
        let vehicle_destination = vehicle.get_destination();
        let vehicle_destination = Vec3::new(vehicle_destination[0],vehicle_destination[1],vehicle_destination[2]);
        let mut p = machine_location.get(parent.get()).expect("vehicle");
        if((p.translation - vehicle_destination).length() < MARGINE)
        {

        }
    }
}