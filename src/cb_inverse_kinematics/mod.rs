extern crate nalgebra as na;
use na::Vector2;

use crate::cb_math::sqrt_f32;

const UNIT_VALUE: i32 = 100000; // The number of values equal to 1.0 or 360*

extern crate fixed;
use fixed::types::I24F8;
use fixed::FixedI32;

pub type CbMatrix = na::Vector2<i32>; // Note: using a self aliased type here for 2d/3d implementations. First, get 2d working, then can easily export that to use 3d

/*
Based off of:
https://www.gamasutra.com/blogs/LuisBermudez/20170804/303066/3_Simple_Steps_to_Implement_Inverse_Kinematics.php

Assuming all the joints are revolute, then O is a pose vector which represents the initial orientation of every joint.
T is the pose vector which represents the final orientation of every joint, such that the end effector reaches its target position.
dO is the vector which represents the change in orientation for each joint, such that the articulated body reaches T from O.
For example, O would be (45°, 15°, -60°) in the Figure below.



T = O + dO


*/

#[derive(Clone)]
pub struct IkRig {
    pub target: Option<CbMatrix>,
    pub segments: Vec<CbMatrix>,
    pub bones: Vec<Bone>,
    pub position: CbMatrix,
    root_joint: Joint,
}

impl IkRig {
    pub fn new() -> Self {
        return Self {
            target: None,
            bones: vec![Bone::new()],
            position: CbMatrix::new(0, 0),
            segments: vec![
                CbMatrix::new(100, 100),
                CbMatrix::new(200, 100),
                CbMatrix::new(300, 100),
            ],
            root_joint: Joint::new(),
        };
    }
}

#[derive(Clone)]
pub struct Bone {
    length: u32,
    end_joint: Joint,
}

impl Bone {
    pub fn new() -> Self {
        unimplemented!();
    }

    pub fn get_local_end_position(&self) -> CbMatrix {
        unimplemented!();
    }

    pub fn get_root_bone(&self) -> Option<Bone> {
        unimplemented!();
    }
}

#[derive(Clone)]
pub struct Joint {
    current_position: CbMatrix,
    current_angle: CbMatrix,
    max_angle: CbMatrix,
    children: Vec<Bone>,
}

impl Joint {
    pub fn new() -> Self {
        unimplemented!();
    }
}

pub fn fabrik(rig: &mut IkRig) {
    // Boundary condition checks
    {
        // Errors if only 1 bone in chain, as the following code uses direct indexing. Need to add checks in algorithm to solve.
    }

    let mut joint_positions: Vec<CbMatrix> = Vec::<CbMatrix>::new();
    let joint_pair_distances = vec![0]; // Note: need to populate. Will be of size N-1, where N is number of positions (since it's distances per joint pairs)
    let target_position = CbMatrix::new(0, 0);
    /*
        Right now just going for single IK chain, will update it as time goes
    */

    let n = joint_positions.len();
    // Note, in the algorithm, it indexes from 1 to N. Rust, however, indexes from 0..N, so not much actually changes. Just pointing it out. May be a few instances where the index
    // in code looks different from the algorithm; this is expected.

    // Base algorithm found on: http://www.andreasaristidou.com/publications/papers/FABRIK.pdf

    //1.1 Distance between root and target
    let dist: i32 = abs(distance(rig.root_joint.current_position, target_position)); // 1.2
                                                                                     //1.3 Check whether target is in reach
    if dist > sum(&joint_pair_distances) {
        // 1.4
        //1.5 Target is unreachable
        for i in 0..(n - 1) {
            // 1.6
            // 1.7 Find the distance ri between target t and joint position pi
            let ri = abs(distance(target_position, joint_positions[i])); //1.8
            let lambda_i = joint_pair_distances[i] / ri; //1.9
                                                         //1.10 Find new joint positions pi
            joint_positions[i + 1] =
                (1 - lambda_i) * joint_positions[i] + lambda_i * target_position;
            //1.11
        } // 1.12
    } else {
        //1.13
        //1.14 target is reachable, set b as the initial position of p0
        let b = joint_positions[0]; // 1.15
                                    //1.16 Check whether distance between end effector pn and the target is greater than a tolerance
        let mut diff_a = abs(distance(joint_positions[n - 1], target_position)); //1.17
        let tolerance = 1; // TODO: figure out
        while diff_a > tolerance {
            //1.18
            //1.19 Stage 1: Forward reaching
            //1.20 Set end effector pn as target t
            joint_positions[n - 1] = target_position; //1.21

            //note: skip the last value, as it's being handled elsewhere
            for i in (0..n - 1).rev() {
                //1.22
                //1.23 Find the distance ri between the new joint position pi+1 and the joint pi
                let ri = abs(distance(joint_positions[i + 1], joint_positions[i])); //1.24
                let lambda_i = joint_pair_distances[i] / ri; //1.25
                                                             //1.26 Find the new joint positions pi
                joint_positions[i] =
                    (1 - lambda_i) * joint_positions[i + 1] + lambda_i * joint_positions[i];
                //1.27
            } // 1.28
              //1.29 Stage 2: Backwards reaching
              //1.30 Set the root p0 it's initial position
            joint_positions[0] = b; //1.31

            for i in 0..n - 1 {
                //1.32
                //1.33 Find the distance ri between the new joint position pi and the joint pi+1
                let ri = abs(distance(joint_positions[i + 1], joint_positions[i])); //1.34
                let lambda_i = joint_pair_distances[i] / ri; //1.35
                                                             //1.36 Find the new joint positions pi
                joint_positions[i] =
                    (1 - lambda_i) * joint_positions[i] + lambda_i * joint_positions[i + 1];
                //1.37
            } //1.38
            diff_a = abs(distance(joint_positions[n - 1], target_position)); // 1.39
        } //1.40
    } //1.41
}

fn sum(values: &Vec<i32>) -> i32 {
    let mut value = 0;
    for v in values.iter() {
        value += v;
    }

    return value;
}

fn distance(vector_a: CbMatrix, vector_b: CbMatrix) -> i32 {
    unimplemented!();
}

fn abs<T>(value: T) -> T {
    unimplemented!();
}
/*
/// Returns a new head and new tail, where 'new_head' has been moved to the target
pub fn fabrik_reach_2d(head: CbMatrix, tail: CbMatrix, target: CbMatrix) -> (CbMatrix, CbMatrix) {
    // returns new head and tail in the format of:
    //   [new_head, new_tail]
    // where `new_head` has been moved to `tgt`

    // calculate the current length
    // (in practice, this should be calculated once and saved,
    //  not re-calculated every time `reach` is called)
    /*
    let c_dx = tail.x - head.x;
    let c_dy = tail.y - head.y;
    let c_dist = Math.sqrt(c_dx * c_dx + c_dy * c_dy);

    // calculate the stretched length
    let s_dx = tail.x - target.x;
    let s_dy = tail.y - target.y;
    let s_dist = Math.sqrt(s_dx * s_dx + s_dy * s_dy);

    // calculate how much to scale the stretched line
    let scale = c_dist / s_dist;

    // return the result
    return [
      // copy the target for the new head
      { x: target.x, y: target.y },

      // scale the new tail based on distance from target
      { x: target.x + s_dx * scale, y: target.y + s_dy * scale }
    ];
    */

    // Note: Converted to use powers instead of sqrt, as it's less computationally expensive
    // However, need to cast up. May not be worth it?
    let c_dx = (tail.x - head.x) as f32;
    let c_dy = (tail.y - head.y) as f32;
    let c_dist = sqrt_f32(c_dx * c_dx + c_dy * c_dy);

    // calculate the stretched length
    let s_dx = (tail.x - target.x) as f32;
    let s_dy = (tail.y - target.y) as f32;
    let s_dist = sqrt_f32((s_dx * s_dx + s_dy * s_dy));

    // calculate how much to scale the stretched line
    let scale = c_dist / s_dist;

    let new_head = CbMatrix::new(target.x, target.y);
    let new_tail = CbMatrix::new(
        target.x + (s_dx as f32 * scale) as i32,
        target.y + (s_dy as f32 * scale) as i32,
    );
    return (new_head, new_tail);
}

pub fn fabrik_execute(rig: &mut IkRig) {
    if rig.target.is_none() {
        return;
    }

    if rig.bones.is_empty() {
        return;
    }

    // Note: dumb implementation for a single bone
    let rig_pos = rig.position;
    for bone in rig.bones.iter_mut() {
        let bone_start_position = rig_pos;
        let target = rig.target.unwrap();
        bone.end_position = target;

        let actual_len = bone.length;

        /*

        */

        continue;

        // Determine how much to scale endposition down
        let actual_len = bone.length;
        let target_xdiff = target.x + bone_start_position.x;
        let target_xdiff = (target_xdiff * target_xdiff) as u32;

        let target_ydiff = target.y + bone_start_position.y;
        let target_ydiff = (target_ydiff * target_ydiff) as u32;

        // Operating without square roots, as it's more performant?
        let sqrd_target_len = target_xdiff + target_ydiff;
        let sqrd_actual_len = actual_len * actual_len;

        // Note; May need to handle dividing by zero?

        if sqrd_actual_len < sqrd_target_len {
            // Divide
            let ratio = sqrd_actual_len / sqrd_target_len;
            if ratio == 0 {
                return;
            }
            bone.end_position.x = bone.end_position.x / (ratio as i32);
            bone.end_position.y = bone.end_position.y / (ratio as i32);
        } else {
            // Multiply
            let ratio = sqrd_actual_len / sqrd_target_len;
            bone.end_position.x = bone.end_position.x * (ratio as i32);
            bone.end_position.y = bone.end_position.y * (ratio as i32);
        }

        let ratio = sqrd_actual_len / sqrd_target_len;
        println!("ration: {}", ratio);

        if ratio == 0 {
            return;
        }

        bone.end_position.x = bone.end_position.x / (ratio as i32);
        bone.end_position.y = bone.end_position.y / (ratio as i32);
    }
}

pub fn fabrik_execute_old(rig: &mut IkRig) {
    return;

    if rig.target.is_none() {
        return;
    }

    if rig.segments.is_empty() {
        return;
    }

    let mut target = rig.target.unwrap().clone();

    // Need at least two segments to run, otherwise indexing issues will occur
    let segment_len = rig.segments.len();
    let can_run = segment_len >= 2;

    let base = rig.segments[segment_len - 1].clone();

    // Forward iterations
    {
        if can_run {
            for i in 0..(rig.segments.len() - 1) {
                let (new_head, new_tail) =
                    fabrik_reach_2d(rig.segments[i], rig.segments[i + 1], target);

                rig.segments[i] = new_head;
                target = new_tail;
            }
        }

        // Adjust the last segment to equal the target
        rig.segments[segment_len - 1] = target;
    }

    // Backward iterations
    {
        target = base;

        if can_run {
            for i in segment_len - 1..0 {
                let (new_head, new_tail) =
                    fabrik_reach_2d(rig.segments[i], rig.segments[i - 1], target);

                rig.segments[i] = new_head;
                target = new_tail;
            }
        }

        // Adjust the last segment to equal the target
        rig.segments[0] = target;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // new tests
    #[test]
    fn IkRig_Fabrik2d_Fails() {
        let mut rig = IkRig {
            target: Some(CbMatrix::new(1032, 656)),
            segments: vec![
                CbMatrix::new(100, 100),
                CbMatrix::new(200, 200),
                CbMatrix::new(300, 300),
            ],
        };

        let old_rig = rig.clone();

        fabrik_execute(&mut rig);

        assert_eq!(old_rig.segments, rig.segments);
    }
}
*/
