pub fn compute() {
    let mut stretch = stretch::Stretch::new();
    let node00: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(50f32),
                    height: stretch::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    start: stretch::style::Dimension::Points(10f32),
                    end: stretch::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node0: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style { align_items: stretch::style::AlignItems::Center, ..Default::default() },
            vec![node00],
        )
        .unwrap();
    let node: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                align_items: stretch::style::AlignItems::Center,
                justify_content: stretch::style::JustifyContent::Center,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(50f32),
                    height: stretch::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![node0],
        )
        .unwrap();
    stretch.compute_layout(&node, stretch::geometry::Size::undefined()).unwrap();
}
