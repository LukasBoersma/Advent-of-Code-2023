/// Advent of Code 2023 - Day 24
/// https://adventofcode.com/2023/day/24
/// 
/// Oof! By far the hardest puzzle. The input specifies 3D positions and
/// velocities of "hailstones" (at t=0).
/// The hailstones are moving in straight lines, changing their position by
/// their velocity in each time step.
/// 
/// Part 1 asks for the number of hailstone pairs with intersecting trajectories
/// inside a given test area.
/// Implementation is just simple intersection tests. Took the chance to get
/// my old "real-time rendering" book out of the basement :)
/// 
/// Part 2 is the tricky one: It asks for the trajectory of a single
/// rock that will hit all the hailstones. It is not enough to just intersect
/// the hailstone paths, the rock needs to be at the intersection point at the
/// same time as the hailstone to actually hit them.
/// 
/// My solution is to brute-force the x and y component of the rock's velocity
/// and then first solve the problem in 2D. When a 2D solution is found, I can
/// derive the remaining Z axis from the 2D intersection point and the time.
/// There are probably easier ways to solve this, but I already spent many
/// nights thinking about a solution and this is the best I could come up with.
/// I know that I could probably solve this algebraically from three hailstone
/// trajectories or so, but I couldn't figure it out.
/// 
/// This one is not pretty, but it works, runs in under 208ms, and my brain
/// can rest :)


use core::panic;

use crate::utils::*;
use crate::vec2_128::Vec2L;
use crate::vec3_128::Vec3L;

/// Hailstones are rays,
/// defined by a position and a velocity vector
type Ray = (Vec3L, Vec3L);

/// Evaluates a hailstone position for the given time
fn eval_ray(ray: Ray, t: i128) -> Vec3L {
    ray.0 + ray.1 * t
}

/// Parses a hailstone ray.
/// (example: "19, 13, 30 @ -2,  1, -2")
fn parse_line(line: &str) -> Ray {
    let items = line
        .split(|c| c == ',' || c == '@')
        .map(|part| part.trim().parse::<i128>().unwrap())
        .vec();
    (items[0..3].into(), items[3..6].into())
}

/// Parses the list of hailstones
fn parse(input: &str) -> Vec<Ray> {
    input.lines().map(parse_line).vec()
}

/// Tests if two rays intersect in the xy plane.
/// Returns the intersection point if they do.
fn ray_intersection_2d((ap, ad): (Vec2L, Vec2L), (bp, bd): (Vec2L, Vec2L)) -> Option<Vec2L> {
    
    let d = bp - ap;
    let det = bd.cross(ad);

    if det == 0 {
        None
    } else {
        let u = (d.y() * bd.x() - d.x() * bd.y()) as f64 / det as f64;
        let v = (d.y() * ad.x() - d.x() * ad.y()) as f64 / det as f64;
        if u < 0.0 || v < 0.0 {
            None
        } else {
            let offset = Vec2L(
                (ad.x() as f64 * u).round() as i128,
                (ad.y() as f64 * u).round() as i128,
            );
            Some(ap + offset)
        }
    }
}

/// Tests if two rays intersect in 3D space
fn intersect_ray_3d((p1, d1): Ray, (p2, d2): Ray) -> bool {
    // We want to find a time t when the two rays intersect.
    // Solving this equation for t:
    // p1 + d1 * t = p2 + d2 * t
    // => p1 - p2 = (d2 - d1) * t
    // => t = (p1 - p2) / (d2 - d1)

    let pd = p1 - p2;
    let dd = d2 - d1;

    // Try to find one axis where we can compute t = (p1 - p2) / (d2 - d1)
    // (we only need one axis, but if d2 - d1 is zero on that axis we can't use it)
    let maybe_t =
        pd.x.checked_div(dd.x)
        .or(pd.y.checked_div(dd.y))
        .or(pd.z.checked_div(dd.z));

    // If we found a t, check if it is valid (i.e. that t is not in the past and
    // that the rays really do intersect at that point)
    if let Some(t) = maybe_t {
        t >= 0
        && p1 + t * d1 == p2 + t * d2
    } else {
        false
    }
}

/// Finds the rock trajectory that hits all hailstones.
/// Returns the position of the rock at t=0.
fn solve_rock_trajectory(rays: Vec<(Vec3L, Vec3L)>) -> Vec3L {
    // Iterate over all possible velocity vectors of the rock.
    // Start by looking only at x/y coordinates.
    for x in -1000..1000 {
        for y in -1000..1000 {
            let rock_velocity = Vec2L(x, y);
            // We change the frame of reference so that the rock is not moving.
            // This is done by subtracting the rock velocity from all the ray velocities.
            // We don't know the position of the rock, but we know that now all the
            // Rays must intersect at a single point. That point is the position of the rock at t=0.

            let mut shifted_rays = rays.iter()
                .enumerate()
                .map(|(i, &(p, d))| (i, (Vec2L(p.x, p.y), Vec2L(d.x, d.y) - rock_velocity)));

            let (_, ray1) = shifted_rays.next().unwrap();
            while let Some((ray2_index, ray2)) = shifted_rays.next() {
                // Get the intersection point of the first two rays
                if let Some(intersection_candidate) = ray_intersection_2d(ray1, ray2) {
                    if shifted_rays.all(|(_, ray)| match ray_intersection_2d(ray, ray1) { Some(intersection) => intersection == intersection_candidate, None => false }) {
                        // We found a solution, at least in the xy plane.
                        // To get the full 3D position, just check when a ray intersects our xy point and calculate its
                        // intersection = p0 + t * d0
                        // => t = (intersection - p0) / d0
                        let pd1 = intersection_candidate - ray1.0;
                        let t1 = pd1.x().checked_div(ray1.1.x()).or(pd1.y().checked_div(ray1.1.y())).unwrap();
                        let pd2 = intersection_candidate - ray2.0;
                        let t2 = pd2.x().checked_div(ray2.1.x()).or(pd2.y().checked_div(ray2.1.y())).unwrap();

                        let intersection1_3d = eval_ray(rays[0], t1);
                        let intersection2_3d = eval_ray(rays[ray2_index], t2);

                        let rock_velocity_3d = (intersection1_3d - intersection2_3d) / (t1 - t2);
                        let rock_position = intersection1_3d - rock_velocity_3d * t1;
                        return rock_position;
                    }
                    break;
                }
            }
        }
    }

    panic!("No solution found");
}

fn count_collisions_in_area(stones: Vec<Ray>, test_area_min: Vec2L, test_area_max: Vec2L) -> I {

    // Only look at the xy plane
    let stones_2d = stones.map(|&(pos, dir)| (pos.xy(), dir.xy())).vec();

    // Iterate over all hailstone pairs
    stones_2d.iter().tuple_combinations()
        // Find the ones that intersect
        .map(|(&a, &b)| ray_intersection_2d(a, b))
        .flatten()
        // Find the intersections that are inside the test area
        .filter(|p| {
            p.x() >= test_area_min.x() && p.x() <= test_area_max.x() &&
            p.y() >= test_area_min.y() && p.y() <= test_area_max.y()
        })
        .count() as I
}

/// Part 1: Find the number of hailstone pairs with intersecting trajectories
/// inside the 2D test area.
pub fn part1(input: &str) -> I {
    let stones = parse(input);
    let test_area_min = Vec2L(200000000000000, 200000000000000);
    let test_area_max = Vec2L(400000000000000, 400000000000000);
    count_collisions_in_area(stones, test_area_min, test_area_max)
}


pub fn part2(input: &str) -> I  {
    let rays = parse(input);
    let center_position = Vec3L::new(
        rays.iter().map(|ray| ray.0.x).sum::<i128>() / rays.len() as i128,
        rays.iter().map(|ray| ray.0.y).sum::<i128>() / rays.len() as i128,
        rays.iter().map(|ray| ray.0.z).sum::<i128>() / rays.len() as i128,
    );

    let rays = rays.iter().map(|ray| (ray.0 - center_position, ray.1)).vec();

    let rock_start = solve_rock_trajectory(rays) + center_position;
    (rock_start.x + rock_start.y + rock_start.z) as I
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "\
        19, 13, 30 @ -2,  1, -2
        18, 19, 22 @ -1, -1, -2
        20, 25, 34 @ -2, -2, -4
        12, 31, 28 @ -1, -2, -1
        20, 19, 15 @  1, -5, -3";

        assert_eq!(count_collisions_in_area(parse(input), Vec2L(7, 7), Vec2L(27, 27)), 2);
    }

    #[test]
    fn test_intersection_points() {
        {
            let a = (Vec3L::new(19, 13, 30).xy(), Vec3L::new(-2, 1, -2).xy());
            let b = (Vec3L::new(18, 19, 22).xy(), Vec3L::new(-1, -1, -2).xy());
            assert_eq!(ray_intersection_2d(a, b), Some(Vec2L(14, 15)));
        }
        {
            let a = (Vec3L::new(19, 13, 30).xy(), Vec3L::new(-2, 1, -2).xy());
            let b = (Vec3L::new(20, 19, 15).xy(), Vec3L::new(1, -5, -3).xy());
            assert_eq!(ray_intersection_2d(a, b), None);
        }
        {
            let a = (Vec3L::new(20, 25, 34).xy(), Vec3L::new(-2, -2, -4).xy());
            let b = (Vec3L::new(18, 19, 22).xy(), Vec3L::new(-1, -1, -2).xy());
            assert_eq!(ray_intersection_2d(a, b), None);
        }
        {
            let a = (Vec3L::new(19, 13, 30).xy(), Vec3L::new(-2, 1, -2).xy());
            let b = (Vec3L::new(12, 31, 28).xy(), Vec3L::new(-1, -2, -1).xy());
            assert_eq!(ray_intersection_2d(a, b), Some(Vec2L(6, 19)));
        }
    }

    #[test]
    fn test_part2() {
        let input = "\
            19, 13, 30 @ -2,  1, -2
            18, 19, 22 @ -1, -1, -2
            20, 25, 34 @ -2, -2, -4
            12, 31, 28 @ -1, -2, -1
            20, 19, 15 @  1, -5, -3";

        let rays = parse(input);

        assert_eq!(solve_rock_trajectory(rays), Vec3L::new(24, 13, 10));
    }

    #[test]
    fn test_part2_1() {
        let input = "\
            0,0,-1 @ 0,0,1
            3,0,0 @ -1,0,0
            2,3,0 @ 0,-1,0
            3,-4,-8 @ 0,1,2";

        let rays = parse(input);

        assert_eq!(solve_rock_trajectory(rays), Vec3L::new(-1,0,0));
    }
}