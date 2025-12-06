    let sample = Card {
        id: 1,
        deck_id: 1,
        created_at: 0,
        times_seen: 0,
        times_correct: 0,
        tags: None,
        name: "myfirst_card".into(),

        front_blocks: vec![
            Block::Text { value: "What is the derivative of sin(x)?".into() },
            Block::Math { value: r#"\frac{d}{dx}\sin(x) = \cos(x)"#.into() },
            Block::Text { value: "Final note: derivative is periodic.".into() }
        ],
        back_blocks: vec![
            Block::Text { value: "The answer is 42".into() }
        ],
    };