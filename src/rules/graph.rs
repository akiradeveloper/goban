use std::collections::HashSet;
use crate::pieces::stones::Stone;
use crate::pieces::goban::Goban;
use crate::pieces::stones::StoneColor;

impl Goban {
    ///
    /// Get all the groups of connected atari stones
    ///
    pub fn get_atari_stones(&self) -> Vec<HashSet<Stone>> {
        let atari_stones: Vec<Stone> = self
            // get all stones without liberties
            .get_stones().into_iter()
            .filter(|point| !self.has_liberties(point))
            .collect();

        self.get_strongly_connected_stones(atari_stones)
    }

    ///
    /// Get all the groups of connected atari stones of a color.
    /// if the color it's an undefined behaviour
    ///
    pub fn get_atari_stones_color(&self, color: StoneColor) ->
    Vec<HashSet<Stone>> {
        let atari_stones: Vec<Stone> = self
            .get_stones_by_color(&color).into_iter()
            // get all stones without liberties
            .filter(|point| !self.has_liberties(point))
            .collect();
        self.get_strongly_connected_stones(atari_stones)
    }

    ///
    /// Can get a group of stones and his neigboors with a bfs,
    /// works for Empty stones too.
    ///
    pub fn bfs(&self, point: &Stone) -> HashSet<Stone> {
        let mut explored: HashSet<Stone> = HashSet::new();
        explored.insert(point.clone());

        let mut to_explore: Vec<Stone> = self.get_neighbors(&point.coord)
            .into_iter()
            .filter(|p| p.color == point.color)
            .collect(); // Acquiring all the neighbors

        while let Some(point_to_explore) = to_explore.pop() { // exploring the graph
            explored.insert(point_to_explore);
            let neighbors: Vec<Stone> = self.get_neighbors(&point_to_explore.coord)
                .into_iter()
                .filter(|p| p.color == point.color && !explored.contains(p))
                .collect();
            for p in neighbors {
                to_explore.push(p);
            }
        }
        explored
    }

    ///
    /// Use a bfs to deduce the groups of connected stones.
    /// Get stones connected. [[x,y,z],[a,e,r]] exemple of return.
    ///
    pub fn get_strongly_connected_stones(&self, stones: Vec<Stone>) ->
    Vec<HashSet<Stone>> {
        let mut strongly_connected_stones: Vec<HashSet<Stone>> = Vec::new();
        for atari_stone in stones {
            // if the stone is already in a group of stones
            let is_handled = strongly_connected_stones
                .iter()
                .any(|set| set.contains(&atari_stone));

            if !is_handled {
                strongly_connected_stones.push(self.bfs(&atari_stone))
            }
        }
        strongly_connected_stones
    }
}