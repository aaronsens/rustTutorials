#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ball_move() {
        let mut ball = Ball::new();
        ball.move_ball();
        assert!(ball.x != 0 || ball.y != 0);
    }

    #[test]
    fn test_paddle_move_up() {
        let mut paddle = Paddle::new(10, 10);
        paddle.move_up();
        assert_eq!(paddle.y, 9);
    }

    #[test]
    fn test_paddle_move_down() {
        let mut paddle = Paddle::new(10, 10);
        paddle.move_down();
        assert_eq!(paddle.y, 11);
    }

    #[test]
    fn test_score_reset() {
        let mut score = Score::new();
        score.increment_score(Player::One);
        score.reset_score();
        assert_eq!(score.player1_score, 0);
        assert_eq!(score.player2_score, 0);
    }
}
