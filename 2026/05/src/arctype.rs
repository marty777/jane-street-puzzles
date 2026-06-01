use std::collections::{HashMap,HashSet};
use crate::dir::Dir;
use crate::coord::Coord;

/// Enum representing the four orientations of arcs.
/// 
/// - A - Top left to bottom right, with the bottom-left area larger
/// - B - Top left to bottom right, with the top-right area larger
/// - C - Bottom left to top right, with the bottom-right area larger
/// - D - Bottom left to top right, with the top-left area larger
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum ArcType {
    A, 
    B,
    C,
    D,
}
impl std::fmt::Display for ArcType {
    fn fmt(&self, f: &mut  std::fmt::Formatter) ->  std::fmt::Result {
        let char = match self {
            ArcType::A => "◝",
            ArcType::B => "◟",
            ArcType::C => "◜",
            ArcType::D => "◞",      
        };
        write!(f,"{}", char)
    }
}
impl ArcType {
    /// Return the endpoint coordinates, endpoint directions and endpoint-
    /// neighboring cells of a given arc. Neighboring cells outside the bounds
    /// of the grid are omitted.
    pub fn endpoints(t:&ArcType, coord:&Coord, dim:&Coord) -> HashMap<Coord, (Dir,HashSet<Coord>)> {
        let mut result = HashMap::new();
        let endpoint1:Coord;
        let endpoint2:Coord;
        let endpoint1_dir:Dir;
        let endpoint2_dir:Dir;
        let mut endpoint1_neighbors:HashSet<Coord> = HashSet::new();
        let mut endpoint2_neighbors:HashSet<Coord> = HashSet::new();
        match t {
            ArcType::A => {
                endpoint1 = coord.add(0, 0);
                endpoint2 = coord.add(1,1);
                endpoint1_dir = Dir::W;
                endpoint2_dir = Dir::S;
                endpoint1_neighbors.insert(coord.add_dir(&Dir::N));
                endpoint1_neighbors.insert(coord.add_dir(&Dir::W));
                endpoint1_neighbors.insert(coord.add_dir(&Dir::N).add_dir(&Dir::W));
                endpoint2_neighbors.insert(coord.add_dir(&Dir::S));
                endpoint2_neighbors.insert(coord.add_dir(&Dir::E));
                endpoint2_neighbors.insert(coord.add_dir(&Dir::S).add_dir(&Dir::E));
            },
            ArcType::B => {
                endpoint1 = coord.add(0, 0);
                endpoint2 = coord.add(1,1);
                endpoint1_dir = Dir::N;
                endpoint2_dir = Dir::E;
                endpoint1_neighbors.insert(coord.add_dir(&Dir::N));
                endpoint1_neighbors.insert(coord.add_dir(&Dir::W));
                endpoint1_neighbors.insert(coord.add_dir(&Dir::N).add_dir(&Dir::W));
                endpoint2_neighbors.insert(coord.add_dir(&Dir::S));
                endpoint2_neighbors.insert(coord.add_dir(&Dir::E));
                endpoint2_neighbors.insert(coord.add_dir(&Dir::S).add_dir(&Dir::E));
            },
            ArcType::C => {
                endpoint1 = coord.add(0, 1);
                endpoint2 = coord.add(1,0);
                endpoint1_dir = Dir::S;
                endpoint2_dir = Dir::E;
                endpoint1_neighbors.insert(coord.add_dir(&Dir::S));
                endpoint1_neighbors.insert(coord.add_dir(&Dir::W));
                endpoint1_neighbors.insert(coord.add_dir(&Dir::S).add_dir(&Dir::W));
                endpoint2_neighbors.insert(coord.add_dir(&Dir::N));
                endpoint2_neighbors.insert(coord.add_dir(&Dir::E));
                endpoint2_neighbors.insert(coord.add_dir(&Dir::N).add_dir(&Dir::E));
            },
            ArcType::D => {
                endpoint1 = coord.add(0, 1);
                endpoint2 = coord.add(1,0);
                endpoint1_dir = Dir::W;
                endpoint2_dir = Dir::N;
                endpoint1_neighbors.insert(coord.add_dir(&Dir::S));
                endpoint1_neighbors.insert(coord.add_dir(&Dir::W));
                endpoint1_neighbors.insert(coord.add_dir(&Dir::S).add_dir(&Dir::W));
                endpoint2_neighbors.insert(coord.add_dir(&Dir::N));
                endpoint2_neighbors.insert(coord.add_dir(&Dir::E));
                endpoint2_neighbors.insert(coord.add_dir(&Dir::N).add_dir(&Dir::E));
            },
        }
        let mut endpoint1_neighbors_final = HashSet::new();
        let mut endpoint2_neighbors_final = HashSet::new();
        for neighbor in endpoint1_neighbors {
            if neighbor.in_bounds(dim) {
                endpoint1_neighbors_final.insert(neighbor);
            }
        }
        for neighbor in endpoint2_neighbors {
            if neighbor.in_bounds(dim) {
                endpoint2_neighbors_final.insert(neighbor);
            }
        }
        result.insert(endpoint1, (endpoint1_dir, endpoint1_neighbors_final));
        result.insert(endpoint2, (endpoint2_dir, endpoint2_neighbors_final));
        return result;
    }
    /// Return the neighboring cells on the interior direction of the arc of 
    /// type `t` at the given `coord`, omitting any outside the bounds of the 
    /// grid.
    pub fn inside_neighbors(t:&ArcType, coord:&Coord, dim:&Coord) -> HashSet<Coord> {
        let mut result = HashSet::new();
        let neighbor1:Coord;
        let neighbor2:Coord;
        match t {
            ArcType::A => {
                neighbor1 = coord.add_dir(&Dir::W);
                neighbor2 = coord.add_dir(&Dir::S);
            },
            ArcType::B => {
                neighbor1 = coord.add_dir(&Dir::N);
                neighbor2 = coord.add_dir(&Dir::E);
            },
            ArcType::C => {
                neighbor1 = coord.add_dir(&Dir::S);
                neighbor2 = coord.add_dir(&Dir::E);
            },
            ArcType::D => {
                neighbor1 = coord.add_dir(&Dir::N);
                neighbor2 = coord.add_dir(&Dir::W);
            }
        }
        if neighbor1.in_bounds(dim) {
            result.insert(neighbor1);
        }
        if neighbor2.in_bounds(dim) {
            result.insert(neighbor2);
        }
        return result;
    }
    /// Return the neighboring cells on the exterior direction of the arc of 
    /// type `t` at the given `coord`, omitting any outside the bounds of the 
    /// grid.
    pub fn outside_neighbors(t:&ArcType, coord:&Coord, dim:&Coord) -> HashSet<Coord> {
        let mut result = HashSet::new();
        let neighbor1:Coord;
        let neighbor2:Coord;
        match t {
            ArcType::A => {
                neighbor1 = coord.add_dir(&Dir::N);
                neighbor2 = coord.add_dir(&Dir::E);
            },
            ArcType::B => {
                neighbor1 = coord.add_dir(&Dir::S);
                neighbor2 = coord.add_dir(&Dir::W);
            },
            ArcType::C => {
                neighbor1 = coord.add_dir(&Dir::N);
                neighbor2 = coord.add_dir(&Dir::W);
            },
            ArcType::D => {
                neighbor1 = coord.add_dir(&Dir::S);
                neighbor2 = coord.add_dir(&Dir::E);
            }
        }
        if neighbor1.in_bounds(dim) {
            result.insert(neighbor1);
        }
        if neighbor2.in_bounds(dim) {
            result.insert(neighbor2);
        }
        return result;
    }
}