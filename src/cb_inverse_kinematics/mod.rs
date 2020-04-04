extern crate nalgebra as na;
use na::Vector2;

use crate::cb_math::sqrt_f32;

extern crate fixed;
use fixed::types::I24F8;
use fixed::FixedI32;

#[derive(Clone)]
struct Rotor {}

type Tnum = f32;
pub type CbMatrix = na::Vector2<Tnum>; // Note: using a self aliased type here for 2d/3d implementations. First, get 2d working, then can easily export that to use 3d. Change to use fixed point.

pub type CbRotationMatrix = na::Vector1<Tnum>;

const SOLVE_TOLERANCE: f32 = 0.01;

#[derive(Clone)]
pub struct IkRig {
    pub target: Option<CbMatrix>,
    pub joints: Vec<ChildTypes>,
    rotors: Vec<Option<Rotor>>,
    pub joint_distances: Vec<Tnum>, // Will be of size N-1, where N is number of positions (since it's distances per joint pairs)
    pub position: CbMatrix,
}

#[derive(Clone)]
pub enum ChildTypes {
    Joint(CbMatrix),
    SubChain(IkRig),
}

fn get_child_position(child: &ChildTypes) -> CbMatrix {
    match child {
        ChildTypes::Joint(pos) => *pos,
        ChildTypes::SubChain(chain) => get_child_position(&chain.joints[0]),
    }
}

fn set_child_position(child: &mut ChildTypes, position: CbMatrix) {
    match child {
        ChildTypes::Joint(p) => {
            *p = position;
        }
        ChildTypes::SubChain(sub_chain) => {
            set_child_position(&mut sub_chain.joints[0], position);
        }
    };
}

impl IkRig {
    pub fn new() -> Self {
        let mut rig = Self {
            target: None,
            joints: vec![],
            rotors: vec![],
            joint_distances: vec![],
            position: CbMatrix::new(0.0, 0.0),
        };

        for i in 0..6 {
            rig.add_joint(CbMatrix::new(60.0 * i as f32, 0.0), None);
        }

        return rig;
    }

    pub fn get_child_position(&self, i: usize) -> CbMatrix {
        return get_child_position(&self.joints[i]);
    }

    /// Returns whether the current rig is a valid rig or not. A valid rig has at least 2 joints.
    pub fn is_valid_rig(&self) -> bool {
        const MIN_JOINTS_FOR_FABRIK: usize = 2;

        return self.joints.len() >= MIN_JOINTS_FOR_FABRIK;
    }

    /// Add a new joint to the IK rig. Calculates the distance for later use.
    fn add_joint(&mut self, joint_position: CbMatrix, rotor: Option<Rotor>) {
        let last = self.joints.last();

        if last.is_some() {
            let last = last.unwrap();
            let last = get_child_position(last);

            let distance = distance(last, joint_position);
            self.joint_distances.push(distance);
        }

        self.joints.push(ChildTypes::Joint(joint_position));
        self.rotors.push(rotor); //NOTE: not sure if a rotor is between two joints, or each joint has a rotor
    }
}

pub fn fabrik(rig: &mut IkRig) {
    // Boundary condition checks
    {
        if rig.target.is_none() || rig.joints.is_empty() || !rig.is_valid_rig() {
            return;
        }

        // Assert that first child is a matrix and not a subchain.
        match rig.joints[0] {
            ChildTypes::Joint(_) => { // ok
            }
            ChildTypes::SubChain(_) => {
                panic!("Unable to have a subchain as the first child joint!")
            }
        }
    }

    let target_position = rig.target.unwrap();
    /*
        Right now just going for single IK chain with single end effectors, will update it as time goes
    */

    let n = rig.joints.len();
    // Note, in the algorithm, it indexes from 1 to N. Rust, however, indexes from 0..N, so not much actually changes. Just pointing it out. May be a few instances where the index
    // in code looks different from the algorithm; this is expected.

    // Base algorithm found on: http://www.andreasaristidou.com/publications/papers/FABRIK.pdf

    //1.1 Distance between root and target
    let dist = abs(distance(
        get_child_position(&rig.joints[0]),
        target_position,
    )); // 1.2
        //1.3 Check whether target is in reach
    if dist > sum(&rig.joint_distances) {
        // 1.4
        //1.5 Target is unreachable
        for i in 0..(n - 1) {
            // Execute algorithm 1, finding the new position
            {
                // 1.6
                // 1.7 Find the distance ri between target t and joint position pi
                let ri = abs(distance(
                    target_position,
                    get_child_position(&rig.joints[i]),
                )); //1.8
                let lambda_i = rig.joint_distances[i] / ri; //1.9
                                                            //1.10 Find new joint positions pi

                let new_pos = (1.0 - lambda_i) * get_child_position(&rig.joints[i])
                    + lambda_i * target_position;
                set_child_position(&mut rig.joints[i + 1], new_pos);
            }
            // Execute algorithm 2
            {
                rig.joints[i] = apply_orientational_constraints(
                    &rig.joints[i + 1], // NOTE: not sure if this is the proper indexing or if backwards
                    &rig.joints[i], // NOTE: not sure if this is the proper indexing or if backwards
                    &rig.rotors[i], //NOTE: not sure if a rotor is between two joints, or each joint has a rotor
                );
            }

            //TODO: Execute algorithm 3
            {
                apply_rotational_constraints();
            }

            //1.11
        } // 1.12
    } else {
        //1.13
        //1.14 target is reachable, set b as the initial position of p0
        let b = get_child_position(&rig.joints[0]); // 1.15
                                                    //1.16 Check whether distance between end effector pn and the target is greater than a tolerance
        let mut diff_a = abs(distance(
            get_child_position(&rig.joints[n - 1]),
            target_position,
        )); //1.17
        while diff_a > SOLVE_TOLERANCE {
            //1.18
            //1.19 Stage 1: Forward reaching
            //1.20 Set end effector pn as target t
            set_child_position(&mut rig.joints[n - 1], target_position); //1.21

            //note: skip the last value, as it's being handled elsewhere
            for i in (0..n - 1).rev() {
                // Execute algorithm 1, finding the new position
                {
                    //1.22
                    //1.23 Find the distance ri between the new joint position pi+1 and the joint pi
                    let ri = abs(distance(
                        get_child_position(&rig.joints[i + 1]),
                        get_child_position(&rig.joints[i]),
                    )); //1.24
                    let lambda_i = rig.joint_distances[i] / ri; //1.25
                                                                //1.26 Find the new joint positions pi

                    let new_pos = (1.0 - lambda_i) * get_child_position(&rig.joints[i + 1])
                        + lambda_i * get_child_position(&rig.joints[i]);
                    set_child_position(&mut rig.joints[i], new_pos);
                    //1.27
                }
                // Execute algorithm 2
                {
                    rig.joints[i] = apply_orientational_constraints(
                        &rig.joints[i + 1], // NOTE: not sure if this is the proper indexing or if backwards
                        &rig.joints[i], // NOTE: not sure if this is the proper indexing or if backwards
                        &rig.rotors[i], //NOTE: not sure if a rotor is between two joints, or each joint has a rotor
                    );
                }

                //TODO: Execute algorithm 3
                {
                    apply_rotational_constraints();
                }
            } // 1.28
              //1.29 Stage 2: Backwards reaching
              //1.30 Set the root p0 it's initial position

            set_child_position(&mut rig.joints[0], b); //1.31

            for i in 0..n - 1 {
                // Execute algorithm 1, finding the new position
                {
                    //1.32
                    //1.33 Find the distance ri between the new joint position pi and the joint pi+1
                    let ri = abs(distance(
                        get_child_position(&rig.joints[i + 1]),
                        get_child_position(&rig.joints[i]),
                    )); //1.34
                    let lambda_i = rig.joint_distances[i] / ri; //1.35
                                                                //1.36 Find the new joint positions pi

                    let new_pos = (1.0 - lambda_i) * get_child_position(&rig.joints[i])
                        + lambda_i * get_child_position(&rig.joints[i + 1]);
                    set_child_position(&mut rig.joints[i + 1], new_pos);
                }

                // Execute algorithm 2
                {
                    rig.joints[i] = apply_orientational_constraints(
                        &rig.joints[i + 1], // NOTE: not sure if this is the proper indexing or if backwards
                        &rig.joints[i], // NOTE: not sure if this is the proper indexing or if backwards
                        &rig.rotors[i], //NOTE: not sure if a rotor is between two joints, or each joint has a rotor
                    );
                }

                //TODO: Execute algorithm 3
                {
                    apply_rotational_constraints();
                }

                //1.37
            } //1.38
            diff_a = abs(distance(
                get_child_position(&rig.joints[n - 1]),
                target_position,
            )); // 1.39
        } //1.40
    } //1.41
}

fn apply_orientational_constraints(
    joint_pi: &ChildTypes,
    joint_pi_minus_1: &ChildTypes,
    r: &Option<Rotor>,
) -> ChildTypes {
    /*
        Fabrik Algorithm 2
    */

    // 2.1 Check whether the rotor R is within the motion range bounds
    let within_the_bounds = true; //TODO solve
    if within_the_bounds {
        // 2.2
        //2.3 do nothing and exit
        return joint_pi_minus_1.clone();
    } else {
        //2.4
        //2.5 reorient the joint pi-1 in such a way that the rotor will be within the limits

        let mut modified_pi_minus_1 = joint_pi_minus_1.clone();

        return modified_pi_minus_1;
    } // 2.6
}

fn apply_rotational_constraints() {
    /*
        Fabrik Algorithm 3
    */
}

fn sum(values: &Vec<Tnum>) -> Tnum {
    let mut value = 0.0;
    for v in values.iter() {
        value += v;
    }

    return value;
}

fn square(value: Tnum) -> Tnum {
    return value * value;
}

fn sqrt(value: Tnum) -> Tnum {
    return value.sqrt();
}

fn distance(a: CbMatrix, b: CbMatrix) -> Tnum {
    let d_squared = square(a.x - b.x) + square(a.y - b.y);

    return sqrt(d_squared);
}

fn abs(value: Tnum) -> Tnum {
    if value < 0.0 {
        return -value;
    }

    return value;
}
