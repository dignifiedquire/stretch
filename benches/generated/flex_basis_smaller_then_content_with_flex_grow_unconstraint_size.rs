pub fn compute() {
    let mut stretch = stretch::Stretch::new();
    let node00: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(70f32),
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node0: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                flex_direction: stretch::style::FlexDirection::Column,
                flex_grow: 1f32,
                flex_basis: stretch::style::Dimension::Points(0f32),
                ..Default::default()
            },
            vec![node00],
        )
        .unwrap();
    let node10: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(20f32),
                    height: stretch::style::Dimension::Points(100f32),
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
                flex_direction: stretch::style::FlexDirection::Column,
                flex_grow: 1f32,
                flex_basis: stretch::style::Dimension::Points(0f32),
                ..Default::default()
            },
            vec![node10],
        )
        .unwrap();
    let node: stretch::node::StretchNode =
        stretch.new_node(stretch::style::Style { ..Default::default() }, vec![node0, node1]).unwrap();
    stretch.compute_layout(&node, stretch::geometry::Size::undefined()).unwrap();
}
