pub fn compute() {
    let mut stretch = stretch::Stretch::new();
    let node0: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(50f32),
                    height: stretch::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    top: stretch::style::Dimension::Auto,
                    bottom: stretch::style::Dimension::Auto,
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node1: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(50f32),
                    height: stretch::style::Dimension::Points(50f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                flex_direction: stretch::style::FlexDirection::Column,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(200f32),
                    height: stretch::style::Dimension::Points(200f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![node0, node1],
        )
        .unwrap();
    stretch.compute_layout(&node, stretch::geometry::Size::undefined()).unwrap();
}
