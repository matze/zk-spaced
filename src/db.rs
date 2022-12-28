use crate::Card;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use time::ext::NumericalDuration;
use time::{Duration, OffsetDateTime};

pub struct Database {
    path: PathBuf,
    state: HashMap<String, CardState>,
    cards: HashMap<String, Card>,
}

pub struct Review<'a> {
    pub card: &'a Card,
    pub state: &'a mut CardState,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CardState {
    last_reviewed: OffsetDateTime,
    num_recalled: u32,
    easiness_factor: f64,
    interval: Duration,
    failed: bool,
}

/// Write the database.
pub fn write(db: &Database) -> Result<()> {
    serde_json::to_writer(File::create(&db.path)?, &db.state)?;
    Ok(())
}

impl Database {
    /// Create new database and populate it if `path` exists.
    pub fn open(path: PathBuf, now: OffsetDateTime, candidates: Vec<Card>) -> Result<Self> {
        let mut state: HashMap<String, CardState> = path
            .exists()
            .then(|| {
                let file = File::open(&path)?;
                Ok::<_, anyhow::Error>(serde_json::from_reader(file)?)
            })
            .transpose()?
            .unwrap_or_default();

        let mut cards = HashMap::<String, Card>::default();

        for card in candidates {
            if !state.contains_key(&card.filename) {
                state.insert(card.filename.clone(), CardState::new(&now));
            }

            cards.insert(card.filename.clone(), card);
        }

        Ok(Self { path, state, cards })
    }

    /// Find review candidate.
    pub fn candidate(&mut self, now: &OffsetDateTime) -> Option<Review> {
        let name = self.cards.iter().find_map(|(name, _)| {
            self.state
                .get(name)
                .and_then(|state| state.needs_review(now).then_some(name))
        });

        name.map(|name| Review {
            card: self.cards.get(name).unwrap(),
            state: self.state.get_mut(name).unwrap(),
        })
    }
}

impl CardState {
    fn new(now: &OffsetDateTime) -> Self {
        Self {
            last_reviewed: *now,
            num_recalled: 0,
            easiness_factor: 2.5,
            interval: 0.days(),
            failed: true,
        }
    }
}

impl CardState {
    /// Update the card state according to the SM-2 algorithm.
    pub fn update(&mut self, grade: u16) {
        if grade >= 3 {
            match self.num_recalled {
                0 => self.interval = 1.days(),
                1 => self.interval = 6.days(),
                _ => {
                    let days = self.interval.whole_days() as f64;
                    let new_interval = (days * self.easiness_factor).round() as i64;
                    self.interval = new_interval.days();
                }
            }

            if self.num_recalled == 0 {
                self.interval = 1.days();
            }

            self.num_recalled += 1;
            self.failed = false;
        } else {
            self.num_recalled = 0;
            self.interval = 1.days();
            self.failed = true;
        }

        let g = 5.0 - f64::from(grade);
        self.easiness_factor = (self.easiness_factor + (0.1 - g * (0.08 + g * 0.02))).min(1.3);
    }

    /// Check if the card needs a review now.
    fn needs_review(&self, now: &OffsetDateTime) -> bool {
        self.last_reviewed + self.interval <= *now || self.failed
    }
}
