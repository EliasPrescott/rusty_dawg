use std::collections::{HashMap, HashSet};

use crate::utilities;

struct PackedDawgBuilder {
    previous_word: Vec<char>,
    nodes: Vec<bool>,
    minimized_nodes: HashSet<usize>,
    unchecked_nodes: Vec<(usize, char, usize)>,
    edges: HashMap<(usize, char), usize>,
}

pub struct PackedDawg {
    nodes: Vec<bool>,
    edges: HashMap<(usize, char), usize>,
}

impl PackedDawgBuilder {
    fn minimize(&mut self, down_to: usize) {
        for _ in (down_to..self.unchecked_nodes.len()).rev() {
            let (parent_id, letter, child_id) = self.unchecked_nodes.pop().unwrap();
            if self.minimized_nodes.contains(&child_id) {
                self.edges.insert((parent_id, letter), child_id);
            } else {
                self.minimized_nodes.insert(child_id);
            }
        }
    }

    pub fn insert_word(&mut self, word: Vec<char>) {
        let common_prefix = utilities::common_prefix(&word, &self.previous_word);
        self.minimize(common_prefix);
        let mut node_id = if self.unchecked_nodes.len() == 0 {
            0
        } else {
            self.unchecked_nodes[self.unchecked_nodes.len() - 1].2
        };
        for letter in &word[common_prefix..] {
            let next_node_id = self.nodes.len();
            self.nodes.push(false);
            self.edges.insert((node_id, *letter), next_node_id);
            self.unchecked_nodes.push((node_id, *letter, next_node_id));
            node_id = next_node_id;
        }
        self.nodes[node_id] = true;
        self.previous_word = word;
    }

    pub fn finish(mut self) -> PackedDawg {
        self.minimize(0);
        PackedDawg {
            nodes: self.nodes,
            edges: self.edges,
        }
    }
}

impl PackedDawg {
    pub fn from_words(words: &Vec<&str>) -> Self {
        let word_count = words.len();
        // When testing with a +200,000 word dictionary, pre-initializing all vecs keeps the init time close to 130-150ms
        let mut nodes = Vec::with_capacity(word_count * 4);
        nodes.push(false);
        let mut builder = PackedDawgBuilder {
            edges: HashMap::with_capacity(word_count),
            minimized_nodes: HashSet::with_capacity(word_count),
            nodes,
            previous_word: Vec::with_capacity(50),
            unchecked_nodes: Vec::with_capacity(50),
        };
        for word in words {
            let word = utilities::prep_word(word);
            builder.insert_word(word);
        }
        builder.finish()
    }

    pub fn lookup(&self, word: &str) -> Option<usize> {
        let word = utilities::prep_word(word);
        let mut node_id = 0;
        for letter in word {
            if let Some(next_node_id) = self.edges.get(&(node_id, letter)) {
                node_id = *next_node_id
            } else {
                return None;
            }
        }
        Some(node_id)
    }

    pub fn has_word(&self, word: &str) -> bool {
        self.lookup(word)
            .map(|node_id| self.nodes[node_id])
            .unwrap_or(false)
    }

    fn search_recursive(
        &self,
        node_index: usize,
        letter: char,
        search_word: &Vec<char>,
        mut dawg_word: Vec<char>,
        previous_row: &Vec<u32>,
        results: &mut Vec<(String, u32)>,
        max_cost: u32,
    ) {
        dawg_word.push(letter);
        let mut current_row = vec![previous_row[0] + 1];
        for column in 1..=search_word.len() {
            let insert_cost = current_row[column - 1] + 1;
            let delete_cost = previous_row[column] + 1;
            let replace_cost = if search_word[column - 1] != letter {
                previous_row[column - 1] + 1
            } else {
                previous_row[column - 1]
            };
            current_row.push(insert_cost.min(delete_cost).min(replace_cost));
        }

        if current_row.last().unwrap() <= &max_cost && self.nodes[node_index] {
            results.push((dawg_word.iter().collect(), *current_row.last().unwrap()));
        }

        if current_row.iter().min().unwrap() <= &max_cost {
            for letter in 'a'..='z' {
                if let Some(next_node_index) = self.edges.get(&(node_index, letter)) {
                    self.search_recursive(
                        *next_node_index,
                        letter,
                        search_word,
                        dawg_word.clone(),
                        &current_row,
                        results,
                        max_cost,
                    );
                }
            }
        }
    }

    pub fn search(&self, word: &str, max_cost: u32) -> Vec<(String, u32)> {
        let word = utilities::prep_word(word);
        let mut results = vec![];
        let starting_row = (0..(word.len() as u32 + 1)).collect();

        // I would love to find a way around iterating through all possible characters, but I think it might be a necessary tradeoff that comes from minimizing the memory footprint by packing everything into vecs/hashmaps
        // Maybe some numeric property on the nodes with enough bits to track edge connections to all possible characters would work, but that would significantly increase memory usage for large dictionaries.
        for letter in 'a'..='z' {
            if let Some(next_node_index) = self.edges.get(&(0, letter)) {
                self.search_recursive(
                    *next_node_index,
                    letter,
                    &word,
                    vec![],
                    &starting_row,
                    &mut results,
                    max_cost,
                )
            }
        }

        results
    }
}
