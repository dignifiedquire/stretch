pub fn compute() {
    let mut stretch = stretch::Stretch::new();
    let node000: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node00: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                flex_direction: stretch::style::FlexDirection::Column,
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![node000],
        )
        .unwrap();
    let node0: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                flex_wrap: stretch::style::FlexWrap::Wrap,
                align_items: stretch::style::AlignItems::FlexStart,
                ..Default::default()
            },
            vec![node00],
        )
        .unwrap();
    let node1: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(100f32),
                    height: stretch::style::Dimension::Points(100f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            vec![],
        )
        .unwrap();
    let node: stretch::node::StretchNode = stretch
        .new_node(
            stretch::style::Style { flex_direction: stretch::style::FlexDirection::Column, ..Default::default() },
            vec![node0, node1],
        )
        .unwrap();
    stretch.compute_layout(&node, stretch::geometry::Size::undefined()).unwrap();
}
