mod player;
use player::Player;

pub fn sort_by_rating(players: &mut [Player]) -> &[Player] {
    players.sort_by_key(|player| player.rating);
    players
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_by_rating() {
        let mut players = vec![
            Player { name: "Alice".to_string(), rating: 10 },
            Player { name: "Bob".to_string(), rating: 5 },
            Player { name: "Charlie".to_string(), rating: 8 },
        ];

        sort_by_rating(&mut players);

        assert_eq!(
            players,
            vec![
                Player { name: "Bob".to_string(), rating: 5 },
                Player { name: "Charlie".to_string(), rating: 8 },
                Player { name: "Alice".to_string(), rating: 10 },
            ]
        );
    }
}