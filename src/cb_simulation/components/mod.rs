// Note; do this using ECS

/*
    A unit has the following:
    ** Move speed
    ** Point cost
    ** Base size (like 24mm, 40mm, 50mm, 60mm, etc. in 40k)
    ** Flyer
    ** Passive Ability (or something that the player does not control)
    ** Active Ability (can toggle to autouse)
    ** Health points
    ** Armor points
    ** Attack range
    ** Attack damage
    ** Attack rate
*/

pub struct Unit {
    attack_range: u8,
    move_speed: u8,
}
